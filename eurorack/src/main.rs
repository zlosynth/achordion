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
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate lazy_static;

mod cs43l22;
mod hal;

use panic_halt as _;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use cortex_m::peripheral::DWT;

use stm32f4xx_hal::adc::config::AdcConfig;
use stm32f4xx_hal::adc::config::Resolution;
use stm32f4xx_hal::adc::config::SampleTime;
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::dma::config::Priority;
use stm32f4xx_hal::dma::traits::{PeriAddress, Stream};
use stm32f4xx_hal::dma::{Channel0, MemoryToPeripheral, Stream5, StreamsTuple};
use stm32f4xx_hal::gpio::gpioa::{PA1, PA2};
use stm32f4xx_hal::gpio::gpiob::{PB0, PB1};
use stm32f4xx_hal::gpio::gpioc::PC4;
use stm32f4xx_hal::gpio::gpiod::{PD12, PD14, PD15};
use stm32f4xx_hal::gpio::{Analog, Edge, Input, Output, PullUp, PushPull};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::pac::{ADC2, DMA1};
use stm32f4xx_hal::prelude::*;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity};

use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::cs43l22::Cs43L22;
use crate::hal::prelude::*;
use crate::hal::stream::WordSize;

const PERIOD: u32 = 1_000_000;

// 7-bit address
const DAC_ADDRESS: u8 = 0x94 >> 1;

// Volume in decibels
const VOLUME: i8 = -80;

// Audio timing configuration:
// Sample rate 48 kHz
// 16 bits per sample -> SCK rate 1.536 MHz
// MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
const SAMPLE_RATE: u32 = 48000;

const BUFFER_SIZE: usize = 64 * 2;
static mut STEREO_BUFFER: [u16; BUFFER_SIZE * 2] = [16384; BUFFER_SIZE * 2];

lazy_static! {
    static ref BANK_A: [Wavetable<'static>; 4] = [
        Wavetable::new(&waveform::sine::SINE_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::triangle::TRIANGLE_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_500_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::saw::SAW_FACTORS, SAMPLE_RATE),
    ];
    static ref BANK_B: [Wavetable<'static>; 20] = [
        Wavetable::new(&waveform::pulse::PULSE_025_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_050_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_075_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_100_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_125_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_150_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_175_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_200_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_225_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_250_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_275_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_300_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_325_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_350_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_375_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_400_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_425_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_450_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_475_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_500_FACTORS, SAMPLE_RATE),
    ];
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 2] = [&BANK_A[..], &BANK_B[..]];
}

#[app(device = stm32f4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        stream: Stream5<DMA1>,
        adc: Adc<ADC2>,
        note_pot: PA2<Analog>,
        note_buffer: ControlBuffer,
        wavetable_pot: PA1<Analog>,
        wavetable_buffer: ControlBuffer,
        chord_pot: PB0<Analog>,
        chord_buffer: ControlBuffer,
        detune_pot: PB1<Analog>,
        detune_buffer: ControlBuffer,
        green_led: PD12<Output<PushPull>>,
        blue_led: PD15<Output<PushPull>>,
        red_led: PD14<Output<PushPull>>,
        button: PC4<Input<PullUp>>,
        instrument: Instrument<'static>,
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

        // Potentiometers
        let note_pot = gpioa.pa2.into_analog();
        let wavetable_pot = gpioa.pa1.into_analog();
        let chord_pot = gpiob.pb0.into_analog();
        let detune_pot = gpiob.pb1.into_analog();
        let adc = Adc::adc2(
            cx.device.ADC2,
            true,
            AdcConfig::default().resolution(Resolution::Twelve),
        );

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

        // The main instrument used to fill in the circular buffer.
        let instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);

        init::LateResources {
            stream,
            adc,
            note_pot,
            note_buffer: ControlBuffer::new(),
            wavetable_pot,
            wavetable_buffer: ControlBuffer::new(),
            detune_pot,
            detune_buffer: ControlBuffer::new(),
            chord_pot,
            chord_buffer: ControlBuffer::new(),
            green_led,
            blue_led,
            red_led,
            button,
            instrument,
        }
    }

    /// Handling of DMA requests to populate the circular buffer.
    #[task(binds = DMA1_STREAM5, resources = [stream, green_led, red_led, instrument])]
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
        let mut buffer_b = [0; BUFFER_SIZE / 2];
        cx.resources
            .instrument
            .populate(&mut buffer_a[..], &mut buffer_b[..]);

        unsafe {
            for (i, x) in STEREO_BUFFER[start..stop].iter_mut().enumerate() {
                *x = buffer_a[i / 2] / 2 + buffer_b[i / 2] / 2;
            }
        }

        cx.resources.green_led.set_low().unwrap();
    }

    #[task(schedule = [read_pots], resources = [adc, note_pot, wavetable_pot, chord_pot, detune_pot, note_buffer, wavetable_buffer, chord_buffer, detune_buffer, instrument])]
    fn read_pots(cx: read_pots::Context) {
        let adc = cx.resources.adc;

        let chord_root = {
            let sample = adc.convert(cx.resources.note_pot, SampleTime::Cycles_480);
            let millivolts = adc.sample_to_millivolts(sample);
            cx.resources.note_buffer.write(4096 - millivolts);
            let buffered = cx.resources.note_buffer.read() as f32;
            buffered / 4096.0 * 6.0 + 1.0
        };
        cx.resources.instrument.set_chord_root(chord_root);

        let chord_degrees = {
            let sample = adc.convert(cx.resources.chord_pot, SampleTime::Cycles_480);
            let millivolts = adc.sample_to_millivolts(sample);
            cx.resources.chord_buffer.write(4096 - millivolts);
            let buffered = cx.resources.chord_buffer.read() as f32;
            buffered / 4096.0
        };
        cx.resources.instrument.set_chord_degrees(chord_degrees);

        let detune = {
            let sample = adc.convert(cx.resources.detune_pot, SampleTime::Cycles_480);
            let millivolts = adc.sample_to_millivolts(sample);
            cx.resources.detune_buffer.write(4096 - millivolts);
            let buffered = cx.resources.detune_buffer.read() as f32;
            buffered / 4096.0
        };
        cx.resources.instrument.set_detune(detune);

        let wavetable = {
            let sample = adc.convert(cx.resources.wavetable_pot, SampleTime::Cycles_480);
            let millivolts = adc.sample_to_millivolts(sample);
            cx.resources.wavetable_buffer.write(4096 - millivolts);
            let buffered = cx.resources.wavetable_buffer.read() as f32;
            buffered / 4096.0
        };
        cx.resources.instrument.set_wavetable(wavetable);

        cx.schedule
            .read_pots(cx.scheduled + PERIOD.cycles())
            .unwrap();
    }

    /// Control the oscillator frequency using the button.
    #[task(binds = EXTI4, resources = [button, red_led])]
    fn button_click(cx: button_click::Context) {
        cx.resources.button.clear_interrupt_pending_bit();

        cx.resources.red_led.toggle().unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};

const CONTROL_BUFFER_LEN: usize = 16;

pub struct ControlBuffer {
    buffer: [f32; CONTROL_BUFFER_LEN],
    pointer: usize,
}

impl ControlBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; CONTROL_BUFFER_LEN],
            pointer: 0,
        }
    }

    pub fn write(&mut self, value: u16) {
        self.buffer[self.pointer] = value as f32;
        self.pointer = (self.pointer + 1) % CONTROL_BUFFER_LEN;
    }

    pub fn read(&self) -> u16 {
        let sum: f32 = self.buffer.iter().sum();
        (sum / CONTROL_BUFFER_LEN as f32) as u16
    }
}
