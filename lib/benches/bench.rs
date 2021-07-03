#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

const SAMPLE_RATE: u32 = 44_100;

lazy_static! {
    static ref BANK_A: [Wavetable<'static>; 1] = [Wavetable::new(
        &waveform::perfect::PERFECT_3_FACTORS,
        SAMPLE_RATE
    )];
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 1] = [&BANK_A[..]];
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("instrument", |b| {
        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_chord_root(2.0);
        let mut root_buffer = [0.0; 64];
        let mut chord_buffer = [0.0; 64];
        b.iter(|| instrument.populate(black_box(&mut root_buffer), black_box(&mut chord_buffer)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
