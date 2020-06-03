use std::env;

fn main() {
    let mut vers = String::new();
    vers += &format!("!define VERSIONMAJOR {}\n", env!("CARGO_PKG_VERSION_MAJOR"));
    vers += &format!("!define VERSIONMINOR {}\n", env!("CARGO_PKG_VERSION_MINOR"));
    vers += &format!("!define VERSIONBUILD {}\n", env!("CARGO_PKG_VERSION_PATCH"));

    std::fs::write("../installer/versions.nsi", vers).expect("Unable to write to versions.nsi");
}
