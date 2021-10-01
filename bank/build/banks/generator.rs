use std::fs::File;

use super::saving;

const WAVEFORM_LENGTH: usize = 600;

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
            let scaled = scaled::<WAVEFORM_LENGTH>(&waveform);
            to_u16(scaled)
        };

        saving::dump_wavetable(module, &name, oversampled);
    }
}

fn scaled<const N: usize>(data: &[f32]) -> [f32; N] {
    let mut scaled = [0.0; N];
    for (i, x) in scaled.iter_mut().enumerate() {
        let position = i as f32 / N as f32;
        let index = position * data.len() as f32;

        let index_a = index as usize;
        let index_b = (index_a + 1).min(data.len() - 1);

        let a = data[index_a];
        let delta_to_b = data[index_b] - a;

        *x = a + delta_to_b * index.fract();
    }

    scaled
}

fn to_u16<const N: usize>(data: [f32; N]) -> [u16; N] {
    let mut result = [0; N];
    result
        .iter_mut()
        .zip(data.iter())
        .for_each(|(x, y)| *x = ((y + 1.0) * f32::powi(2.0, 15)) as u16);
    result
}
