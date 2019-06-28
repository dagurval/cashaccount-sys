extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::fs;

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
                "The `{}` directory is empty, did you forget to pull the submodules?",
                name
                );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn compile_libcashaccount() {
    let mut compiler = cc::Build::new();
    compiler.opt_level(3);
    compiler.flag("-std=c++11");
    compiler.include("libcashaccount/includes/");
    compiler.include("libcashaccount/src/");

    let source_files = vec!["libcashaccount/src/cashaccount.cpp"];

    for f in source_files {
        compiler.file(f);
    }
    compiler.cpp(true);
    compiler.compile("libcashaccount.a");
}

fn main() {
    println!("cargo:rustc-link-lib=static=cashaccount");
    //println!("cargo:rustc-link-lib=cashaccount");
    println!("cargo:rustc-link-search=native=libcashaccount/src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libcashaccount/");

    fail_on_empty_directory("libcashaccount");
    compile_libcashaccount();

    let bindings = bindgen::Builder::default()
        .header("libcashaccount/includes/cashaccount.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
