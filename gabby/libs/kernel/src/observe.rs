use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RunEvent {
    pub step: u32,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct RunLog {
    pub run_id: String,
    pub agent_id: String,
    pub steps_executed: u32,
    pub final_status: String,
    pub events: Vec<RunEvent>,
}
