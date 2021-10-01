use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::generator;
use crate::rustfmt;

const NAME: &str = "vocal";

pub fn register(module: &mut File, package: &Path) {
    writeln!(module, "pub mod {};", NAME).unwrap();

    let path = package.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generator::generate_bank(
        NAME,
        &[
            "vocal_a.wav",
            "vocal_e.wav",
            "vocal_i.wav",
            "vocal_o.wav",
            "vocal_u.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}
