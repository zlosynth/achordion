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

use achordion_lib::display;
use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::interface::Interface;

const CV_PERIOD: u32 = 1_000_000;

static mut AUDIO_INTERFACE: Option<audio::Interface> = None;
static mut BUFFER: [(f32, f32); audio::BLOCK_LENGTH] = [(0.0, 0.0); audio::BLOCK_LENGTH];
const SAMPLE_RATE: u32 = audio::FS.0;

lazy_static! {
    static ref BANK_PERFECT: [Wavetable<'static>; 4] = [
        Wavetable::new(&waveform::perfect::PERFECT_0_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::perfect::PERFECT_1_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::perfect::PERFECT_2_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::perfect::PERFECT_3_FACTORS, SAMPLE_RATE),
    ];
    static ref BANK_HARSH: [Wavetable<'static>; 6] = [
        Wavetable::new(&waveform::harsh::HARSH_0_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::harsh::HARSH_1_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::harsh::HARSH_2_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::harsh::HARSH_3_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::harsh::HARSH_4_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::harsh::HARSH_5_FACTORS, SAMPLE_RATE),
    ];
    static ref BANK_SOFT: [Wavetable<'static>; 6] = [
        Wavetable::new(&waveform::soft::SOFT_0_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::soft::SOFT_1_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::soft::SOFT_2_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::soft::SOFT_3_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::soft::SOFT_4_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::soft::SOFT_5_FACTORS, SAMPLE_RATE),
    ];
    static ref BANK_VOCAL: [Wavetable<'static>; 5] = [
        Wavetable::new(&waveform::vocal::VOCAL_0_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::vocal::VOCAL_1_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::vocal::VOCAL_2_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::vocal::VOCAL_3_FACTORS, SAMPLE_RATE),
        Wavetable::new(&waveform::vocal::VOCAL_4_FACTORS, SAMPLE_RATE),
    ];
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 4] = [
        &BANK_PERFECT[..],
        &BANK_HARSH[..],
        &BANK_SOFT[..],
        &BANK_VOCAL[..]
    ];
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
        let mut interface = Interface::new(
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
            pins.SEED_PIN_30.into_push_pull_output(),
            pins.SEED_PIN_29.into_push_pull_output(),
            pins.SEED_PIN_26.into_push_pull_output(),
            pins.SEED_PIN_25.into_push_pull_output(),
            pins.SEED_PIN_3.into_push_pull_output(),
            pins.SEED_PIN_4.into_push_pull_output(),
            pins.SEED_PIN_5.into_push_pull_output(),
            pins.SEED_PIN_6.into_push_pull_output(),
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

        interface.set_display(display::reduce(display::Action::SetChord(
            instrument.chord_degrees(),
        )));

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
            instrument.set_scale_root(interface.scale_root());
            instrument.set_scale_mode(interface.scale_mode());
            instrument.set_wavetable(interface.wavetable());
            instrument.set_wavetable_bank(interface.wavetable_bank());
            if let Some(new_degrees) = instrument.set_chord_degrees(interface.chord()) {
                interface.set_display(display::reduce(display::Action::SetChord(new_degrees)));
            }
            instrument.set_detune(interface.detune());
        });

        cx.schedule
            .control(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(binds = DMA1_STR1, priority = 2, resources = [instrument])]
    fn dsp(cx: dsp::Context) {
        let audio_interface: &'static mut audio::Interface =
            unsafe { AUDIO_INTERFACE.as_mut().unwrap() };
        let buffer: &'static mut [(f32, f32); audio::BLOCK_LENGTH] = unsafe { &mut BUFFER };

        let mut buffer_root = [0.0; audio::BLOCK_LENGTH];
        let mut buffer_chord = [0.0; audio::BLOCK_LENGTH];

        cx.resources
            .instrument
            .populate(&mut buffer_root, &mut buffer_chord);

        for i in 0..audio::BLOCK_LENGTH {
            buffer[i] = (buffer_root[i] * 0.9, buffer_chord[i] * 0.9);
        }

        audio_interface.handle_interrupt_dma1_str1().unwrap();
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};
