fn main() {
    cxx_build::CFG.exported_header_links.push("common-rs");
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .compile("middle_rs");
}
