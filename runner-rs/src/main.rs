use cxx::let_cxx_string;

fn main() {
    println!("Hello world from runner-rs!");
    let something_opaque = middle_rs::SomethingOpaque::new();
    println!("something_opaque.owner() -> '{}'", something_opaque.owner());
    let_cxx_string!(owner = "runner-rs");
    let mut middle = middle_rs::ffi::MiddleCpp_new(&owner, Box::new(something_opaque));
    middle.print();

    middle.pin_mut().change_owner();
    middle.print();
}
