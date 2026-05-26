use std::process::Command;

#[test]
fn test_runner_rs() {
    let runner = env!("CARGO_BIN_EXE_runner-rs");
    let output = Command::new(runner).output().unwrap();
    let actual = String::from_utf8_lossy(&output.stdout);
    let expected = "\
Hello world from runner-rs!
[middle-rs::SomethingOpaque::owner] owner is 'rust'
something_opaque.owner() -> 'rust'
";
    assert_eq!(actual, expected);
}
