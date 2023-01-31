use color_eyre::Result;
use slog::Logger;
use smithay::{
    input::{Seat, SeatState},
    reexports::calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction},
    wayland::{
        compositor::CompositorState, data_device::DataDeviceState, shm::ShmState,
        socket::ListeningSocketSource,
    },
};
use tracing::instrument;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason},
    Display,
};
/// State of the compositor
pub struct GyakuState {
    pub(crate) compositor_state: CompositorState,
    pub(crate) shm_state: ShmState,
    pub(crate) seat_state: SeatState<Self>,
    pub(crate) data_device_state: DataDeviceState,

    pub(crate) log: Logger,
    //    pub(crate) seat: Seat<Self>,
}

impl GyakuState {
    pub fn new(display: &mut Display<Self>, log: Logger) -> Self {
        let display_handle = display.handle();

        Self {
            compositor_state: CompositorState::new::<Self, _>(&display_handle, log.clone()),
            shm_state: ShmState::new::<Self, _>(&display_handle, vec![], log.clone()),
            seat_state: SeatState::new(),
            data_device_state: DataDeviceState::new::<Self, _>(&display_handle, log.clone()),

            log,
        }
    }
}

// Client state... might want to move this to another file for cleaniness later

#[derive(Debug, Default)]
pub struct ClientState;
impl ClientData for ClientState {
    /// Notification that a client was initialized
    #[instrument(skip(self))]
    fn initialized(&self, _client_id: ClientId) {}
    /// Notification that a client is disconnected
    #[instrument(skip(self))]
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
