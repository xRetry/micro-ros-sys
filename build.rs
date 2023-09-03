use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let libdir_path = PathBuf::from("hello")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("hello.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let obj_path = libdir_path.join("hello.o");
    let lib_path = libdir_path.join("libhello.a");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=hello");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("hello.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        panic!("could not compile object file");
    }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        panic!("could not create library");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
