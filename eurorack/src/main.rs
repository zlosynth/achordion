#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]

mod bsp;
mod interface;

use panic_halt as _;

use cortex_m::peripheral::DWT;
use cortex_m_semihosting::hprintln;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use hal::adc::Adc;
use hal::delay::DelayFromCountDownTimer;
use hal::delay::Delay;
use hal::gpio::{gpiob::PB4, gpioc::PC7};
use hal::gpio::{Edge, ExtiPin, Floating, Input};
use hal::gpio::{Output, PushPull};
use hal::hal::digital::v2::OutputPin;
use hal::hal::digital::v2::ToggleableOutputPin;
use hal::rcc::rec::AdcClkSel;
use hal::prelude::*;
use stm32h7xx_hal as hal;

use crate::bsp::board::Board;
use crate::interface::Interface;

const CV_PERIOD: u32 = 1_000;

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        button: PB4<Input<Floating>>,
        led: PC7<Output<PushPull>>,
        interface: Interface,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [read_pots])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // AN5212: Improve application performance when fetching instruction and
        // data, from both internal andexternal memories.
        cx.core.SCB.enable_icache();

        let pwrcfg = cx.device.PWR.constrain().freeze();
        let rcc = cx
            .device
            .RCC
            .constrain();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();

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

        // Button
        let mut button = pins.PIN_BUTTON.into_floating_input();
        button.make_interrupt_source(&mut cx.device.SYSCFG);
        button.trigger_on_edge(&mut cx.device.EXTI, Edge::Rising);
        button.enable_interrupt(&mut cx.device.EXTI);

        // LED
        let led = pins.LED_USER.into_push_pull_output();

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
            pins.PIN_POT1,
            pins.PIN_POT2,
            pins.PIN_POT3,
            pins.PIN_POT4,
        );

        cx.schedule
            .read_pots(cx.start + CV_PERIOD.cycles())
            .unwrap();

        init::LateResources {
            button,
            led,
            interface,
        }
    }

    #[task(binds = EXTI4, resources = [button, led])]
    fn button_click(ctx: button_click::Context) {
        ctx.resources.button.clear_interrupt_pending_bit();
        ctx.resources.led.toggle().unwrap();
    }

    #[task(schedule = [read_pots], resources = [led, interface])]
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
