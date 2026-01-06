use crate::{agent, workspace};
use anyhow::Result;

pub fn execute(target: crate::args::InspectTarget) -> Result<()> {
    match target {
        crate::args::InspectTarget::Agent { path } => {
            let cwd = std::env::current_dir()?;
            let _ws = workspace::load(&cwd)?;

            let agent_path = std::path::Path::new(&path);
            let agent = agent::load(agent_path)?;

            println!("Agent ID: {}", agent.config.id);
            println!("Name: {}", agent.config.identity.name);
            println!("Description: {}", agent.config.identity.description);
            println!("Goals:");
            for g in &agent.config.goals {
                println!("  - {}", g);
            }
        }
        crate::args::InspectTarget::Memory { path } => {
            println!("Inspecting memory at {}", path);
        }
    }
    Ok(())
}
