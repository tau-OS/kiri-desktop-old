//! d5 - the Kiri session manager
//! This is the main entry point for the d5 binary.
//! It does some fancy dbus stuff and then starts the main loop.
mod notify;
mod proc;
mod interface;
mod env;
mod cli;
mod session;

use color_eyre::Result;
use tracing::{debug, log};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use zbus::ConnectionBuilder;

// zbus interface trait

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    color_eyre::config::HookBuilder::default()
        .add_default_filters()
        .panic_section("It's not that I won't do it, I just can't!")
        .install()?;
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // load tracing_journald subscriber
    // tracing_subscriber::registry().with(tracing_journald::layer().unwrap()).init();
    // let conn = ConnectionBuilder::session()?.build().await?;
    // conn.monitor_activity().await;


    // futures::pending!();

    // Ok(())
    cli::entrypoint().await
}
