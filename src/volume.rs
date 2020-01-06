use regex::Regex;
use serde::{self, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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


struct VolumeVisitor;

impl<'de> Visitor<'de> for VolumeVisitor {
    type Value = Volume;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("volume in the format of <int>% or <int>")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Volume(value as u32).scale(512, 200))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let re = Regex::new(r"^\d+%$").unwrap();
        if !re.is_match(value) {
            return Err(E::custom(format!(
                "invalid volume format '{}', expected volume to be in the format <int>%",
                value
            )));
        }

        let volume = value[0..(value.len()-1)].parse::<u32>().unwrap();
        Ok(Volume(volume))
    }
}

impl<'de> Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Volume, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(VolumeVisitor)
    }
}
