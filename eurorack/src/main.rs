#![no_main]
#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unknown_clippy_lints)]

mod hal;

use panic_halt as _;
use rtic::app;

use achordion_lib::wavetable;

use crate::hal::dma::_2::CHANNEL3;
use crate::hal::dma::{Direction, Event, Increment, Priority};
use crate::hal::exti::Exti;
use crate::hal::gpio::a::PA0;
use crate::hal::gpio::{Edge, Input};
use crate::hal::prelude::*;

const SAMPLE_RATE: u32 = 44_100;

const DMA_LENGTH: usize = 64;
static mut DMA_BUFFER: [u32; DMA_LENGTH] = [0; DMA_LENGTH];

#[app(device = stm32f3::stm32f303, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        exti: Exti,
        dma: CHANNEL3,
        button: PA0<Input>,

        #[init(40.0)]
        frequency: f32,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.constrain();
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let tim2 = cx.device.TIM2.constrain(&mut rcc.apb1);
        let mut dac = cx.device.DAC1.constrain(
            gpioa.pa4,
            gpioa.pa5,
            &mut rcc.apb1,
            &mut gpioa.moder,
            &mut gpioa.pupdr,
        );
        let mut syscfg = cx.device.SYSCFG.constrain(&mut rcc.apb2);
        let mut exti = cx.device.EXTI.constrain();
        let mut dma = cx.device.DMA2.split(&mut rcc.ahb).ch3;
        let mut flash = cx.device.FLASH.constrain();

        let _clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut tim2 = tim2.into_periodic(SAMPLE_RATE);

        // Configure DAC, disable buffer for better SNR and request data from DMA
        dac.disable_buffer();
        dac.set_trigger_tim2();
        dac.enable_dma();
        dac.enable();

        // Configure DMA for transfer between buffer and DAC
        let ma = unsafe { DMA_BUFFER.as_ptr() } as usize as u32; // source: memory address
        let pa = 0x40007420; // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
        let ndt = DMA_LENGTH as u16; // number of items to transfer
        dma.set_direction(Direction::FromMemory);
        unsafe {
            dma.set_memory_address(ma, Increment::Enable);
            dma.set_peripheral_address(pa, Increment::Disable);
        }
        dma.set_transfer_length(ndt);
        dma.set_word_size::<u32>();
        dma.set_circular(true);
        dma.set_priority_level(Priority::High);
        dma.listen(Event::Any);
        dma.unmask_interrupt();

        // Start DMA transfer to DAC
        dma.enable();
        tim2.enable();

        // Configure PA0 (blue button) to trigger an interrupt when clicked
        let mut button = gpioa.pa0.into_pull_down(&mut gpioa.moder, &mut gpioa.pupdr);
        button.interrupt_exti0(&mut syscfg);
        button.trigger_on_edge(&mut exti, Edge::Rising);
        button.unmask_exti0(&mut exti);

        init::LateResources { exti, dma, button }
    }

    #[task(priority = 2, binds = DMA2_CH3, resources = [dma, frequency])]
    fn dma2_ch3(cx: dma2_ch3::Context) {
        let event = {
            let event = cx.resources.dma.event();
            cx.resources.dma.clear_events();
            event
        };

        if let Some(event) = event {
            match event {
                Event::HalfTransfer => audio_callback(
                    unsafe { &mut DMA_BUFFER },
                    DMA_LENGTH / 2,
                    0,
                    *cx.resources.frequency,
                ),
                Event::TransferComplete => audio_callback(
                    unsafe { &mut DMA_BUFFER },
                    DMA_LENGTH / 2,
                    1,
                    *cx.resources.frequency,
                ),
                _ => (),
            }
        }
    }

    #[task(binds = EXTI0, resources = [button, frequency, exti])]
    fn exti0(mut cx: exti0::Context) {
        cx.resources.button.clear_exti0(&mut cx.resources.exti);

        cx.resources.frequency.lock(|frequency| {
            *frequency *= 1.5;
        });
    }

    extern "C" {
        fn EXTI1();
    }
};

fn audio_callback(buffer: &mut [u32; DMA_LENGTH], length: usize, offset: usize, frequency: f32) {
    static mut PHASE: f32 = 0.;
    let mut phase = unsafe { PHASE };

    let wt_length = wavetable::LENGTH;
    let wt_sin = wavetable::SINE;
    let wt_saw = wavetable::SAW;

    let dx = frequency * (1. / SAMPLE_RATE as f32);

    for t in 0..length {
        let index = (phase * wt_length as f32) as usize;
        let channel_1 = wt_sin[index] as u32;
        let channel_2 = wt_saw[index] as u32;

        let frame = t + (offset * length);
        buffer[frame] = (channel_2 << 16) + channel_1;

        phase += dx;
        if phase >= 1.0 {
            phase -= 1.0;
        }
    }

    unsafe {
        PHASE = phase;
    }
}
