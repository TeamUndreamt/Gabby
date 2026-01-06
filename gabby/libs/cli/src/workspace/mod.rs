mod config;
mod loader;

pub use config::*;
pub use loader::*;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Workspace {
    pub root: PathBuf,
    pub agents_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub logs_dir: PathBuf,
}
