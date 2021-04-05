use std::fs::File;
use std::io::Write;
use std::process::Command;

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
        .map(processing::to_12bit)
        .for_each(|x| {
            write!(module, "{}, ", x).unwrap();
        });

    writeln!(module, "\n];").unwrap();
}

pub fn rustfmt(path: &str) {
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("failed to execute rustfmt");
}
