use std::sync::Arc;

use smithay::{
    backend::input::{
        AbsolutePositionEvent, Axis, AxisSource, ButtonState, Device, DeviceCapability, Event,
        InputBackend, InputEvent, KeyState, KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent,
    },
    input::{
        keyboard::{FilterResult, XkbConfig},
        pointer::{AxisFrame, ButtonEvent, MotionEvent},
    },
    utils::SERIAL_COUNTER,
};
use tracing::{debug, info, trace};
use wayland_server::protocol::wl_surface::WlSurface;

use crate::{state::GyakuState, util::input::surface_under_pointer};

impl GyakuState {
    pub fn dispatch_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => self.handle_keyboard_key_event::<I>(event),
            InputEvent::DeviceAdded { device, .. } => self.handle_device_added_event::<I>(device),
            InputEvent::PointerButton { event, .. } => self.handle_pointer_button_event::<I>(event),
            InputEvent::PointerMotion { event, .. } => {
                // ! Soft TODO: wtf is this
            }
            InputEvent::PointerMotionAbsolute { event, .. } => {
                self.handle_pointer_motion_absolute_event::<I>(event)
            }
            InputEvent::PointerAxis { event } => self.handle_pointer_scroll_event::<I>(event),
            // todo axis is mouse scroll
            // InputEvent::DeviceRemoved { device, .. } => {
            //     println!("device removed: {:?}", device.name());
            //     // self.seat.remove_device(device);
            // }
            // _ => trace!("unhandled input event"),
            _ => {}
        }
    }

    fn handle_pointer_scroll_event<I: InputBackend>(&mut self, event: I::PointerAxisEvent) {
        let pointer = self.seat.get_pointer().unwrap();
        let source = event.source();

        let amount_x = event
            .amount(Axis::Horizontal)
            .unwrap_or_else(|| event.amount_discrete(Axis::Horizontal).unwrap() * 3.0);
        let amount_y = event
            .amount(Axis::Vertical)
            .unwrap_or_else(|| event.amount_discrete(Axis::Vertical).unwrap() * 3.0);

        let mut frame = AxisFrame::new(event.time_msec()).source(source);

        if amount_x != 0.0 {
            frame = frame.value(Axis::Horizontal, amount_x);

            if let Some(discrete) = event.amount_discrete(Axis::Horizontal) {
                frame = frame.discrete(Axis::Horizontal, discrete as i32);
            }
        } else if source == AxisSource::Finger {
            frame = frame.stop(Axis::Horizontal);
        }

        if amount_y != 0.0 {
            frame = frame.value(Axis::Vertical, amount_y);

            if let Some(discrete) = event.amount_discrete(Axis::Vertical) {
                frame = frame.discrete(Axis::Vertical, discrete as i32);
            }
        } else if source == AxisSource::Finger {
            frame = frame.stop(Axis::Vertical);
        }

        // We will actually invert the Y axis here, because that's what most users expect
        // todo: Add an option for "natural scrolling" (don't actually invert)
        pointer.axis(self, frame)
    }

    fn handle_keyboard_key_event<I: InputBackend>(&mut self, event: I::KeyboardKeyEvent) {
        let serial = SERIAL_COUNTER.next_serial();
        let time = Event::time_msec(&event);

        // We assume that keyboard events are only sent when a keyboard is present... if not that's cursed
        let keyboard = self.seat.get_keyboard().unwrap();

        // todo: handle keyboard shortcuts
        keyboard.input::<(), _>(
            self,
            event.key_code(),
            event.state(),
            serial,
            time,
            |_, modifiers, handle| {
                let keysym = handle.modified_sym();

                debug!(state = ?event.state(),
                        ?modifiers,
                        keysym = ?::xkbcommon::xkb::keysym_get_name(keysym),
                        "Keyboard event recieved");

                if let KeyState::Pressed = event.state() {
                    // todo: check inhibitors
                    FilterResult::Forward
                } else {
                    FilterResult::Forward
                }

                // FilterResult::Forward
            },
        );

        // todo: ok... now to actually send these events to a client, so it actually does something
    }

    fn handle_device_added_event<I: InputBackend>(&mut self, device: I::Device) {
        info!("device added: {:?}", device.name());

        if device.has_capability(DeviceCapability::Keyboard) {
            info!("device has keyboard capability");

            // TODO: How does hotplugging work? Multiple keyboards? How about these variables? Can we get them from the device?
            self.seat
                .add_keyboard(XkbConfig::default(), 200, 25)
                .unwrap();
        }

        if device.has_capability(DeviceCapability::Pointer) {
            info!("device has pointer capability");
            self.seat.add_pointer();
        }

        if device.has_capability(DeviceCapability::Touch) {
            info!("device has touch capability");
            self.seat.add_touch();
        }
    }

    // TODO: this is almost an exact copy of the pointer_button event handler from smallvil, not clean
    fn handle_pointer_button_event<I: InputBackend>(&mut self, event: I::PointerButtonEvent) {
        let serial = SERIAL_COUNTER.next_serial();
        let button_state = event.state();

        debug!(state = ?event.state(), "Pointer button event recieved");

        let pointer = self.seat.get_pointer().unwrap();

        if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
            if let Some((window, _loc)) = self
                .space
                .element_under(pointer.current_location())
                .map(|(w, l)| (w.clone(), l))
            {
                self.space.raise_element(&window, true);
                if let Some(kb) = self.seat.get_keyboard() {
                    kb.set_focus(self, Some(window.toplevel().wl_surface().clone()), serial);
                }
                self.space.elements().for_each(|window| {
                    window.toplevel().send_configure();
                });

                info!(window = ?window, "focusing desktop window");
            } else {
                self.space.elements().for_each(|window| {
                    window.set_activated(false);
                    window.toplevel().send_configure();
                });
                if let Some(kb) = self.seat.get_keyboard() {
                    kb.set_focus(self, Option::<WlSurface>::None, serial);
                }

                info!("unfocusing all windows");
            }
        };

        pointer.button(
            self,
            &ButtonEvent {
                button: event.button_code(),
                state: button_state,
                serial,
                time: event.time_msec(),
            },
        );
    }

    // TODO: similar to above, this is ported from smallvil...
    fn handle_pointer_motion_absolute_event<I: InputBackend>(
        &mut self,
        event: I::PointerMotionAbsoluteEvent,
    ) {
        let serial = SERIAL_COUNTER.next_serial();
        let pointer = self.seat.get_pointer().unwrap();

        let output = self.space.outputs().next().unwrap();
        let output_geometry = self.space.output_geometry(output).unwrap();

        let position =
            event.position_transformed(output_geometry.size) + output_geometry.loc.to_f64();

        // commenting this out, because logging this is spammy and kills performance
        // trace!(target: "pointer_location", new_position = ?position, "Pointer motion absolute event recieved");

        let under = surface_under_pointer(&self, &pointer);

        // todo: this is actually kinda slow, we should probably make this async somehow or only fetch x times per second
        pointer.motion(
            self,
            under,
            &MotionEvent {
                location: position,
                serial,
                time: event.time_msec(),
            },
        );
    }
}
