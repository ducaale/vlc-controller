use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod time;
mod vlc_service;
mod printer;

use time::Time;
use printer::Printer;

#[derive(Deserialize, Debug)]
pub struct Status {
    time: Time
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    name: String,
    uri: String,
    duration: Time
}

pub struct Credentials<'a> {
    user: &'a str,
    password: &'a str
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Action {
    Skip { start: Time, end: Time },
    Mute { start: Time, end: Time },
    SetVolume { amount: u32, at: Time }
}

struct VLCController<'a> {
    client: reqwest::Client,
    credentials: Credentials<'a>,
    printer: Printer,
    last_commands_file_uri: Option<String>,
    last_commands: Vec<Action>
}

impl<'a> VLCController<'a> {
    fn new(client: reqwest::Client, credentials: Credentials<'a>) -> VLCController<'a> {
        VLCController {
            client,
            credentials,
            printer: Printer::new(),
            last_commands_file_uri: None,
            last_commands: vec![]
        }
    }

    fn run(&mut self) -> Result<(), reqwest::Error> {
        let meta = vlc_service::get_meta(&self.client, &self.credentials)?;
        let status = vlc_service::get_status(&self.client, &self.credentials)?;
        let progress = format!(
            "[info] Currently playing: '{}' ({} / {})",
            meta.name,
            status.time,
            meta.duration
        );
        self.printer.print_sticky_line(&progress);

        let actions = self.get_commands(&meta);
        for action in actions.iter() {
            match *action {
                Action::Skip { start, end } => {
                    if status.time >= start && status.time < end {
                        vlc_service::seek_to(&self.client, &self.credentials, end)?;
                    }
                },
                Action::SetVolume { at, amount } => {
                    if status.time == at {
                        vlc_service::set_volume(&self.client, &self.credentials, amount)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn get_commands(&mut self, meta: &Meta) -> Vec<Action> {
        if let Some(last_commands_file_uri) = &self.last_commands_file_uri {
            if meta.uri == *last_commands_file_uri {
                return self.last_commands.clone()
            }
        }
        self.last_commands_file_uri = Some(meta.uri.clone());

        let commads_file_path = &self.get_commands_file_path(&meta.uri);
        match self.read_commands(&commads_file_path) {
            Ok(commands) => {
                self.printer.print_line(&format!("[info] using commands in '{}'", commads_file_path));
                self.last_commands = commands;
                self.last_commands.clone()
            },
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    self.printer.print_line(&format!("[err] No commands file found for '{}'", meta.name));
                    self.last_commands = vec![];
                    self.last_commands.clone()
                },
                _ => panic!("{}", e)
            }
        }
    }

    fn get_commands_file_path(&self, video_uri: &str) -> String {
        let path = Path::new(video_uri).with_extension("yml");
        let prefix = "file:///";
        path.to_str().unwrap()[prefix.len()..].to_string()
    }

    fn read_commands(&self, path: &str) -> io::Result<Vec<Action>> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let actions: Vec<Action> = serde_yaml::from_str(&data).unwrap();
        Ok(actions)
    }
}

fn main() -> Result<(), reqwest::Error> {
    let credentials = Credentials { user: "", password: "12345" };
    let client = reqwest::Client::new();
    let mut vlc_controller = VLCController::new(client, credentials);
    loop {
        if let Err(e) = vlc_controller.run() {
            if e.is_http() && e.status() == None {
                println!("[err] Unable to connect to vlc");
            }
            else {
                return Err(e);
            }
        };
        sleep(Duration::from_millis(500));
    }
}