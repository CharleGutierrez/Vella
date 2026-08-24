use tracing::{info, warn};

#[derive(Debug, PartialEq, Eq)]
pub enum AlarmState {
    Normal,
    UnackActive,    // UNACKNOWLEDGED, ACTIVE
    AckActive,      // ACKNOWLEDGED, ACTIVE
    UnackCleared,   // UNACKNOWLEDGED, CLEARED
    Shelved,        // Intentionally muted for maintenance
}

pub struct Isa18Alarm {
    pub tag: String,
    pub state: AlarmState,
}

impl Isa18Alarm {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            state: AlarmState::Normal,
        }
    }

    /// State Machine Transition: A physical sensor breaches a threshold
    pub fn trigger_breach(&mut self) {
        if self.state == AlarmState::Shelved {
            info!("Alarm {}: Breach detected, but alarm is Shelved. Ignoring.", self.tag);
            return;
        }
        
        warn!("🚨 ALARM ACTIVE [{}]: Entering UNACKNOWLEDGED_ACTIVE state.", self.tag);
        self.state = AlarmState::UnackActive;
    }

    /// State Machine Transition: Control Room Operator clicks "Acknowledge"
    pub fn operator_acknowledge(&mut self) {
        if self.state == AlarmState::UnackActive {
            info!("Operator Acknowledged Alarm [{}]. State -> ACKNOWLEDGED_ACTIVE", self.tag);
            self.state = AlarmState::AckActive;
        } else if self.state == AlarmState::UnackCleared {
            info!("Operator Acknowledged cleared Alarm [{}]. State -> NORMAL", self.tag);
            self.state = AlarmState::Normal;
        }
    }

    /// State Machine Transition: Physical sensor returns to safe levels
    pub fn trigger_clear(&mut self) {
        if self.state == AlarmState::AckActive {
            info!("Alarm [{}] physical values restored. State -> NORMAL", self.tag);
            self.state = AlarmState::Normal;
        } else if self.state == AlarmState::UnackActive {
            info!("Alarm [{}] physical values restored before acknowledgement. State -> UNACKNOWLEDGED_CLEARED", self.tag);
            self.state = AlarmState::UnackCleared;
        }
    }
}
