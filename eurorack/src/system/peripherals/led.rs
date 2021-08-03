use crate::system::hal::hal::digital::v2::OutputPin;

pub struct Led<P> {
    pin: P,
}

impl<P: OutputPin> Led<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn set(&mut self, high: bool) {
        if high {
            self.pin.set_high().ok().unwrap();
        } else {
            self.pin.set_low().ok().unwrap();
        }
    }
}
