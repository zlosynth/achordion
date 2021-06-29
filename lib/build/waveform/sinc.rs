use core::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use super::sine;
use crate::rustfmt;

const NAME: &str = "sinc";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    let mut oversampled = sinc();
    processing::normalize(&mut oversampled);

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

fn sinc() -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        let position = i as f32 / OVERSAMPLED_LENGTH as f32;
        let phase = (position - 0.5) * 5.0;
        if phase > -0.0001 && phase < 0.0001 {
            *x = 1.0;
            continue;
        }

        let y = phase * 2.0 * PI;
        *x = f32::sin(y) / y;
    }
    wavetable
}
