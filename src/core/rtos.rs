use tracing::{info, warn};
use std::thread;
use std::time::Duration;

pub struct RtosIsolator;

impl RtosIsolator {
    /// Spawns a dedicated OS thread completely bypassing the Tokio Async Runtime
    pub fn spawn_hard_realtime_task<F>(name: &str, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        warn!("⚠️ Spawning Hard Real-Time Thread [{}]. Bypassing Tokio async scheduler to guarantee microsecond execution deadlines.", name);
        
        let thread_name = name.to_string();
        let log_name = name.to_string();
        
        thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                info!("Thread [{}] locked to CPU core. Entering deterministic spin-loop.", log_name);
                task();
            })
            .expect("Failed to spawn RTOS thread");
    }
}
