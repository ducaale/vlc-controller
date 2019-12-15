use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::fmt;

#[derive(Deserialize, Debug)]
struct Status {
    time: u32
}

struct Meta {
    name: String,
    uri: String,
    duration: u32
}

struct Credentials<'a> {
    user: &'a str,
    password: &'a str
}

struct Time(u32);
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

fn get_meta(client: &reqwest::Client, credentials: &Credentials) -> Result<Meta, Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let name = &data["children"][0]["children"][0]["name"];
    let uri = &data["children"][0]["children"][0]["uri"];
    let duration = &data["children"][0]["children"][0]["duration"];
    let meta = Meta {
        name: Value::to_string(&name),
        uri: Value::to_string(&uri),
        duration: Value::as_u64(&duration).unwrap() as u32
    };
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
    position: u32
) -> Result<(), Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "seek"), ("val", &position.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

fn _set_volume(
    client: &reqwest::Client,
    credentials: &Credentials,
    level: u32
) -> Result<(), Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "volume"), ("val", &level.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

fn control_vlc(client: &reqwest::Client, credentials: &Credentials) -> Result<(), Error> {
    // TODO: read skip_times from files in a function and implement memo so we
    // don't read the same file again and again
    let skip_times = vec![(10, 15), (20, 25), (40, 45)];

    let meta = get_meta(&client, &credentials)?;
    let status = get_status(&client, &credentials)?;
    print!("\r[info] Currently playing: {} ({} / {})", meta.name, Time(status.time), Time(meta.duration));
    io::stdout().flush().unwrap();

    for &(skip_start, skip_end) in skip_times.iter() {
        if status.time >= skip_start && status.time < skip_end {
            seek_to(&client, &credentials, skip_end)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    let credentials = Credentials { user: "", password: "12345" };
    let client = reqwest::Client::new();
    loop {
        if let Err(e) = control_vlc(&client, &credentials) {
            if e.status() == None {
                print!("\n[err] Unable to connect to vlc")
            }
            else {
                return Err(e);
            }
        };
        sleep(Duration::from_millis(500));
    }
}