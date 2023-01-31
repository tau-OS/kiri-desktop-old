use super::state::{ClientState, GyakuState};
use color_eyre::Result;
use smithay::{
    reexports::calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction},
    wayland::socket::ListeningSocketSource,
};
use std::ffi::OsString;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use wayland_server::Display;

pub struct EventLoopData {
    pub state: GyakuState,
    pub display: Display<GyakuState>,
}

pub fn setup_listeners(
    event_loop: &mut EventLoop<EventLoopData>,
    data: &mut EventLoopData,
) -> Result<OsString> {
    // find wayland sockets first
    let listener = ListeningSocketSource::new_auto(data.state.log.clone())?;
    let listener_address = listener.socket_name().into();

    let loop_handle = event_loop.handle();

    loop_handle.insert_source(listener, move |client_stream, _, state| {
        state
            .display
            .handle()
            .insert_client(client_stream, Arc::new(ClientState))
            .unwrap();
    })?;

    loop_handle.insert_source(
        Generic::new(
            data.display.backend().poll_fd().as_raw_fd(),
            Interest::READ,
            Mode::Level,
        ),
        |_, _, data| {
            data.display.dispatch_clients(&mut data.state).unwrap();
            Ok(PostAction::Continue)
        },
    )?;

    Ok(listener_address)
}
