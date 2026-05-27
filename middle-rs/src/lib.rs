#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("middle-cpp/MiddleCpp.h");

        type MiddleCpp;
        fn MiddleCpp_new(owner: &CxxString) -> UniquePtr<MiddleCpp>;
        fn print(self: &MiddleCpp) -> &CxxString;
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
        let_cxx_string!(owner = "middle-rs");
        let middle_cpp = ffi::MiddleCpp_new(&owner);
        let ret = middle_cpp.print();
        assert_eq!(ret, "middle-rs")
    }
}
