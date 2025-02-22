pub struct Middle {
    caller: String,
}

impl Middle {
    pub fn new(caller: String) -> Middle {
        Middle { caller }
    }

    pub fn run(&self) {
        println!("[{}] middle_rs::Middle::run()", self.caller);
    }
}
