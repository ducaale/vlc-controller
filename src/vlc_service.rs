use serde_json::Value;

use crate::{Credentials, Meta, Error, Status, Time, Volume};

pub fn get_meta(
    client: &reqwest::Client,
    credentials: &Credentials,
) -> Result<Option<Meta>, Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/playlist.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data: Value = serde_json::from_str(&resp.text()?)?;
    let data = data["children"][0]["children"].clone();
    let metas: Vec<Meta> = serde_json::from_value(data)?;
    Ok(metas.last().cloned())
}

pub fn get_status(
    client: &reqwest::Client,
    credentials: &Credentials,
) -> Result<Status, Error> {
    let mut resp = client
        .get("http://localhost:8080/requests/status.json")
        .basic_auth(credentials.user, Some(credentials.password))
        .send()?;

    let data = resp.text()?;
    let data = serde_json::from_str(&data)?;
    Ok(data)
}

pub fn seek_to(
    client: &reqwest::Client,
    credentials: &Credentials,
    position: Time,
) -> Result<(), Error> {
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
) -> Result<(), Error> {
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
