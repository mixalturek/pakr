use failure::Error;

use libbackend::logging;

use crate::config::AppConfig;

pub mod config;

pub fn run() -> Result<(), Error> {
    let config = AppConfig::new()?;

    logging::init(&config.logging)?;

    logging::log_life_cycle_event("STARTING APPLICATION");

    logging::log_life_cycle_event("APPLICATION STARTED");

    logging::log_life_cycle_event("STOPPING APPLICATION");

    logging::log_life_cycle_event("APPLICATION STOPPED");

    Ok(())
}
