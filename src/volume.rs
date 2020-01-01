use serde::{self, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Volume(u32);

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Volume {
    pub fn new(amount: u32) -> Volume {
        Volume(amount)
    }

    pub fn scale(&self, from: u32, to: u32) -> Volume {
        let amount = (self.0 as f32 / from as f32) * to as f32;
        Volume(amount as u32)
    }
}

pub fn abs_difference(volume1: Volume, volume2: Volume) -> u32 {
    (volume1.0 as i32 - volume2.0 as i32).abs() as u32
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Volume, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    match v {
        Value::Number(_) => {
            let volume = Volume(Value::as_u64(&v).unwrap() as u32);
            Ok(volume.scale(512, 200))
        }
        _ => panic!("invalid volume"),
    }
}
