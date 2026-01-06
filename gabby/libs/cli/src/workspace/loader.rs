use crate::workspace::{Workspace, WorkspaceConfig};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn load(start: &Path) -> Result<Workspace> {
    let root = find_workspace_root(start)?;
    let config_path = root.join("gabby.yaml");

    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;

    let config: WorkspaceConfig =
        serde_yaml::from_str(&raw).context("Invalid gabby.yaml format")?;

    validate_version(&config.version)?;

    Ok(Workspace {
        root: root.clone(),
        agents_dir: root.join(&config.workspace.agents_dir),
        memory_dir: root.join(&config.workspace.memory_dir),
        logs_dir: root.join(&config.workspace.logs_dir),
    })
}

fn find_workspace_root(start: &Path) -> Result<PathBuf> {
    let mut current = Some(start);

    while let Some(dir) = current {
        if dir.join("gabby.yaml").exists() {
            return Ok(dir.to_path_buf());
        }
        current = dir.parent();
    }

    bail!("Not inside a Gabby workspace (gabby.yaml not found)")
}

fn validate_version(version: &str) -> Result<()> {
    if version != "0.1" {
        bail!("Unsupported workspace version: {}", version);
    }
    Ok(())
}
