mod generator;
mod harsh;
mod perfect;
mod processing;
mod saving;
mod soft;

use std::fs::File;
use std::path::Path;

pub fn register(module: &mut File, package: &Path) {
    generator::register_in_package(module);
    generator::generate_module(package);

    perfect::register(module, package);
    harsh::register(module, package);
    soft::register(module, package);
}
