#![no_main]
#![no_std]

use core::convert::Infallible;

use panic_halt as _;
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};
use stm32f3xx_hal::{
    gpio::{
        gpioe::{MODER, OTYPER, PE13},
        Output, PushPull,
    },
    prelude::*,
};

const PERIOD: u32 = 8_000_000;

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
    }

    #[init(schedule = [toggle_led])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.constrain();
        let mut gpioe = cx.device.GPIOE.split(&mut rcc.ahb);

        let mut led = Led::new(gpioe.pe13, &mut gpioe.moder, &mut gpioe.otyper);
        led.set_low().unwrap();

        let now = cx.start;
        cx.schedule.toggle_led(now + PERIOD.cycles()).unwrap();

        init::LateResources { led }
    }

    #[task(schedule = [toggle_led], resources = [led])]
    fn toggle_led(cx: toggle_led::Context) {
        let now = Instant::now();
        cx.resources.led.toggle().unwrap();
        cx.schedule.toggle_led(now + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};

pub struct Led {
    led: PE13<Output<PushPull>>,
}

impl Led {
    pub fn new<T>(pe13: PE13<T>, moder: &mut MODER, otyper: &mut OTYPER) -> Self {
        let led = pe13.into_push_pull_output(moder, otyper);
        Self { led }
    }

    pub fn set_low(&mut self) -> Result<(), Infallible> {
        self.led.set_low()
    }

    pub fn toggle(&mut self) -> Result<(), Infallible> {
        self.led.toggle()
    }
}
