#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]

mod interface;

#[macro_use]
extern crate lazy_static;

use panic_halt as _;

#[allow(unused_imports)]
use micromath::F32Ext;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use daisy::audio;
use daisy::hal;
use daisy_bsp as daisy;
use hal::adc::Adc;
use hal::delay::DelayFromCountDownTimer;
use hal::pac::DWT;
use hal::prelude::*;

use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::interface::Interface;

const CV_PERIOD: u32 = 1_000_000;

static mut AUDIO_INTERFACE: Option<audio::Interface> = None;
static mut BUFFER: [(f32, f32); audio::BLOCK_LENGTH] = [(0.0, 0.0); audio::BLOCK_LENGTH];
const SAMPLE_RATE: u32 = audio::FS.0;

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

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        interface: Interface,
        instrument: Instrument<'static>,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [control])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // AN5212: Improve application performance when fetching instruction and
        // data, from both internal andexternal memories.
        cx.core.SCB.enable_icache();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        let board = daisy::Board::take().unwrap();

        let rcc = cx.device.RCC.constrain().pll2_p_ck(4.mhz());
        let ccdr = board.freeze_clocks(cx.device.PWR.constrain(), rcc, &cx.device.SYSCFG);

        let pins = board.split_gpios(
            cx.device.GPIOA.split(ccdr.peripheral.GPIOA),
            cx.device.GPIOB.split(ccdr.peripheral.GPIOB),
            cx.device.GPIOC.split(ccdr.peripheral.GPIOC),
            cx.device.GPIOD.split(ccdr.peripheral.GPIOD),
            cx.device.GPIOE.split(ccdr.peripheral.GPIOE),
            cx.device.GPIOF.split(ccdr.peripheral.GPIOF),
            cx.device.GPIOG.split(ccdr.peripheral.GPIOG),
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
            pins.SEED_PIN_9.into_pull_up_input(),
            pins.SEED_PIN_23,
            pins.SEED_PIN_24,
            pins.SEED_PIN_22,
            pins.SEED_PIN_21,
        );

        cx.schedule.control(cx.start + CV_PERIOD.cycles()).unwrap();

        let pins = (
            pins.AK4556.PDN.into_push_pull_output(),
            pins.AK4556.MCLK_A.into_alternate_af6(),
            pins.AK4556.SCK_A.into_alternate_af6(),
            pins.AK4556.FS_A.into_alternate_af6(),
            pins.AK4556.SD_A.into_alternate_af6(),
            pins.AK4556.SD_B.into_alternate_af6(),
        );

        let sai1_prec = ccdr
            .peripheral
            .SAI1
            .kernel_clk_mux(hal::rcc::rec::Sai1ClkSel::PLL3_P);

        let audio_interface =
            audio::Interface::init(&ccdr.clocks, sai1_prec, pins, ccdr.peripheral.DMA1).unwrap();

        let audio_interface = {
            fn callback(fs: f32, block: &mut audio::Block) {
                let buffer: &'static mut [(f32, f32); audio::BLOCK_LENGTH] = unsafe { &mut BUFFER };
                for (source, target) in buffer.iter().zip(block.iter_mut()) {
                    *target = *source;
                }
            }

            audio_interface.start(callback).unwrap()
        };

        unsafe {
            AUDIO_INTERFACE = Some(audio_interface);
        }

        let instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);

        init::LateResources {
            interface,
            instrument,
        }
    }

    #[task(schedule = [control], resources = [interface, instrument])]
    fn control(cx: control::Context) {
        cx.resources.interface.sample();

        let instrument = cx.resources.instrument;
        let interface = cx.resources.interface;

        instrument.set_chord_root(interface.note());
        instrument.set_wavetable(interface.wavetable());
        instrument.set_chord_degrees(interface.chord());
        instrument.set_detune(interface.detune());

        cx.schedule
            .control(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = DMA1_STR1, resources = [instrument])]
    fn dsp(cx: dsp::Context) {
        let audio_interface: &'static mut audio::Interface =
            unsafe { AUDIO_INTERFACE.as_mut().unwrap() };
        let buffer: &'static mut [(f32, f32); audio::BLOCK_LENGTH] = unsafe { &mut BUFFER };

        let mut buffer_root = [0; audio::BLOCK_LENGTH];
        let mut buffer_chord = [0; audio::BLOCK_LENGTH];

        cx.resources
            .instrument
            .populate(&mut buffer_root, &mut buffer_chord);

        for i in 0..audio::BLOCK_LENGTH {
            let x1 = buffer_root[i] as f32 / f32::powi(2.0, 15) - 1.0;
            let x2 = buffer_chord[i] as f32 / f32::powi(2.0, 15) - 1.0;
            let x = (x1 + x2) / 2.0;
            buffer[i] = (x, x);
        }

        audio_interface.handle_interrupt_dma1_str1().unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
