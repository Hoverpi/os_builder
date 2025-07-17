use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-lib=c");

    let bindings = bindgen::Builder::default()
        .header("./wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("No se pudieron generar los bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Error escribiendo bindings.rs");
}
