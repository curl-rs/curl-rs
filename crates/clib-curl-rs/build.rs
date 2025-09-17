extern crate cbindgen;

use std::{
    env,
    path::{Path, PathBuf},
};

use cbindgen::{Config, Language};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let header_out = PathBuf::from(&out_dir).join("include/bindings.h");
    let src_path = Path::new("lib.rs");

    let mut config = Config::default();

    config.language = Language::C;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_src(src_path)
        .with_config(config)
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file(header_out);
            },
        );

    println!("cargo:rerun-if-changed=build.rs");
}
