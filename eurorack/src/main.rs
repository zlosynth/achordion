#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]

mod interface;

use panic_halt as _;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::Adc;
use hal::delay::DelayFromCountDownTimer;
use hal::pac::DWT;
use hal::prelude::*;

use crate::interface::Interface;

const CV_PERIOD: u32 = 1_000;

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        interface: Interface,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [read_pots])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // AN5212: Improve application performance when fetching instruction and
        // data, from both internal andexternal memories.
        cx.core.SCB.enable_icache();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        let board = daisy::Board::take().unwrap();

        let rcc = cx.device.RCC.constrain().pll2_p_ck(4.mhz());
        let ccdr = board.freeze_clocks(cx.device.PWR.constrain(), rcc, &cx.device.SYSCFG);

        let pins = board.split_gpios(
            cx.device.GPIOA.split(ccdr.peripheral.GPIOA),
            cx.device.GPIOB.split(ccdr.peripheral.GPIOB),
            cx.device.GPIOC.split(ccdr.peripheral.GPIOC),
            cx.device.GPIOD.split(ccdr.peripheral.GPIOD),
            cx.device.GPIOE.split(ccdr.peripheral.GPIOE),
            cx.device.GPIOF.split(ccdr.peripheral.GPIOF),
            cx.device.GPIOG.split(ccdr.peripheral.GPIOG),
        );

        let mut delay = DelayFromCountDownTimer::new(cx.device.TIM2.timer(
            10.ms(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));
        let adc1 = Adc::adc1(
            cx.device.ADC1,
            &mut delay,
            ccdr.peripheral.ADC12,
            &ccdr.clocks,
        );
        let interface = Interface::new(
            adc1,
            pins.SEED_PIN_9.into_pull_up_input(),
            pins.SEED_PIN_23,
            pins.SEED_PIN_24,
            pins.SEED_PIN_22,
            pins.SEED_PIN_21,
        );

        cx.schedule
            .read_pots(cx.start + CV_PERIOD.cycles())
            .unwrap();

        init::LateResources { interface }
    }

    #[task(schedule = [read_pots], resources = [interface])]
    fn read_pots(cx: read_pots::Context) {
        cx.resources.interface.sample();

        cx.schedule
            .read_pots(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
