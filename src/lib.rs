use failure::Error;

use libbackend::logging;

pub fn run() -> Result<(), Error> {
    logging::init()?;

    logging::log_life_cycle_event("STARTING APPLICATION");

    logging::log_life_cycle_event("APPLICATION STARTED");

    logging::log_life_cycle_event("STOPPING APPLICATION");

    logging::log_life_cycle_event("APPLICATION STOPPED");

    Ok(())
}
