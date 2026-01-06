use std::process::Command;
use tempfile::tempdir;

#[test]
fn init_creates_workspace() {
    let dir = tempdir().expect("failed to create temp dir");

    let output = Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(dir.path())
        .arg("init")
        .output()
        .expect("failed to run gabby init");

    assert!(output.status.success(), "gabby init failed: {:?}", output);

    assert!(dir.path().join("gabby.yaml").exists());
    assert!(dir.path().join("agents").exists());
    assert!(dir.path().join("memory").exists());
    assert!(dir.path().join("logs").exists());
}
