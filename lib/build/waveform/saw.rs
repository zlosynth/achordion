use core::f32::consts::PI;

use super::consts::OVERSAMPLED_LENGTH;
use super::processing;

pub fn saw() -> [f32; OVERSAMPLED_LENGTH] {
    let niquist = OVERSAMPLED_LENGTH / 2;
    let harmonics = niquist - 1;
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];

    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = sin(i);
        for j in 2..harmonics {
            if j % 2 == 0 {
                *x -= sin(i * j) / j as f32;
            } else {
                *x += sin(i * j) / j as f32;
            }
        }
    }

    processing::normalize(&mut wavetable);

    wavetable
}

fn sin(phase: usize) -> f32 {
    f32::sin(phase as f32 / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI)
}
