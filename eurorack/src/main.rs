#![no_main]
#![no_std]

mod hal;
mod wavetable;

use panic_halt as _;
use rtic::app;

use crate::hal::dma::_2::CHANNEL3;
use crate::hal::dma::{Direction, Event, Increment, Priority};
use crate::hal::prelude::*;

const SAMPLE_RATE: u32 = 44_100;

const DMA_LENGTH: usize = 64;
static mut DMA_BUFFER: [u32; DMA_LENGTH] = [0; DMA_LENGTH];

#[app(device = stm32f3::stm32f303, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        dma: CHANNEL3,
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
        let mut dma = cx.device.DMA2.split(&mut rcc.ahb).ch3;

        let mut tim2 = tim2.into_periodic(SAMPLE_RATE);

        dac.disable_buffer();
        dac.set_trigger_tim2();
        dac.enable_dma();
        dac.enable();

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

        dma.enable();
        tim2.enable();

        let mut _button = gpioa.pa0.into_pull_down(&mut gpioa.moder, &mut gpioa.pupdr);

        init::LateResources { dma }
    }

    #[task(binds = DMA2_CH3, resources = [dma, frequency])]
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

    extern "C" {
        fn EXTI0();
    }
};

fn audio_callback(buffer: &mut [u32; DMA_LENGTH], length: usize, offset: usize, frequency: f32) {
    static mut PHASE: f32 = 0.;
    let mut phase = unsafe { PHASE };

    let wt_length = wavetable::LENGTH;
    let wt_sin = wavetable::SIN;
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
