use std::thread;

use failure::Error;
use log::info;

pub fn init_logging() -> Result<(), Error> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {:>5} {:>10} {:>10}: {} ({}:{})",
                // TODO: UTC?
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                record.level(),
                record.target(),
                thread::current().name().unwrap_or("UNKNOWN"),
                message,
                record.file().unwrap_or("UNKNOWN"),
                record.line().unwrap_or(0)
            ))
        })
        .level(log::LevelFilter::Debug)
        // .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stderr())
        // .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

pub fn log_life_cycle_event(message: &str) {
    info!("====================== {} ======================", message);
}

pub fn run() -> Result<(), Error> {
    init_logging()?;

    log_life_cycle_event("STARTING APPLICATION");

    log_life_cycle_event("APPLICATION STARTED");

    log_life_cycle_event("STOPPING APPLICATION");

    log_life_cycle_event("APPLICATION STOPPED");

    Ok(())
}
