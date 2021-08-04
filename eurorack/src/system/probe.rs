use crate::system::hal::hal::digital::v2::OutputPin;

use achordion_lib::probe::{ProbeGenerator, PROBE_SEQUENCE};

pub struct Probe<P> {
    pin: P,
    generator: ProbeGenerator<'static>,
}

impl<P: OutputPin> Probe<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,

            generator: ProbeGenerator::new(&PROBE_SEQUENCE),
        }
    }

    pub fn tick(&mut self) {
        if self.generator.read() {
            self.pin.set_high().ok().unwrap();
        } else {
            self.pin.set_low().ok().unwrap();
        }
    }
}
