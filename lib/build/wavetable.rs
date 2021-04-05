// TODO: Split helpers and individual waveforms
// TODO: Waveform code rendering should happen here, so the implementor can
// select how hifi the form should be

use core::f32::consts::PI;

use sirena::state_variable_filter::{LowPass, StateVariableFilter};

pub const OVERSAMPLED_LENGTH: usize = 1024 * 4;

pub fn sine() -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = sin(i as f32);
    }
    wavetable
}

pub fn saw() -> [f32; OVERSAMPLED_LENGTH] {
    let niquist = OVERSAMPLED_LENGTH / 2;
    let harmonics = niquist - 1;
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];

    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = sin(i as f32);
        for j in 2..harmonics {
            if j % 2 == 0 {
                *x -= sin(i as f32 * j as f32) / j as f32;
            } else {
                *x += sin(i as f32 * j as f32) / j as f32;
            }
        }
    }

    normalize(&mut wavetable);

    wavetable
}

fn sin(phase: f32) -> f32 {
    f32::sin(phase / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI)
}

pub fn to_u16(x: f32) -> u16 {
    ((x + 1.0) * f32::powi(2.0, 15)) as u16
}

pub fn to_12bit(x: u16) -> u16 {
    x >> 4
}

fn normalize(data: &mut [f32]) {
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

macro_rules! fn_undersampled {
    ( $func_name:ident, $target_size:expr ) => {
        pub fn $func_name(data: [f32; OVERSAMPLED_LENGTH]) -> [f32; $target_size] {
            assert!(data.len() >= $target_size);
            assert!(data.len() % $target_size == 0);

            let ratio = data.len() / $target_size;

            let mut undersampled_data = [0.0; $target_size];
            for i in 0..$target_size {
                undersampled_data[i] = data[i * ratio];
            }

            undersampled_data
        }
    };
}

fn_undersampled!(undersampled_1024, 1024);
fn_undersampled!(undersampled_512, 512);
fn_undersampled!(undersampled_256, 256);
fn_undersampled!(undersampled_128, 128);
fn_undersampled!(undersampled_64, 64);
