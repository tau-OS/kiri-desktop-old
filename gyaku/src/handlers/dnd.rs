use smithay::wayland::data_device::{ClientDndGrabHandler, ServerDndGrabHandler};

use crate::state::GyakuState;

impl ClientDndGrabHandler for GyakuState {
    fn started(
        &mut self,
        source: Option<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource>,
        icon: Option<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface>,
        seat: smithay::input::Seat<Self>,
    ) {
        // ! Soft TODO
    }

    fn dropped(&mut self, seat: smithay::input::Seat<Self>) {
        // ! Soft TODO
    }
}

impl ServerDndGrabHandler for GyakuState {
    fn action(
        &mut self,
        action: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
    ) {
        // ! Soft TODO
    }

    fn dropped(&mut self) {
        // ! Soft TODO
    }

    fn cancelled(&mut self) {
        // ! Soft TODO
    }

    fn send(&mut self, mime_type: String, fd: std::os::fd::OwnedFd) {
        // ! Soft TODO
    }

    fn finished(&mut self) {
        // ! Soft TODO
    }
}
