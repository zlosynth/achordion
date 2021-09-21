mod generator;
mod harsh;
mod perfect;
mod saving;
mod soft;
mod vocal;

use std::fs::File;
use std::path::Path;

pub fn register(module: &mut File, package: &Path) {
    perfect::register(module, package);
    harsh::register(module, package);
    soft::register(module, package);
    vocal::register(module, package);
}
