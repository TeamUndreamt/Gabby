pub mod cognition;

mod observe;

use crate::cognition::{think, CognitionInput, CognitionOutput};
use std::path::PathBuf;
use uuid::Uuid;
use observe::{RunEvent, RunLog};

#[derive(Debug)]
pub struct KernelInput {
    pub agent_id: String,
    pub workspace_root: PathBuf,
    pub max_steps: u32,
    pub max_memory_mb: u32,
    pub goals: Vec<String>,
}

#[derive(Debug)]
pub struct KernelResult {
    pub run_id: String,
    pub status: RunStatus,
    pub steps_executed: u32,
    pub log: RunLog,
}

#[derive(Debug)]
pub enum RunStatus {
    Created,
    Initialized,
    Running,
    Completed,
    Failed(String),
    Terminated(String),
}

#[derive(Debug)]
struct RunState {
    run_id: String,
    status: RunStatus,
    step_count: u32,
}

pub fn run(input: KernelInput) -> KernelResult {
    // 1. Create run
    let mut state = RunState {
        run_id: Uuid::new_v4().to_string(),
        status: RunStatus::Created,
        step_count: 0,
    };

    let mut events: Vec<RunEvent> = Vec::new();

    // 2. Initialize
    state.status = RunStatus::Initialized;
    let mut wm = WorkingMemory::new(input.max_memory_mb);

    // 3. Enter running loop
    state.status = RunStatus::Running;

    while state.step_count < input.max_steps {
        state.step_count += 1;

        let cognition_input = CognitionInput {
            step: state.step_count,
            goals: input.goals.clone(),
            working_memory: wm.items.clone(),
        };

        match think(cognition_input) {
            CognitionOutput::Think(msg) => {
                events.push(RunEvent {
                    step: state.step_count,
                    kind: "think".to_string(),
                    detail: msg,
                });
            }

            CognitionOutput::Remember { key, value } => {
                events.push(RunEvent {
                    step: state.step_count,
                    kind: "memory_write".to_string(),
                    detail: key.clone(),
                });

                if let Err(err) = wm.write(MemoryItem { key, value }) {
                    state.status = RunStatus::Terminated(err);
                    break;
                }
            }

            CognitionOutput::NoOp => {}
        }
    }

    // 4. Enforce max steps only if still running
    if matches!(state.status, RunStatus::Running) {
        state.status = RunStatus::Terminated(format!(
            "Max steps ({}) reached",
            input.max_steps
        ));
    }

    // 5. Build run log
    let final_status = match &state.status {
        RunStatus::Terminated(msg) => msg.clone(),
        other => format!("{:?}", other),
    };

    let log = RunLog {
        run_id: state.run_id.clone(),
        agent_id: input.agent_id.clone(),
        steps_executed: state.step_count,
        final_status,
        events,
    };

    KernelResult {
        run_id: state.run_id,
        status: state.status,
        steps_executed: state.step_count,
        log,
    }
}

#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub key: String,
    pub value: String,
}

struct WorkingMemory {
    items: Vec<MemoryItem>,
    used_bytes: usize,
    max_bytes: usize,
}

impl WorkingMemory {
    fn new(max_memory_mb: u32) -> Self {
        Self {
            items: Vec::new(),
            used_bytes: 0,
            max_bytes: (max_memory_mb as usize) * 1024 * 1024,
        }
    }

    fn write(&mut self, item: MemoryItem) -> Result<(), String> {
        let size = item.key.len() + item.value.len();

        if self.used_bytes + size > self.max_bytes {
            return Err("Working memory limit exceeded".to_string());
        }

        self.used_bytes += size;
        self.items.push(item);
        Ok(())
    }

    fn read(&self, key: &str) -> Option<&MemoryItem> {
        self.items.iter().rev().find(|i| i.key == key)
    }
}
