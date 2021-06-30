use sirena::state_variable_filter::{LowPass, StateVariableFilter};

use super::consts::OVERSAMPLED_LENGTH;

pub fn to_u16(x: f32) -> u16 {
    ((x + 1.0) * f32::powi(2.0, 15)) as u16
}

pub fn normalize(data: &mut [f32]) {
    let ratio = normalization_ratio(data);
    for x in data.iter_mut() {
        *x *= ratio;
    }
}

fn normalization_ratio(data: &[f32]) -> f32 {
    let max = data.iter().fold(0.0, |a, b| f32::max(a, *b));
    let min = data.iter().fold(0.0, |a, b| f32::min(a, *b));
    let max_delta = f32::max(max, f32::abs(min));
    1.0 / max_delta
}

pub fn filtered(
    wavetable: &[f32; OVERSAMPLED_LENGTH],
    frequency: f32,
) -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = *wavetable;

    let mut filter = StateVariableFilter::new((OVERSAMPLED_LENGTH * 2) as u32);
    filter
        .set_bandform(LowPass)
        .set_frequency(frequency)
        .set_q_factor(0.7);
    for _ in 0..3 {
        filter.pass(&wavetable);
    }
    filter.process(&mut wavetable);

    normalize(&mut wavetable);

    wavetable
}

pub fn scale<const N: usize>(data: &[f32]) -> [f32; N] {
    let mut scaled = [0.0; N];
    for (i, x) in scaled.iter_mut().enumerate() {
        let position = i as f32 / N as f32;
        let index = position * data.len() as f32;

        let index_a = index as usize;
        let index_b = (index_a + 1).min(data.len() - 1);

        let a = data[index_a];
        let delta_to_b = data[index_b] - a;

        *x = a + delta_to_b * index.fract();
    }

    scaled
}
