use serde::{self, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time(u32);

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

impl From<&str> for Time {
    fn from(value: &str) -> Self {
        let parts = value.split(':');
        let mut time = 0;
        for (i, p) in parts.rev().enumerate() {
            let p = p.parse::<u32>().expect("cannot serialize time");
            time += p * (60u32.pow(i as u32));
        }
        Time(time)
    }
}

impl From<u32> for Time {
    fn from(value: u32) -> Self {
        Time(value)
    }
}


impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        match v {
            Value::Number(_) => Ok(Time::from(Value::as_u64(&v).unwrap() as u32)),
            Value::String(s) => Ok(Time::from(s.as_str())),
            _ => panic!("invalid time format")
        }
    }
}