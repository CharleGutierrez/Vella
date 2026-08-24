use tracing::info;
use crate::ai::tuner::AiTuner;
use std::sync::Arc;

pub struct SwingingDoorCompressor {
    base_deviation_threshold: f64,
    last_archived_value: Option<f64>,
    ai_tuner: Arc<AiTuner>,
}

impl SwingingDoorCompressor {
    pub fn new(deviation_threshold: f64, ai_tuner: Arc<AiTuner>) -> Self {
        info!("Initializing Swinging Door Trend Compressor (Base Threshold: {})", deviation_threshold);
        Self {
            base_deviation_threshold: deviation_threshold,
            last_archived_value: None,
            ai_tuner,
        }
    }

    /// Takes a high-frequency analog signal. Returns `Some(value)` only if it breaks the compression geometry.
    pub fn process_signal(&mut self, current_value: f64, simulated_disk_usage: f64) -> Option<f64> {
        let active_threshold = self.ai_tuner.tune_compression_deviation(self.base_deviation_threshold, simulated_disk_usage);
        
        match self.last_archived_value {
            None => {
                self.last_archived_value = Some(current_value);
                Some(current_value)
            }
            Some(last_val) => {
                if (current_value - last_val).abs() >= active_threshold {
                    self.last_archived_value = Some(current_value);
                    Some(current_value)
                } else {
                    None
                }
            }
        }
    }
}
