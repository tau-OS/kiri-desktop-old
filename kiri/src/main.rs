use clap::{Parser, ValueEnum};
use color_eyre::Result;
use tracing::metadata::LevelFilter;
// use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, short = 'B', default_value = "auto")]
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
    Auto,
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

    let default_env = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env_lossy();

    tracing_subscriber::FmtSubscriber::builder()
        .with_level(true)
        .with_file(true)
        .with_thread_names(true)
        .with_ansi(true)
        .pretty()
        .without_time()
        .with_env_filter(default_env)
        .finish()
        .with(
            tracing_journald::layer()
                .unwrap()
                .with_syslog_identifier("kiri".to_string()),
        )
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
        _ => {
            // auto-detect backend
            if std::env::var_os("DISPLAY").is_some()
                || std::env::var_os("WAYLAND_DISPLAY").is_some()
            {
                slog::info!(log, "Starting anvil with winit backend");
                kiri::winit::run_winit(log);
            } else {
                slog::info!(log, "Starting anvil on a tty using udev");
                kiri::udev::run_udev(log);
            }
        }
    }
    Ok(())
}
