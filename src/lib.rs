use std::time::Instant;

use failure::Error;
use tokio::signal;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use warp::Filter;

use libbackend::config::ServerConfig;
use libbackend::logging;
use libbackend::server::ServerStatus;

use crate::config::AppConfig;

pub mod config;

const APPLICATION: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub async fn run() -> Result<(), Error> {
    let start_time = Instant::now();
    let config = AppConfig::new()?;

    logging::init(&config.logging)?;

    logging::log_life_cycle_event("STARTING APPLICATION");
    log::debug!("Application version: {} {}", APPLICATION, VERSION);

    {
        let shutdown_tx = bind_server(&config.server, start_time)?;

        logging::log_life_cycle_event("APPLICATION STARTED");

        signal::ctrl_c().await?;
        log::debug!("SIGINT/Ctrl-C detected, gracefully shutting down");
        let _ = shutdown_tx.send(());

        logging::log_life_cycle_event("STOPPING APPLICATION");

        // All destructors are called on exit from this code block.
    }

    logging::log_life_cycle_event("APPLICATION STOPPED");
    Ok(())
}

fn bind_server(config: &ServerConfig, start_time: Instant) -> Result<Sender<()>, Error> {
    // TODO: Make the FS path configurable
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("assets/index.html"));

    // TODO: Make the FS path configurable
    let assets = warp::path("assets")
        .and(warp::fs::dir("assets"));

    // http get http://localhost:3030/api/status
    let status = warp::path("status")
        .and(warp::path::end())
        .map(move || {
            let status = ServerStatus {
                application: APPLICATION,
                version: VERSION,
                uptime_seconds: start_time.elapsed().as_secs(),
            };

            warp::reply::json(&status)
        });

    let api = warp::path("api")
        .and(status);

    let routes = index.or(assets)
        .or(api)
        // TODO: Put the log to a submodule, warp::log::custom()
        .with(warp::log(module_path!()));

    let listen_addr = format!("{}:{}", config.listen_address, config.listen_port)
        .parse::<std::net::SocketAddr>()?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let (bind_addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(listen_addr, async {
            shutdown_rx.await.ok();
        });

    tokio::task::spawn(server);

    log::info!("Server is listening: {}", bind_addr);
    Ok(shutdown_tx)
}
