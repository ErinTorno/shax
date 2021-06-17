use core::time::Duration;
use serde::{Deserialize, Deserializer};

pub fn deserialize_duration_from_f32<'de, D>(deserializer: D) -> Result<Duration, D::Error> where D: Deserializer<'de> {
    let seconds = f32::deserialize(deserializer)?;
    Result::Ok(Duration::from_secs_f32(seconds))
}