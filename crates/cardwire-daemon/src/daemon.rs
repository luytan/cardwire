mod dbus;
mod config;
mod models;

use std::{error::Error, future::pending};

use config::Config;
use log::info;
use zbus::connection;
use crate::models::{Daemon};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // log
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .try_init();
    let config: Config = Config::new().await;
    let daemon = Daemon::new(config)?;

    let conn_builder = connection::Builder::system()?;
    let _conn = conn_builder
        .name("com.cardwire.daemon")?
        .serve_at("/com/cardwire/daemon", daemon)?
        .build()
        .await?;

    info!("Daemon started");
    pending::<()>().await;
    Ok(())
}
