mod waveform;

use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build/main.rs");
    for path in fs::read_dir("build").unwrap() {
        println!(
            "cargo:rerun-if-changed=build/{}",
            path.unwrap().path().display()
        );
    }

    let mut wavetable_module = std::fs::File::create("src/wavetable/mod.rs").unwrap();

    generate_wavetables(&mut wavetable_module, "saw", waveform::saw::saw);
    generate_wavetables(&mut wavetable_module, "sine", waveform::sine::sine);
}

fn generate_wavetables(
    wavetable_module: &mut File,
    name: &str,
    generator: fn() -> [f32; waveform::consts::OVERSAMPLED_LENGTH],
) {
    expose_wavetables_in_module(wavetable_module, name);
    generate_wavetables_module(name, generator);
}

fn expose_wavetables_in_module(module: &mut File, name: &str) {
    writeln!(module, "pub mod {};", name).unwrap();
}

fn generate_wavetables_module(
    name: &str,
    generator: fn() -> [f32; waveform::consts::OVERSAMPLED_LENGTH],
) {
    let path = format!("src/wavetable/{}.rs", name);
    let mut module = std::fs::File::create(&path).unwrap();

    let oversampled = generator();

    // TODO: wavetable structure will be passed array of references to these
    // it will use factor 1 (index 0) for anything between niquist and niquist / 2
    // it will use factor 2 (index 1) for anything between niquist / 2 and niquist / 4
    // it will use factor 4 max(index 2, size - 1) for anything between niquist / 4 and niquist / 8

    // TODO: To save space, one could link factor 2 and 4 both to wavetable 2
    // TODO: To save space, sinusoid can be a single factor

    macro_rules! dump {
        ( $factor:expr, $cutoff:expr, $undersampler:expr ) => {
            let wavetable = $undersampler(waveform::processing::filtered(&oversampled, $cutoff));
            dump_wavetable(&mut module, name, $factor, &wavetable);
        };
    }

    dump!(1024, 512.0, waveform::processing::undersampled_1024);
    dump!(512, 256.0, waveform::processing::undersampled_512);
    dump!(256, 128.0, waveform::processing::undersampled_256);
    dump!(128, 64.0, waveform::processing::undersampled_128);
    dump!(64, 32.0, waveform::processing::undersampled_64);
    dump!(32, 16.0, waveform::processing::undersampled_64);
    dump!(16, 8.0, waveform::processing::undersampled_64);
    dump!(8, 4.0, waveform::processing::undersampled_64);
    dump!(4, 2.0, waveform::processing::undersampled_64);
    dump!(2, 1.0, waveform::processing::undersampled_64);

    let wavetable = waveform::processing::undersampled_64(waveform::sine::sine());
    dump_wavetable(&mut module, name, 1, &wavetable);

    rustfmt(&path);
}

fn dump_wavetable(module: &mut File, name: &str, factor: usize, wavetable: &[f32]) {
    writeln!(
        module,
        "pub const {}_FACTOR_{}: [u16; {}] = [",
        name.to_uppercase(),
        factor,
        wavetable.len(),
    )
    .unwrap();

    wavetable
        .iter()
        .copied()
        .map(waveform::processing::to_u16)
        .map(waveform::processing::to_12bit)
        .for_each(|x| {
            write!(module, "{}, ", x).unwrap();
        });

    writeln!(module, "\n];").unwrap();
}

fn rustfmt(path: &str) {
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("failed to execute rustfmt");
}
