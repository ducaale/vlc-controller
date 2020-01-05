use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod printer;
mod time;
mod vlc_service;
mod volume;

use printer::Printer;
use time::Time;
use volume::Volume;

#[derive(Deserialize, Debug)]
pub struct Status {
    time: Time,
    #[serde(with = "volume")]
    volume: Volume,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    name: String,
    uri: String,
    duration: Time,
}

pub struct Credentials<'a> {
    user: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Command {
    Skip { start: Time, end: Time },
    Mute { start: Time, end: Time },
    SetVolume { amount: Volume, at: Time },
}

struct VLCController<'a> {
    client: reqwest::Client,
    credentials: Credentials<'a>,
    printer: Printer,
    last_volume: Option<Volume>,
    last_commands_file_uri: Option<String>,
    last_commands: Vec<Command>,
}

impl<'a> VLCController<'a> {
    fn new(credentials: Credentials) -> VLCController {
        VLCController {
            client: reqwest::Client::new(),
            credentials,
            printer: Printer::new(),
            last_volume: None,
            last_commands_file_uri: None,
            last_commands: vec![],
        }
    }

    fn run(&mut self) -> Result<(), reqwest::Error> {
        let meta = match vlc_service::get_meta(&self.client, &self.credentials)? {
            Some(meta) => meta,
            None => {
                self.printer
                    .print_sticky_line("[info] No File is currently playing");
                return Ok(());
            }
        };
        let status = vlc_service::get_status(&self.client, &self.credentials)?;
        let progress = format!(
            "[info] Currently playing: '{}' ({} / {})",
            meta.name, status.time, meta.duration
        );
        self.printer.print_sticky_line(&progress);

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
        self.last_commands_file_uri = Some(meta.uri.clone());

        let commands_file_path = &self.get_commands_file_path(&meta.uri);
        match self.read_commands(&commands_file_path) {
            Ok(commands) => {
                self.printer.print_line(&format!(
                    "[info] reading commands from '{}'",
                    commands_file_path
                ));
                self.last_commands = commands;
                self.last_commands.clone()
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    self.printer
                        .print_line(&format!("[err] No commands file found for '{}'", meta.name));
                    self.last_commands = vec![];
                    self.last_commands.clone()
                }
                _ => panic!("{}", e),
            },
        }
    }

    fn get_commands_file_path(&self, video_uri: &str) -> String {
        let path = Path::new(video_uri).with_extension("yml");
        let prefix = "file:///";
        path.to_str().unwrap()[prefix.len()..].to_string()
    }

    fn read_commands(&self, path: &str) -> io::Result<Vec<Command>> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let actions: Vec<Command> = serde_yaml::from_str(&data).unwrap();
        Ok(actions)
    }
}

fn main() -> Result<(), reqwest::Error> {
    let credentials = Credentials {
        user: "",
        password: "12345",
    };
    let mut vlc_controller = VLCController::new(credentials);
    loop {
        if let Err(e) = vlc_controller.run() {
            if e.is_http() && e.status() == None {
                println!("[err] Unable to connect to vlc");
            } else {
                return Err(e);
            }
        };
        sleep(Duration::from_millis(200));
    }
}
