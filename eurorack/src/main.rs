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
use daisy::led::Led;
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
    // static ref BANK_B: [Wavetable<'static>; 20] = [
    static ref BANK_B: [Wavetable<'static>; 10] = [
        Wavetable::new(&waveform::pulse::PULSE_025_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_050_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_075_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_100_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_125_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_150_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_175_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_200_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_225_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_250_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_275_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_300_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_325_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_350_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_375_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_400_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_425_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_450_FACTORS, SAMPLE_RATE),
        // Wavetable::new(&waveform::pulse::PULSE_475_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::pulse::PULSE_500_FACTORS, SAMPLE_RATE),
    ];
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 2] = [&BANK_A[..], &BANK_B[..]];
    // static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 1] = [&BANK_A[..]];
}

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led_user: daisy::led::LedUser,
        interface: Interface,
        instrument: Instrument<'static>,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [control], spawn = [fade_in])]
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

        let led_user = daisy::led::LedUser::new(pins.LED_USER);

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
            pins.SEED_PIN_20,
            pins.SEED_PIN_19,
            pins.SEED_PIN_15,
            pins.SEED_PIN_16,
            pins.SEED_PIN_17,
            pins.SEED_PIN_18,
            pins.SEED_PIN_10.into_push_pull_output(),
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
            fn callback(_fs: f32, block: &mut audio::Block) {
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

        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_amplitude(0.0);

        cx.spawn.fade_in().unwrap();

        init::LateResources {
            led_user,
            interface,
            instrument,
        }
    }

    #[task(schedule = [fade_in], resources = [instrument])]
    fn fade_in(mut cx: fade_in::Context) {
        let mut amplitude = 0.0;

        cx.resources.instrument.lock(|instrument| {
            amplitude = instrument.amplitude() + 0.01;
            instrument.set_amplitude(amplitude.min(1.0));
        });

        if amplitude < 1.0 {
            cx.schedule
                .fade_in(cx.scheduled + 2_000_000.cycles())
                .unwrap();
        }
    }

    #[task(schedule = [control], resources = [interface, instrument, led_user])]
    fn control(mut cx: control::Context) {
        let interface = cx.resources.interface;
        interface.update();

        cx.resources.instrument.lock(|instrument| {
            instrument.set_chord_root(interface.note());
            instrument.set_scale_root(interface.root());
            instrument.set_scale_mode(interface.mode());
            instrument.set_wavetable(interface.wavetable());
            instrument.set_wavetable_bank(interface.wavetable_bank());
            instrument.set_chord_degrees(interface.chord());
            instrument.set_detune(interface.detune());
        });

        if interface.foo() {
            cx.resources.led_user.on();
        } else {
            cx.resources.led_user.off();
        }

        cx.schedule
            .control(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = DMA1_STR1, priority = 2, resources = [instrument])]
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
            let x1 = buffer_root[i] as f32 / f32::powi(2.0, 14) - 1.0;
            let x2 = buffer_chord[i] as f32 / f32::powi(2.0, 14) - 1.0;
            buffer[i] = (x1, x2);
        }

        audio_interface.handle_interrupt_dma1_str1().unwrap();
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};
