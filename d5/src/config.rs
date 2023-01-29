//! d5 configuration module

// d5 will use a configuration file to store the session configuration.
// Why don't we use systemd's target files? Because we cannot monitor them. We do not know when they are stopped.
// We need to know when the session is stopped so we can kill d5.

// The configuration file will be in TOML format. It will be located in /etc/d5.conf.d/ and will be named after the session name.

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub session: SessionConfig,
    #[serde(default)]
    pub services: BTreeMap<String, ServiceConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionConfig {
    /// The command to launch the leader process
    pub leader: String,
}

// services config would be:
// [services]
// [services.foo]
// script = "echo foo"
// type = "script"

#[derive(Serialize, Deserialize)]
pub enum ServiceType {
    Script,
    Systemd,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    /// The command to launch the service
    pub command: String,
    /// The type of service
    #[serde(rename = "type")]
    pub service_type: ServiceType,
}

// load config
pub fn load_config(name: &str) -> Result<Config> {
    let config = std::fs::read_to_string(format!("/etc/d5.conf.d/{}.toml", name))?;
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}
