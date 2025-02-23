fn main() {
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .compile("common_rs");
}
