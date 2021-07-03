use core::f32::consts::PI;
use std::fs::File;

use super::processing;
use super::saving;

const OVERSAMPLED_LENGTH: usize = 1024 * 4;

pub fn generate_bank(name: &str, sources: &[&str], module: &mut File) {
    for (i, source) in sources.iter().enumerate() {
        let name = format!("{}_{}", name, i);

        let oversampled = {
            let mut raw_waveform = File::open(format!("build/banks/sources/{}", source)).unwrap();
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
                saving::dump_wavetable(module, &name, $factor, &wavetable);
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

        let wavetable = processing::scale::<64>(&sine());
        saving::dump_wavetable(module, &name, 1, &wavetable);

        saving::dump_factor_list(
            module,
            &name,
            &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
        );
    }
}

fn sine() -> [f32; OVERSAMPLED_LENGTH] {
    let mut wavetable = [0.0; OVERSAMPLED_LENGTH];
    for (i, x) in wavetable.iter_mut().enumerate() {
        *x = f32::sin(i as f32 / (OVERSAMPLED_LENGTH as f32) * 2.0 * PI);
    }
    wavetable
}
