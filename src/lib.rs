use failure::Error;
use tokio::signal;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use warp::Filter;

use libbackend::config::ServerConfig;
use libbackend::logging;

use crate::config::AppConfig;

pub mod config;

pub async fn run() -> Result<(), Error> {
    let config = AppConfig::new()?;

    logging::init(&config.logging)?;

    logging::log_life_cycle_event("STARTING APPLICATION");

    {
        let shutdown_tx = bind_server(&config.server)?;

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

fn bind_server(config: &ServerConfig) -> Result<Sender<()>, Error> {
    let routes = warp::any()
        .map(|| "Hello, World!");

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
