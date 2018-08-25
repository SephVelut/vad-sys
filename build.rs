extern crate bindgen;

use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rustc-link-search=./lib");
    println!("cargo:rustc-link-lib=static=vad");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("output");

    //let out_path = PathBuf::from(dir.display().to_string());
    let out_path = PathBuf::from("output");
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
