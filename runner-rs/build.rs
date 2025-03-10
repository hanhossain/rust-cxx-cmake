fn main() {
    let mut config = cxx_build::bridge("src/main.rs");
    config
        .std("c++17")
        .include("../middle-cpp")
        .compile("bindings");

    let dst = cmake::Config::new("..")
        .init_cxx_cfg(config.clone())
        .generator("Ninja")
        .build_target("middle-cpp")
        .build();
    println!(
        "cargo::rustc-link-search=native={}/build/middle-cpp",
        dst.display()
    );
    println!("cargo::rustc-link-lib=static=middle-cpp");

    println!("cargo::rerun-if-changed=src/main.rs");
    println!("cargo::rerun-if-changed=../middle-cpp");
    println!("cargo::rerun-if-changed=../CMakeLists.txt");
}
