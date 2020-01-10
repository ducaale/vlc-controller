use regex::Regex;
use serde::de::{self, Visitor};
use serde::{self, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time(u32);

impl Time {
    pub fn as_seconds(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 >= (60 * 60) {
            write!(
                f,
                "{:02}:{:02}:{:02}",
                self.0 / (60 * 60),
                (self.0 % (60 * 60)) / 60,
                self.0 % 60
            )
        } else {
            write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
        }
    }
}

impl From<u32> for Time {
    fn from(value: u32) -> Self {
        Time(value)
    }
}

pub fn difference(time1: Time, time2: Time) -> u32 {
    time1.0 - time2.0
}

struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("time in the format of [<int>:][<int>:]<int>")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Time::from(value as u32))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let re = Regex::new(r"^(\d+:){0,2}\d+$").unwrap();
        if !re.is_match(value) {
            return Err(E::custom(format!(
                "invalid time format '{}', expected time to be in the format [<int>:][<int>:]<int>",
                value
            )));
        }

        let parts = value.split(':');
        let mut time = 0;
        for (i, p) in parts.rev().enumerate() {
            let p = p.parse::<u32>().unwrap();
            time += p * (60u32.pow(i as u32));
        }
        Ok(Time(time))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimeVisitor)
    }
}
