use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::{time, vlc_service, volume, Command, Credentials, Error, Meta, Printer, Volume};

pub struct VLCController<'a> {
    client: reqwest::Client,
    credentials: Credentials<'a>,
    printer: Printer,
    last_volume: Option<Volume>,
    last_commands_file_uri: Option<String>,
    last_commands: Vec<Command>,
}

impl<'a> VLCController<'a> {
    pub fn new(credentials: Credentials) -> VLCController {
        VLCController {
            client: reqwest::Client::new(),
            credentials,
            printer: Printer::new(),
            last_volume: None,
            last_commands_file_uri: None,
            last_commands: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let meta = match vlc_service::get_meta(&self.client, &self.credentials)? {
            Some(meta) => meta,
            None => {
                self.printer
                    .print_sticky_line("[info] No File is currently playing");
                return Ok(());
            }
        };
        let status = vlc_service::get_status(&self.client, &self.credentials)?;
        self.printer.print_sticky_line(&format!(
            "[info] Currently playing: '{}' ({} / {})",
            meta.name, status.time, meta.duration
        ));

        let actions = self.get_commands(&meta);
        for action in actions.iter() {
            match *action {
                Command::Skip { start, end } => {
                    if status.time >= start && status.time < end {
                        self.printer.print_line(&format!(
                            "[info] skipping {} seconds",
                            time::difference(end, start)
                        ));
                        vlc_service::seek_to(&self.client, &self.credentials, end)?;
                    }
                }
                Command::Mute { start, end } => {
                    if status.time == start && self.last_volume == None {
                        self.printer.print_line(&format!(
                            "[info] muting audio for {} seconds",
                            time::difference(end, start)
                        ));
                        vlc_service::set_volume(&self.client, &self.credentials, Volume::new(0))?;
                        self.last_volume = Some(status.volume);
                    } else if status.time == end {
                        if let Some(last_volume) = self.last_volume {
                            self.printer.print_line("[info] unmuting audio");
                            vlc_service::set_volume(&self.client, &self.credentials, last_volume)?;
                            self.last_volume = None;
                        }
                    }
                }
                Command::SetVolume { at, amount } => {
                    if status.time == at && volume::abs_difference(status.volume, amount) > 2 {
                        self.printer.print_line(&format!(
                            "[info] changing volume from {}% to {}%",
                            status.volume, amount
                        ));
                        vlc_service::set_volume(&self.client, &self.credentials, amount)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_commands(&mut self, meta: &Meta) -> Vec<Command> {
        if let Some(last_commands_file_uri) = &self.last_commands_file_uri {
            if meta.uri == *last_commands_file_uri {
                return self.last_commands.clone();
            }
        }

        let commands_file_path = &self.get_commands_file_path(&meta.uri);
        let commands = match self.read_commands(&commands_file_path) {
            Ok(commands) => {
                self.printer.print_line(&format!(
                    "[info] reading commands from '{}'",
                    commands_file_path
                ));
                Some(commands)
            }
            Err(Error::IO(e)) if e.kind() == io::ErrorKind::NotFound => {
                self.printer
                    .print_line(&format!("[err] No commands file found for '{}'", meta.name));
                None
            }
            Err(Error::YAML(e)) => {
                self.printer.print_line(&format!(
                    "[err] cannot parse commands file '{}', reason: {}",
                    commands_file_path, e
                ));
                None
            }
            Err(e) => panic!("{}", e),
        };

        self.last_commands_file_uri = Some(meta.uri.clone());
        self.last_commands = commands.unwrap_or(vec![]);
        self.last_commands.clone()
    }

    fn get_commands_file_path(&self, video_uri: &str) -> String {
        let path = Path::new(video_uri).with_extension("yml");
        let prefix = "file:///";
        path.to_str().unwrap()[prefix.len()..].to_string()
    }

    fn read_commands(&self, path: &str) -> Result<Vec<Command>, Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let actions: Vec<Command> = serde_yaml::from_str(&data)?;
        Ok(actions)
    }
}
