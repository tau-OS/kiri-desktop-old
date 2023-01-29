use clap::{Parser, ValueEnum};
use color_eyre::Result;
use slog::{crit, o, Drain, Logger};
use tracing::{debug, log};

#[derive(Parser)]
pub struct Cli {
    #[clap(long)]
    backend: DisplayBackend,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum DisplayBackend {
    #[cfg(feature = "winit")]
    Winit,
    #[cfg(feature = "udev")]
    TtyUdev,
    #[cfg(feature = "x11")]
    X11,
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .add_default_filters()
        .install()?;
    // slog_stdlog::init()?;

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let cli = Cli::parse();

    let drain = tracing_slog::TracingSlogDrain;
    let log = Logger::root(drain, o!());
    let _guard = slog_scope::set_global_logger(log.clone());
    match cli.backend {
        #[cfg(feature = "winit")]
        DisplayBackend::Winit => {
            slog::info!(log, "Starting anvil with winit backend");
            anvil::winit::run_winit(log);
        }
        #[cfg(feature = "udev")]
        DisplayBackend::TtyUdev => {
            slog::info!(log, "Starting anvil on a tty using udev");
            anvil::udev::run_udev(log);
        }
        #[cfg(feature = "x11")]
        DisplayBackend::X11 => {
            slog::info!(log, "Starting anvil with x11 backend");
            anvil::x11::run_x11(log);
        }
    }
    Ok(())
}
