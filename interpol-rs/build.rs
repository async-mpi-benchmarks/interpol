extern crate cbindgen;

use std::path::Path;

fn main() {
    // Generate the C header file to be able to call the Rust `register_*` functions
    cbindgen::generate(Path::new("./"))
        .expect("Unable to generate bindings")
        .write_to_file("../include/interpol.h");
}
