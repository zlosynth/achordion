use crate::system::hal::hal::digital::v2::InputPin;

pub struct Button<P> {
    pin: P,
}

impl<P: InputPin> Button<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn active(&self) -> bool {
        self.pin.is_low().ok().unwrap()
    }
}
