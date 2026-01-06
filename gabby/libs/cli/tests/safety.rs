use std::path::PathBuf;
use std::process::Command;

#[test]
fn init_refuses_source_repo() {
    // CARGO_MANIFEST_DIR points to libs/cli
    let cli_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Go up to Gabby/gabby (workspace root)
    let repo_root = cli_dir
        .parent() // libs
        .and_then(|p| p.parent()) // gabby
        .expect("failed to locate repo root");

    let output = Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(repo_root)
        .arg("init")
        .output()
        .expect("failed to run gabby init");

    assert!(
        !output.status.success(),
        "gabby init should fail in source repo"
    );
}
