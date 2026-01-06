use std::process::Command;

#[test]
fn gabby_help_works() {
    let output = Command::new(env!("CARGO_BIN_EXE_gabby"))
        .arg("--help")
        .output()
        .expect("failed to run gabby");

    assert!(output.status.success(), "gabby --help failed: {:?}", output);
}
