//! Reusable configuration structures and related code.

use std::collections::HashMap;

use log::LevelFilter;
use serde::{Deserialize, Deserializer};
use serde::de::Error;

/// Configuration of logging.
#[derive(Clone, Debug, Deserialize)]
pub struct LoggingConfig {
    /// Default logging level to be used when no concrete one is specified.
    pub default_level: LevelFilterSerde,

    /// Optional log levels defined per module.
    pub modules: Option<HashMap<String, LevelFilterSerde>>,
}


/// Helper structure for deserialization of `LevelFilter`.
#[derive(Copy, Clone, Debug)]
pub struct LevelFilterSerde(LevelFilter);

const LEVEL_FILTERS: &[&str] = &["off", "error", "warn", "info", "debug", "trace"];

impl<'de> Deserialize<'de> for LevelFilterSerde {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<LevelFilterSerde, D::Error> {
        let s = String::deserialize(d)?;
        s.parse()
            .map(LevelFilterSerde)
            .map_err(|_| D::Error::unknown_variant(&s, LEVEL_FILTERS))
    }
}

impl From<LevelFilterSerde> for LevelFilter {
    fn from(serde: LevelFilterSerde) -> Self {
        serde.0
    }
}

impl From<&LevelFilterSerde> for LevelFilter {
    fn from(serde: &LevelFilterSerde) -> Self {
        serde.0
    }
}


/// Configuration of the server and its binding.
#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_address: String,
    pub listen_port: String,
}
