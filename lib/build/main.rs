mod rustfmt;
mod waveform;

use std::fs::File;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/waveform/");

    let wavetable_package = Path::new("src/waveform");
    let mut wavetable_module = File::create(wavetable_package.join("mod.rs")).unwrap();

    waveform::sine::register_in_package(&mut wavetable_module);
    waveform::sine::generate_module(wavetable_package);

    waveform::akwf::register_in_package(&mut wavetable_module);
    waveform::akwf::generate_module(wavetable_package);

    rustfmt::format(wavetable_package.join("mod.rs").to_str().unwrap());
}
