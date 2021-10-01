use achordion_bank::factor::Factors;
use achordion_bank::waveform;
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

#[link_section = ".sram"]
static mut FACTORS_5: Option<Factors> = None;
static mut FACTORS_5_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_6: Option<Factors> = None;
static mut FACTORS_6_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_7: Option<Factors> = None;
static mut FACTORS_7_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_8: Option<Factors> = None;
static mut FACTORS_8_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_9: Option<Factors> = None;
static mut FACTORS_9_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_10: Option<Factors> = None;
static mut FACTORS_10_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_11: Option<Factors> = None;
static mut FACTORS_11_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_12: Option<Factors> = None;
static mut FACTORS_12_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_13: Option<Factors> = None;
static mut FACTORS_13_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_14: Option<Factors> = None;
static mut FACTORS_14_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_15: Option<Factors> = None;
static mut FACTORS_15_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_16: Option<Factors> = None;
static mut FACTORS_16_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_17: Option<Factors> = None;
static mut FACTORS_17_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_18: Option<Factors> = None;
static mut FACTORS_18_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_19: Option<Factors> = None;
static mut FACTORS_19_REF: Option<FactorsRef> = None;

#[link_section = ".sram"]
static mut FACTORS_20: Option<Factors> = None;
static mut FACTORS_20_REF: Option<FactorsRef> = None;

const BANK_SIZE: usize = 21;
pub static mut BANK: Option<[Wavetable<'static>; BANK_SIZE]> = None;

pub fn setup(progress: &mut Progress) {
    unsafe {
        FACTORS_0 = Some(Factors::from_raw(&waveform::sins::SINS_0));
        FACTORS_0_REF = Some(factors_ref!(FACTORS_0));
        progress.tick();

        FACTORS_1 = Some(Factors::from_raw(&waveform::sins::SINS_1));
        FACTORS_1_REF = Some(factors_ref!(FACTORS_1));
        progress.tick();

        FACTORS_2 = Some(Factors::from_raw(&waveform::sins::SINS_2));
        FACTORS_2_REF = Some(factors_ref!(FACTORS_2));
        progress.tick();

        FACTORS_3 = Some(Factors::from_raw(&waveform::sins::SINS_3));
        FACTORS_3_REF = Some(factors_ref!(FACTORS_3));
        progress.tick();

        FACTORS_4 = Some(Factors::from_raw(&waveform::sins::SINS_4));
        FACTORS_4_REF = Some(factors_ref!(FACTORS_4));
        progress.tick();

        FACTORS_5 = Some(Factors::from_raw(&waveform::sins::SINS_5));
        FACTORS_5_REF = Some(factors_ref!(FACTORS_5));
        progress.tick();

        FACTORS_6 = Some(Factors::from_raw(&waveform::sins::SINS_6));
        FACTORS_6_REF = Some(factors_ref!(FACTORS_6));
        progress.tick();

        FACTORS_7 = Some(Factors::from_raw(&waveform::sins::SINS_7));
        FACTORS_7_REF = Some(factors_ref!(FACTORS_7));
        progress.tick();

        FACTORS_8 = Some(Factors::from_raw(&waveform::sins::SINS_8));
        FACTORS_8_REF = Some(factors_ref!(FACTORS_8));
        progress.tick();

        FACTORS_9 = Some(Factors::from_raw(&waveform::sins::SINS_9));
        FACTORS_9_REF = Some(factors_ref!(FACTORS_9));
        progress.tick();

        FACTORS_10 = Some(Factors::from_raw(&waveform::sins::SINS_10));
        FACTORS_10_REF = Some(factors_ref!(FACTORS_10));
        progress.tick();

        FACTORS_11 = Some(Factors::from_raw(&waveform::sins::SINS_11));
        FACTORS_11_REF = Some(factors_ref!(FACTORS_11));
        progress.tick();

        FACTORS_12 = Some(Factors::from_raw(&waveform::sins::SINS_12));
        FACTORS_12_REF = Some(factors_ref!(FACTORS_12));
        progress.tick();

        FACTORS_13 = Some(Factors::from_raw(&waveform::sins::SINS_13));
        FACTORS_13_REF = Some(factors_ref!(FACTORS_13));
        progress.tick();

        FACTORS_14 = Some(Factors::from_raw(&waveform::sins::SINS_14));
        FACTORS_14_REF = Some(factors_ref!(FACTORS_14));
        progress.tick();

        FACTORS_15 = Some(Factors::from_raw(&waveform::sins::SINS_15));
        FACTORS_15_REF = Some(factors_ref!(FACTORS_15));
        progress.tick();

        FACTORS_16 = Some(Factors::from_raw(&waveform::sins::SINS_16));
        FACTORS_16_REF = Some(factors_ref!(FACTORS_16));
        progress.tick();

        FACTORS_17 = Some(Factors::from_raw(&waveform::sins::SINS_17));
        FACTORS_17_REF = Some(factors_ref!(FACTORS_17));
        progress.tick();

        FACTORS_18 = Some(Factors::from_raw(&waveform::sins::SINS_18));
        FACTORS_18_REF = Some(factors_ref!(FACTORS_18));
        progress.tick();

        FACTORS_19 = Some(Factors::from_raw(&waveform::sins::SINS_19));
        FACTORS_19_REF = Some(factors_ref!(FACTORS_19));
        progress.tick();

        FACTORS_20 = Some(Factors::from_raw(&waveform::sins::SINS_20));
        FACTORS_20_REF = Some(factors_ref!(FACTORS_20));
        progress.tick();

        BANK = Some([
            Wavetable::new(FACTORS_0_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_1_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_2_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_3_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_4_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_5_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_6_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_7_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_8_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_9_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_10_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_11_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_12_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_13_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_14_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_15_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_16_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_17_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_18_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_19_REF.as_ref().unwrap(), SAMPLE_RATE),
            Wavetable::new(FACTORS_20_REF.as_ref().unwrap(), SAMPLE_RATE),
        ]);
    }
}

pub fn len() -> usize {
    BANK_SIZE
}
