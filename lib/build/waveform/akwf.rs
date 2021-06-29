use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::builder;
use super::consts::OVERSAMPLED_LENGTH;
use super::processing;
use super::sine;
use crate::rustfmt;

const NAME: &str = "akwf";

pub fn register_in_package(module: &mut File) {
    writeln!(module, "pub mod {};", NAME).unwrap();
}

pub fn generate_module(directory: &Path) {
    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generate_bank(
        "fm",
        &[
            "AKWF_fmsynth_0041.wav",
            "AKWF_fmsynth_0042.wav",
            "AKWF_fmsynth_0022.wav",
            "AKWF_fmsynth_0086.wav",
            "AKWF_fmsynth_0008.wav",
            "AKWF_fmsynth_0026.wav",
        ],
        &mut module,
    );

    generate_bank(
        "eguitar",
        &["AKWF_eguitar_0021.wav", "AKWF_eguitar_0011.wav"],
        &mut module,
    );

    generate_bank(
        "distorted",
        &[
            "AKWF_distorted_0016.wav",
            "AKWF_distorted_0021.wav",
            "AKWF_distorted_0043.wav",
            "AKWF_distorted_0006.wav",
            "AKWF_distorted_0033.wav",
        ],
        &mut module,
    );

    generate_bank(
        "granular",
        &[
            "AKWF_granular_0022.wav",
            "AKWF_granular_0010.wav",
            "AKWF_granular_0033.wav",
            "AKWF_granular_0014.wav",
            "AKWF_granular_0005.wav",
            "AKWF_granular_0003.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}

fn generate_bank(name: &str, sources: &[&str], module: &mut File) {
    for (i, source) in sources.iter().enumerate() {
        let name = format!("{}_{}", name, i);

        let oversampled = {
            let mut raw_waveform =
                File::open(format!("build/waveform/sources/{}", source)).unwrap();
            let (_, data) = wav::read(&mut raw_waveform).unwrap();
            let mut waveform = Vec::new();
            for x in data.as_sixteen().unwrap() {
                waveform.push(*x as f32 / f32::powi(2.0, 15));
            }
            processing::scale::<OVERSAMPLED_LENGTH>(&waveform)
        };

        macro_rules! dump {
            ( $factor:expr, $cutoff:expr, $len:expr ) => {
                let wavetable =
                    processing::scale::<$len>(&processing::filtered(&oversampled, $cutoff));
                builder::dump_wavetable(module, &name, $factor, &wavetable);
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
        builder::dump_wavetable(module, &name, 1, &wavetable);

        builder::dump_factor_list(
            module,
            &name,
            &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
        );
    }
}
