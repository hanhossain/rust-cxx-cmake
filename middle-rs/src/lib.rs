use common_rs::ComplexContext;
pub use common_rs::ffi::{Language, LanguageContext};

#[cxx::bridge(namespace = "middle_rs")]
mod ffi {
    extern "Rust" {
        type Middle;
        fn run(&self);

        #[cxx_name = "Middle_new"]
        fn middle_new(caller: String, language: Language) -> Box<Middle>;
    }

    extern "C++" {
        include!("common-rs/src/lib.rs.h");

        #[namespace = "common_rs"]
        type Language = common_rs::ffi::Language;
    }
}

pub struct Middle {
    caller: String,
    language: Language,
}

impl Middle {
    pub fn new(caller: String, language: Language) -> Middle {
        Middle { caller, language }
    }

    pub fn run(&self) {
        println!(
            "[{}] middle_rs::Middle::run({:?})",
            self.caller, self.language
        );
        let mut language_context = LanguageContext {
            language: Language::Rust,
            value: 10,
        };
        language_context.increment();

        let mut complex_context = ComplexContext::new();
        complex_context.add_language(Language::Rust);
        complex_context.print_languages();
    }
}

fn middle_new(caller: String, language: Language) -> Box<Middle> {
    Box::new(Middle::new(caller, language))
}
