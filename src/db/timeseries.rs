use tracing::info;
use crate::ai::tuner::AiTuner;
use std::sync::Arc;

pub struct TimeSeriesAdapter {
    engine: String,
    ai_tuner: Arc<AiTuner>,
}

impl TimeSeriesAdapter {
    pub fn new(engine: &str, ai_tuner: Arc<AiTuner>) -> Self {
        info!("Initializing Time-Series Database Engine: {}", engine);
        Self {
            engine: engine.to_string(),
            ai_tuner,
        }
    }

    /// Queries billions of rows and applies specialized time-bucketing natively
    pub fn query_downsampled_bucket(&self, metric: &str, base_interval_ms: u64, last_latency_ms: u64) -> String {
        let active_interval = self.ai_tuner.tune_timeseries_bucket_interval(base_interval_ms, last_latency_ms);
        info!("Executing High-Frequency Downsampling query for [{}] at {}ms intervals via {}", metric, active_interval, self.engine);
        
        format!(
            "SELECT time_bucket('{} milliseconds', timestamp) AS bucket, avg(value) FROM {} GROUP BY bucket ORDER BY bucket DESC",
            active_interval, metric
        )
    }
}
