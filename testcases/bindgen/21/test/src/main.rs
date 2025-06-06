use bindgen;
use std::env;
use std::path::PathBuf;
fn main() {
    bindgen::builder()
        .header("../../input.h")
        .header_contents("input.h", "")
        .generate()
        .expect("Unable to generate bindings");
}