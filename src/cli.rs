//! CLI interface for d5

use color_eyre::Result;
use clap::{Parser, ValueEnum};
// enum for display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DisplayMode {
    X11,
    Wayland,
}




#[derive(Parser)]
pub struct D5Entrypoint {
    // no subcommands, because it literally just launches a systemd user target
    // session manager is fun

    /// systemd target to launch
    #[clap(short, long, required = true)]
    pub target: String,

    /// Display mode: either "x11" or "wayland"
    #[clap(short, long, default_value = "x11")]
    #[arg(value_enum)]
    pub display: DisplayMode,
}

/// Parse the CLI arguments


pub async fn entrypoint() -> Result<()> {
    let args = D5Entrypoint::parse();
    crate::env::load_envs(args.display)?;
    crate::session::new_session("test".to_string()).await?;
    Ok(())
}