use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::generator;
use crate::rustfmt;

const NAME: &str = "perfect";

pub fn register(module: &mut File, directory: &Path) {
    writeln!(module, "pub mod {};", NAME).unwrap();

    let path = directory.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generator::generate_bank(
        NAME,
        &[
            "AKWF_tri.wav",
            "AKWF_sin.wav",
            "AKWF_squ.wav",
            "AKWF_saw.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}
