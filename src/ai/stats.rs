use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowQueryLog {
    pub model: String,
    pub query_snippet: String,
    pub duration_ms: f64,
    pub timestamp: String,
}

/// Real-time workload & latency performance telemetry for Vella AI Tuner
#[derive(Debug, Clone)]
pub struct WorkloadStats {
    start_time: Instant,
    total_queries: Arc<AtomicU64>,
    total_duration_ms: Arc<AtomicU64>,
    latencies: Arc<RwLock<VecDeque<f64>>>,
    slow_queries: Arc<RwLock<VecDeque<SlowQueryLog>>>,
}

impl Default for WorkloadStats {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkloadStats {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_queries: Arc::new(AtomicU64::new(0)),
            total_duration_ms: Arc::new(AtomicU64::new(0)),
            latencies: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            slow_queries: Arc::new(RwLock::new(VecDeque::with_capacity(50))),
        }
    }

    /// Record a query execution latency
    pub fn record_query(&self, model: &str, query_snippet: &str, duration_ms: f64) {
        self.total_queries.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ms.fetch_add(duration_ms.round() as u64, Ordering::Relaxed);

        // Record rolling latency buffer
        {
            let mut lats = self.latencies.write().unwrap();
            if lats.len() >= 1000 {
                lats.pop_front();
            }
            lats.push_back(duration_ms);
        }

        // Record slow queries (> 15ms)
        if duration_ms >= 15.0 {
            let mut slow = self.slow_queries.write().unwrap();
            if slow.len() >= 50 {
                slow.pop_front();
            }
            slow.push_back(SlowQueryLog {
                model: model.to_string(),
                query_snippet: query_snippet.to_string(),
                duration_ms,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }
    }

    /// Calculate percentile latencies (p50, p95, p99)
    pub fn percentiles(&self) -> (f64, f64, f64) {
        let lats = self.latencies.read().unwrap();
        if lats.is_empty() {
            return (0.1, 0.2, 0.5);
        }

        let mut sorted: Vec<f64> = lats.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = sorted.len();
        let p50 = sorted[(len as f64 * 0.50) as usize];
        let p95 = sorted[((len as f64 * 0.95) as usize).min(len - 1)];
        let p99 = sorted[((len as f64 * 0.99) as usize).min(len - 1)];

        (p50, p95, p99)
    }

    pub fn total_queries(&self) -> u64 {
        self.total_queries.load(Ordering::Relaxed)
    }

    pub fn qps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed < 1.0 {
            self.total_queries() as f64
        } else {
            self.total_queries() as f64 / elapsed
        }
    }

    pub fn slow_queries_list(&self) -> Vec<SlowQueryLog> {
        self.slow_queries.read().unwrap().iter().cloned().collect()
    }
}
