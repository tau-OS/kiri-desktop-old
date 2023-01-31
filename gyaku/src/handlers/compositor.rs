use crate::state::GyakuState;
use smithay::{delegate_compositor, wayland::compositor::CompositorHandler};

impl CompositorHandler for GyakuState {
    fn compositor_state(&mut self) -> &mut smithay::wayland::compositor::CompositorState {
        &mut self.compositor_state
    }

    fn commit(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        todo!()
    }
}

delegate_compositor!(GyakuState);
