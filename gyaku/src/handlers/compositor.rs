use crate::state::GyakuState;
use smithay::{
    backend::renderer::utils::on_commit_buffer_handler, delegate_compositor,
    wayland::compositor::CompositorHandler,
};
use tracing::instrument;

impl CompositorHandler for GyakuState {
    #[instrument(skip(self))]
    fn compositor_state(&mut self) -> &mut smithay::wayland::compositor::CompositorState {
        &mut self.compositor_state
    }
    #[instrument(skip(self))]
    fn commit(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        on_commit_buffer_handler(surface);
        // ! Soft TODO
    }
}

delegate_compositor!(GyakuState);
