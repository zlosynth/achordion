use std::process::Command;

pub fn format(path: &str) {
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("failed to execute rustfmt");
}
