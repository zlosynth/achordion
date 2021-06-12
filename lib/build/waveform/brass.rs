use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use super::sine;
use crate::rustfmt;

const NAME: &str = "brass";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let oversampled = {
        let mut raw_waveform = File::open("build/waveform/brass.raw").unwrap().bytes();
        let mut waveform = Vec::new();
        while let Some(a) = raw_waveform.next() {
            let b = raw_waveform.next().unwrap();
            let c = i16::from_ne_bytes([a.unwrap(), b.unwrap()]);
            waveform.push(c as f32 / f32::powi(2.0, 15));
        }
        processing::scale::<OVERSAMPLED_LENGTH>(&waveform)
    };

    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    macro_rules! dump {
        ( $factor:expr, $cutoff:expr, $len:expr ) => {
            let wavetable = processing::scale::<$len>(&processing::filtered(&oversampled, $cutoff));
            builder::dump_wavetable(&mut module, NAME, $factor, &wavetable);
        };
    }

    dump!(1024, 512.0, 1024);
    dump!(512, 256.0, 512);
    dump!(256, 128.0, 256);
    dump!(128, 64.0, 128);
    dump!(64, 32.0, 64);
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
