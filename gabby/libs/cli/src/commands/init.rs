use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

fn is_gabby_source_repo(dir: &Path) -> bool {
    dir.join("Cargo.toml").exists()
        && dir.join("libs/cli").exists()
        && dir.join("libs/kernel").exists()
}

pub fn execute(force: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    
    if is_gabby_source_repo(&cwd) {
        bail!("gabby init cannot be run inside the Gabby source repository");
    }
    
    let config_path = cwd.join("gabby.yaml");

    // Guard: existing workspace
    if config_path.exists() && !force {
        bail!(
            "Gabby workspace already exists here. Use `gabby init --force` to overwrite."
        );
    }

    // Guard: nested workspace
    if !force {
        let mut current = cwd.parent();
        while let Some(dir) = current {
            if dir.join("gabby.yaml").exists() {
                bail!("Cannot initialize a Gabby workspace inside another workspace");
            }
            current = dir.parent();
        }
    }

    // Safe to proceed
    fs::create_dir_all(cwd.join("agents"))?;
    fs::create_dir_all(cwd.join("logs"))?;
    fs::create_dir_all(cwd.join("memory"))?;

    let config = r#"version: "0.1"
workspace:
  agents_dir: agents
  logs_dir: logs
  memory_dir: memory
"#;

    fs::write(config_path, config)?;

    println!("Initialized Gabby workspace at {}", cwd.display());
    Ok(())
}
