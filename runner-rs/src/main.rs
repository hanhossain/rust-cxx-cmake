fn main() {
    println!("Hello world from runner-rs!");
    let middle = middle_rs::Middle::new("runner-rs".to_string());
    middle.run();
}
