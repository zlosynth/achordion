use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::generator;
use crate::rustfmt;

const NAME: &str = "soft";

pub fn register(module: &mut File, package: &Path) {
    writeln!(module, "pub mod {};", NAME).unwrap();

    let path = package.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generator::generate_bank(
        NAME,
        &[
            "AKWF_granular_0033.wav",
            "AKWF_granular_0022.wav",
            "AKWF_flute_0009.wav",
            "AKWF_fmsynth_0117.wav",
            "AKWF_fmsynth_0121.wav",
            "AKWF_fmsynth_0118.wav",
            "AKWF_fmsynth_0054.wav",
            "AKWF_violin_0003.wav",
            "AKWF_violin_0012.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}
