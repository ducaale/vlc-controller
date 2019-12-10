use std::error::Error;
use serde::Deserialize;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct Status {
    time: u32
}

struct Meta {
    name: String,
    uri: String,
    duration: u32
}

pub struct Credentials<'a> {
    user: &'a str,
    password: &'a str
}

fn get_meta(client: &reqwest::Client, credentials: &Credentials) -> Result<Meta, Box<dyn Error>> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text()?)?;
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
) -> Result<Status, Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "volume"), ("val", &level.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let credentials = Credentials { user: "", password: "12345" };
    let skip_times = vec![(10, 15), (20, 25), (40, 45)];
    let client = reqwest::Client::new();

    let meta = get_meta(&client, &credentials)?;
    println!("Currently playing: {} duration: {} seconds", meta.name, meta.duration);
    println!("Searching for commands file: {}", meta.uri);

    loop {
        let status = get_status(&client, &credentials)?;
        print!("\rCurrent time: {} seconds", status.time);
        io::stdout().flush()?;

        for &(skip_start, skip_end) in skip_times.iter() {
            if status.time >= skip_start && status.time < skip_end {
                seek_to(&client, &credentials, skip_end)?;
            }
        }
        sleep(Duration::from_millis(500));
    }
}