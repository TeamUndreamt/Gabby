use crate::MemoryItem;

#[derive(Debug)]
pub struct CognitionInput {
    pub step: u32,
    pub goals: Vec<String>,
    pub working_memory: Vec<MemoryItem>,
}

#[derive(Debug)]
pub enum CognitionOutput {
    Think(String),
    Remember { key: String, value: String },
    NoOp,
}

pub fn think(input: CognitionInput) -> CognitionOutput {
    if input.step == 1 {
        if let Some(goal) = input.goals.get(0) {
            return CognitionOutput::Think(format!("Starting work on goal: {}", goal));
        }
    }

    if input.step == 2 {
        return CognitionOutput::Remember {
            key: "active_goal".to_string(),
            value: input.goals.get(0).cloned().unwrap_or("unknown".to_string()),
        };
    }

    CognitionOutput::NoOp
}
