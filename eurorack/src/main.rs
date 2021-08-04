#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::inconsistent_struct_constructor)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_map)]

mod interface;
mod system;

#[macro_use]
extern crate lazy_static;

use panic_halt as _;

#[allow(unused_imports)]
use micromath::F32Ext;

use rtic::app;
use rtic::cyccnt::U32Ext as _;

use daisy::audio;
use daisy::flash::Flash;
use daisy_bsp as daisy;

use achordion_lib::display::{self, Action as DisplayAction};
use achordion_lib::instrument::Instrument;
use achordion_lib::store::{Parameters, Store};
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::interface::Interface;
use crate::system::System;

const CV_PERIOD: u32 = 1_000_000;

static mut AUDIO_INTERFACE: Option<audio::Interface> = None;
static mut BUFFER: [(f32, f32); audio::BLOCK_LENGTH] = [(0.0, 0.0); audio::BLOCK_LENGTH];
const SAMPLE_RATE: u32 = audio::FS.0;

static mut FLASH: Option<Flash> = None;
const STORE_ADDRESSES: [u32; 2] = [0x0000, 0x1000];

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
        interface: Interface,
        instrument: Instrument<'static>,
    }

    /// Initialize all the peripherals.
    #[init(schedule = [control], spawn = [fade_in, store_parameters])]
    fn init(cx: init::Context) -> init::LateResources {
        let system = System::init(cx.core, cx.device);

        let mut flash = system.flash;

        let initial_parameters = {
            let mut store_buffer = [0; Store::SIZE];

            let mut stores = [None; STORE_ADDRESSES.len()];
            for (i, address) in STORE_ADDRESSES.iter().enumerate() {
                flash.read(*address, &mut store_buffer);
                stores[i] = Store::from_bytes(store_buffer).ok();
            }

            get_initial_parameters(stores)
        };

        unsafe {
            FLASH = Some(flash);
        }

        cx.spawn.store_parameters(0).unwrap();
        // TODO: Accept config in a struct - to keep all these named
        let interface = Interface::new(
            system.adc,
            system.button,
            system.pots.pot1,
            system.pots.pot2,
            system.pots.pot3,
            system.pots.pot4,
            system.cvs.cv1,
            system.cvs.cv2,
            system.cvs.cv3,
            system.cvs.cv4,
            system.cvs.cv5,
            system.cvs.cv6,
            system.cvs.cv_probe,
            system.leds.led1,
            system.leds.led2,
            system.leds.led3,
            system.leds.led4,
            system.leds.led5,
            system.leds.led6,
            system.leds.led7,
            system.leds.led8,
            initial_parameters,
        );

        cx.schedule.control(cx.start + CV_PERIOD.cycles()).unwrap();

        let audio_interface = system.audio;

        let audio_interface = {
            fn callback(_fs: f32, block: &mut audio::Block) {
                let buffer: &'static mut [(f32, f32); audio::BLOCK_LENGTH] = unsafe { &mut BUFFER };
                for (source, target) in buffer.iter().zip(block.iter_mut()) {
                    *target = *source;
                }
            }

            audio_interface.spawn(callback).unwrap()
        };

        unsafe {
            AUDIO_INTERFACE = Some(audio_interface);
        }

        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_amplitude(0.0);

        cx.spawn.fade_in().unwrap();

        init::LateResources {
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

    #[task(schedule = [control], resources = [interface, instrument])]
    fn control(mut cx: control::Context) {
        static mut ACTIVITY: Option<Activity> = None;
        if ACTIVITY.is_none() {
            *ACTIVITY = Some(Activity::new());
        }
        let activity = ACTIVITY.as_mut().unwrap();

        let interface = cx.resources.interface;
        interface.update();

        cx.resources.instrument.lock(|instrument| {
            let any_action = reconcile_all_changes(interface, instrument);
            let pot_action = reconcile_pot_activity(interface, instrument);

            if interface.active() {
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
                interface.set_display(display::reduce(action));
            } else if let (Some(action), true) = (any_action, activity.idle_pots()) {
                // Reset only once shown, so it can never bling quickly through
                // pot to CV to default.
                activity.reset_cv();
                interface.set_display(display::reduce(action));
            } else if activity.idle_cv() && activity.idle_cv() {
                interface.set_display(display::reduce(display::Action::SetChord(
                    instrument.chord_degrees(),
                )));
            }

            // XXX: Temporary for testing
            let amplitude = if interface.amplitude() > 0.5 {
                0.2
            } else {
                0.0
            };
            instrument.set_amplitude(amplitude);
        });

        cx.schedule
            .control(cx.scheduled + CV_PERIOD.cycles())
            .unwrap();
    }

    #[task(schedule = [store_parameters], resources = [interface])]
    fn store_parameters(cx: store_parameters::Context, version: u16) {
        let flash = unsafe { FLASH.as_mut().unwrap() };
        let data = Store::new(cx.resources.interface.parameters(), version).to_bytes();
        flash.write(
            STORE_ADDRESSES[version as usize % STORE_ADDRESSES.len()],
            &data,
        );

        cx.schedule
            .store_parameters(cx.scheduled + 480_000_000.cycles(), version.wrapping_add(1))
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
    }
};

fn reconcile_all_changes(
    interface: &mut Interface,
    instrument: &mut Instrument,
) -> Option<DisplayAction> {
    let new_chord_root_degree = instrument.set_chord_root(interface.note());
    let new_scale_root = instrument.set_scale_root(interface.scale_root());
    let new_scale_mode = instrument.set_scale_mode(interface.scale_mode());
    let new_wavetable = instrument.set_wavetable(interface.wavetable());
    let new_wavetable_bank = instrument.set_wavetable_bank(interface.wavetable_bank());
    let new_degrees = instrument.set_chord_degrees(interface.chord());
    let new_detune = instrument.set_detune(interface.detune());

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
    interface: &mut Interface,
    instrument: &mut Instrument,
) -> Option<DisplayAction> {
    if interface.chord_pot_active() {
        let chord_degrees = instrument.chord_degrees();
        Some(DisplayAction::SetChord(chord_degrees))
    } else if interface.wavetable_bank_pot_active() {
        let wavetable_bank = instrument.wavetable_bank();
        Some(DisplayAction::SetWavetableBank(wavetable_bank))
    } else if interface.note_pot_active() {
        let chord_root_degree = instrument.chord_root_degree();
        Some(DisplayAction::SetChordRootDegree(chord_root_degree))
    } else if interface.scale_root_pot_active() {
        let scale_root = instrument.scale_root();
        Some(DisplayAction::SetScaleRoot(scale_root))
    } else if interface.scale_mode_pot_active() {
        let scale_mode = instrument.scale_mode();
        Some(DisplayAction::SetScaleMode(scale_mode))
    } else if interface.wavetable_pot_active() {
        let wavetable = instrument.wavetable();
        Some(DisplayAction::SetWavetable(wavetable))
    } else if interface.detune_pot_active() {
        let (detune_index, detune_phase) = instrument.detune();
        Some(DisplayAction::SetDetune(detune_index, detune_phase))
    } else {
        None
    }
}

struct Activity {
    pot_idle: u32,
    cv_idle: u32,
}

impl Activity {
    const MAX_IDLE_POT: u32 = 300;
    const MAX_IDLE_CV: u32 = 600;

    pub fn new() -> Self {
        Self {
            pot_idle: u32::MAX,
            cv_idle: u32::MAX,
        }
    }

    pub fn reset_pots(&mut self) {
        self.pot_idle = 0;
    }

    pub fn reset_cv(&mut self) {
        self.cv_idle = 0;
    }

    pub fn tick_all(&mut self) {
        self.pot_idle += 1;
        self.cv_idle += 1;
    }

    pub fn idle_pots(&self) -> bool {
        self.pot_idle > Self::MAX_IDLE_POT
    }

    pub fn idle_cv(&self) -> bool {
        self.cv_idle > Self::MAX_IDLE_CV
    }
}

fn get_initial_parameters<const L: usize>(stores: [Option<Store>; L]) -> Parameters {
    let mut latest_store: Option<Store> = None;

    for store in stores.iter().flatten() {
        if let Some(latest) = latest_store {
            if store.version() > latest.version() {
                latest_store = Some(*store);
            }
        } else {
            latest_store = Some(*store);
        }
    }

    if let Some(latest) = latest_store {
        latest.parameters()
    } else {
        Parameters::default()
    }
}
