#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]

mod bsp;
mod interface;

use panic_halt as _;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use hal::adc::Adc;
use hal::delay::DelayFromCountDownTimer;
use hal::prelude::*;
use hal::rcc::rec::AdcClkSel;
use stm32h7xx_hal as hal;

use crate::bsp::board::Board;
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

        let pwrcfg = cx.device.PWR.constrain().freeze();
        let rcc = cx.device.RCC.constrain();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        // We need to configure a clock for adc_ker_ck_input. The default
        // adc_ker_ck_input is pll2_p_ck, but we will use per_ck. Here we
        // set per_ck to 4MHz.
        //
        // The maximum adc_ker_ck_input frequency is 100MHz for revision V and 36MHz
        // otherwise
        let mut ccdr = rcc
            .sys_ck(100.mhz())
            .per_ck(4.mhz())
            .freeze(pwrcfg, &cx.device.SYSCFG);

        // Switch adc_ker_ck_input multiplexer to per_ck
        ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::PER);

        let board = Board::take().unwrap();
        let pins = board.split_gpios(
            cx.device.GPIOA.split(ccdr.peripheral.GPIOA),
            cx.device.GPIOB.split(ccdr.peripheral.GPIOB),
            cx.device.GPIOC.split(ccdr.peripheral.GPIOC),
            cx.device.GPIOD.split(ccdr.peripheral.GPIOD),
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
            pins.PIN_BUTTON,
            pins.PIN_POT1,
            pins.PIN_POT2,
            pins.PIN_POT3,
            pins.PIN_POT4,
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
