extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // cbindgen::Builder::new()
    //     .with_crate(crate_dir)
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file("target/release/libffout.h");
    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("target/release/libffout.h");
}
