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
extern crate lazy_static;

use panic_halt as _;

#[allow(unused_imports)]
use micromath::F32Ext;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use daisy_bsp::audio;

use achordion_lib::display::{self as display_lib, Action as DisplayAction};
use achordion_lib::instrument::Instrument;

use crate::bank::WAVETABLE_BANKS;
use crate::controls::{Controls, ControlsConfig};
use crate::display::{Display, DisplayConfig};
use crate::input_activity::InputActivity;
use crate::storage::Storage;
use crate::system::audio::Audio;
use crate::system::System;

const CV_PERIOD: u32 = 1_000_000;
const STORE_PERIOD: u32 = 480_000_000;

const SAMPLE_RATE: u32 = audio::FS.0;

#[app(device = stm32h7xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        controls: Controls,
        display: Display,
        storage: Storage,
        input_activity: InputActivity,
        audio: Audio<'static>,
        instrument: Instrument<'static>,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [reconcile_controls], spawn = [fade_in, store_parameters])]
    fn init(cx: init::Context) -> init::LateResources {
        let system = System::init(cx.core, cx.device);

        let mut storage = Storage::new(system.flash);

        let controls = Controls::new(
            ControlsConfig {
                adc: system.adc,
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
            storage.load_parameters(),
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

        cx.schedule
            .reconcile_controls(cx.start + CV_PERIOD.cycles())
            .unwrap();

        let mut audio = system.audio;
        audio.spawn();

        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_amplitude(0.0);
        cx.spawn.fade_in().unwrap();

        cx.spawn.store_parameters(0).unwrap();

        init::LateResources {
            controls,
            display,
            audio,
            storage,
            instrument,
            input_activity: InputActivity::new(),
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

    #[task(schedule = [reconcile_controls], resources = [controls, display, instrument, input_activity])]
    fn reconcile_controls(mut cx: reconcile_controls::Context) {
        let activity = cx.resources.input_activity;
        let controls = cx.resources.controls;
        let display = cx.resources.display;

        controls.update();

        cx.resources.instrument.lock(|instrument| {
            let any_action = reconcile_all_changes(controls, instrument);
            let pot_action = reconcile_pot_activity(controls, instrument);

            if controls.active() {
                activity.reset_pots();
                activity.reset_cv();
            } else {
                activity.tick_all();
            }

            // 1. If there is any pot activity, prioritize showing it.
            // 2. If pots are idle and there is a change caused through CV,
            //    display that.
            // 3. If all activity is idle, display the default page.
            if let Some(action) = pot_action {
                display.set(display_lib::reduce(action));
            } else if let (Some(action), true) = (any_action, activity.idle_pots()) {
                // Reset only once shown, so it can never bling quickly through
                // pot to CV to default.
                activity.reset_cv();
                display.set(display_lib::reduce(action));
            } else if activity.idle_cv() && activity.idle_cv() {
                display.set(display_lib::reduce(DisplayAction::SetChord(
                    instrument.chord_degrees(),
                )));
            }

            // XXX: Temporary for testing
            let amplitude = if controls.amplitude() > 0.5 { 0.2 } else { 0.0 };
            instrument.set_amplitude(amplitude);
        });

        cx.schedule
            .reconcile_controls(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(schedule = [store_parameters], resources = [storage, controls])]
    fn store_parameters(cx: store_parameters::Context, version: u16) {
        let storage = cx.resources.storage;
        let controls = cx.resources.controls;

        storage.save_parameters(controls.parameters(), version);

        cx.schedule
            .store_parameters(
                cx.scheduled + STORE_PERIOD.cycles(),
                version.wrapping_add(1),
            )
            .unwrap();
    }

    #[task(binds = DMA1_STR1, priority = 2, resources = [audio, instrument])]
    fn dsp(cx: dsp::Context) {
        let audio = cx.resources.audio;

        let mut buffer_root = [0.0; audio::BLOCK_LENGTH];
        let mut buffer_chord = [0.0; audio::BLOCK_LENGTH];

        cx.resources
            .instrument
            .populate(&mut buffer_root, &mut buffer_chord);

        audio.update_buffer(|buffer| {
            buffer.iter_mut().enumerate().for_each(|(i, x)| {
                *x = (buffer_root[i] * 0.9, buffer_chord[i] * 0.9);
            })
        });
    }

    extern "C" {
        fn EXTI0();
    }
};

fn reconcile_all_changes(
    controls: &mut Controls,
    instrument: &mut Instrument,
) -> Option<DisplayAction> {
    let new_chord_root_degree = instrument.set_chord_root(controls.note());
    let new_scale_root = instrument.set_scale_root(controls.scale_root());
    let new_scale_mode = instrument.set_scale_mode(controls.scale_mode());
    let new_wavetable = instrument.set_wavetable(controls.wavetable());
    let new_wavetable_bank = instrument.set_wavetable_bank(controls.wavetable_bank());
    let new_degrees = instrument.set_chord_degrees(controls.chord());
    let new_detune = instrument.set_detune(controls.detune());

    if let Some(new_degrees) = new_degrees {
        Some(DisplayAction::SetChord(new_degrees))
    } else if let Some(new_chord_root_degree) = new_chord_root_degree {
        Some(DisplayAction::SetChordRootDegree(new_chord_root_degree))
    } else if let Some(new_scale_root) = new_scale_root {
        Some(DisplayAction::SetScaleRoot(new_scale_root))
    } else if let Some(new_scale_mode) = new_scale_mode {
        Some(DisplayAction::SetScaleMode(new_scale_mode))
    } else if let Some(new_wavetable_bank) = new_wavetable_bank {
        Some(DisplayAction::SetWavetableBank(new_wavetable_bank))
    } else if let Some(new_wavetable) = new_wavetable {
        Some(DisplayAction::SetWavetable(new_wavetable))
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
    } else if controls.detune_pot_active() {
        let (detune_index, detune_phase) = instrument.detune();
        Some(DisplayAction::SetDetune(detune_index, detune_phase))
    } else {
        None
    }
}
