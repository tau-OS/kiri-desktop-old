use clap::{Parser, ValueEnum};
use color_eyre::Result;
use slog::Drain;
// use tracing_subscriber::fmt;
use smithay::reexports::calloop::EventLoop;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use wayland_server::Display;

use crate::backend::Backend;
use crate::backend::determine_backend;

mod backend;
mod event_loop;
mod handlers;
mod shell;
mod state;
mod util;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, short = 'B')]
    backend: Option<DisplayBackend>,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum DisplayBackend {
    Winit,
    TtyUdev,
    X11,
}

fn log() -> ::slog::Logger {
    use tracing_slog::TracingSlogDrain;
    let drain = TracingSlogDrain;
    ::slog::Logger::root(
        slog::LevelFilter::new(drain, slog::Level::Info).fuse(),
        slog::o!(),
    )
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .add_default_filters()
        .install()?;

    let cli = Cli::parse();
    let log = log();
    // let _guard = slog_scope::set_global_logger(log.clone());

    let default_env = EnvFilter::builder()
        // .with_default_directive(LevelFilter::INFO.into())
        .parse("trace,smithay::backend::renderer::gles2=off")?;
    // .from_env_lossy();
    // smithay::backend::renderer::gles2

    tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_level(true)
        .with_file(true)
        .with_thread_names(true)
        .with_ansi(true)
        .without_time()
        .with_env_filter(default_env)
        .finish()
        .with(
            tracing_journald::layer()
                .unwrap()
                .with_syslog_identifier("gyaku".to_string()),
        )
        .init();

    let mut ev: EventLoop<event_loop::EventLoopData> = EventLoop::try_new()?;

    // todo: move to backend
    let mut display: Display<state::GyakuState> = Display::new()?;
    let state = state::GyakuState::new(&mut display, log.clone());

    let mut data = event_loop::EventLoopData {
        state,
        display,
        start_time: std::time::Instant::now(),
    };

    let address = event_loop::setup_listeners(&mut ev, &mut data)?;
    println!("listening on {}", address.into_string().unwrap());

    let backend_type = cli.backend.unwrap_or_else(determine_backend);

    let display_backend = match backend_type {
        DisplayBackend::Winit => {
            slog::info!(log, "Starting with winit backend");
            backend::winit::WinitBackend::new(&mut data, log.clone())?
        }
        DisplayBackend::TtyUdev => {
            slog::info!(log, "Starting on a tty using udev");
            todo!();
        }
        DisplayBackend::X11 => {
            slog::info!(log, "Starting with x11 backend");
            todo!();
        }
    };

    display_backend.start(&mut ev)?;

    ev.run(None, &mut data, move |_| {
        // Smallvil is running
    })?;

    Ok(())
}
