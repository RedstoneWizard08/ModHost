//! Admin panel configuration.

use duration_str::{HumanFormat, deserialize_duration};
use serde::Serializer;
use std::time::Duration;

/// The ModHost admin panel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    /// How often the server will refresh instance and system statistics.
    #[serde(deserialize_with = "deserialize_duration")]
    #[serde(serialize_with = "serialize_duration")]
    pub stats_interval: Duration,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            stats_interval: Duration::from_secs(5),
        }
    }
}

/// Serialize a duration as a human-readable format.
pub fn serialize_duration<S: Serializer>(val: &Duration, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&val.human_format())
}
