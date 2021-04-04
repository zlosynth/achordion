use core::f32::consts::PI;

pub const FULL_LENGTH: usize = 1024;

pub fn sine() -> [f32; FULL_LENGTH] {
    let mut wavetable = [0.0; FULL_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = sin(i as f32);
    }
    wavetable
}

pub fn saw() -> [f32; FULL_LENGTH] {
    let niquist = FULL_LENGTH / 2;
    let harmonics = niquist - 1;
    let mut wavetable = [0.0; FULL_LENGTH];

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
    f32::sin(phase / (FULL_LENGTH as f32) * 2.0 * PI)
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
