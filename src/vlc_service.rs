use serde_json::Value;

use crate::{Credentials, Meta, Status, Time, Volume};

pub fn get_meta(
    client: &reqwest::Client,
    credentials: &Credentials,
) -> Result<Option<Meta>, reqwest::Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text()?).unwrap();
    let data = data["children"][0]["children"].clone();
    let metas: Vec<Meta> = serde_json::from_value(data).unwrap();
    Ok(metas.last().cloned())
}

pub fn get_status(
    client: &reqwest::Client,
    credentials: &Credentials,
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
    position: Time,
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
    amount: Volume,
) -> Result<(), reqwest::Error> {
    client
        .get("http://localhost:8080/requests/status.json")
        .query(&[
            ("command", "volume"),
            ("val", &amount.scale(200, 512).to_string()),
        ])
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    Ok(())
}
