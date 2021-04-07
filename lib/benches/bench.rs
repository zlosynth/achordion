#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use achordion_lib::oscillator::Oscillator;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

const SAMPLE_RATE: u32 = 44_100;

lazy_static! {
    static ref WAVETABLE: Wavetable<'static> =
        Wavetable::new(&waveform::saw::SAW_FACTORS, SAMPLE_RATE);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("oscillator", |b| {
        let mut oscillator = Oscillator::new(&WAVETABLE, SAMPLE_RATE);
        oscillator.frequency = 440.0;
        let mut buffer = [0; 64];
        b.iter(|| oscillator.populate(black_box(&mut buffer)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
