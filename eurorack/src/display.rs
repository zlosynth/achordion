pub use achordion_lib::display::State as DisplayState;

use crate::system::{Led1, Led2, Led3, Led4, Led5, Led6, Led7, Led8};

pub struct DisplayConfig {
    pub led1: Led1,
    pub led2: Led2,
    pub led3: Led3,
    pub led4: Led4,
    pub led5: Led5,
    pub led6: Led6,
    pub led7: Led7,
    pub led_sharp: Led8,
}

pub struct Display {
    led1: Led1,
    led2: Led2,
    led3: Led3,
    led4: Led4,
    led5: Led5,
    led6: Led6,
    led7: Led7,
    led_sharp: Led8,
    minus_1_state: DisplayState,
    minus_2_state: DisplayState,
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
            minus_1_state: DisplayState::default(),
            minus_2_state: DisplayState::default(),
        }
    }

    pub fn set(&mut self, display_state: DisplayState) {
        if self.minus_1_state.led1 == display_state.led1
            && self.minus_2_state.led1 == display_state.led1
        {
            self.led1.set(display_state.led1);
        }
        if self.minus_1_state.led2 == display_state.led2
            && self.minus_2_state.led2 == display_state.led2
        {
            self.led2.set(display_state.led2);
        }
        if self.minus_1_state.led3 == display_state.led3
            && self.minus_2_state.led3 == display_state.led3
        {
            self.led3.set(display_state.led3);
        }
        if self.minus_1_state.led4 == display_state.led4
            && self.minus_2_state.led4 == display_state.led4
        {
            self.led4.set(display_state.led4);
        }
        if self.minus_1_state.led5 == display_state.led5
            && self.minus_2_state.led5 == display_state.led5
        {
            self.led5.set(display_state.led5);
        }
        if self.minus_1_state.led6 == display_state.led6
            && self.minus_2_state.led6 == display_state.led6
        {
            self.led6.set(display_state.led6);
        }
        if self.minus_1_state.led7 == display_state.led7
            && self.minus_2_state.led7 == display_state.led7
        {
            self.led7.set(display_state.led7);
        }
        if self.minus_1_state.led_sharp == display_state.led_sharp
            && self.minus_2_state.led_sharp == display_state.led_sharp
        {
            self.led_sharp.set(display_state.led_sharp);
        }

        self.minus_2_state = self.minus_1_state;
        self.minus_1_state = display_state;
    }
}
