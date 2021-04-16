use core::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use super::sine;
use crate::rustfmt;

const NAME: &str = "saw";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    let oversampled = saw();

    macro_rules! dump {
        ( $factor:expr, $cutoff:expr, $undersampler:expr ) => {
            let wavetable = $undersampler(processing::filtered(&oversampled, $cutoff));
            builder::dump_wavetable(&mut module, NAME, $factor, &wavetable);
        };
    }

    dump!(1024, 512.0, processing::undersampled_1024);
    dump!(512, 256.0, processing::undersampled_512);
    dump!(256, 128.0, processing::undersampled_256);
    dump!(128, 64.0, processing::undersampled_128);
    dump!(64, 32.0, processing::undersampled_64);
    dump!(32, 16.0, processing::undersampled_64);
    dump!(16, 8.0, processing::undersampled_64);
    dump!(8, 4.0, processing::undersampled_64);
    dump!(4, 2.0, processing::undersampled_64);
    dump!(2, 1.0, processing::undersampled_64);

    let wavetable = processing::undersampled_64(sine::sine());
    builder::dump_wavetable(&mut module, NAME, 1, &wavetable);

    builder::dump_factor_list(
        &mut module,
        NAME,
        &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
    );

    rustfmt::format(path.to_str().unwrap());
}

fn saw() -> [f32; OVERSAMPLED_LENGTH] {
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
