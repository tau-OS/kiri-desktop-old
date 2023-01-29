use clap::{Parser, ValueEnum};
use color_eyre::Result;
use tracing::{debug, instrument::WithSubscriber, log};

#[derive(Parser)]
pub struct Cli {
    #[clap(long, short = 'B')]
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

fn log() -> ::slog::Logger {
    use tracing_slog::TracingSlogDrain;
    let drain = TracingSlogDrain;
    ::slog::Logger::root(drain, slog::o!())
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .add_default_filters()
        .install()?;

    let cli = Cli::parse();
    let log = log();
    let _guard = slog_scope::set_global_logger(log.clone());

    pretty_env_logger::formatted_builder()
        .filter_level(tracing::log::LevelFilter::Debug)
        .init();

    // let log = log.clone();
    // todo: slog-tracing bridge doesn't work

    match cli.backend {
        #[cfg(feature = "winit")]
        DisplayBackend::Winit => {
            slog::info!(log, "Starting anvil with winit backend");
            kiri::winit::run_winit(log);
        }
        #[cfg(feature = "udev")]
        DisplayBackend::TtyUdev => {
            slog::info!(log, "Starting anvil on a tty using udev");
            kiri::udev::run_udev(log);
        }
        #[cfg(feature = "x11")]
        DisplayBackend::X11 => {
            slog::info!(log, "Starting anvil with x11 backend");
            kiri::x11::run_x11(log);
        }
    }
    Ok(())
}
