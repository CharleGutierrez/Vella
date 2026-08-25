use crate::ai::stats::WorkloadStats;
use crate::core::error::VellaError;
use crate::model::SchemaRegistry;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRecommendation {
    pub id: String,
    pub model: String,
    pub table_name: String,
    pub column: String,
    pub reason: String,
    pub estimated_speedup: String,
    pub ddl: String,
    pub is_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTunerReport {
    pub engine_status: String,
    pub total_queries_analyzed: u64,
    pub qps: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub recommendations: Vec<IndexRecommendation>,
    pub workload_summary: String,
}

#[derive(Debug)]
pub struct AiTuner {
    pub stats: Arc<WorkloadStats>,
    column_access_counts: Arc<RwLock<HashMap<String, u64>>>,
    applied_indexes: Arc<RwLock<Vec<String>>>,
    pub current_system_load: AtomicUsize,
}

impl Default for AiTuner {
    fn default() -> Self {
        Self::new()
    }
}

impl AiTuner {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(WorkloadStats::new()),
            column_access_counts: Arc::new(RwLock::new(HashMap::new())),
            applied_indexes: Arc::new(RwLock::new(Vec::new())),
            current_system_load: AtomicUsize::new(50),
        }
    }

    // --- Original Methods ---

    pub fn record_query_pattern(&self, model: &str, table: &str, filtered_columns: &[&str], duration_ms: f64) {
        let snippet = if filtered_columns.is_empty() {
            format!("SELECT * FROM \"{}\"", table)
        } else {
            format!("SELECT * FROM \"{}\" WHERE ({})", table, filtered_columns.join(", "))
        };

        self.stats.record_query(model, &snippet, duration_ms);

        let mut counts = self.column_access_counts.write().unwrap();
        for col in filtered_columns {
            let key = format!("{}.{}", table, col);
            *counts.entry(key).or_insert(0) += 1;
        }
    }

    pub fn generate_recommendations(&self, registry: &SchemaRegistry) -> Vec<IndexRecommendation> {
        let counts = self.column_access_counts.read().unwrap();
        let applied = self.applied_indexes.read().unwrap();
        let mut recs = Vec::new();

        for schema in registry.all() {
            for field in &schema.fields {
                if field.name == "id" || field.unique {
                    continue;
                }

                let key = format!("{}.{}", schema.table_name, field.name);
                let hit_count = counts.get(&key).copied().unwrap_or(0);

                if hit_count >= 1 || field.searchable || field.filterable {
                    let idx_name = format!("idx_ai_{}_{}", schema.table_name, field.name);
                    let is_applied = applied.contains(&idx_name);

                    recs.push(IndexRecommendation {
                        id: format!("{}_{}", schema.table_name, field.name),
                        model: schema.name.clone(),
                        table_name: schema.table_name.clone(),
                        column: field.name.clone(),
                        reason: format!(
                            "Column '{}.{}' was queried {} times.",
                            schema.table_name, field.name, hit_count
                        ),
                        estimated_speedup: "10x - 50x faster queries".to_string(),
                        ddl: format!(
                            "CREATE INDEX IF NOT EXISTS \"{}\" ON \"{}\" (\"{}\");",
                            idx_name, schema.table_name, field.name
                        ),
                        is_applied,
                    });
                }
            }
        }
        recs
    }

    pub async fn apply_index(&self, pool: &Pool<Sqlite>, table: &str, column: &str) -> Result<String, VellaError> {
        let idx_name = format!("idx_ai_{}_{}", table, column);
        let ddl = format!("CREATE INDEX IF NOT EXISTS \"{}\" ON \"{}\" (\"{}\");", idx_name, table, column);

        sqlx::query(&ddl).execute(pool).await?;

        let mut applied = self.applied_indexes.write().unwrap();
        if !applied.contains(&idx_name) {
            applied.push(idx_name.clone());
        }

        Ok(format!("Successfully created index '{}'", idx_name))
    }

    pub fn generate_report(&self, registry: &SchemaRegistry) -> AiTunerReport {
        let (p50, p95, p99) = self.stats.percentiles();
        let recs = self.generate_recommendations(registry);

        AiTunerReport {
            engine_status: "AI Optimization Active & Telemetry Online".to_string(),
            total_queries_analyzed: self.stats.total_queries(),
            qps: (self.stats.qps() * 100.0).round() / 100.0,
            p50_latency_ms: (p50 * 100.0).round() / 100.0,
            p95_latency_ms: (p95 * 100.0).round() / 100.0,
            p99_latency_ms: (p99 * 100.0).round() / 100.0,
            recommendations: recs,
            workload_summary: "AI Telemetry Online".to_string(),
        }
    }

    // --- New Predictive AI Methods ---

    pub fn predict_optimal_delay(&self, base_cron: &str) -> u64 {
        let load = self.current_system_load.load(Ordering::SeqCst);
        if load > 80 {
            warn!("AI Tuner: System load at {}%. Delaying background job by 15 minutes.", load);
            900
        } else {
            info!("AI Tuner: System load optimal ({}%). Executing on schedule ({}).", load, base_cron);
            0
        }
    }

    pub fn analyze_slow_join(&self, table: &str, related: &str, latency_ms: u64) -> Option<IndexRecommendation> {
        if latency_ms > 50 {
            info!("AI Tuner: Detected slow join between {} and {} ({}ms). Recommending index.", table, related, latency_ms);
            Some(IndexRecommendation {
                id: format!("auto_{}_{}", table, related),
                model: table.to_string(),
                table_name: table.to_string(),
                column: format!("{}_id", related),
                reason: "Auto AI Relational Join Index".to_string(),
                estimated_speedup: "10x".to_string(),
                ddl: format!("CREATE INDEX idx_ai_auto_{}_{} ON {} ({}_id);", table, related, table, related),
                is_applied: false,
            })
        } else {
            None
        }
    }

    pub fn recommend_storage_tier(&self, file_path: &str, access_count: usize) -> &'static str {
        if access_count > 1000 {
            info!("AI Tuner: {} accessed {} times. Promoting to InMemory Cache.", file_path, access_count);
            "Memory"
        } else {
            "S3"
        }
    }

    pub fn determine_optimal_chunk_size(&self, document_preview: &str) -> usize {
        if document_preview.matches("```").count() > 2 {
            info!("AI Tuner: High code-block density detected. Increasing chunk size to preserve logic blocks.");
            1024
        } else {
            512
        }
    }

    /// Dynamic Semantic Cache Tuning: Optimize threshold based on system hit rates
    pub fn tune_semantic_cache_threshold(&self, false_positive_rate: f64) -> f32 {
        if false_positive_rate > 0.05 {
            info!("AI Tuner: High false-positive rate detected in RAG cache. Tightening Cosine Similarity threshold to 0.95.");
            0.95
        } else {
            // Drop to 0.85 to save token costs when accuracy is stable
            0.85
        }
    }

    /// Dynamic Circuit Breaker Cooldown: Adjust timeout windows based on trip frequency
    pub fn tune_circuit_breaker_cooldown(&self, recent_trip_frequency: u64, base_cooldown_secs: u64) -> Duration {
        if recent_trip_frequency > 3 {
            info!("AI Tuner: Downstream service is highly volatile (tripped {} times). Stretching cooldown window.", recent_trip_frequency);
            Duration::from_secs(base_cooldown_secs * 2)
        } else {
            Duration::from_secs(base_cooldown_secs)
        }
    }

    /// Dynamic SCADA Data Compression: Widens or tightens deviation thresholds based on disk space
    pub fn tune_compression_deviation(&self, base_deviation: f64, disk_usage_percent: f64) -> f64 {
        if disk_usage_percent > 85.0 {
            warn!("AI Tuner: Storage at {}%. Widening Swinging Door Compression tolerance to aggressively drop sensor packets.", disk_usage_percent);
            base_deviation * 2.0
        } else if disk_usage_percent < 40.0 {
            info!("AI Tuner: Storage plentiful. Tightening compression tolerance to increase Historian data fidelity.");
            base_deviation * 0.5
        } else {
            base_deviation
        }
    }

    /// Dynamic Time-Series Auto-Bucketing: Adjusts resolution based on query performance
    pub fn tune_timeseries_bucket_interval(&self, base_interval_ms: u64, last_query_latency_ms: u64) -> u64 {
        if last_query_latency_ms > 200 {
            warn!("AI Tuner: Time-Series query extremely slow ({}ms). Increasing downsampling bucket size to speed up dashboards.", last_query_latency_ms);
            base_interval_ms * 5
        } else {
            base_interval_ms
        }
    }

    // --- Web3 / Blockchain Auto-Tuning Methods ---

    /// Dynamically tunes the ZK-Rollup Sequencer based on Ethereum Mainnet Gas Prices
    pub fn tune_zk_rollup_batch_interval(&self, current_eth_gas_gwei: f64, base_interval_secs: u64) -> Duration {
        if current_eth_gas_gwei > 50.0 {
            warn!("AI Tuner: Ethereum network heavily congested ({} Gwei). Stretching ZK-Rollup interval to save on Layer 1 settlement costs.", current_eth_gas_gwei);
            Duration::from_secs(base_interval_secs * 4)
        } else if current_eth_gas_gwei < 15.0 {
            info!("AI Tuner: Ethereum gas is cheap ({} Gwei). Tightening ZK-Rollup interval for faster Layer 2 finality.", current_eth_gas_gwei);
            Duration::from_secs(base_interval_secs / 2)
        } else {
            Duration::from_secs(base_interval_secs)
        }
    }

    /// Evaluates whether the Vella Paymaster should sponsor gas for a specific user based on spam risk
    pub fn predict_gas_sponsorship_viability(&self, user_daily_tx_count: u64, bot_probability: f64) -> bool {
        if bot_probability > 0.85 || user_daily_tx_count > 100 {
            warn!("AI Tuner: Paymaster risk threshold exceeded (Bot Prob: {}). Rejecting Account Abstraction gas sponsorship.", bot_probability);
            false
        } else {
            info!("AI Tuner: Wallet action approved. Sponsoring Gas via EIP-4337.");
            true
        }
    }

    // --- High-Frequency Trading & Advanced Web3 Auto-Tuning ---

    /// Dynamically adjusts the Limit Order Book (LOB) matching batch size during high market volatility
    pub fn tune_lob_matching_batch_size(&self, orders_per_second: u64) -> u64 {
        if orders_per_second > 1_000_000 {
            warn!("AI Tuner: Market Open Volatility detected ({} OPS). Batching LOB matching to preserve L3 CPU Cache.", orders_per_second);
            100 // Batch 100 orders at a time
        } else {
            1 // Real-time FIFO nanosecond matching
        }
    }

    /// Predictive Flash-Crash Kill Switch for the FIX Protocol
    pub fn predict_market_volatility_circuit_breaker(&self, vix_index_level: f64, order_rejection_rate: f64) -> bool {
        if vix_index_level > 40.0 || order_rejection_rate > 0.15 {
            warn!("AI Tuner: SEVERE MARKET ANOMALY DETECTED. Triggering FIX Protocol Kill Switch to prevent Flash Crash wipeout.");
            true // Trip the circuit breaker (halt trading)
        } else {
            false // Safe to trade
        }
    }

    /// Optimizes Fully Homomorphic Encryption (FHE) polynomial depth based on CPU load
    pub fn tune_fhe_encryption_depth(&self, system_load_percent: f64) -> u32 {
        if system_load_percent > 90.0 {
            warn!("AI Tuner: CPU load critical ({}%). Reducing FHE polynomial degree to maintain throughput.", system_load_percent);
            4096 // Lower cryptographic hardness for faster AI inference
        } else {
            8192 // Maximum mathematical privacy
        }
    }

    /// Dynamically adjusts Cross-Chain Oracle Slippage based on DEX Liquidity
    pub fn tune_cross_chain_oracle_slippage(&self, source_chain_liquidity_usd: f64) -> f64 {
        if source_chain_liquidity_usd < 1_000_000.0 {
            warn!("AI Tuner: Low liquidity detected on target chain. Increasing Oracle slippage tolerance to 3.0% to guarantee execution.");
            3.0
        } else {
            0.5 // Standard 0.5% slippage
        }
    }

    // --- Frontier Expansion Auto-Tuning (Space, Robotics, Gaming) ---

    /// Dynamically adjusts Delay Tolerant Networking (DTN) latency thresholds based on Solar Weather / Link Quality
    pub fn tune_dtn_latency_tolerance(&self, solar_flare_activity_index: f64, base_tolerance: u64) -> u64 {
        if solar_flare_activity_index > 7.0 {
            warn!("AI Tuner: High solar flare activity ({}) detected. Increasing deep space DTN queue tolerance by 4 hours.", solar_flare_activity_index);
            base_tolerance + 14400 
        } else {
            base_tolerance
        }
    }

    /// Dynamically scales Lidar/SLAM point cloud downsampling to maintain server stability during rapid drone movement
    pub fn tune_slam_downsample_rate(&self, drone_velocity_ms: f64) -> usize {
        if drone_velocity_ms > 20.0 {
            warn!("AI Tuner: Drone fleet moving at extreme velocity ({}m/s). Downsampling point cloud by 4x to preserve processing bandwidth.", drone_velocity_ms);
            4
        } else {
            1
        }
    }

    /// Dynamically expands multiplayer matchmaking ELO tolerance if the active player pool is small, preventing infinite queues
    pub fn tune_matchmaking_elo_tolerance(&self, active_player_pool: u32, base_elo: u32) -> u32 {
        if active_player_pool < 500 {
            warn!("AI Tuner: Low multiplayer population ({} players). Expanding ELO tolerance to ensure fast lobby filling.", active_player_pool);
            base_elo * 2
        } else {
            base_elo
        }
    }
}
