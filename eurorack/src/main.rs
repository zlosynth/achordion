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
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]

#[macro_use]
extern crate lazy_static;

mod cs43l22;
mod hal;

use core::convert::TryInto;

use panic_halt as _;

use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};

use cortex_m::peripheral::DWT;

use stm32f4xx_hal::adc::config::AdcConfig;
use stm32f4xx_hal::adc::config::SampleTime;
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::dma::config::Priority;
use stm32f4xx_hal::dma::traits::{PeriAddress, Stream};
use stm32f4xx_hal::dma::{Channel0, MemoryToPeripheral, Stream5, StreamsTuple};
use stm32f4xx_hal::gpio::gpioc::{PC1, PC4};
use stm32f4xx_hal::gpio::gpiod::{PD12, PD14, PD15};
use stm32f4xx_hal::gpio::{Edge, Input, Output, PullUp, PushPull, Analog};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::pac::{DMA1, ADC2};
use stm32f4xx_hal::prelude::*;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

use usbd_midi::data::usb::constants::*;
use usbd_midi::data::usb_midi::midi_packet_reader::MidiPacketBufferReader;
use usbd_midi::midi_device::MidiClass;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity};

use achordion_lib::chords;
use achordion_lib::midi::instrument::Instrument as MidiInstrument;
use achordion_lib::oscillator::Oscillator;
use achordion_lib::quantizer;
use achordion_lib::scales;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::cs43l22::Cs43L22;
use crate::hal::prelude::*;
use crate::hal::stream::WordSize;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

const PERIOD: u32 = 8_000_000;

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
        adc: Adc<ADC2>,
        note_pot: PC1<Analog>,
        green_led: PD12<Output<PushPull>>,
        blue_led: PD15<Output<PushPull>>,
        red_led: PD14<Output<PushPull>>,
        button: PC4<Input<PullUp>>,
        oscillator_a: Oscillator<'static>,
        oscillator_b: Oscillator<'static>,
        oscillator_c: Oscillator<'static>,
        midi_instrument: MidiInstrument,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [read_pots])]
    fn init(mut cx: init::Context) -> init::LateResources {
        let mut syscfg = cx.device.SYSCFG.constrain();
        let rcc = cx.device.RCC.constrain();
        let gpioa = cx.device.GPIOA.split();
        let gpiob = cx.device.GPIOB.split();
        let gpioc = cx.device.GPIOC.split();
        let gpiod = cx.device.GPIOD.split();

        cx.core.DCB.enable_trace();
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

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

        // 7 segment display
        let mut display_a = gpioa.pa1.into_push_pull_output();
        display_a.set_high().unwrap();
        let mut display_b = gpiob.pb0.into_push_pull_output();
        display_b.set_high().unwrap();
        let mut display_c = gpioa.pa3.into_push_pull_output();
        display_c.set_high().unwrap();
        let mut display_d = gpioc.pc5.into_push_pull_output();
        display_d.set_high().unwrap();
        let mut display_e = gpioa.pa7.into_push_pull_output();
        display_e.set_high().unwrap();
        let mut display_f = gpiob.pb1.into_push_pull_output();
        display_f.set_high().unwrap();
        let mut display_g = gpioa.pa5.into_push_pull_output();
        display_g.set_high().unwrap();
        let mut display_dp = gpioa.pa2.into_push_pull_output();
        display_dp.set_high().unwrap();

        let note_pot = gpioc.pc1.into_analog();
        let adc = Adc::adc2(cx.device.ADC2, true, AdcConfig::default());
        cx.schedule.read_pots(cx.start + PERIOD.cycles()).unwrap();

        // Configure Cirrus DAC.
        {
            let i2c = I2c::new(
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
            let mut button = gpioc.pc4.into_pull_up_input();
            button.make_interrupt_source(&mut syscfg);
            button.enable_interrupt(&mut cx.device.EXTI);
            button.trigger_on_edge(&mut cx.device.EXTI, Edge::FALLING);
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
        let oscillator_a = Oscillator::new(&WAVETABLES[..], SAMPLE_RATE);
        let oscillator_b = Oscillator::new(&WAVETABLES[..], SAMPLE_RATE);
        let oscillator_c = Oscillator::new(&WAVETABLES[..], SAMPLE_RATE);

        init::LateResources {
            stream,
            adc,
            note_pot,
            green_led,
            blue_led,
            red_led,
            button,
            oscillator_a,
            oscillator_b,
            oscillator_c,
            midi_instrument,
        }
    }

    /// Handling of DMA requests to populate the circular buffer.
    #[task(binds = DMA1_STREAM5, resources = [stream, green_led, red_led, oscillator_a, oscillator_b, oscillator_c])]
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

        let mut buffer_a = [0; BUFFER_SIZE / 2];
        cx.resources.oscillator_a.populate(&mut buffer_a[..]);
        let mut buffer_b = [0; BUFFER_SIZE / 2];
        cx.resources.oscillator_b.populate(&mut buffer_b[..]);
        let mut buffer_c = [0; BUFFER_SIZE / 2];
        cx.resources.oscillator_c.populate(&mut buffer_c[..]);

        unsafe {
            for (i, x) in STEREO_BUFFER[start..stop].iter_mut().enumerate() {
                *x = buffer_a[i / 2] / 3 + buffer_b[i / 2] / 3 + buffer_c[i / 2] / 3;
            }
        }

        cx.resources.green_led.set_low().unwrap();
    }

    #[task(schedule = [read_pots], resources = [adc, note_pot])]
    fn read_pots(cx: read_pots::Context) {
        let sample = cx.resources.adc.convert(cx.resources.note_pot, SampleTime::Cycles_480);
        let millivolts = cx.resources.adc.sample_to_millivolts(sample);

        cx.schedule.read_pots(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    /// Reconcile incoming MIDI messages.
    #[task(binds = OTG_FS, resources = [blue_led, midi_instrument, oscillator_a, oscillator_b, oscillator_c])]
    fn midi_request(cx: midi_request::Context) {
        let usb_device = unsafe { USB_DEVICE.as_mut().unwrap() };
        let usb_midi = unsafe { USB_MIDI.as_mut().unwrap() };

        cx.resources.blue_led.set_high().unwrap();
        while usb_device.poll(&mut [usb_midi]) {
            let mut buffer = [0; 64];

            if let Ok(size) = usb_midi.read(&mut buffer) {
                let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
                for packet in buffer_reader.into_iter().flatten() {
                    if let Ok(message) = packet.message.try_into() {
                        let state = cx.resources.midi_instrument.reconcile(message);
                        let oscillation_disabled = state.frequency < 0.1;
                        let chord_mode = state.cc4 > 0.1;
                        let wavetable = state.cc1;

                        if !chord_mode {
                            cx.resources.oscillator_b.frequency = 0.0;
                            cx.resources.oscillator_c.frequency = 0.0;
                        }

                        if oscillation_disabled {
                            cx.resources.oscillator_a.frequency = 0.0;
                            cx.resources.oscillator_b.frequency = 0.0;
                            cx.resources.oscillator_c.frequency = 0.0;
                        } else {
                            let scale_base = quantizer::chromatic::quantize(state.cc2);

                            let mode = if state.cc3 < 1.0 / 7.0 {
                                scales::diatonic::Ionian
                            } else if state.cc3 < 2.0 / 7.0 {
                                scales::diatonic::Dorian
                            } else if state.cc3 < 3.0 / 7.0 {
                                scales::diatonic::Phrygian
                            } else if state.cc3 < 4.0 / 7.0 {
                                scales::diatonic::Lydian
                            } else if state.cc3 < 5.0 / 7.0 {
                                scales::diatonic::Mixolydian
                            } else if state.cc3 < 6.0 / 7.0 {
                                scales::diatonic::Aeolian
                            } else {
                                scales::diatonic::Locrian
                            };

                            let chord_base =
                                quantizer::diatonic::quantize(mode, scale_base, state.voct);

                            let notes = if state.cc4 < 0.2 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 3, 5])
                            } else if state.cc4 < 0.3 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 2, 5])
                            } else if state.cc4 < 0.4 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 4, 5])
                            } else if state.cc4 < 0.5 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 5, 7])
                            } else if state.cc4 < 0.6 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 3, 7])
                            } else if state.cc4 < 0.7 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 4, 7])
                            } else if state.cc4 < 0.8 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 2, 7])
                            } else if state.cc4 < 0.9 {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 5, 9])
                            } else {
                                chords::diatonic::build(mode, scale_base, chord_base, [1, 2, 9])
                            };

                            cx.resources.oscillator_a.frequency = notes[0].unwrap().to_freq_f32();

                            if chord_mode {
                                cx.resources.oscillator_b.frequency =
                                    notes[1].unwrap().to_freq_f32();
                                cx.resources.oscillator_c.frequency =
                                    notes[2].unwrap().to_freq_f32();
                            }
                        }

                        cx.resources.oscillator_a.wavetable = wavetable;
                        cx.resources.oscillator_b.wavetable = wavetable;
                        cx.resources.oscillator_c.wavetable = wavetable;
                    }
                }
            }
        }
        cx.resources.blue_led.set_low().unwrap();
    }

    /// Control the oscillator frequency using the button.
    #[task(binds = EXTI4, resources = [button, oscillator_a, red_led])]
    fn button_click(cx: button_click::Context) {
        cx.resources.button.clear_interrupt_pending_bit();

        let oscillator = cx.resources.oscillator_a;
        if oscillator.frequency == 0.0 {
            oscillator.frequency = 40.0;
        } else {
            oscillator.frequency *= 1.5;
        }
    }

    extern "C" {
        fn EXTI0();
    }
};
