extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){
    // Use user-provided header and lib files in the `liquid` subdirectory if provided
    // Otherwise use the system-provided header and lib
    let header = match std::fs::metadata("liquid/liquid.h") {
        // `liquid/liquid.h` exists. Use the user-provided static lib
        Ok(_metadata) => {
            println!("cargo:rustc-link-search={}", PathBuf::from("liquid").canonicalize().unwrap().to_str().unwrap());
            println!("cargo:rustc-link-lib=static=liquid");
            "liquid/liquid.h"
        },
        // Use the system-provided header and lib
        Err(_err) => {
            println!("cargo:rustc-link-lib=liquid");
            "wrapper.h"
        }
    };

    // Generate the bindings
    let bindings = bindgen::Builder::default()
    .generate_comments(true)
    .header(header)
    .generate()
    .expect("Failed to generate bindings");

    // Write the bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
