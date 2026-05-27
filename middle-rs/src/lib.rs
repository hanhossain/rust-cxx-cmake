#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type SomethingOpaque;
        fn set_owner(self: &mut SomethingOpaque, owner: &str);
        fn print(self: &SomethingOpaque);
    }

    unsafe extern "C++" {
        include!("middle-cpp/MiddleCpp.h");

        type MiddleCpp;
        fn MiddleCpp_new(
            owner: &CxxString,
            something_opaque: Box<SomethingOpaque>,
        ) -> UniquePtr<MiddleCpp>;
        fn print(self: &MiddleCpp) -> &CxxString;
        fn change_owner(self: Pin<&mut MiddleCpp>);
    }
}

pub struct SomethingOpaque {
    owner: String,
}

impl SomethingOpaque {
    pub fn new() -> Self {
        Self {
            owner: "rust".into(),
        }
    }

    pub fn owner(&self) -> String {
        println!(
            "[middle-rs::SomethingOpaque::owner] owner is '{}'",
            self.owner
        );
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: &str) {
        println!("[middle-rs::SomethingOpaque::set_owner] setting owner to '{owner}'");
        self.owner = owner.into();
    }

    fn print(&self) {
        println!("[middle-rs::SomethingOpaque::print] owner: {}", self.owner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx::let_cxx_string;

    #[test]
    fn test_something_opaque_owner() {
        let s = SomethingOpaque::new();
        let owner = s.owner();
        assert_eq!(owner, "rust");
    }

    #[test]
    fn test_middle_cpp_print() {
        let opaque = Box::new(SomethingOpaque::new());
        let_cxx_string!(owner = "middle-rs");
        let middle_cpp = ffi::MiddleCpp_new(&owner, opaque);
        let ret = middle_cpp.print();
        assert_eq!(ret, "middle-rs")
    }
}
