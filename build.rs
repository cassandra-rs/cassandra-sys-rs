//#![feature(plugin)]
//#![plugin(bindgen_plugin)]



//#[allow(dead_code, uppercase_variables, non_camel_case_types)]
//#[plugin(bindgen_plugin)]    
//mod mysql_bindings {
//    bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql");
//}

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
