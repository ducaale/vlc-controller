use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

mod error;
mod printer;
mod time;
mod vlc_controller;
mod vlc_service;
mod volume;

use error::Error;
use printer::Printer;
use time::Time;
use vlc_controller::VLCController;
use vlc_service::VLCService;
use volume::Volume;

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

pub struct Credentials {
    user: String,
    password: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Command {
    Skip { start: Time, end: Time },
    Mute { start: Time, end: Time },
    SetVolume { amount: Volume, at: Time },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "vlc-controller", author = "ducaale <sharaf.13@hotmail.com>")]
struct Opt {
    /// vlc http intf password
    #[structopt(long, short)]
    password: String,

    /// vlc http intf address
    #[structopt(long, default_value = "localhost")]
    http_host: String,

    /// vlc http intf port
    #[structopt(long, default_value = "8080")]
    http_port: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let credentials = Credentials {
        user: "".to_string(),
        password: opt.password,
    };
    let base_url = format!("http://{}:{}", opt.http_host, opt.http_port);
    let mut vlc_controller = VLCController::new(credentials, base_url);
    loop {
        match vlc_controller.run() {
            Ok(()) => {}
            Err(Error::Network(e)) if e.is_http() && e.status() == None => {
                println!("[err] Unable to connect to vlc");
            }
            Err(e) => return Err(e),
        }
        sleep(Duration::from_millis(200));
    }
}
