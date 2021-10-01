#[allow(unused_imports)]
use micromath::F32Ext;

use sirena::state_variable_filter::{LowPass, StateVariableFilter};

use super::factor::Factors;

const WAVEFORM_LENGTH: usize = 600;

const SINE: [u16; 64] = [
    32768, 35979, 39160, 42280, 45307, 48214, 50972, 53555, 55938, 58098, 60013, 61666, 63041,
    64125, 64906, 65378, 65535, 65378, 64906, 64125, 63041, 61666, 60013, 58098, 55938, 53555,
    50972, 48214, 45307, 42280, 39160, 35979, 32767, 29556, 26375, 23255, 20228, 17321, 14563,
    11980, 9597, 7437, 5522, 3869, 2494, 1410, 629, 157, 0, 157, 629, 1410, 2494, 3869, 5522, 7437,
    9597, 11980, 14563, 17321, 20228, 23255, 26375, 29556,
];

impl Factors {
    pub fn from_raw(raw: &[u16; WAVEFORM_LENGTH]) -> Self {
        Self {
            factor1024: process::<1024>(raw, 1024.0),
            factor512: process::<512>(raw, 512.0),
            factor256: process::<256>(raw, 256.0),
            factor128: process::<128>(raw, 128.0),
            factor64: process::<64>(raw, 64.0),
            factor32: process::<64>(raw, 16.0),
            factor16: process::<64>(raw, 8.0),
            factor8: process::<64>(raw, 4.0),
            factor4: process::<64>(raw, 2.0),
            factor2: process::<64>(raw, 1.0),
            factor1: SINE,
        }
    }
}

fn process<const N: usize>(data: &[u16; WAVEFORM_LENGTH], frequency: f32) -> [u16; N] {
    let data_f32 = to_f32(data);
    let oversampled = scale::<4096>(&data_f32);
    let filtered = filter(oversampled, frequency);
    let undersampled = scale(&filtered);
    to_u16(&undersampled)
}

fn filter<const N: usize>(mut wavetable: [f32; N], frequency: f32) -> [f32; N] {
    let mut filter = StateVariableFilter::new((N * 2) as u32);
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

fn scale<const N: usize>(data: &[f32]) -> [f32; N] {
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

fn to_f32<const N: usize>(data: &[u16; N]) -> [f32; N] {
    let mut result = [0.0; N];
    result
        .iter_mut()
        .zip(data.iter())
        .for_each(|(x, y)| *x = *y as f32 / f32::powi(2.0, 15) - 1.0);
    result
}

fn to_u16<const N: usize>(data: &[f32; N]) -> [u16; N] {
    let mut result = [0; N];
    result
        .iter_mut()
        .zip(data.iter())
        .for_each(|(x, y)| *x = ((y + 1.0) * f32::powi(2.0, 15)) as u16);
    result
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
