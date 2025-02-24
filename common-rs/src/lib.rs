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
        type ComplexContext;

        fn increment(self: &mut LanguageContext);

        #[cxx_name = "ComplexContext_new"]
        fn complex_context_new() -> Box<ComplexContext>;
        fn add_language(self: &mut ComplexContext, language: Language);
        fn print_languages(self: &ComplexContext);
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

pub struct ComplexContext {
    languages: Vec<ffi::Language>,
}

impl ComplexContext {
    pub fn new() -> ComplexContext {
        ComplexContext {
            languages: Vec::new(),
        }
    }

    pub fn add_language(&mut self, language: ffi::Language) {
        self.languages.push(language);
    }

    pub fn print_languages(&self) {
        println!(
            "common_rs::ComplexContext::print_languages - languages={:?}",
            self.languages
        );
    }
}

fn complex_context_new() -> Box<ComplexContext> {
    Box::new(ComplexContext::new())
}
