use smithay::{
    backend::input::{Device, Event, InputBackend, InputEvent, KeyboardKeyEvent},
    input::keyboard::FilterResult,
    utils::SERIAL_COUNTER,
};

use crate::state::GyakuState;

impl GyakuState {
    pub fn dispatch_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => self.handle_keyboard_key_event::<I>(event),
            InputEvent::DeviceAdded { device, .. } => self.handle_device_added_event::<I>(device),
            // InputEvent::DeviceRemoved { device, .. } => {
            //     println!("device removed: {:?}", device.name());
            //     // self.seat.remove_device(device);
            // }
            _ => println!("owooowow"),
        }
    }

    fn handle_keyboard_key_event<I: InputBackend>(&mut self, event: I::KeyboardKeyEvent) {
        let serial = SERIAL_COUNTER.next_serial();
        let time = Event::time_msec(&event);

        self.seat.get_keyboard().unwrap().input::<(), _>(
            self,
            event.key_code(),
            event.state(),
            serial,
            time,
            |_, _, _| FilterResult::Forward,
        );
    }

    fn handle_device_added_event<I: InputBackend>(&mut self, device: I::Device) {
        println!("device added: {:?}", device.name());
        // self.seat.add_device(device);
    }
}
