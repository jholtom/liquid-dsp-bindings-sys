extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){
    // If on windows, search in the `./liquid` dir for the libliquid.a file
    // NOTE: On windows, you should be buildling the static library yourself under MingW64 (or similar). Put the `libliquid.a` file in the `./liquid` dir manually.
    #[cfg(target_os="windows")]
    println!("cargo:rustc-link-search={}", PathBuf::from("liquid").canonicalize().unwrap().to_str().unwrap());
    
    // Link the liquid library
    println!("cargo:rustc-link-lib=liquid");

    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("liquid/liquid.h")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
