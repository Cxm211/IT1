use bindgen;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct RenameCallbacks {}

impl bindgen::callbacks::ParseCallbacks for RenameCallbacks {
    fn item_name(&self, original_variant_name: &str) -> Option<String> {
        if original_variant_name == "Test" {
            Some("AnotherName".into())
        } else {
            None
        }
    }
}

fn main() {

    let bindings = bindgen::Builder::default()
        .header("../../input.h")
        .parse_callbacks(Box::new(RenameCallbacks {}))
        .allowlist_type("Test")
        .bitfield_enum("Test")
        .generate()
        .expect("Couldn't generate bindings!");
    bindings
        .write_to_file("../../generated_bindings.rs")
        .expect("Couldn't write bindings!");
}