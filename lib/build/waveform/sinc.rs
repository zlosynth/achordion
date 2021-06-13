use core::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use crate::rustfmt;

const NAME: &str = "sinc";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    let wavetable = processing::scale::<1024>(&sinc());
    builder::dump_wavetable(&mut module, NAME, 1, &wavetable);
    builder::dump_factor_list(&mut module, NAME, &[1]);
    rustfmt::format(path.to_str().unwrap());
}

pub fn sinc() -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        if i == 0 {
            *x = 1.0;
            continue;
        }

        let y = i as f32 / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI;
        *x = f32::sin(y) / y;
    }
    wavetable
}
