use serde_json::Value;

use crate::{Credentials, Error, Meta, Status, Time, Volume};

pub struct VLCService {
    client: reqwest::Client,
    base_url: String,
    credentials: Credentials,
}

impl VLCService {
    pub fn new(credentials: Credentials, base_url: String) -> VLCService {
        VLCService {
            client: reqwest::Client::new(),
            base_url,
            credentials,
        }
    }

    pub fn get_meta(&self) -> Result<Option<Meta>, Error> {
        let mut resp = self
            .client
            .get(&format!("{}/requests/playlist.json", self.base_url))
            .basic_auth(&self.credentials.user, Some(&self.credentials.password))
            .send()?;

        let data: Value = serde_json::from_str(&resp.text()?)?;
        let data = data["children"][0]["children"].clone();
        let metas: Vec<Meta> = serde_json::from_value(data)?;
        Ok(metas.last().cloned())
    }

    pub fn get_status(&self) -> Result<Status, Error> {
        let mut resp = self
            .client
            .get(&format!("{}/requests/status.json", self.base_url))
            .basic_auth(&self.credentials.user, Some(&self.credentials.password))
            .send()?;

        let data = resp.text()?;
        let data = serde_json::from_str(&data)?;
        Ok(data)
    }

    pub fn seek_to(&self, position: Time) -> Result<(), Error> {
        self.client
            .get(&format!("{}/requests/status.json", self.base_url))
            .query(&[("command", "seek"), ("val", &position.to_string())])
            .basic_auth(&self.credentials.user, Some(&self.credentials.password))
            .send()?;

        Ok(())
    }

    pub fn set_volume(&self, amount: Volume) -> Result<(), Error> {
        self.client
            .get(&format!("{}/requests/status.json", self.base_url))
            .query(&[
                ("command", "volume"),
                ("val", &amount.scale(200, 512).to_string()),
            ])
            .basic_auth(&self.credentials.user, Some(&self.credentials.password))
            .send()?;

        Ok(())
    }
}
