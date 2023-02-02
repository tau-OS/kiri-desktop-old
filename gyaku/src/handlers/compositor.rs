use crate::state::GyakuState;
use smithay::{
    backend::renderer::utils::on_commit_buffer_handler, delegate_compositor,
    wayland::compositor::CompositorHandler,
};
use smithay::wayland::compositor::{get_parent, is_sync_subsurface};
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

        // TODO: Why the fuck does this fix the bounding box???
        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self.space.elements().find(|w| w.toplevel().wl_surface() == &root) {
                window.on_commit();
            }
        };


        self.commit_xdg_shell_surface(surface);
        self.popup_manager.commit(surface);
        // ! Soft TODO
    }
}

delegate_compositor!(GyakuState);
