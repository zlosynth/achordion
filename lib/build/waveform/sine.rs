use core::f32::consts::PI;

use super::consts::OVERSAMPLED_LENGTH;

pub fn sine() -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = f32::sin(i as f32 / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI);
    }
    wavetable
}
