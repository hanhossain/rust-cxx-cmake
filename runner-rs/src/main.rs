fn main() {
    println!("Hello world from runner-rs!");
    let something_opaque = middle_rs::SomethingOpaque::new();
    println!("something_opaque.owner() -> '{}'", something_opaque.owner());
}
