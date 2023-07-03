extern crate cbindgen;

use std::{env, error, fs::File, io::Write};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("target/release/libffout.h");

    let pc_file_contents = r#"
prefix=/usr/local
exec_prefix=${prefix}
libdir=${exec_prefix}/lib
includedir=${prefix}/include

Name: ffout
Description: Demo ffmpeg output protocol library
Version: 0.1.0
Libs: -L${libdir} -lffout
Cflags: -I${includedir}/ffout
"#;

    let mut pc_file = File::create("target/release/ffout.pc")?;
    pc_file.write_all(pc_file_contents.as_bytes())?;
    Ok(())
}
