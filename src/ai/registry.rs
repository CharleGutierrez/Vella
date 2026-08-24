use tracing::{info, warn};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ModelRegistry {
    pub active_model_version: String,
    pub shadow_model_version: Option<String>,
    shadow_traffic_routed: AtomicUsize,
}

impl ModelRegistry {
    pub fn new(active: &str, shadow: Option<&str>) -> Self {
        info!("Initializing ML Model Registry (Active: {}, Shadow: {:?})", active, shadow);
        Self {
            active_model_version: active.to_string(),
            shadow_model_version: shadow.map(|s| s.to_string()),
            shadow_traffic_routed: AtomicUsize::new(0),
        }
    }

    /// Primary inference routing.
    pub fn execute_inference(&self, payload: &str) -> String {
        info!("Executing inference on primary model [{}]", self.active_model_version);
        
        // Shadow routing logic: If a shadow model exists, fire-and-forget the payload to it
        if let Some(ref shadow) = self.shadow_model_version {
            let routed = self.shadow_traffic_routed.fetch_add(1, Ordering::Relaxed);
            // Simulate firing to the shadow model on a background tokio thread
            info!("Shadow Routing: Sending payload copy to shadow model [{}] (Total Shadow Requests: {})", shadow, routed + 1);
        }

        format!("Inference Result from {}", self.active_model_version)
    }

    pub fn get_shadow_traffic_count(&self) -> usize {
        self.shadow_traffic_routed.load(Ordering::Relaxed)
    }
}
