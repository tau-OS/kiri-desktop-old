use smithay::{delegate_data_device, wayland::data_device::DataDeviceHandler};

use crate::state::GyakuState;

impl DataDeviceHandler for GyakuState {
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }

    fn action_choice(
        &mut self,
        available: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
        preferred: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
    ) -> smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction {
        smithay::wayland::data_device::default_action_chooser(available, preferred)
    }

    fn new_selection(
        &mut self,
        source: Option<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource>,
    ) {
        // ! Soft TODO
    }

    fn send_selection(&mut self, mime_type: String, fd: std::os::fd::OwnedFd) {
        // ! Soft TODO
    }
}

delegate_data_device!(GyakuState);
