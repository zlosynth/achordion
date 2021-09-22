use achordion_lib::bank;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::system::audio::SAMPLE_RATE;

macro_rules! bank {
    ( $module:ident, $bank:ident, $len:expr, $( $waveform:ident, $factors:ident, $ref:ident ),* ) => {
        $(
        #[link_section = ".sram"]
        static mut $factors: Option<bank::factor::Factors> = None;
        )*

        lazy_static! {
            $(
            static ref $ref: [&'static [u16]; 11] = {
                let factors = unsafe {
                    $factors = Some(bank::factor::Factors::from_raw(&waveform::$module::$waveform));
                    $factors.as_mut().unwrap()
                };
                [
                    &factors.factor1,
                    &factors.factor2,
                    &factors.factor4,
                    &factors.factor8,
                    &factors.factor16,
                    &factors.factor32,
                    &factors.factor64,
                    &factors.factor128,
                    &factors.factor256,
                    &factors.factor512,
                    &factors.factor1024,
                ]
            };
            )*
            static ref $bank: [Wavetable<'static>; $len] = {
                [
                    $(
                    Wavetable::new(&(*$ref), SAMPLE_RATE),
                    )*
                ]
            };
        }
    };
}

bank!(
    perfect,
    BANK_PERFECT,
    4,
    PERFECT_0,
    PERFECT_0_FACTORS,
    PERFECT_0_FACTORS_REF,
    PERFECT_1,
    PERFECT_1_FACTORS,
    PERFECT_1_FACTORS_REF,
    PERFECT_2,
    PERFECT_2_FACTORS,
    PERFECT_2_FACTORS_REF,
    PERFECT_3,
    PERFECT_3_FACTORS,
    PERFECT_3_FACTORS_REF
);

bank!(
    harsh,
    BANK_HARSH,
    6,
    HARSH_0,
    HARSH_0_FACTORS,
    HARSH_0_FACTORS_REF,
    HARSH_1,
    HARSH_1_FACTORS,
    HARSH_1_FACTORS_REF,
    HARSH_2,
    HARSH_2_FACTORS,
    HARSH_2_FACTORS_REF,
    HARSH_3,
    HARSH_3_FACTORS,
    HARSH_3_FACTORS_REF,
    HARSH_4,
    HARSH_4_FACTORS,
    HARSH_4_FACTORS_REF,
    HARSH_5,
    HARSH_5_FACTORS,
    HARSH_5_FACTORS_REF
);

bank!(
    soft,
    BANK_SOFT,
    6,
    SOFT_0,
    SOFT_0_FACTORS,
    SOFT_0_FACTORS_REF,
    SOFT_1,
    SOFT_1_FACTORS,
    SOFT_1_FACTORS_REF,
    SOFT_2,
    SOFT_2_FACTORS,
    SOFT_2_FACTORS_REF,
    SOFT_3,
    SOFT_3_FACTORS,
    SOFT_3_FACTORS_REF,
    SOFT_4,
    SOFT_4_FACTORS,
    SOFT_4_FACTORS_REF,
    SOFT_5,
    SOFT_5_FACTORS,
    SOFT_5_FACTORS_REF
);

bank!(
    vocal,
    BANK_VOCAL,
    5,
    VOCAL_0,
    VOCAL_0_FACTORS,
    VOCAL_0_FACTORS_REF,
    VOCAL_1,
    VOCAL_1_FACTORS,
    VOCAL_1_FACTORS_REF,
    VOCAL_2,
    VOCAL_2_FACTORS,
    VOCAL_2_FACTORS_REF,
    VOCAL_3,
    VOCAL_3_FACTORS,
    VOCAL_3_FACTORS_REF,
    VOCAL_4,
    VOCAL_4_FACTORS,
    VOCAL_4_FACTORS_REF
);

lazy_static! {
    pub static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 4] = {
        [
            &BANK_PERFECT[..],
            &BANK_HARSH[..],
            &BANK_SOFT[..],
            &BANK_VOCAL[..],
        ]
    };
}
