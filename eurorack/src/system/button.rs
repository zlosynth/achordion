use crate::system::hal::hal::digital::v2::InputPin;

use super::debounce_buffer::DebounceBuffer;

pub struct Button<P> {
    pin: P,
    debounce_filter: DebounceBuffer<5>,
    active: bool,
    clicked: bool,
}

impl<P: InputPin> Button<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            debounce_filter: DebounceBuffer::new(),
            active: false,
            clicked: false,
        }
    }

    pub fn sample(&mut self) {
        let was_active = self.active;
        self.debounce_filter.write(self.pin.is_low().ok().unwrap());
        self.active = self.debounce_filter.read();
        self.clicked = !was_active && self.active;
    }

    pub fn active(&self) -> bool {
        self.debounce_filter.read()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}
