use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn inspect_valid_agent() {
    let dir = tempdir().unwrap();

    Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(dir.path())
        .arg("init")
        .output()
        .unwrap();

    let agent_yaml = r#"
id : test_agent
version : "0.1"

identity :
  name : Test Agent
  description : Minimal test agent

goals :
  - Test correctness

limits :
  max_steps : 10
  max_memory_mb : 8

modules :
  reasoning : true
  memory : true
"#;

    let agent_path = dir.path().join("agents/test.yaml");
    fs::write(&agent_path, agent_yaml).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(dir.path())
        .args(["inspect", "agent", "agents/test.yaml"])
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn reject_invalid_agent() {
    let dir = tempdir().unwrap();

    Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(dir.path())
        .arg("init")
        .output()
        .unwrap();

    let bad_agent = r#"
id : bad_agent
version : "0.1"
"#;

    let agent_path = dir.path().join("agents/bad.yaml");
    fs::write(&agent_path, bad_agent).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_gabby"))
        .current_dir(dir.path())
        .args(["inspect", "agent", "agents/bad.yaml"])
        .output()
        .unwrap();

    assert!(!output.status.success());
}
