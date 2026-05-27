fn main() {
    let mut cxx_bridge = cxx_build::bridge("src/lib.rs");
    cxx_bridge
        .include("../middle-cpp/include")
        .compile("middle-rs-binding");
    let dst = cmake::Config::new("..")
        .init_cxx_cfg(cxx_bridge)
        .define("CALLED_FROM_RUST", "ON")
        .build();
    println!("cargo::rustc-link-search=native={}/lib", dst.display());
    println!("cargo::rustc-link-lib=middle-cpp");
    println!("cargo::rerun-if-changed=../middle-cpp/");
    println!("cargo::rerun-if-changed=../CMakeLists.txt");
}
