// adapted from
// https://github.com/gokberkkocak/rust_glucose/blob/master/build.rs
// and
// 1https://rust-lang.github.io/rust-bindgen/non-system-libraries.html

use std::process::Command;

fn main() {
    vendor_configure();
    bind();
    build();
}

fn vendor_configure() {
    // Generate cpp and h files in /vendor/build
    Command::new("./prebuild.sh").output();
}

fn bind() {
    let bindings = bindgen::Builder::default()
        .header("vendor/wrapper.h")
        .
}
