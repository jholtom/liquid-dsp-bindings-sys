use std::env;
use std::env::consts;
use std::path::Path;

use std::fs::File;
use std::io::Write;

fn format_write(builder: bindgen::Builder, output: &str) {
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = File::create(output).unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(deprecated)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn main() {

    let builder = common_builder()
        .header("wrapper.h");

    format_write(builder, "src/lib.rs");

    if let Some(lib_dir) = env::var_os("LIQUID_LIB_DIR") {

        let lib_dir = Path::new(&lib_dir);
        let dylib_name;
        let target = env::var("TARGET").unwrap();

        if target.contains("windows") {
            dylib_name = "liquid.lib".to_owned();
        } else {
            dylib_name = format!("{}liquid{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
        }

        if lib_dir.join(dylib_name).exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=liquid");
            return;
        }

        println!("cargo:warning=library not found in {}", lib_dir.display());
        println!("export LIQUID_LIB_DIR with the path to the liquid library, for example ");
        println!("export LIQUID_LIB_DIR=/usr/lib  ");

        panic!();
    }
}
