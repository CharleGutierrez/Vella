use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Circuit Breaker states for self-healing failure isolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakerState {
    Closed,   // Normal operation
    Open,     // Failure threshold reached, fail fast
    HalfOpen, // Testing if downstream service has self-healed
}

/// A thread-safe, lock-free Circuit Breaker for automatic self-healing in Vella
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    name: String,
    failure_threshold: u32,
    cooldown_duration: Duration,
    consecutive_failures: Arc<AtomicU32>,
    last_state_change: Arc<RwLock<Instant>>,
    state: Arc<RwLock<BreakerState>>,
    total_trips: Arc<AtomicU64>,
    total_heals: Arc<AtomicU64>,
}

impl CircuitBreaker {
    pub fn new(name: impl Into<String>, failure_threshold: u32, cooldown_secs: u64) -> Self {
        Self {
            name: name.into(),
            failure_threshold,
            cooldown_duration: Duration::from_secs(cooldown_secs),
            consecutive_failures: Arc::new(AtomicU32::new(0)),
            last_state_change: Arc::new(RwLock::new(Instant::now())),
            state: Arc::new(RwLock::new(BreakerState::Closed)),
            total_trips: Arc::new(AtomicU64::new(0)),
            total_heals: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Check if execution is allowed or if circuit is open
    pub fn allow_execution(&self) -> bool {
        let current_state = *self.state.read().unwrap();
        match current_state {
            BreakerState::Closed => true,
            BreakerState::HalfOpen => true,
            BreakerState::Open => {
                let elapsed = self.last_state_change.read().unwrap().elapsed();
                if elapsed >= self.cooldown_duration {
                    // Transition to HalfOpen to probe if system has self-healed
                    let mut s = self.state.write().unwrap();
                    *s = BreakerState::HalfOpen;
                    *self.last_state_change.write().unwrap() = Instant::now();
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Record a successful operation (resets failures and completes self-healing)
    pub fn record_success(&self) {
        let mut s = self.state.write().unwrap();
        if *s == BreakerState::HalfOpen {
            *s = BreakerState::Closed;
            self.total_heals.fetch_add(1, Ordering::Relaxed);
        }
        self.consecutive_failures.store(0, Ordering::Relaxed);
    }

    /// Record a failed operation (triggers breaker if threshold exceeded)
    pub fn record_failure(&self) {
        let failures = self.consecutive_failures.fetch_add(1, Ordering::Relaxed) + 1;
        if failures >= self.failure_threshold {
            let mut s = self.state.write().unwrap();
            if *s != BreakerState::Open {
                *s = BreakerState::Open;
                *self.last_state_change.write().unwrap() = Instant::now();
                self.total_trips.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn state(&self) -> BreakerState {
        *self.state.read().unwrap()
    }

    pub fn status_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "state": match self.state() {
                BreakerState::Closed => "CLOSED (Healthy)",
                BreakerState::Open => "OPEN (Isolated)",
                BreakerState::HalfOpen => "HALF_OPEN (Probing Recovery)",
            },
            "consecutive_failures": self.consecutive_failures.load(Ordering::Relaxed),
            "total_trips": self.total_trips.load(Ordering::Relaxed),
            "total_heals": self.total_heals.load(Ordering::Relaxed)
        })
    }
}
