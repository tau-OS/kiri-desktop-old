use smithay::wayland::buffer::BufferHandler;

use crate::state::GyakuState;

impl BufferHandler for GyakuState {
    fn buffer_destroyed(
        &mut self,
        buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
        // ! Soft TODO
    }
}
