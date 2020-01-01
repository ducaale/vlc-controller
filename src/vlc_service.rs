use serde_json::Value;

use crate::{Credentials, Meta, Status, Time};

pub fn get_meta(client: &reqwest::Client, credentials: &Credentials) -> Result<Meta, reqwest::Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let data = data["children"][0]["children"].clone();
    let metas : Vec<Meta> = serde_json::from_value(data).unwrap();
    let meta : Meta = metas.last().unwrap().clone();
    Ok(meta)
}

pub fn get_status(
    client: &reqwest::Client,
    credentials: &Credentials
) -> Result<Status, reqwest::Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/status.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Status = resp.json()?;
    Ok(data)
}

pub fn seek_to(
    client: &reqwest::Client,
    credentials: &Credentials,
    position: Time
) -> Result<(), reqwest::Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "seek"), ("val", &position.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}

pub fn set_volume(
    client: &reqwest::Client,
    credentials: &Credentials,
    amount: u32
) -> Result<(), reqwest::Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[("command", "volume"), ("val", &amount.to_string())])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}
