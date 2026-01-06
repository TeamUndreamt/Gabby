mod agent;
mod args;
mod commands;
mod workspace;

use anyhow::Result;
use args::{Cli, Commands, GenerateTarget};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            commands::init::execute(force)?;
        }
        Commands::Run { agent } => {
            commands::run::execute(agent)?;
        }
        Commands::Generate { target } => {
            match target {
                GenerateTarget::Agent { profile } => {
                    commands::generate::execute(profile)?;
                }
            }
        }
        Commands::Inspect { target } => {
            commands::inspect::execute(target)?;
        }
        Commands::Version => {
            commands::version::execute()?;
        }
    }

    Ok(())
}
