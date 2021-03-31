#![no_main]
#![no_std]

#[macro_use]
extern crate lazy_static;

mod led;
mod midi;
mod wavetable;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;

use panic_halt as _;
use rtic::app;
use stm32f3xx_hal::{prelude::*, usb::UsbBusType};
use usb_device::bus::UsbBusAllocator;
use usbd_midi::{
    data::{
        midi::{message::Message, notes::Note},
        usb_midi::{
            midi_packet_reader::MidiPacketBufferReader, usb_midi_event_packet::UsbMidiEventPacket,
        },
    },
    midi_device::MAX_PACKET_SIZE,
};

use crate::led::Led;
use crate::midi::Midi;

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

const DMA_LENGTH: usize = 64;
static mut DMA_BUFFER: [u32; DMA_LENGTH] = [0; DMA_LENGTH];

lazy_static! {
    static ref MUTEX_DMA2: Mutex<RefCell<Option<stm32f3xx_hal::pac::DMA2>>> =
        Mutex::new(RefCell::new(None));
}

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
        midi: Midi,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // init tim2
        cx.device.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());
        // calculate timer frequency
        let sysclk = 8_000_000; // the stmf32f3 discovery board CPU runs at 8Mhz by default
        let fs = 44_100; // we want an audio sampling rate of 44.1KHz
        let arr = sysclk / fs; // value to use for auto reload register (arr)
                               // configure TIM2
        cx.device.TIM2.cr2.write(|w| w.mms().update()); // update when counter reaches arr value
        cx.device.TIM2.arr.write(|w| w.arr().bits(arr)); // set timer period (sysclk / fs)
                                                         // enable TIM2
        cx.device.TIM2.cr1.modify(|_, w| w.cen().enabled());

        // enable GPIOA and DAC clocks
        cx.device.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());
        cx.device.RCC.apb1enr.modify(|_, w| w.dac1en().set_bit());
        cx.device.RCC.apb1enr.modify(|_, w| w.dac2en().set_bit());

        // configure PA04, PA05 (DAC_OUT1 & DAC_OUT2) as analog, floating
        cx.device
            .GPIOA
            .moder
            .modify(|_, w| w.moder4().analog().moder5().analog());
        cx.device
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr4().floating().pupdr5().floating());

        // configure DAC
        cx.device.DAC.cr.write(|w| {
            w.boff1()
                .disabled() // disable dac output buffer for channel 1
                .boff2()
                .disabled() // disable dac output buffer for channel 2
                .ten1()
                .enabled() // enable trigger for channel 1
                .ten2()
                .enabled() // enable trigger for channel 2
                .tsel1()
                .tim2_trgo() // set trigger for channel 1 to TIM2
                .tsel2()
                .tim2_trgo()
        }); // set trigger for channel 2 to TIM2

        // enable DAC
        cx.device.DAC.cr.modify(|_, w| {
            w.en1()
                .enabled() // enable dac channel 1
                .en2()
                .enabled()
        }); // enable dac channel 2

        // init dma2
        cx.device.RCC.ahbenr.modify(|_, w| w.dma2en().set_bit());

        // dma parameters
        let ma = unsafe { DMA_BUFFER.as_ptr() } as usize as u32; // source: memory address
        let pa = 0x40007420; // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
        let ndt = DMA_LENGTH as u16; // number of items to transfer

        // configure and enable DMA2 channel 3
        cx.device.DMA2.ch3.mar.write(|w| unsafe { w.ma().bits(ma) }); // source memory address
        cx.device.DMA2.ch3.par.write(|w| unsafe { w.pa().bits(pa) }); // destination peripheral address
        cx.device.DMA2.ch3.ndtr.write(|w| w.ndt().bits(ndt)); // number of items to transfer

        cx.device.DMA2.ch3.cr.write(|w| {
            w.dir()
                .from_memory() // source is memory
                .mem2mem()
                .disabled() // disable memory to memory transfer
                .minc()
                .enabled() // increment memory address every transfer
                .pinc()
                .disabled() // don't increment peripheral address every transfer
                .msize()
                .bits32() // memory word size is 32 bits
                .psize()
                .bits32() // peripheral word size is 32 bits
                .circ()
                .enabled() // dma mode is circular
                .pl()
                .high() // set dma priority to high
                .teie()
                .enabled() // trigger an interrupt if an error occurs
                .tcie()
                .enabled() // trigger an interrupt when transfer is complete
                .htie()
                .enabled() // trigger an interrupt when half the transfer is complete
        });

        // enable DMA interrupt
        #[allow(deprecated)]
        cx.core.NVIC.enable(stm32f3xx_hal::pac::interrupt::DMA2_CH3);

        // enable DMA for DAC
        cx.device.DAC.cr.modify(|_, w| w.dmaen1().enabled());

        // wrap shared peripherals
        let dma2 = cx.device.DMA2;
        cortex_m::interrupt::free(|cs| {
            MUTEX_DMA2.borrow(cs).replace(Some(dma2));
        });

        // start dma transfer
        cortex_m::interrupt::free(|cs| {
            let refcell = MUTEX_DMA2.borrow(cs).borrow();
            let dma2 = refcell.as_ref().unwrap();
            dma2.ch3.cr.modify(|_, w| w.en().enabled());
        });

        let mut rcc = cx.device.RCC.constrain();
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut gpioe = cx.device.GPIOE.split(&mut rcc.ahb);
        let mut flash = cx.device.FLASH.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8u32.mhz())
            .sysclk(48u32.mhz())
            .pclk1(24u32.mhz())
            .pclk2(24u32.mhz())
            .freeze(&mut flash.acr);

        let mut led = Led::new(gpioe.pe13, &mut gpioe.moder, &mut gpioe.otyper);
        led.set_low().unwrap();

        let midi = Midi::new(
            gpioa.pa11,
            gpioa.pa12,
            cx.device.USB,
            &clocks,
            &mut gpioa.moder,
            &mut gpioa.afrh,
            &mut gpioa.otyper,
            unsafe { &mut USB_BUS },
        );

        init::LateResources { led, midi }
    }

    #[task(binds = USB_HP_CAN_TX, resources = [midi, led])]
    fn usb_tx(mut cx: usb_tx::Context) {
        midi_poll(&mut cx.resources.midi, &mut cx.resources.led);
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [midi, led])]
    fn usb_rx0(mut cx: usb_rx0::Context) {
        midi_poll(&mut cx.resources.midi, &mut cx.resources.led);
    }

    #[task(binds = DMA2_CH3)]
    fn dma2_ch3(_: dma2_ch3::Context) {
        // determine interrupt event
        let isr = cortex_m::interrupt::free(|cs| {
            let refcell = MUTEX_DMA2.borrow(cs).borrow();
            let dma2 = refcell.as_ref();

            // cache interrupt state register (before we clear the flags!)
            let isr = dma2.unwrap().isr.read();

            // clear interrupt flags
            dma2.unwrap()
                .ifcr
                .write(|w| w.ctcif3().clear().chtif3().clear().cteif3().clear());

            isr
        });

        // handle interrupt events
        if isr.htif3().is_half() {
            audio_callback(unsafe { &mut DMA_BUFFER }, DMA_LENGTH / 2, 0);
        } else if isr.tcif3().is_complete() {
            audio_callback(unsafe { &mut DMA_BUFFER }, DMA_LENGTH / 2, 1);
        // } else if isr.teif3().is_error() {
        // handle dma error
        } else {
            // handle unknown interrupt
        }
    }

    extern "C" {
        fn EXTI0();
    }
};

fn midi_poll(midi: &mut Midi, led: &mut Led) {
    if !midi.usb_device.poll(&mut [&mut midi.midi_class]) {
        return;
    }

    let mut buffer = [0u8; MAX_PACKET_SIZE];

    if let Ok(size) = midi.midi_class.read(&mut buffer) {
        let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
        for packet in buffer_reader.into_iter() {
            if let Ok(packet) = packet {
                process_midi_message(packet, led);
            }
        }
    }
}

fn process_midi_message(packet: UsbMidiEventPacket, led: &mut Led) {
    match packet.message {
        Message::NoteOn(_, Note::C2, ..) => {
            led.set_high().unwrap();
        }
        Message::NoteOff(_, Note::C2, ..) => {
            led.set_low().unwrap();
        }
        _ => {}
    }
}

fn audio_callback(buffer: &mut [u32; DMA_LENGTH], length: usize, offset: usize) {
    static mut PHASE: f32 = 0.;
    let mut phase = unsafe { PHASE };

    let wt_length = wavetable::LENGTH;
    let wt_sin = wavetable::SIN;
    let wt_saw = wavetable::SAW;

    let dx = 261.6 * (1. / 44100.); // 261.6 Hz = Middle-C

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
