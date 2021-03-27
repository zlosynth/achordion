#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};

const PERIOD: u32 = 8_000_000;

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        gpioe: stm32f3xx_hal::pac::GPIOE,
    }

    #[init(schedule = [toggle_led])]
    fn init(cx: init::Context) -> init::LateResources {
        cx.device
            .RCC
            .ahbenr
            .write(|w| w.iopeen().set_bit());

        cx.device.GPIOE.moder.write(|w| w.moder8().output());

        let now = cx.start;
        cx.schedule.toggle_led(now + PERIOD.cycles()).unwrap();

        init::LateResources {
            gpioe: cx.device.GPIOE,
        }
    }

    #[task(schedule = [toggle_led], resources = [gpioe])]
    fn toggle_led(cx: toggle_led::Context) {
        static mut ENABLED: bool = false;

        let now = Instant::now();

        if *ENABLED {
            cx.resources.gpioe.odr.modify(|_, w| w.odr8().clear_bit());
        } else {
            cx.resources.gpioe.odr.modify(|_, w| w.odr8().set_bit());
        }

        *ENABLED = !*ENABLED;

        cx.schedule.toggle_led(now + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
