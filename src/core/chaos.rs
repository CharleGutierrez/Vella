use tracing::{info, warn};
use rand::Rng;

pub struct ChaosMonkeyMiddleware {
    pub fault_probability: f64, // 0.0 to 1.0
    pub max_latency_ms: u64,
}

impl ChaosMonkeyMiddleware {
    pub fn new(fault_probability: f64, max_latency_ms: u64) -> Self {
        warn!("⚠️ INITIALIZING CHAOS MONKEY: Fault Probability: {}%, Max Latency: {}ms", fault_probability * 100.0, max_latency_ms);
        Self {
            fault_probability,
            max_latency_ms,
        }
    }

    /// Evaluates if the current request should be disrupted (dropped or delayed)
    pub async fn inject_chaos(&self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen();

        if roll < self.fault_probability {
            let severity: u32 = rng.gen_range(1..=10);
            if severity <= 3 {
                warn!("Chaos Monkey: Dropping request (Simulated HTTP 503 / Network Partition)");
                return Err("Simulated Network Partition");
            } else {
                let delay = rng.gen_range(10..=self.max_latency_ms);
                warn!("Chaos Monkey: Injecting latency spike of {}ms", delay);
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }
        }
        
        Ok(())
    }
}
