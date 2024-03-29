use core::f32::consts::PI;

#[allow(unused_imports)]
use micromath::F32Ext;

use microfft::{complex::cfft_2048, Complex32};

pub fn filter(mut wavetable: [f32; 2048], fraction: f32) -> [f32; 2048] {
    if fraction < 2.0 / 1024.0 {
        wavetable.iter_mut().enumerate().for_each(|(i, x)| {
            let phase = i as f32 / 2048.0;
            *x = f32::sin(2.0 * PI * phase);
        });
        return wavetable;
    }

    let mut complex_wavetable = {
        let mut complex_wavetable = [Complex32::default(); 2048];
        complex_wavetable
            .iter_mut()
            .zip(wavetable.iter())
            .for_each(|(c, f)| c.re = *f);
        complex_wavetable
    };

    // Convert to the frequency domain
    let complex_wavetable = microfft::complex::cfft_2048(&mut complex_wavetable);

    // Clear frequency bins above given cutoff
    complex_wavetable[(2048.0 * fraction) as usize..]
        .iter_mut()
        .for_each(|c| *c = Complex32::new(0.0, 0.0));

    // Flip real and imaginary parts of the number to prepare for inverse FFT
    complex_wavetable
        .iter_mut()
        .for_each(|c| core::mem::swap(&mut c.re, &mut c.im));

    // Convert back to the time domain
    let complex_wavetable = cfft_2048(complex_wavetable);

    // Finish inverse FFT by flipping real and imaginary numbers back
    complex_wavetable
        .iter_mut()
        .for_each(|c| core::mem::swap(&mut c.re, &mut c.im));

    let real_wavetable = {
        let mut wavetable = [0.0; 2048];
        wavetable
            .iter_mut()
            .zip(complex_wavetable.iter())
            .for_each(|(f, c)| *f = c.re);
        wavetable
    };

    real_wavetable
}
