fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("../middle-cpp/include")
        .compile("middle-rs-binding");
    let dst = cmake::Config::new("..").build();
    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-lib=middle-cpp");
    println!("cargo::rerun-if-changed=../middle-cpp/");
    println!("cargo::rerun-if-changed=../CMakeLists.txt");
}
