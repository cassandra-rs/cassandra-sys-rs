//use std::env;
//use std::fs;
//use std::path::Path;
//use std::process::Command;

fn main() {
    println!("cargo:rustc-flags=-l dylib=crypto");
    println!("cargo:rustc-flags=-l dylib=ssl");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=uv");
    println!("cargo:rustc-link-search={}", "/usr/lib/");
    println!("cargo:rustc-link-search={}", "/usr/local/lib64");
    println!("cargo:rustc-link-lib=static=cassandra_static");
}
