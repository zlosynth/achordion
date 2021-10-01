#[allow(unused_imports)]
use micromath::F32Ext;

use crate::factor::Factors;
use crate::filter::filter;

const WAVEFORM_LENGTH: usize = 600;

impl Factors {
    pub fn from_raw(raw: &[u16; WAVEFORM_LENGTH]) -> Self {
        Self {
            factor1024: process::<1024>(raw, 1.0),
            factor512: process::<512>(raw, 1.0 / 2.0),
            factor256: process::<256>(raw, 1.0 / 4.0),
            factor128: process::<128>(raw, 1.0 / 8.0),
            factor64: process::<64>(raw, 1.0 / 16.0),
            factor32: process::<64>(raw, 1.0 / 32.0),
            // Apply extra filtering on everything below, otherwise aliasing happens
            factor16: process::<64>(raw, 1.0 / 64.0 / 2.0),
            factor8: process::<64>(raw, 1.0 / 128.0 / 2.0),
            factor4: process::<64>(raw, 1.0 / 256.0 / 2.0),
            factor2: process::<64>(raw, 1.0 / 512.0 / 2.0),
            factor1: process::<64>(raw, 1.0 / 1024.0 / 2.0),
        }
    }
}

fn process<const N: usize>(data: &[u16; WAVEFORM_LENGTH], fraction: f32) -> [u16; N] {
    let data_f32 = to_f32(data);
    let oversampled = scale::<2048>(&data_f32);
    let mut filtered = filter(oversampled, fraction);
    normalize(&mut filtered);
    let undersampled = scale(&filtered);
    to_u16(&undersampled)
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
