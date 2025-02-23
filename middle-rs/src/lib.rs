pub use common_rs::ffi::Language;

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
    }
}

fn middle_new(caller: String, language: Language) -> Box<Middle> {
    Box::new(Middle::new(caller, language))
}
