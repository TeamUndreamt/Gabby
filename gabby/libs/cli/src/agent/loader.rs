use crate::agent::{Agent, AgentConfig};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> Result<Agent> {
    if !path.exists() {
        bail!("Agent file not found: {}", path.display());
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("Failed to read agent file {}", path.display()))?;

    let config: AgentConfig = serde_yaml::from_str(&raw).context("Invalid agent YAML format")?;

    validate_version(&config.version)?;
    validate_limits(&config.limits)?;

    Ok(Agent {
        path: path.to_path_buf(),
        config,
    })
}

fn validate_version(version: &str) -> Result<()> {
    if version != "0.1" {
        bail!("Unsupported agent version: {}", version);
    }
    Ok(())
}

fn validate_limits(limits: &crate::agent::Limits) -> Result<()> {
    if limits.max_steps == 0 {
        bail!("max_steps must be > 0");
    }
    if limits.max_memory_mb == 0 {
        bail!("max_memory_mb must be > 0");
    }
    Ok(())
}
