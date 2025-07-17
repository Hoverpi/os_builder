use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=parted");

    let bindings = bindgen::Builder::default()
        .header("./wrapper.h")
        .use_core()
        .ctypes_prefix("cty")
        .derive_debug(false)
        .derive_default(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Cannot generate the bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:warning=OUT_DIR is: {:?}", &out);
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Error writing bindings.rs");
}
