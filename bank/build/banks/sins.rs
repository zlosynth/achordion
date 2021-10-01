use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::generator;
use crate::rustfmt;

const NAME: &str = "sins";

pub fn register(module: &mut File, package: &Path) {
    writeln!(module, "pub mod {};", NAME).unwrap();

    let path = package.join(format!("{}.rs", NAME));
    let mut module = std::fs::File::create(&path).unwrap();

    generator::generate_bank(
        NAME,
        &[
            "sin_mul_10.wav",
            "sin_mul_9.wav",
            "sin_mul_8.wav",
            "sin_mul_7.wav",
            "sin_mul_6.wav",
            "sin_mul_5.wav",
            "sin_mul_4.wav",
            "sin_mul_3.wav",
            "sin_mul_2.wav",
            "sin_mul_1.wav",
            "sin_seq_2.wav",
            "sin_seq_3.wav",
            "sin_seq_5.wav",
            "sin_seq_6.wav",
            "sin_seq_7.wav",
            "sin_seq_9.wav",
            "sin_seq_11.wav",
            "sin_seq_13.wav",
            "sin_seq_16.wav",
            "sin_seq_30.wav",
            "sin_seq_60.wav",
        ],
        &mut module,
    );

    rustfmt::format(path.to_str().unwrap());
}
