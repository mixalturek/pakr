//! Everything related to logging.

use std::thread;

use failure::Error;
use log::info;

/// Setup logging infrastructure. This function should be called at the very beginning of the
/// application life cycle.
pub fn init() -> Result<(), Error> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {:>5} {:>25} {:>10}: {} ({}:{})",
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

/// Log an important application life cycle event. Highlight application starts and stops in logs.
pub fn log_life_cycle_event(message: &str) {
    info!("====================== {} ======================", message);
}
