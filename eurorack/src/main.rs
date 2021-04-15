//! # Pins and addresses
//!
//! * PD4 -> DAC ~RESET (pulled low)
//!
//! * PB9 -> SDA (pulled high)
//! * PB6 -> SCL (pulled high)
//!
//! * PC7 -> MCLK
//! * PC10 -> SCK (bit clock)
//! * PC12 -> SD
//! * PA4 -> WS
//!
//! DAC I2C address 0x94

#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

mod cs43l22;
mod hal;

use core::convert::TryInto;

use panic_halt as _;

use rtic::app;

use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::dma::config::Priority;
use stm32f4xx_hal::dma::traits::{PeriAddress, Stream};
use stm32f4xx_hal::dma::{Channel0, MemoryToPeripheral, Stream5, StreamsTuple};
use stm32f4xx_hal::gpio::gpioa::PA0;
use stm32f4xx_hal::gpio::gpiod::{PD12, PD14, PD15};
use stm32f4xx_hal::gpio::{Edge, Input, Output, PullDown, PushPull};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::pac::DMA1;
use stm32f4xx_hal::prelude::*;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

use usbd_midi::data::usb::constants::*;
use usbd_midi::data::usb_midi::midi_packet_reader::MidiPacketBufferReader;
use usbd_midi::midi_device::MidiClass;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity};

use achordion_lib::midi::instrument::Instrument as MidiInstrument;
use achordion_lib::midi::note::Note as MidiNote;
use achordion_lib::oscillator::Oscillator;
use achordion_lib::quantizer;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::cs43l22::Cs43L22;
use crate::hal::prelude::*;
use crate::hal::stream::WordSize;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

// 7-bit address
const DAC_ADDRESS: u8 = 0x94 >> 1;

// Volume in decibels
const VOLUME: i8 = -100;

// Audio timing configuration:
// Sample rate 48 kHz
// 16 bits per sample -> SCK rate 1.536 MHz
// MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
const SAMPLE_RATE: u32 = 48000;

const BUFFER_SIZE: usize = 64 * 2;
static mut STEREO_BUFFER: [u16; BUFFER_SIZE * 2] = [16384; BUFFER_SIZE * 2];

lazy_static! {
    static ref SINE: Wavetable<'static> =
        Wavetable::new(&waveform::sine::SINE_FACTORS, SAMPLE_RATE);
    static ref TRIANGLE: Wavetable<'static> =
        Wavetable::new(&waveform::triangle::TRIANGLE_FACTORS, SAMPLE_RATE);
    static ref SQUARE: Wavetable<'static> =
        Wavetable::new(&waveform::square::SQUARE_FACTORS, SAMPLE_RATE);
    static ref SAW: Wavetable<'static> = Wavetable::new(&waveform::saw::SAW_FACTORS, SAMPLE_RATE);
    static ref WAVETABLES: [&'static Wavetable<'static>; 4] = [&SINE, &TRIANGLE, &SQUARE, &SAW];
}

// Static globals used to keep the state of MIDI interface
static mut USB_BUS: Option<UsbBusAllocator<UsbBus<USB>>> = None;
static mut USB_MIDI: Option<MidiClass<UsbBus<USB>>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus<USB>>> = None;

#[app(device = stm32f4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        stream: Stream5<DMA1>,
        green_led: PD12<Output<PushPull>>,
        blue_led: PD15<Output<PushPull>>,
        red_led: PD14<Output<PushPull>>,
        button: PA0<Input<PullDown>>,
        oscillator: Oscillator<'static>,
        midi_instrument: MidiInstrument,
    }

    /// Initialize all the peripherals.
    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        let mut syscfg = cx.device.SYSCFG.constrain();
        let rcc = cx.device.RCC.constrain();
        let gpioa = cx.device.GPIOA.split();
        let gpiob = cx.device.GPIOB.split();
        let gpioc = cx.device.GPIOC.split();
        let gpiod = cx.device.GPIOD.split();

        // Maximum system frequency supported by the board. The I2S clock should
        // align well with 48 khz sample rate.
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(168.mhz())
            .i2s_clk(86.mhz())
            .require_pll48clk()
            .freeze();

        // Status LEDs to indicate the state of the execution.
        let green_led = gpiod.pd12.into_push_pull_output();
        let blue_led = gpiod.pd15.into_push_pull_output();
        let red_led = gpiod.pd14.into_push_pull_output();

        // Configure Cirrus DAC.
        {
            let i2c = I2c::i2c1(
                cx.device.I2C1,
                (
                    gpiob.pb6.into_alternate_af4_open_drain(), // SDA
                    gpiob.pb9.into_alternate_af4_open_drain(), // SDC
                ),
                100.khz(),
                clocks,
            );

            let mut dac = Cs43L22::new(
                i2c,
                DAC_ADDRESS,
                gpiod.pd4.into_push_pull_output(), // DAC RESET
                Delay::new(cx.core.SYST, clocks),
            )
            .unwrap();

            dac.set_volume_a(VOLUME).unwrap();
            dac.set_volume_b(VOLUME).unwrap();

            dac.enable().unwrap();
        }

        // Configure I2S connected to the DAC.
        let mut i2s = {
            let i2s_pins = (
                gpioa.pa4.into_alternate_af6(),  // WS for I2S3
                gpioc.pc10.into_alternate_af6(), // CK
                gpioc.pc7.into_alternate_af6(),  // MCLK
                gpioc.pc12.into_alternate_af6(), // SD
            );
            let hal_i2s = I2s::i2s3(cx.device.SPI3, i2s_pins, clocks);
            let i2s_clock = hal_i2s.input_clock();

            let i2s = stm32_i2s_v12x::I2s::new(hal_i2s);
            let mut i2s = i2s.configure_master_transmit(MasterConfig::with_sample_rate(
                i2s_clock.0,
                SAMPLE_RATE,
                Data16Frame16,
                FrameFormat::PhilipsI2s,
                Polarity::IdleHigh,
                MasterClock::Enable,
            ));
            i2s.set_dma_enabled(true);

            i2s
        };

        // Configure DMA 1 stream 5 channel 0 reading from a circular buffer and
        // writting into I2S peripheral.
        let stream = {
            let source_address = unsafe { STEREO_BUFFER.as_ptr() } as u32;
            let destination_address = i2s.address();
            let items_to_transfer = unsafe { STEREO_BUFFER.len() as u16 };

            let dma1_streams = StreamsTuple::new(cx.device.DMA1);
            let mut stream = dma1_streams.5;
            stream.set_channel(Channel0);

            stream.set_priority(Priority::VeryHigh);

            stream.set_direction(MemoryToPeripheral);
            stream.set_number_of_transfers(items_to_transfer);
            stream.set_word_size(WordSize::HalfWord);
            stream.set_circular(true);

            stream.set_memory_address(source_address);
            stream.set_memory_increment(true);

            stream.set_peripheral_address(destination_address);
            stream.set_peripheral_increment(false);

            stream.set_transfer_complete_interrupt_enable(true);
            stream.set_half_transfer_interrupt_enable(true);
            stream.set_transfer_error_interrupt_enable(true);

            unsafe { stream.enable() };
            i2s.enable();

            stream
        };

        // The blue button on the board. When clicked, it raises an EXTI
        // interrupt.
        let button = {
            let mut button = gpioa.pa0.into_pull_down_input();
            button.make_interrupt_source(&mut syscfg);
            button.enable_interrupt(&mut cx.device.EXTI);
            button.trigger_on_edge(&mut cx.device.EXTI, Edge::RISING);
            button
        };

        // MIDI over USB, reconciling incomming MIDI messages in an interrupt
        // handler. Unsafe to allow access to static variables. This is safe
        // since all of these variables are accessed only by the interrupt
        // handler.
        unsafe {
            let usb = USB {
                usb_global: cx.device.OTG_FS_GLOBAL,
                usb_device: cx.device.OTG_FS_DEVICE,
                usb_pwrclk: cx.device.OTG_FS_PWRCLK,
                pin_dm: gpioa.pa11.into_alternate_af10(),
                pin_dp: gpioa.pa12.into_alternate_af10(),
                hclk: clocks.hclk(),
            };

            USB_BUS = Some(UsbBus::new(usb, &mut EP_MEMORY));

            USB_MIDI = Some(MidiClass::new(USB_BUS.as_ref().unwrap()));

            USB_DEVICE = Some(
                UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x5e4))
                    .product("Achordion")
                    .device_class(USB_AUDIO_CLASS)
                    .device_sub_class(USB_MIDISTREAMING_SUBCLASS)
                    .build(),
            );
        }

        // MIDI instrument is used to interpret received MIDI messages and
        // control the Oscillator based on it.
        let midi_instrument = MidiInstrument::new();

        // The main instrument used to fill in the circular buffer.
        let oscillator = Oscillator::new(&WAVETABLES[..], SAMPLE_RATE);

        init::LateResources {
            stream,
            green_led,
            blue_led,
            red_led,
            button,
            oscillator,
            midi_instrument,
        }
    }

    /// Handling of DMA requests to populate the circular buffer.
    #[task(binds = DMA1_STREAM5, resources = [stream, green_led, red_led, oscillator])]
    fn dsp_request(cx: dsp_request::Context) {
        let stream = cx.resources.stream;

        let (start, stop) = if Stream5::<DMA1>::get_transfer_complete_flag() {
            stream.clear_transfer_complete_interrupt();
            cx.resources.green_led.set_high().unwrap();
            (BUFFER_SIZE, BUFFER_SIZE * 2)
        } else if Stream5::<DMA1>::get_half_transfer_flag() {
            stream.clear_half_transfer_interrupt();
            cx.resources.green_led.set_high().unwrap();
            (0, BUFFER_SIZE)
        } else {
            stream.clear_interrupts();
            cx.resources.red_led.set_high().unwrap();
            return;
        };

        let mut buffer = [0; BUFFER_SIZE / 2];
        cx.resources.oscillator.populate(&mut buffer[..]);

        unsafe {
            for (i, x) in STEREO_BUFFER[start..stop].iter_mut().enumerate() {
                *x = buffer[i / 2];
            }
        }

        cx.resources.green_led.set_low().unwrap();
    }

    /// Reconcile incoming MIDI messages.
    #[task(binds = OTG_FS, resources = [blue_led, midi_instrument, oscillator])]
    fn midi_request(cx: midi_request::Context) {
        let usb_device = unsafe { USB_DEVICE.as_mut().unwrap() };
        let usb_midi = unsafe { USB_MIDI.as_mut().unwrap() };

        while usb_device.poll(&mut [usb_midi]) {
            let mut buffer = [0; 64];

            if let Ok(size) = usb_midi.read(&mut buffer) {
                let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
                for packet in buffer_reader.into_iter() {
                    if let Ok(packet) = packet {
                        if let Ok(message) = packet.message.try_into() {
                            let state = cx.resources.midi_instrument.reconcile(message);

                            if state.frequency < 0.1 {
                                cx.resources.oscillator.frequency = 0.0;
                            } else {
                                let note = quantizer::ionian::quantize(MidiNote::A0, state.voct);
                                cx.resources.oscillator.frequency = note.to_freq_f32();
                            }

                            cx.resources.oscillator.wavetable = state.cc1;
                        }
                    }
                }
            }
        }
    }

    /// Control the oscillator frequency using the button.
    #[task(binds = EXTI0, resources = [button, oscillator])]
    fn button_click(cx: button_click::Context) {
        cx.resources.button.clear_interrupt_pending_bit();

        let oscillator = cx.resources.oscillator;
        if oscillator.frequency == 0.0 {
            oscillator.frequency = 40.0;
        } else {
            oscillator.frequency *= 1.5;
        }
    }
};
