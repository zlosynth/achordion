use std::fs::File;
use std::io::Write;

use super::processing;

pub fn dump_wavetable(module: &mut File, name: &str, factor: usize, wavetable: &[f32]) {
    writeln!(
        module,
        "pub const {}_FACTOR_{}: [u16; {}] = [",
        name.to_uppercase(),
        factor,
        wavetable.len(),
    )
    .unwrap();

    wavetable
        .iter()
        .copied()
        .map(processing::to_u16)
        .map(processing::to_15bit)
        .for_each(|x| {
            write!(module, "{}, ", x).unwrap();
        });

    writeln!(module, "\n];").unwrap();
}

pub fn dump_factor_list(module: &mut File, name: &str, factors: &[usize]) {
    writeln!(
        module,
        "pub const {}_FACTORS: [&[u16]; {}] = [",
        name.to_uppercase(),
        factors.len(),
    )
    .unwrap();

    factors.iter().for_each(|x| {
        write!(module, "&{}_FACTOR_{}, ", name.to_uppercase(), x).unwrap();
    });

    writeln!(module, "\n];").unwrap();
}
