#[cxx::bridge(namespace = "common_rs")]
pub mod ffi {
    #[derive(Debug, Clone, Copy)]
    enum Language {
        Rust,
        Cpp,
    }
}
