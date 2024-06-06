use std::{path::PathBuf, str::FromStr};

fn main() {
    println!("cargo:rustc-link-search=libs");
    println!("cargo:rustc-link-lib=cubiomes");

    let bindings = bindgen::Builder::default()
        .header("libs/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .derive_default(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_ord(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from_str("out_bindings").unwrap();
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
