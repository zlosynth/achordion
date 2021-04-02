mod wavetable;

use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/wavetable.rs");

    let mut wavetable_module = std::fs::File::create("src/wavetable/mod.rs").unwrap();
    writeln!(
        wavetable_module,
        "pub const LENGTH: usize = {};",
        wavetable::LENGTH
    )
    .unwrap();

    generate_wavetable(&mut wavetable_module, "saw", wavetable::saw);
    generate_wavetable(&mut wavetable_module, "sine", wavetable::sine);
}

fn generate_wavetable(
    wavetable_module: &mut File,
    name: &str,
    generator: fn() -> [f32; wavetable::LENGTH],
) {
    expose_wavetable_in_module(wavetable_module, name);
    generate_wavetable_array(name, generator);
}

fn expose_wavetable_in_module(module: &mut File, name: &str) {
    writeln!(module, "pub mod {};", name).unwrap();
    writeln!(module, "pub use {}::{};", name, name.to_uppercase()).unwrap();
}

fn generate_wavetable_array(name: &str, generator: fn() -> [f32; wavetable::LENGTH]) {
    let path = format!("src/wavetable/{}.rs", name);
    let mut module = std::fs::File::create(&path).unwrap();

    writeln!(
        module,
        "pub const {}: [u16; {}] = [",
        name.to_uppercase(),
        wavetable::LENGTH
    )
    .unwrap();

    generator()
        .iter()
        .copied()
        .map(wavetable::to_u16)
        .map(wavetable::to_12bit)
        .for_each(|x| {
            write!(module, "{}, ", x).unwrap();
        });
    writeln!(module, "\n];").unwrap();
}
