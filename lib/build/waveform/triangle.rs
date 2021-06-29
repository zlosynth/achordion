use core::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use super::sine;
use crate::rustfmt;

const NAME: &str = "triangle";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    let oversampled = triangle();

    macro_rules! dump {
        ( $factor:expr, $cutoff:expr, $len:expr ) => {
            let wavetable = processing::scale::<$len>(&processing::filtered(&oversampled, $cutoff));
            builder::dump_wavetable(&mut module, NAME, $factor, &wavetable);
        };
    }

    dump!(1024, 1024.0, 1024);
    dump!(512, 512.0, 512);
    dump!(256, 256.0, 256);
    dump!(128, 128.0, 128);
    dump!(64, 64.0, 64);
    dump!(32, 16.0, 64);
    dump!(16, 8.0, 64);
    dump!(8, 4.0, 64);
    dump!(4, 2.0, 64);
    dump!(2, 1.0, 64);

    let wavetable = processing::scale::<64>(&sine::sine());
    builder::dump_wavetable(&mut module, NAME, 1, &wavetable);

    builder::dump_factor_list(
        &mut module,
        NAME,
        &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
    );

    rustfmt::format(path.to_str().unwrap());
}

pub fn triangle() -> [f32; OVERSAMPLED_LENGTH] {
    let niquist = OVERSAMPLED_LENGTH / 2;
    let harmonics = niquist - 1;
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];

    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = sin(i);
        for j in 2..harmonics {
            if j % 4 == 3 {
                *x -= sin(i * j) / (j as f32).powi(2);
            } else if j % 4 == 1 {
                *x += sin(i * j) / (j as f32).powi(2);
            }
        }
    }

    processing::normalize(&mut wavetable);

    wavetable
}

fn sin(phase: usize) -> f32 {
    f32::sin(phase as f32 / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI)
}
