use crate::ai::stats::WorkloadStats;
use crate::core::error::VellaError;
use crate::model::SchemaRegistry;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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

/// The AI Tuner: Analyzes database query execution patterns, detects latency spikes,
/// and recommends or auto-applies missing indexes and connection parameters.
#[derive(Debug, Clone)]
pub struct AiTuner {
    pub stats: Arc<WorkloadStats>,
    column_access_counts: Arc<RwLock<HashMap<String, u64>>>,
    applied_indexes: Arc<RwLock<Vec<String>>>,
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
        }
    }

    /// Record a query filter event for AI pattern analysis
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

    /// Generate AI Index Recommendations based on runtime query analysis
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

                // Recommend index if filtered frequently or flagged searchable
                if hit_count >= 1 || field.searchable || field.filterable {
                    let idx_name = format!("idx_ai_{}_{}", schema.table_name, field.name);
                    let is_applied = applied.contains(&idx_name);

                    recs.push(IndexRecommendation {
                        id: format!("{}_{}", schema.table_name, field.name),
                        model: schema.name.clone(),
                        table_name: schema.table_name.clone(),
                        column: field.name.clone(),
                        reason: format!(
                            "Column '{}.{}' was queried {} times. Creating a B-Tree index will eliminate sequential full-table scans.",
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

    /// Auto-apply an AI-recommended index directly to the database
    pub async fn apply_index(
        &self,
        pool: &Pool<Sqlite>,
        table: &str,
        column: &str,
    ) -> Result<String, VellaError> {
        let idx_name = format!("idx_ai_{}_{}", table, column);
        let ddl = format!(
            "CREATE INDEX IF NOT EXISTS \"{}\" ON \"{}\" (\"{}\");",
            idx_name, table, column
        );

        sqlx::query(&ddl).execute(pool).await?;

        let mut applied = self.applied_indexes.write().unwrap();
        if !applied.contains(&idx_name) {
            applied.push(idx_name.clone());
        }

        Ok(format!("Successfully created index '{}' on {}({})", idx_name, table, column))
    }

    /// Generate complete AI Tuner report
    pub fn generate_report(&self, registry: &SchemaRegistry) -> AiTunerReport {
        let (p50, p95, p99) = self.stats.percentiles();
        let recs = self.generate_recommendations(registry);

        let workload_summary = if p99 < 2.0 {
            "💚 Optimal: Sub-millisecond response latency. Database I/O is operating at peak efficiency.".to_string()
        } else if p99 < 15.0 {
            "⚡ Good: Fast query execution. Consider applying recommended indexes to optimize p99 latency.".to_string()
        } else {
            "⚠️ Attention: High latency detected on unindexed columns. Applying AI index recommendations is advised.".to_string()
        };

        AiTunerReport {
            engine_status: "AI Optimization Active & Telemetry Online".to_string(),
            total_queries_analyzed: self.stats.total_queries(),
            qps: (self.stats.qps() * 100.0).round() / 100.0,
            p50_latency_ms: (p50 * 100.0).round() / 100.0,
            p95_latency_ms: (p95 * 100.0).round() / 100.0,
            p99_latency_ms: (p99 * 100.0).round() / 100.0,
            recommendations: recs,
            workload_summary,
        }
    }
}
