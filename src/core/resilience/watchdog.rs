use sqlx::{Pool, Sqlite};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

/// System health watchdog and connection auto-healer for Vella
#[derive(Debug, Clone)]
pub struct SystemWatchdog {
    start_time: Instant,
    is_healthy: Arc<AtomicBool>,
    auto_heal_count: Arc<AtomicU64>,
    check_interval: Duration,
}

impl Default for SystemWatchdog {
    fn default() -> Self {
        Self::new(Duration::from_secs(5))
    }
}

impl SystemWatchdog {
    pub fn new(check_interval: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            is_healthy: Arc::new(AtomicBool::new(true)),
            auto_heal_count: Arc::new(AtomicU64::new(0)),
            check_interval,
        }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    pub fn is_healthy(&self) -> bool {
        self.is_healthy.load(Ordering::Relaxed)
    }

    pub fn auto_heal_count(&self) -> u64 {
        self.auto_heal_count.load(Ordering::Relaxed)
    }

    /// Spawn background self-healing watchdog task
    pub fn start(&self, pool: Pool<Sqlite>) {
        let is_healthy = self.is_healthy.clone();
        let auto_heal_count = self.auto_heal_count.clone();
        let interval = self.check_interval;

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;

                // Ping database connection
                match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => {
                        if !is_healthy.load(Ordering::Relaxed) {
                            info!("💚 [Vella Watchdog] Database connection successfully recovered and healthy");
                            is_healthy.store(true, Ordering::Relaxed);
                            auto_heal_count.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    Err(err) => {
                        warn!("⚠️ [Vella Watchdog] Database ping failed: {}. Initiating connection recovery...", err);
                        is_healthy.store(false, Ordering::Relaxed);

                        // Retry with exponential backoff probe
                        for retry in 1..=3 {
                            tokio::time::sleep(Duration::from_millis(100 * (1 << retry))).await;
                            if let Ok(_) = sqlx::query("SELECT 1").execute(&pool).await {
                                info!("✨ [Vella Watchdog] Auto-healed database connection on retry #{}", retry);
                                is_healthy.store(true, Ordering::Relaxed);
                                auto_heal_count.fetch_add(1, Ordering::Relaxed);
                                break;
                            }
                        }

                        if !is_healthy.load(Ordering::Relaxed) {
                            error!("❌ [Vella Watchdog] Critical: Database remains unresponsive after retry attempts");
                        }
                    }
                }
            }
        });
    }

    pub fn status_json(&self) -> serde_json::Value {
        serde_json::json!({
            "status": if self.is_healthy() { "HEALTHY" } else { "DEGRADED" },
            "uptime_seconds": self.uptime_secs(),
            "auto_healing_events": self.auto_heal_count(),
            "watchdog_active": true
        })
    }
}
