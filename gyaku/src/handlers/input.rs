use smithay::{
    backend::input::{Device, Event, InputBackend, InputEvent, KeyState, KeyboardKeyEvent},
    input::keyboard::{FilterResult, XkbConfig},
    utils::SERIAL_COUNTER,
};
use tracing::{debug, info};

use crate::state::GyakuState;

impl GyakuState {
    pub fn dispatch_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => self.handle_keyboard_key_event::<I>(event),
            InputEvent::DeviceAdded { device, .. } => self.handle_device_added_event::<I>(device),
            // todo axis is mouse scroll
            // InputEvent::DeviceRemoved { device, .. } => {
            //     println!("device removed: {:?}", device.name());
            //     // self.seat.remove_device(device);
            // }
            // _ => trace!("unhandled input event"),
            _ => {}
        }
    }

    fn handle_keyboard_key_event<I: InputBackend>(&mut self, event: I::KeyboardKeyEvent) {
        let serial = SERIAL_COUNTER.next_serial();
        let time = Event::time_msec(&event);

        if let Some(kb) = self.seat.get_keyboard() {
            kb.input::<(), _>(
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
        }
    }

    fn handle_device_added_event<I: InputBackend>(&mut self, device: I::Device) {
        info!("device added: {:?}", device.name());

        if device.has_capability(smithay::backend::input::DeviceCapability::Keyboard) {
            info!("device has keyboard capability");

            // TODO: How does hotplugging work? Multiple keyboards? How about these variables? Can we get them from the device?
            self.seat
                .add_keyboard(XkbConfig::default(), 200, 25)
                .unwrap();
        }
    }
}
