#![no_main]
#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::let_and_return)]

mod hal;

use panic_halt as _;
use rtic::app;

use stm32f3xx_hal::dma::dma2::C3;
use stm32f3xx_hal::dma::Channel;
use stm32f3xx_hal::dma::{self, Direction, Increment, Priority};
use stm32f3xx_hal::gpio::{Edge, Gpioa, Input, Pin};
use stm32f3xx_hal::pac::{Interrupt, NVIC, USART1};
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::serial::{self, Serial};
use stm32f3xx_hal::timer::Timer;
use typenum::UTerm;

use achordion_lib::midi::controller::Controller as MidiController;
use achordion_lib::oscillator::Oscillator;
use achordion_lib::wavetable;

use crate::hal::prelude::*;

const SAMPLE_RATE: u32 = 44_100;

const DMA_LENGTH: usize = 64;
static mut DMA_BUFFER: [u32; DMA_LENGTH] = [0; DMA_LENGTH];

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        dsp_dma: C3,
        button: Pin<Gpioa, UTerm, Input>,
        midi_rx: serial::Rx<USART1>,
        midi_controller: MidiController,
        oscillator: Oscillator<'static>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.constrain();
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.ahb);
        let mut exti = cx.device.EXTI;
        let mut syscfg = cx.device.SYSCFG.constrain(&mut rcc.apb2);
        let mut flash = cx.device.FLASH.constrain();
        let mut dma2 = cx.device.DMA2.split(&mut rcc.ahb);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // Configure DSP
        let dsp_dma = {
            // Start periodic timer on TIM2
            {
                let mut tim2 = Timer::tim2(cx.device.TIM2, SAMPLE_RATE.Hz(), clocks, &mut rcc.apb1);
                tim2.reset_on_overflow();
            }

            // Configure DAC, triggered by TIM2 and requesting data from DMA
            {
                let mut dac = cx.device.DAC1.constrain(
                    gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr),
                    gpioa.pa5.into_analog(&mut gpioa.moder, &mut gpioa.pupdr),
                    &mut rcc.apb1,
                    &mut gpioa.moder,
                    &mut gpioa.pupdr,
                );
                // disable buffer for better SNR
                dac.disable_buffer();
                dac.set_trigger_tim2();
                dac.enable_dma();
                dac.enable();
            }

            // Configure DMA for transfer between buffer and DAC
            let dsp_dma = {
                let ma = unsafe { DMA_BUFFER.as_ptr() } as usize as u32; // source: memory address
                let pa = 0x40007420; // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
                let ndt = DMA_LENGTH as u16; // number of items to transfer

                dma2.ch3.set_direction(Direction::FromMemory);
                unsafe {
                    dma2.ch3.set_memory_address(ma, Increment::Enable);
                    dma2.ch3.set_peripheral_address(pa, Increment::Disable);
                }
                dma2.ch3.set_transfer_length(ndt);
                dma2.ch3.set_word_size::<u32>();
                dma2.ch3.set_circular(true);
                dma2.ch3.set_priority_level(Priority::High);
                dma2.ch3.listen(dma::Event::Any);

                unsafe { NVIC::unmask(Interrupt::DMA2_CH3) };

                dma2.ch3.enable();

                dma2.ch3
            };

            dsp_dma
        };

        // Configure PA0 (blue button) to trigger an interrupt when clicked
        let button = {
            let mut button = gpioa
                .pa0
                .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);
            button.make_interrupt_source(&mut syscfg);
            button.trigger_on_edge(&mut exti, Edge::Rising);
            button.enable_interrupt(&mut exti);

            let interrupt_num = button.nvic();
            unsafe { NVIC::unmask(interrupt_num) };

            button
        };

        // Configure USART for MIDI
        let midi_rx = {
            let pins = (
                gpioc
                    .pc4
                    .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
                gpioc
                    .pc5
                    .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
            );

            let mut serial =
                Serial::usart1(cx.device.USART1, pins, 9600.Bd(), clocks, &mut rcc.apb2);
            serial.listen(serial::Event::Rxne);

            unsafe { NVIC::unmask(Interrupt::USART1_EXTI25) };

            let (_tx, rx) = serial.split();
            rx
        };

        init::LateResources {
            button,
            dsp_dma,
            midi_rx,
            midi_controller: MidiController::new(),
            oscillator: Oscillator::new(&wavetable::saw::SAW_FACTORS, SAMPLE_RATE),
        }
    }

    #[task(priority = 2, binds = DMA2_CH3, resources = [dsp_dma, oscillator])]
    fn dsp_request(cx: dsp_request::Context) {
        use dma::Event::*;

        let event = {
            let event = if cx.resources.dsp_dma.event_occurred(HalfTransfer) {
                Some(HalfTransfer)
            } else if cx.resources.dsp_dma.event_occurred(TransferComplete) {
                Some(TransferComplete)
            } else {
                None
            };
            cx.resources.dsp_dma.clear_event(Any);
            event
        };

        if let Some(event) = event {
            match event {
                HalfTransfer => audio_callback(
                    unsafe { &mut DMA_BUFFER },
                    DMA_LENGTH / 2,
                    0,
                    cx.resources.oscillator,
                ),
                TransferComplete => audio_callback(
                    unsafe { &mut DMA_BUFFER },
                    DMA_LENGTH / 2,
                    1,
                    cx.resources.oscillator,
                ),
                _ => (),
            }
        }
    }

    #[task(binds = USART1_EXTI25, resources = [oscillator, midi_rx, midi_controller])]
    fn midi_rx(mut cx: midi_rx::Context) {
        while let Ok(x) = cx.resources.midi_rx.read() {
            if let Some(state) = cx.resources.midi_controller.reconcile_byte(x) {
                cx.resources.oscillator.lock(|oscillator| {
                    oscillator.frequency = state.frequency;
                });
            }
        }
    }

    #[task(binds = EXTI0, resources = [button, oscillator])]
    fn button_click(mut cx: button_click::Context) {
        cx.resources.button.clear_interrupt_pending_bit();
        cx.resources.oscillator.lock(|oscillator| {
            oscillator.frequency *= 1.5;
        });
    }

    extern "C" {
        fn EXTI1();
    }
};

fn audio_callback(
    buffer: &mut [u32; DMA_LENGTH],
    length: usize,
    offset: usize,
    oscillator: &mut Oscillator,
) {
    let mut buffer_osc = [0; DMA_LENGTH / 2];
    oscillator.populate(&mut buffer_osc);

    for (i, x) in buffer[offset * length..offset * length + length]
        .iter_mut()
        .enumerate()
    {
        let value = buffer_osc[i] as u32;
        *x = (value << 16) + value;
    }
}
