use cxx::let_cxx_string;

fn main() {
    println!("Hello world from runner-rs!");
    let something_opaque = middle_rs::SomethingOpaque::new();
    println!("something_opaque.owner() -> '{}'", something_opaque.owner());
    let_cxx_string!(owner = "runner-rs");
    let middle = middle_rs::ffi::MiddleCpp_new(&owner);
    middle.print();
}
