use achordion_lib::bank::factor::Factors;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use super::FactorsRef;
use super::Progress;
use crate::system::audio::SAMPLE_RATE;

#[link_section = ".sram"]
static mut FACTORS_0: Option<Factors> = None;
static mut FACTORS_0_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_1: Option<Factors> = None;
static mut FACTORS_1_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_2: Option<Factors> = None;
static mut FACTORS_2_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_3: Option<Factors> = None;
static mut FACTORS_3_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_4: Option<Factors> = None;
static mut FACTORS_4_REF: Option<FactorsRef> = None;

pub static mut BANK: Option<[Wavetable<'static>; 5]> = None;

pub fn setup(progress: &mut Progress) {
    unsafe {
        FACTORS_0 = Some(Factors::from_raw(&waveform::vocal::VOCAL_0));
        FACTORS_0_REF = Some(factors_ref!(FACTORS_0));
        progress.tick();

        FACTORS_1 = Some(Factors::from_raw(&waveform::vocal::VOCAL_1));
        FACTORS_1_REF = Some(factors_ref!(FACTORS_1));
        progress.tick();

        FACTORS_2 = Some(Factors::from_raw(&waveform::vocal::VOCAL_2));
        FACTORS_2_REF = Some(factors_ref!(FACTORS_2));
        progress.tick();

        FACTORS_3 = Some(Factors::from_raw(&waveform::vocal::VOCAL_3));
        FACTORS_3_REF = Some(factors_ref!(FACTORS_3));
        progress.tick();

        FACTORS_4 = Some(Factors::from_raw(&waveform::vocal::VOCAL_4));
        FACTORS_4_REF = Some(factors_ref!(FACTORS_4));
        progress.tick();

        BANK = Some([
            Wavetable::new(FACTORS_0_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_1_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_2_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_3_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_4_REF.as_ref().unwrap(), SAMPLE_RATE),
        ]);
    }
}

pub fn len() -> usize {
    5
}
