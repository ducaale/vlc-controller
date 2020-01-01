use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod time;
use time::Time;

#[derive(Deserialize, Debug)]
struct Status {
    time: Time
}

#[derive(Deserialize, Debug)]
struct Meta {
    name: String,
    uri: String,
    duration: Time
}

struct Credentials<'a> {
    user: &'a str,
    password: &'a str
}

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Action {
    Skip { start: Time, end: Time },
    Mute { start: Time, end: Time },
    SetVolume { amount: u32, at: Time }
}

fn get_meta(client: &reqwest::Client, credentials: &Credentials) -> Result<Meta, Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let data = data["children"][0]["children"][0].clone();
    let meta : Meta = serde_json::from_value(data).unwrap();
    Ok(meta)
}

fn get_status(
    client: &reqwest::Client,
    credentials: &Credentials
) -> Result<Status, Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/status.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Status = resp.json()?;
    Ok(data)
}

fn seek_to(
    client: &reqwest::Client,
    credentials: &Credentials,
    position: Time
) -> Result<(), Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "seek"), ("val", &position.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

fn set_volume(
    client: &reqwest::Client,
    credentials: &Credentials,
    amount: u32
) -> Result<(), Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "volume"), ("val", &amount.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

fn get_commands_file_path(video_uri: &str) -> String {
    let path = Path::new(video_uri).with_extension("yml");
    let prefix = "file:///";
    path.to_str().unwrap()[prefix.len()..].to_string()
}

// TODO: implement memoization so we don't read the same file again and again
fn read_commands(path: &str) -> Vec<Action> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("\n[err] Commands File not found");
                return vec![];
            },
            _ => panic!("{}", e)
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).expect("unable to read file");
    let actions: Vec<Action> = serde_yaml::from_str(&data).unwrap();
    actions
}

fn control_vlc(client: &reqwest::Client, credentials: &Credentials) -> Result<(), Error> {
    let meta = get_meta(&client, &credentials)?;
    let status = get_status(&client, &credentials)?;
    print!(
        "\r[info] Currently playing: '{}' ({} / {})",
        meta.name,
        status.time,
        meta.duration
    );
    io::stdout().flush().unwrap();

    let actions = read_commands(&get_commands_file_path(&meta.uri));
    for action in actions.iter() {
        match *action {
            Action::Skip { start, end } => {
                if status.time >= start && status.time < end {
                    seek_to(&client, &credentials, end)?;
                }
            },
            Action::SetVolume { at, amount } => {
                if status.time == at {
                    set_volume(&client, &credentials, amount)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    let credentials = Credentials { user: "", password: "12345" };
    let client = reqwest::Client::new();
    loop {
        if let Err(e) = control_vlc(&client, &credentials) {
            if e.is_http() && e.status() == None {
                println!("[err] Unable to connect to vlc")
            }
            else {
                return Err(e);
            }
        };
        sleep(Duration::from_millis(500));
    }
}