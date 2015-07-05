//use std::env;
//use std::fs;
//use std::path::Path;
//use std::process::Command;

fn main() {
//    let dst = env::var("OUT_DIR").unwrap();

  //  assert!(Command::new("scons").current_dir("mongo-c-driver").status().unwrap().success());

    {
//        let src = Path::new("/usr/lib/libcassandra_static.a");
//        let dst = Path::new(&dst).join("libmongoc.a");
//        if fs::rename(&src, &dst).is_err() {
//            fs::copy(&src, &dst).unwrap();
//            fs::remove_file(&src).unwrap();
//        }
    }
    
  //  gcc::compile_library("libhello.a", &Default::default(), &["cpp/hello.cpp"]);
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=uv");    
    println!("cargo:rustc-link-search={}", "/usr/lib/");
    println!("cargo:rustc-link-lib=static=cassandra_static");
}