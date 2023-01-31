use crate::state::GyakuState;
use smithay::{
    delegate_seat, input::SeatHandler, reexports::wayland_server::protocol::wl_surface::WlSurface,
};

impl SeatHandler for GyakuState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut smithay::input::SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _focused: Option<&Self::KeyboardFocus>,
    ) {
        // ! Soft TODO
    }

    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
        // ! Soft TODO
    }
}

delegate_seat!(GyakuState);
