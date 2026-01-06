use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub version: String,

    pub identity: Identity,
    pub goals: Vec<String>,
    pub limits: Limits,
    pub modules: Modules,
}

#[derive(Debug, Deserialize)]
pub struct Identity {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Limits {
    pub max_steps: u32,
    pub max_memory_mb: u32,
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    pub reasoning: bool,
    pub memory: bool,
}
