use cxx::let_cxx_string;
use ffi::middle_cpp_new;
use middle_rs::Language;

#[cxx::bridge]
mod ffi {
    #[namespace = "middle_cpp"]
    unsafe extern "C++" {
        include!("middle_cpp.h");

        type MiddleCpp;

        #[cxx_name = "MiddleCpp_new"]
        fn middle_cpp_new(caller: &CxxString) -> UniquePtr<MiddleCpp>;
        fn print(&self);
    }
}

fn main() {
    println!("Hello world from runner-rs!");
    let middle = middle_rs::Middle::new("runner-rs".to_string(), Language::Rust);
    middle.run();

    let_cxx_string!(caller = "runner-rs");
    let middle_cpp = middle_cpp_new(&caller);
    middle_cpp.print();
}
