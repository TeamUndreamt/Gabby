use std::fs;

use crate::{agent, workspace};
use anyhow::{Context, Result};
use gabby_kernel::{run, KernelInput};

pub fn execute(agent_path: String) -> Result<()> {
    let cwd = std::env::current_dir()?;

    let ws = workspace::load(&cwd)?;
    let ag = agent::load(std::path::Path::new(&agent_path))?;

    let input = KernelInput {
        agent_id: ag.config.id.clone(),
        workspace_root: ws.root.clone(),
        max_steps: ag.config.limits.max_steps,
        max_memory_mb: ag.config.limits.max_memory_mb,
        goals: ag.config.goals.clone(),
    };

    let result = run(input);

    // --- Persist run log ---
    let log_path = ws
        .logs_dir
        .join(format!("run_{}.json", result.run_id));

    let json = serde_json::to_string_pretty(&result.log)
        .context("Failed to serialize run log")?;

    fs::write(&log_path, json)
        .with_context(|| format!("Failed to write log {}", log_path.display()))?;

    // --- User-facing summary ---
    println!("Run ID: {}", result.run_id);
    println!("Steps executed: {}", result.steps_executed);
    println!("Final status: {:?}", result.status);
    println!("Log written to {}", log_path.display());

    Ok(())
}
