extern crate cbindgen;

use std::path::Path;

fn main() {
    cbindgen::generate(Path::new("./"))
        .expect("Unable to generate bindings")
        .write_to_file("../include/interpol.h");
}
