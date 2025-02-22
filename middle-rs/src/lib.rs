pub use common_rs::Language;

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
