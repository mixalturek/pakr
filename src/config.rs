use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use libbackend::config::Logging;

/// Top level structure for configuration of the application.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: Logging,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        Self::load().map_err(|error| {
            // TODO: Logging hasn't been initialized yet.
            eprintln!("ERROR: Loading of configuration failed: {}", error);
            error
        })
    }

    fn load() -> Result<Self, ConfigError> {
        let mut config = Config::new();

        // TODO: Define it somehow better, make the path configurable (perameter, env).
        // TODO: Document the behavior.
        // https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/src/settings.rs
        config.merge(File::with_name("config/reference.yaml"))?;
        config.merge(File::with_name("config/application.yaml").required(false))?;
        config.merge(File::with_name("config/local.yaml").required(false))?;
        config.merge(Environment::with_prefix("app"))?;

        config.try_into()
    }
}
