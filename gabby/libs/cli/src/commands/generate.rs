use anyhow::{bail, Result};
use std::fs;

pub fn execute(profile: String) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let agents_dir = cwd.join("agents");

    if !agents_dir.exists() {
        bail!("Not a Gabby workspace (missing agents/ directory)");
    }

    let agent_path = agents_dir.join(format!("{}.yaml", profile));

    if agent_path.exists() {
        bail!("Agent already exists: {}", agent_path.display());
    }

    let template = format!(
        r#"id: {id}
version: "0.1"

identity:
  name: {id}
  description: Generated agent

goals:
  - Describe your goals here

limits:
  max_steps: 10
  max_memory_mb: 16

modules:
  reasoning: true
  memory: true
"#,
        id = profile
    );

    fs::write(&agent_path, template)?;

    println!("Created agent: {}", agent_path.display());
    Ok(())
}
