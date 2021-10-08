#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use achordion_bank as bank;
use achordion_bank::waveform;
use achordion_lib::instrument::Instrument;
use achordion_lib::wavetable::Wavetable;

const SAMPLE_RATE: u32 = 44_100;

lazy_static! {
    static ref FACTORS: bank::factor::Factors =
        bank::factor::Factors::from_raw(&waveform::perfect::PERFECT_3);
    static ref FACTORS_REF: [&'static [u16]; 11] = {
        [
            &FACTORS.factor1,
            &FACTORS.factor2,
            &FACTORS.factor4,
            &FACTORS.factor8,
            &FACTORS.factor16,
            &FACTORS.factor32,
            &FACTORS.factor64,
            &FACTORS.factor128,
            &FACTORS.factor256,
            &FACTORS.factor512,
            &FACTORS.factor1024,
        ]
    };
    static ref BANK_A: [Wavetable<'static>; 1] = [Wavetable::new(&*FACTORS_REF, SAMPLE_RATE)];
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 1] = [&BANK_A[..]];
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("instrument", |b| {
        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_chord_root_voct(2.0);
        instrument.set_chord_degrees(1.0);
        instrument.set_solo_voct(Some(3.5));
        instrument.set_detune(1.0);
        let mut solo_buffer = [0.0; 64];
        let mut chord_buffer = [0.0; 64];
        b.iter(|| instrument.populate(black_box(&mut solo_buffer), black_box(&mut chord_buffer)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
