fn main() {
    // cxx_build::bridge("src/main.rs")
    //     .file("src/test.cc")
    //     .include("include") 
    //     .std("c++14")
    //     .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/test.cc");
    println!("cargo:rerun-if-changed=include/test.h");
}
