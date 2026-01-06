use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkspaceConfig {
    pub version: String,
    pub workspace: WorkspacePaths,
}

#[derive(Debug, Deserialize)]
pub struct WorkspacePaths {
    pub agents_dir: String,
    pub memory_dir: String,
    pub logs_dir: String,
}
