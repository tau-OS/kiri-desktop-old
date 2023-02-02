use smithay::{
    desktop::WindowSurfaceType,
    input::{pointer::{PointerHandle, GrabStartData}, Seat},
    utils::{Logical, Point, Serial},
};
use wayland_server::{protocol::wl_surface::WlSurface, Resource};

use crate::state::GyakuState;

// See https://github.com/Smithay/smithay/blob/e9bdcb982f9c242dfe7d1c3629be6c0a18a4a1ee/smallvil/src/state.rs#L143
// Not sure if this belonds in the input utils.. maybe as an extension trait?
pub fn surface_under_pointer(
    state: &GyakuState,
    pointer: &PointerHandle<GyakuState>,
) -> Option<(WlSurface, Point<i32, Logical>)> {
    let pos = pointer.current_location();
    state
        .space
        .element_under(pos)
        .and_then(|(window, location)| {
            window
                .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                .map(|(s, p)| (s, p + location))
        })
}

// https://github.com/Smithay/smithay/blob/e9bdcb982f9c242dfe7d1c3629be6c0a18a4a1ee/smallvil/src/handlers/xdg_shell.rs#L118
fn check_grab(
    seat: &Seat<GyakuState>,
    surface: &WlSurface,
    serial: Serial,
) -> Option<GrabStartData<GyakuState>> {
    let pointer = seat.get_pointer()?;

    // Check that this surface has a click grab.
    if !pointer.has_grab(serial) {
        return None;
    }

    let start_data = pointer.grab_start_data()?;

    let (focus, _) = start_data.focus.as_ref()?;
    // If the focus was for a different surface, ignore the request.
    if !focus.id().same_client_as(&surface.id()) {
        return None;
    }

    Some(start_data)
}
