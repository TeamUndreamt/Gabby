use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gabby")]
#[command(about = "Gabby AGI CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(long)]
        force: bool,
    },

    Run {
        #[arg(long)]
        agent: String,
    },

    Generate {
        #[command(subcommand)]
        target: GenerateTarget,
    },

    Inspect {
        #[command(subcommand)]
        target: InspectTarget,
    },

    Version,
}

#[derive(Subcommand)]
pub enum GenerateTarget {
    Agent {
        #[arg(long)]
        profile: String,
    },
}

#[derive(Subcommand)]
pub enum InspectTarget {
    ///Inspect an agent definition
    Agent { path: String },

    ///Inspect memory contents
    Memory { path: String },
}
