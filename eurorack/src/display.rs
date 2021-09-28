pub use achordion_lib::display::State as DisplayState;

use crate::system::{Led1, Led2, Led3, Led4, Led5, Led6, Led7, Led8};

pub struct DisplayConfig {
    pub led1: Led4,
    pub led2: Led8,
    pub led3: Led3,
    pub led4: Led7,
    pub led5: Led2,
    pub led6: Led6,
    pub led7: Led1,
    pub led_sharp: Led5,
}

pub struct Display {
    led1: Led4,
    led2: Led8,
    led3: Led3,
    led4: Led7,
    led5: Led2,
    led6: Led6,
    led7: Led1,
    led_sharp: Led5,
}

impl Display {
    pub fn new(config: DisplayConfig) -> Self {
        Self {
            led1: config.led1,
            led2: config.led2,
            led3: config.led3,
            led4: config.led4,
            led5: config.led5,
            led6: config.led6,
            led7: config.led7,
            led_sharp: config.led_sharp,
        }
    }

    pub fn set(&mut self, display_state: DisplayState) {
        self.led1.set(display_state.led1);
        self.led2.set(display_state.led2);
        self.led3.set(display_state.led3);
        self.led4.set(display_state.led4);
        self.led5.set(display_state.led5);
        self.led6.set(display_state.led6);
        self.led7.set(display_state.led7);
        self.led_sharp.set(display_state.led_sharp);
    }
}
