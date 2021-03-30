use core::convert::Infallible;

use stm32f3xx_hal::{
    gpio::{
        gpioe::{self, PE13},
        Output, PushPull,
    },
    prelude::*,
};

pub struct Led {
    led: PE13<Output<PushPull>>,
}

impl Led {
    pub fn new<T>(pe13: PE13<T>, moder: &mut gpioe::MODER, otyper: &mut gpioe::OTYPER) -> Self {
        let led = pe13.into_push_pull_output(moder, otyper);
        Self { led }
    }

    pub fn set_low(&mut self) -> Result<(), Infallible> {
        self.led.set_low()
    }

    pub fn set_high(&mut self) -> Result<(), Infallible> {
        self.led.set_high()
    }
}
