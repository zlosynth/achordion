use core::f32::consts::PI;

#[allow(unused_imports)]
use micromath::F32Ext;

use sirena::state_variable_filter::{LowPass, StateVariableFilter};

pub fn filter(mut wavetable: [f32; 2048], fraction: f32) -> [f32; 2048] {
    if fraction < 2.0 / 1024.0 {
        wavetable.iter_mut().enumerate().for_each(|(i, x)| {
            let phase = i as f32 / 2048.0;
            *x = f32::sin(2.0 * PI * phase);
        });
        return wavetable;
    }

    let niquist = 2048.0 / 2.0;
    let cutoff = fraction * niquist;
    let mut filter = StateVariableFilter::new(2048 * 2);
    filter
        .set_bandform(LowPass)
        .set_frequency(cutoff)
        .set_q_factor(0.7);
    for _ in 0..3 {
        filter.pass(&wavetable);
    }
    filter.process(&mut wavetable);

    wavetable
}
