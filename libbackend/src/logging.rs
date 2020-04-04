//! Everything related to logging.

use std::thread;

use failure::Error;
use log::info;

use crate::config;

/// Setup logging infrastructure. This function should be called at the very beginning of the
/// application life cycle.
pub fn init(config: &config::Logging) -> Result<(), Error> {
    let mut builder = fern::Dispatch::new()
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
        .level(config.default_level.into());

    if let Some(modules) = &config.modules {
        for (module, level) in modules {
            builder = builder.level_for(module.clone(), level.into());
        }
    }

    builder.chain(std::io::stderr())
        // TODO: Output to a file.
        // .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

/// Log an important application life cycle event. Highlight application starts and stops in logs.
pub fn log_life_cycle_event(message: &str) {
    info!("====================== {} ======================", message);
}
