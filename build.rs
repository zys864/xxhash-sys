use std::{env, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    // Configure C build
    env::set_var(
        "CFLAGS",
        format!(
            "-I{dir}/vendor/xxhash/ {old}",
            dir = env::var("CARGO_MANIFEST_DIR")?,
            old = env::var("CFLAGS").unwrap_or_else(|_| "".to_string())
        ),
    );
    // Compile xxhash
    cc::Build::new()
        .file("vendor/xxHash/xxhash.c")
        .compile("xxhash");

    // Generate rust bindings
    println!("cargo:rerun-if-changed=src/bindings.h");
    bindgen::Builder::default()
        .clang_arg("-I./vendor/xxhash/")
        .header("vendor/xxHash/xxhash.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("xxhash_bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
