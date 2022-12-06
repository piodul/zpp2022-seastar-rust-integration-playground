use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=RUST_FFI_DIR");

    match env::var("RUST_FFI_DIR") {
        Ok(dir) => println!(r"cargo:rustc-link-search={dir}"),
        Err(_) => panic!("RUST_FFI_DIR not set.")
    }
}
