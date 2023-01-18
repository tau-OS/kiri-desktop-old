//! d5 - the Kiri session manager
//! This is the main entry point for the d5 binary.
//! It does some fancy dbus stuff and then starts the main loop.
mod notify;
mod proc;
mod interface;
mod env;

use color_eyre::Result;
use tracing::{debug, log};
use zbus::ConnectionBuilder;

// zbus interface trait
struct D5 {
    pub data: String,
}

#[zbus::dbus_interface(name = "com.fyralabs.d5")]
impl D5 {
    fn add_name(&mut self, name: String) {
        self.data = name;
    }

    fn hello(&self) -> String {
        format!("Hello from d5! {}", self.data)
    }
    fn exit(&self) {
        // reply to the client
        std::process::exit(0);
    }
}

struct NotifDaemon;

#[zbus::dbus_interface(name = "org.freedesktop.Notifications")]
impl NotifDaemon {
    fn notify(&self) {
        println!("Hello from the notification daemon!");
    }
}

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

    // notify::listen().await?;\
    crate::env::load_envs()?;

    let conn = ConnectionBuilder::session()?.build().await?;
    conn.monitor_activity().await;


    futures::pending!();

    Ok(())
}
