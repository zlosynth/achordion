use std::fs::File;
use std::io::Write;

pub fn dump_wavetable(module: &mut File, name: &str, wavetable: [u16; 600]) {
    writeln!(module, "pub const {}: [u16; 600] = [", name.to_uppercase(),).unwrap();

    wavetable.iter().for_each(|x| {
        write!(module, "{}_u16, ", x).unwrap();
    });

    writeln!(module, "\n];").unwrap();
}
