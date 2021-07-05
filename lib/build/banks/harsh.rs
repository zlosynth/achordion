use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::generator;
use crate::rustfmt;

const NAME: &str = "harsh";

pub fn register(module: &mut File, package: &Path) {
    writeln!(module, "pub mod {};", NAME).unwrap();

    let path = package.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generator::generate_bank(
        NAME,
        &[
            "AKWF_fmsynth_0022.wav",
            "AKWF_distorted_0043.wav",
            "AKWF_oscchip_0010.wav",
            "AKWF_oscchip_0009.wav",
            "AKWF_eguitar_0011.wav",
            "AKWF_eguitar_0021.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}
