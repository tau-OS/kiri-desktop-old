use smithay::wayland::buffer::BufferHandler;
use tracing::instrument;

use crate::state::GyakuState;

impl BufferHandler for GyakuState {
    #[instrument(skip(self))]
    fn buffer_destroyed(
        &mut self,
        buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
        // ! Soft TODO
    }
}
