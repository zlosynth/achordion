#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_map)]

mod bank;
mod controls;
mod display;
mod input_activity;
mod storage;
mod system;

#[macro_use]
mod profiling;

use panic_halt as _;

#[allow(unused_imports)]
use micromath::F32Ext;

use rtic::app;
use rtic::cyccnt::Instant;
use rtic::cyccnt::U32Ext as _;

use achordion_lib::display::{self as display_lib, Action as DisplayAction, CalibrationPhase};
use achordion_lib::instrument::Instrument;
use achordion_lib::store::Parameters;

use crate::bank::WAVETABLE_BANKS;
use crate::controls::{Controls, ControlsConfig};
use crate::display::{Display, DisplayConfig};
use crate::input_activity::InputActivity;
use crate::storage::Storage;
use crate::system::audio::{Audio, BLOCK_LENGTH, SAMPLE_RATE};
use crate::system::System;

const SECOND: u32 = 480_000_000;
const CV_PERIOD: u32 = SECOND / 2000;
const STORE_PERIOD: u32 = 4 * SECOND;

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        controls: Controls,
        display: Display,
        storage: Storage,
        input_activity: InputActivity,
        audio: Audio<'static>,
        instrument: Option<Instrument<'static>>,
    }

    /// Initialize all the peripherals.
    #[init(spawn = [initialize])]
    fn init(cx: init::Context) -> init::LateResources {
        let system = System::init(cx.core, cx.device);

        let mut storage = Storage::new(system.flash);

        let parameters = if system.button.active_no_filter() {
            let parameters = Parameters::default();
            while system.button.active_no_filter() {}
            parameters
        } else {
            storage.load_parameters()
        };

        let controls = Controls::new(
            ControlsConfig {
                adc1: system.adc1,
                adc2: system.adc2,
                alt_button: system.button,
                pot_note: system.pots.pot1,
                pot_wavetable: system.pots.pot2,
                pot_chord: system.pots.pot3,
                pot_detune: system.pots.pot4,
                cv_voct: system.cvs.cv1,
                cv_scale_tonic: system.cvs.cv2,
                cv_scale_mode: system.cvs.cv3,
                cv_chord: system.cvs.cv4,
                cv_detune: system.cvs.cv5,
                cv_wavetable: system.cvs.cv6,
                cv_probe: system.cvs.cv_probe,
            },
            parameters,
        );

        let display = Display::new(DisplayConfig {
            led1: system.leds.led4,
            led2: system.leds.led8,
            led3: system.leds.led3,
            led4: system.leds.led7,
            led5: system.leds.led2,
            led6: system.leds.led6,
            led7: system.leds.led1,
            led_sharp: system.leds.led5,
        });

        let mut audio = system.audio;
        audio.spawn();

        cx.spawn.initialize().unwrap();

        init::LateResources {
            controls,
            display,
            audio,
            storage,
            instrument: None,
            input_activity: InputActivity::new(),
        }
    }

    #[task(schedule = [reconcile_controls], spawn = [fade_in, backup_collector], resources = [display, instrument], priority = 2)]
    fn initialize(mut cx: initialize::Context) {
        let mut display = cx.resources.display;
        bank::setup(&mut display);

        let mut instrument = Instrument::new(
            unsafe { &WAVETABLE_BANKS.as_ref().unwrap()[..] },
            SAMPLE_RATE,
        );
        instrument.set_amplitude(0.0);

        cx.resources
            .instrument
            .lock(|resource| *resource = Some(instrument));

        cx.schedule
            .reconcile_controls(Instant::now() + CV_PERIOD.cycles())
            .unwrap();
        cx.spawn.fade_in().unwrap();
        cx.spawn.backup_collector(0).unwrap();
    }

    #[task(binds = DMA1_STR1, priority = 3, resources = [audio, instrument])]
    fn dsp(cx: dsp::Context) {
        let audio = cx.resources.audio;

        let mut buffer_solo = [0.0; BLOCK_LENGTH];
        let mut buffer_chord = [0.0; BLOCK_LENGTH];

        if let Some(instrument) = cx.resources.instrument {
            instrument.populate(&mut buffer_solo, &mut buffer_chord);
        }

        audio.update_buffer(|buffer| {
            buffer.iter_mut().enumerate().for_each(|(i, x)| {
                *x = (buffer_solo[i] * 0.1, buffer_chord[i] * 0.1);
            })
        });
    }

    #[task(schedule = [fade_in], resources = [instrument], priority = 2)]
    fn fade_in(mut cx: fade_in::Context) {
        let mut amplitude = 0.0;

        cx.resources.instrument.lock(|instrument| {
            let instrument = instrument.as_mut().unwrap();
            amplitude = if instrument.amplitude() < 0.00005 {
                0.0001
            } else {
                instrument.amplitude() * 1.05
            };
            instrument.set_amplitude(amplitude.min(1.0));
        });

        if amplitude < 1.0 {
            cx.schedule
                .fade_in(cx.scheduled + 6_000_000.cycles())
                .unwrap();
        }
    }

    #[task(schedule = [reconcile_controls], resources = [controls, display, instrument, input_activity], priority = 2)]
    fn reconcile_controls(mut cx: reconcile_controls::Context) {
        let activity = cx.resources.input_activity;
        let controls = cx.resources.controls;
        let display = cx.resources.display;

        controls.update();

        cx.resources.instrument.lock(|instrument| {
            let instrument = instrument.as_mut().unwrap();

            let calibration_action = reconcile_calibration(controls);
            let any_action = reconcile_all_changes(controls, instrument);
            let pot_action = reconcile_pot_activity(controls, instrument);

            if controls.active() {
                activity.reset_pots();
                activity.reset_cv();
            } else {
                activity.tick_all();
            }

            // 1. If calibration is in progress, prioritize showing that.
            // 2. If there is any pot activity, show that.
            // 3. If pots are idle and there is a change caused through CV,
            //    display that.
            // 4. If all activity is idle, display the default page.
            if let Some(action) = calibration_action {
                display.set(display_lib::reduce(action));
            } else if let Some(action) = pot_action {
                display.set(display_lib::reduce(action));
            } else if let Some(action) = any_action {
                // Reset only once shown, so it can never blink quickly through
                // pot to CV to default.
                activity.reset_cv();
                display.set(display_lib::reduce(action));
            } else if activity.idle_cv() && activity.idle_pots() {
                display.set(display_lib::reduce(DisplayAction::SetChord(
                    instrument.chord_degrees(),
                )));
            }
        });

        cx.schedule
            .reconcile_controls(Instant::now() + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(spawn = [backup_executor], resources = [controls], priority = 2)]
    fn backup_collector(cx: backup_collector::Context, version: u16) {
        let controls = cx.resources.controls;
        cx.spawn
            .backup_executor(controls.parameters(), version)
            .ok()
            .unwrap();
    }

    #[task(schedule = [backup_collector], resources = [storage])]
    fn backup_executor(cx: backup_executor::Context, parameters: Parameters, version: u16) {
        let storage = cx.resources.storage;
        storage.save_parameters(parameters, version);

        cx.schedule
            .backup_collector(
                Instant::now() + STORE_PERIOD.cycles(),
                version.wrapping_add(1),
            )
            .unwrap();
    }

    extern "C" {
        fn EXTI0();
        fn EXTI1();
    }
};

fn reconcile_calibration(controls: &mut Controls) -> Option<DisplayAction> {
    if controls.calibration_waiting_low() {
        Some(DisplayAction::SetCalibration(CalibrationPhase::WaitingLow))
    } else if controls.calibration_waiting_high() {
        Some(DisplayAction::SetCalibration(CalibrationPhase::WaitingHigh))
    } else if controls.calibration_succeeded() {
        Some(DisplayAction::SetCalibration(CalibrationPhase::Succeeded))
    } else if controls.calibration_failed() {
        Some(DisplayAction::SetCalibration(CalibrationPhase::Failed))
    } else {
        None
    }
}

fn reconcile_all_changes(
    controls: &mut Controls,
    instrument: &mut Instrument,
) -> Option<DisplayAction> {
    let new_chord_root_degree = if controls.note_from_pot() {
        instrument.set_chord_root_linear(controls.note())
    } else {
        instrument.set_chord_root_voct(controls.note())
    };
    let new_solo = instrument.set_solo_voct(controls.solo());
    let new_scale_root = instrument.set_scale_root_voct(controls.scale_root());
    let new_scale_mode = instrument.set_scale_mode(controls.scale_mode());
    let new_wavetable = instrument.set_wavetable(controls.wavetable());
    let new_style = instrument.set_style(controls.style());
    let new_wavetable_bank = instrument.set_wavetable_bank(controls.wavetable_bank());
    let new_degrees = instrument.set_chord_degrees(controls.chord());
    let new_detune = instrument.set_detune(controls.detune());

    if let Some(new_degrees) = new_degrees {
        Some(DisplayAction::SetChord(new_degrees))
    } else if let Some(new_chord_root_degree) = new_chord_root_degree {
        Some(DisplayAction::SetChordRootDegree(new_chord_root_degree))
    } else if let Some(new_solo) = new_solo {
        Some(DisplayAction::SetSolo(new_solo))
    } else if let Some(new_scale_root) = new_scale_root {
        Some(DisplayAction::SetScaleRoot(new_scale_root))
    } else if let Some(new_scale_mode) = new_scale_mode {
        Some(DisplayAction::SetScaleMode(new_scale_mode))
    } else if let Some(new_wavetable_bank) = new_wavetable_bank {
        Some(DisplayAction::SetWavetableBank(new_wavetable_bank))
    } else if let Some(new_wavetable) = new_wavetable {
        Some(DisplayAction::SetWavetable(new_wavetable))
    } else if let Some(new_style) = new_style {
        Some(DisplayAction::SetStyle(new_style))
    } else if let Some((new_detune_index, new_detune_phase)) = new_detune {
        Some(DisplayAction::SetDetune(new_detune_index, new_detune_phase))
    } else {
        None
    }
}

fn reconcile_pot_activity(
    controls: &mut Controls,
    instrument: &mut Instrument,
) -> Option<DisplayAction> {
    if controls.chord_pot_active() {
        let chord_degrees = instrument.chord_degrees();
        Some(DisplayAction::SetChord(chord_degrees))
    } else if controls.wavetable_bank_pot_active() {
        let wavetable_bank = instrument.wavetable_bank();
        Some(DisplayAction::SetWavetableBank(wavetable_bank))
    } else if controls.note_pot_active() {
        let chord_root_degree = instrument.chord_root_degree();
        Some(DisplayAction::SetChordRootDegree(chord_root_degree))
    } else if controls.scale_root_pot_active() {
        let scale_root = instrument.scale_root();
        Some(DisplayAction::SetScaleRoot(scale_root))
    } else if controls.scale_mode_pot_active() {
        let scale_mode = instrument.scale_mode();
        Some(DisplayAction::SetScaleMode(scale_mode))
    } else if controls.wavetable_pot_active() {
        let wavetable = instrument.wavetable();
        Some(DisplayAction::SetWavetable(wavetable))
    } else if controls.style_pot_active() {
        let style = instrument.style();
        Some(DisplayAction::SetStyle(style))
    } else if controls.detune_pot_active() {
        let (detune_index, detune_phase) = instrument.detune();
        Some(DisplayAction::SetDetune(detune_index, detune_phase))
    } else {
        None
    }
}
