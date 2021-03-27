#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};
use rtic::export::NVIC;

const PERIOD: u32 = 8_000_000;

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        gpioa: stm32f3xx_hal::pac::GPIOA,
        gpioe: stm32f3xx_hal::pac::GPIOE,
        exti: stm32f3xx_hal::pac::EXTI,
    }

    #[init(schedule = [toggle_led])]
    fn init(mut cx: init::Context) -> init::LateResources {
        cx.core.DWT.enable_cycle_counter();

        cx.device
            .RCC
            .ahbenr
            .write(|w| w.iopaen().set_bit().iopeen().set_bit());
        cx.device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit()); // enable clock for SYSCFG

        cx.device
            .GPIOE
            .moder
            .write(|w| w.moder9().output().moder8().output());

        cx.device.GPIOA.moder.modify(|_, w| w.moder0().input()); // moder0 corresponds to pin 0 on GPIOA
        cx.device
            .GPIOA
            .pupdr
            .modify(|_, w| unsafe { w.pupdr0().bits(0b10) }); // set mode to pull-down
        cx.device
            .SYSCFG
            .exticr1
            .modify(|_, w| unsafe { w.exti0().bits(0b000) }); // connect EXTI0 to PA0 pin
        cx.device.EXTI.imr1.modify(|_, w| w.mr0().set_bit()); // unmask interrupt
        cx.device.EXTI.rtsr1.modify(|_, w| w.tr0().set_bit()); // trigger on rising-edge
        unsafe {
            NVIC::unmask(stm32f3xx_hal::pac::Interrupt::EXTI0);
        }

        let now = cx.start;
        cx.schedule.toggle_led(now + PERIOD.cycles()).unwrap();

        init::LateResources {
            gpioa: cx.device.GPIOA,
            gpioe: cx.device.GPIOE,
            exti: cx.device.EXTI,
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

    #[task(binds = EXTI0, resources = [gpioe, exti])]
    fn button_click(cx: button_click::Context) {
        cx.resources.exti.pr1.modify(|_, w| w.pr0().set_bit()); // clear the EXTI line 0 pending bit

        cx.resources.gpioe.odr.modify(|r, w| {
            let led4 = r.odr9().bit();
            if led4 {
                w.odr9().clear_bit()
            } else {
                w.odr9().set_bit()
            }
        });
    }

    extern "C" {
        fn EXTI1();
    }
};
