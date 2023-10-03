use std::env;
use std::path::{PathBuf, Path};

use bindgen::CargoCallbacks;

fn main() {
    let repos = [
        ["ros2", "eProsima/micro-CDR", "micro-CDR", "include"],
        ["iron", "micro-ROS/rcl", "rcl", "rcl/rcl/include"],
	    ["iron", "ros2/rclc", "rclc", "rclc/rclc/include"],
	    ["iron", "micro-ROS/rcutils", "rcutils", "include"],
	    ["iron", "micro-ROS/micro_ros_msgs", "micro_ros_msgs", ""], // TODO
	    ["iron", "micro-ROS/rosidl_typesupport", "rosidl_typesupport", "rosidl_typesupport_c/include"],
	    ["iron", "micro-ROS/rosidl_typesupport_microxrcedds", "rosidl_typesupport_microxrcedds", "rosidl_typesupport_microxrcedds/include"],
	    ["iron", "ros2/rosidl", "rosidl", ""], // TODO
	    ["iron", "ros2/rosidl_dynamic_typesupport", "rosidl_dynamic_typesupport", "include"],
	    ["iron", "ros2/rmw", "rmw", "rmw/include"],
	    ["iron", "ros2/rcl_interfaces", "rcl_interfaces", ""], // TODO
	    ["iron", "ros2/rosidl_defaults", "rosidl_defaults", ""], // TODO
	    ["iron", "ros2/unique_identifier_msgs", "unique_identifier_msgs", ""], // TODO
	    ["iron", "ros2/common_interfaces", "common_interfaces", ""], // TODO
	    ["iron", "ros2/example_interfaces", "example_interfaces", ""], // TODO
	    ["iron", "ros2/test_interface_files", "test_interface_files", ""], // TODO
	    ["iron", "ros2/rmw_implementation", "rmw_implementation", "rmw_implementation/src"],
	    ["iron", "ros2/rcl_logging", "rcl_logging", "rcl_logging_interface/include"],
	    ["iron", "ros2/ros2_tracing", "ros2_tracing", "tracetools/include"],
	    ["iron", "micro-ROS/micro_ros_utilities", "micro_ros_utilities", "include"],
	    ["iron", "ros2/rosidl_core", "rosidl_core", ""], // TODO
    ];

    for repo in repos {
        if Path::new(&format!("lib/{}", repo[2])).exists() { continue; }
        let mut cmd = std::process::Command::new("git");
        cmd.arg("clone")
            .arg("-b")
            .arg(repo[0])
            .arg(format!("https://github.com/{}", repo[1]))
            .arg(format!("lib/{}", repo[2]));

        //panic!("{:?}", cmd.output());

        if !cmd.output().unwrap().status.success() {
            panic!("could not clone repository {}", repo[1]);
        }
    }

    let libdir_path = PathBuf::from("lib")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("wrapper.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let obj_path = libdir_path.join("rclc.o");
    let lib_path = libdir_path.join("librclc.a");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=hello");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    //if !std::process::Command::new("clang")
    //    .arg("-c")
    //    .arg("-o")
    //    .arg(&obj_path)
    //    .arg(libdir_path.join("hello.c"))
    //    .output()
    //    .expect("could not spawn `clang`")
    //    .status
    //    .success()
    //{
    //    panic!("could not compile object file");
    //}

    //if !std::process::Command::new("ar")
    //    .arg("rcs")
    //    .arg(lib_path)
    //    .arg(obj_path)
    //    .output()
    //    .expect("could not spawn `ar`")
    //    .status
    //    .success()
    //{
    //    panic!("could not create library");
    //}

    let mut bindings = bindgen::Builder::default()
        .header(headers_path_str);
    
    for repo in repos {
        bindings = bindings.clang_arg(format!("-Ilib/{}/{}", repo[2], repo[3]));
    }
    
    let bindings = bindings
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("rclc.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
