/// Deterministic Rollback Netcode Engine
pub struct RollbackEngine {
    current_frame: u32,
    state_history: Vec<Vec<u8>>,
}

impl RollbackEngine {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            state_history: Vec::with_capacity(60),
        }
    }

    pub fn advance_frame(&mut self, state_snapshot: Vec<u8>) {
        self.current_frame += 1;
        if self.state_history.len() >= 60 {
            self.state_history.remove(0);
        }
        self.state_history.push(state_snapshot);
    }

    pub fn rollback_to(&mut self, frame: u32) -> Result<Vec<u8>, String> {
        let diff = self.current_frame.saturating_sub(frame) as usize;
        if diff >= self.state_history.len() {
            return Err("Frame out of rollback window".to_string());
        }
        let state = self.state_history[self.state_history.len() - 1 - diff].clone();
        Ok(state)
    }
}
