use crate::state::GyakuState;
use smithay::{delegate_data_device, wayland::data_device::DataDeviceHandler};
use tracing::instrument;

impl DataDeviceHandler for GyakuState {
    #[instrument(skip(self))]
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }
    #[instrument(skip(self))]
    fn action_choice(
        &mut self,
        available: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
        preferred: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
    ) -> smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction {
        smithay::wayland::data_device::default_action_chooser(available, preferred)
    }

    #[instrument(skip(self))]
    fn new_selection(
        &mut self,
        source: Option<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource>,
    ) {
        // ! Soft TODO
    }
    #[instrument(skip(self))]
    fn send_selection(&mut self, mime_type: String, fd: std::os::fd::OwnedFd) {
        // ! Soft TODO
    }
}

delegate_data_device!(GyakuState);
