mod loader;
mod schema;

pub use loader::*;
pub use schema::*;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Agent {
    pub path: PathBuf,
    pub config: AgentConfig,
}
