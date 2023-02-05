use crate::{event_loop::EventLoopData, DisplayBackend};
use color_eyre::{eyre::eyre, Result};
use smithay::reexports::calloop::{
    timer::{TimeoutAction, Timer},
    EventLoop,
};

pub mod drm;
pub mod winit;
pub mod x11;

// Generic trait for backend
pub trait Backend {
    fn dispatch(&mut self, data: &mut EventLoopData) -> Result<TimeoutAction>;

    fn start<'a>(mut self, event_loop: &mut EventLoop<'a, EventLoopData>) -> Result<()>
    where
        Self: Sized + 'a,
    {
        let handle = event_loop.handle();

        handle
            .insert_source(Timer::immediate(), move |_, _, data| {
                // TODO: handle error
                self.dispatch(data).unwrap()
            })
            .map_err(|e| eyre!("Could not setup backend loop: {}", e))?;

        Ok(())
    }
}

/// Automatically determine backend based on the current environment
pub fn determine_backend() -> DisplayBackend {
    if std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        DisplayBackend::Winit
    } else {
        DisplayBackend::TtyUdev
    }
}
