use stm32h7xx_hal as hal;

use super::pins::*;

static mut TAKEN: bool = false;

pub struct Board;

impl Board {
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { TAKEN } { None } else { Some(Board) })
    }

    pub fn split_gpios(
        &self,
        gpioa: hal::gpio::gpioa::Parts,
        gpiob: hal::gpio::gpiob::Parts,
        gpioc: hal::gpio::gpioc::Parts,
        gpiod: hal::gpio::gpiod::Parts,
    ) -> Pins {
        Pins {
            PIN_BUTTON: gpiob.pb4,
            PIN_CV1: gpioc.pc1,
            PIN_CV2: gpioa.pa6,
            PIN_CV3: gpioc.pc0,
            PIN_CV4: gpioa.pa3,
            PIN_CV5: gpiob.pb1,
            PIN_CV6: gpioa.pa7,
            PIN_LED1: gpiob.pb15,
            PIN_LED2: gpiob.pb14,
            PIN_LED3: gpiod.pd11,
            PIN_LED4: gpioa.pa0,
            PIN_LED5: gpioc.pc9,
            PIN_LED6: gpioc.pc8,
            PIN_LED7: gpiod.pd2,
            PIN_LED8: gpioc.pc12,
            PIN_POT1: gpioa.pa4,
            PIN_POT2: gpioa.pa1,
            PIN_POT3: gpioa.pa5,
            PIN_POT4: gpioc.pc4,
            PIN_PROBE: gpiob.pb5,

            LED_USER: gpioc.pc7,
        }
    }
}
