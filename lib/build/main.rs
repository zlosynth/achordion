mod banks;
mod rustfmt;

use std::fs::File;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/banks/");
    println!("cargo:rerun-if-changed=build/banks/sources");

    let wavetable_package = Path::new("src/waveform");
    let mut wavetable_module = File::create(wavetable_package.join("mod.rs")).unwrap();

    banks::register(&mut wavetable_module, wavetable_package);

    rustfmt::format(wavetable_package.join("mod.rs").to_str().unwrap());
}
