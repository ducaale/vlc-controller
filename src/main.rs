use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;

mod printer;
mod time;
mod vlc_service;
mod volume;
mod vlc_controller;
mod error;

use printer::Printer;
use time::Time;
use volume::Volume;
use vlc_controller::VLCController;
use error::Error;

#[derive(Deserialize, Debug)]
pub struct Status {
    time: Time,
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

fn main() -> Result<(), Error> {
    let credentials = Credentials {
        user: "",
        password: "12345",
    };
    let mut vlc_controller = VLCController::new(credentials);
    loop {
        match vlc_controller.run() {
            Ok(()) => {}
            Err(Error::Network(e)) if e.is_http() && e.status() == None => {
                println!("[err] Unable to connect to vlc");
            }
            Err(e) => return Err(e)
        }
        sleep(Duration::from_millis(200));
    }
}
