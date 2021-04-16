mod waveform;

use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/waveform/");

    let wavetable_package = Path::new("src/waveform");
    let mut wavetable_module = File::create(wavetable_package.join("mod.rs")).unwrap();

    waveform::saw::register_in_package(&mut wavetable_module);
    waveform::saw::generate_module(wavetable_package);

    waveform::sine::register_in_package(&mut wavetable_module);
    waveform::sine::generate_module(wavetable_package);

    waveform::triangle::register_in_package(&mut wavetable_module);
    waveform::triangle::generate_module(wavetable_package);

    waveform::square::register_in_package(&mut wavetable_module);
    waveform::square::generate_module(wavetable_package);

    rustfmt(wavetable_package.join("mod.rs").to_str().unwrap());
}

fn rustfmt(path: &str) {
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("failed to execute rustfmt");
}
