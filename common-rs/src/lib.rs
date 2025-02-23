#[cxx::bridge(namespace = "common_rs")]
pub mod ffi {
    #[derive(Debug, Clone, Copy)]
    enum Language {
        Rust,
        Cpp,
    }

    #[derive(Debug, Copy, Clone)]
    struct LanguageContext {
        language: Language,
        value: i32,
    }

    extern "Rust" {
        fn increment(self: &mut LanguageContext);
    }
}

impl ffi::LanguageContext {
    pub fn increment(&mut self) {
        println!(
            "common_rs::LanguageContext::increment - original=[{}], language=[{:?}]",
            self.value, self.language
        );
        self.value += 1;
    }
}
