#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate lazy_static;

mod bsp;

use panic_halt as _;

use rtic::app;

use stm32h7xx_hal::gpio::{gpiob::PB4, gpioc::PC7};
use stm32h7xx_hal::gpio::{Edge, ExtiPin, Floating, Input};
use stm32h7xx_hal::gpio::{Output, PushPull};
use stm32h7xx_hal::hal::digital::v2::InputPin;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::hal::digital::v2::ToggleableOutputPin;
use stm32h7xx_hal::prelude::*;

use crate::bsp::board::Board;
use crate::bsp::pins::*;

#[app(device = stm32h7xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        button: PB4<Input<Floating>>,
        led: PC7<Output<PushPull>>,
    }

    /// Initialize all the peripherals.
    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // AN5212: Improve application performance when fetching instruction and
        // data, from both internal andexternal memories.
        cx.core.SCB.enable_icache();

        // RCC
        let pwrcfg = cx.device.PWR.constrain().freeze();
        let ccdr = cx
            .device
            .RCC
            .constrain()
            .sys_ck(100.mhz())
            .freeze(pwrcfg, &cx.device.SYSCFG);

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
        let mut led = pins.LED_USER.into_push_pull_output();

        init::LateResources { button, led }
    }

    #[task(binds = EXTI4, resources = [button, led])]
    fn button_click(ctx: button_click::Context) {
        ctx.resources.button.clear_interrupt_pending_bit();
        ctx.resources.led.toggle().unwrap();
    }
};
