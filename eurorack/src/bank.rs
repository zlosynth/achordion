use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

use crate::system::audio::SAMPLE_RATE;

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
    // static ref BANK_VOCAL: [Wavetable<'static>; 5] = [
    //     Wavetable::new(&waveform::vocal::VOCAL_0_FACTORS, SAMPLE_RATE),
    //     Wavetable::new(&waveform::vocal::VOCAL_1_FACTORS, SAMPLE_RATE),
    //     Wavetable::new(&waveform::vocal::VOCAL_2_FACTORS, SAMPLE_RATE),
    //     Wavetable::new(&waveform::vocal::VOCAL_3_FACTORS, SAMPLE_RATE),
    //     Wavetable::new(&waveform::vocal::VOCAL_4_FACTORS, SAMPLE_RATE),
    // ];
    pub static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 3] = [
        &BANK_PERFECT[..],
        &BANK_HARSH[..],
        &BANK_SOFT[..],
        // &BANK_VOCAL[..]
    ];
}
