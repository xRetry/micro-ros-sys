use std::env;
use std::path::{PathBuf, Path};

use bindgen::CargoCallbacks;

fn main() {

    let root_dir = env::current_dir().unwrap();

    if !Path::new("lib/libmicroros.a").exists() {
        let mut cmd = std::process::Command::new("docker");
        cmd.arg("build")
            .arg("-t")
            .arg("microros-build")
            .arg(".");

        if !cmd.output().unwrap().status.success() {
            panic!("Could not build docker image");
        }

        let mut cmd = std::process::Command::new("docker");
        cmd.arg("run")
            .arg("-it")
            .arg("--net=host")
            .arg("--name")
            .arg("microros-build")
            .arg("microros-build")
            .arg("-p")
            .arg("esp32");

        if !cmd.output().unwrap().status.success() {
            panic!("Could not build libmicroros");
        }

        let mut cmd = std::process::Command::new("docker");
        cmd.arg("cp")
            .arg("microros-build:/uros_ws/firmware/build")
            .arg("lib");

        if !cmd.output().unwrap().status.success() {
            panic!("Could not copy libmicroros");
        }
    }

    //let libdir_path = Path::new("lib");
    let headers_path = Path::new("wrapper.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");
    //let lib_path = libdir_path.join("libmicroros.a");

    println!("cargo:rustc-link-search=native=lib/include");
    println!("cargo:rustc-link-lib=static=lib/libmicroros");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .clang_arg("-I/lib/include")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("rclc.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
