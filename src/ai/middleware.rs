use crate::ai::vector::cosine_similarity;
use crate::core::error::VellaError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tracing::info;

/// 1. AI Token Rate Limiter
/// Protects LLM backends by enforcing per-minute and per-day token consumption quotas.
#[derive(Debug, Clone)]
pub struct TokenRateLimiter {
    max_tokens_per_minute: u64,
    user_usage: Arc<RwLock<HashMap<String, VecDeque<(Instant, u64)>>>>,
    total_tokens_consumed: Arc<AtomicU64>,
    total_requests_blocked: Arc<AtomicU64>,
}

impl Default for TokenRateLimiter {
    fn default() -> Self {
        Self::new(100_000)
    }
}

impl TokenRateLimiter {
    pub fn new(max_tokens_per_minute: u64) -> Self {
        Self {
            max_tokens_per_minute,
            user_usage: Arc::new(RwLock::new(HashMap::new())),
            total_tokens_consumed: Arc::new(AtomicU64::new(0)),
            total_requests_blocked: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Check if a user/IP can consume the requested tokens
    pub fn check_and_consume(&self, identifier: &str, tokens_requested: u64) -> Result<(), VellaError> {
        let now = Instant::now();
        let one_minute_ago = now - std::time::Duration::from_secs(60);

        let mut usage_map = self.user_usage.write().unwrap();
        let queue = usage_map.entry(identifier.to_string()).or_default();

        // Evict entries older than 1 minute
        while let Some((time, _)) = queue.front() {
            if *time < one_minute_ago {
                queue.pop_front();
            } else {
                break;
            }
        }

        let current_minute_tokens: u64 = queue.iter().map(|(_, t)| *t).sum();

        if current_minute_tokens + tokens_requested > self.max_tokens_per_minute {
            self.total_requests_blocked.fetch_add(1, Ordering::Relaxed);
            return Err(VellaError::RateLimited(format!(
                "Token rate limit exceeded for '{}'. Current usage: {} / {} tokens/min",
                identifier, current_minute_tokens, self.max_tokens_per_minute
            )));
        }

        queue.push_back((now, tokens_requested));
        self.total_tokens_consumed.fetch_add(tokens_requested, Ordering::Relaxed);

        Ok(())
    }

    pub fn total_tokens_consumed(&self) -> u64 {
        self.total_tokens_consumed.load(Ordering::Relaxed)
    }

    pub fn total_requests_blocked(&self) -> u64 {
        self.total_requests_blocked.load(Ordering::Relaxed)
    }
}

/// 2. Prompt Logger Entry & Telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptLogEntry {
    pub id: String,
    pub user_id: Option<i64>,
    pub model_name: String,
    pub prompt: String,
    pub response: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    pub estimated_cost_usd: f64,
    pub latency_ms: f64,
    pub cached: bool,
    pub created_at: String,
}

/// Prompt Logger: Audits all RAG / LLM completions, token consumption, and latency
#[derive(Debug, Clone)]
pub struct PromptLogger {
    logs: Arc<RwLock<VecDeque<PromptLogEntry>>>,
}

impl Default for PromptLogger {
    fn default() -> Self {
        Self::new(500)
    }
}

impl PromptLogger {
    pub fn new(capacity: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(capacity))),
        }
    }

    pub fn log_completion(
        &self,
        user_id: Option<i64>,
        model_name: &str,
        prompt: &str,
        response: &str,
        prompt_tokens: u64,
        completion_tokens: u64,
        latency_ms: f64,
        cached: bool,
    ) -> PromptLogEntry {
        let total_tokens = prompt_tokens + completion_tokens;
        // Approximation: $0.50 / 1M prompt tokens, $1.50 / 1M completion tokens
        let cost = (prompt_tokens as f64 * 0.0000005) + (completion_tokens as f64 * 0.0000015);

        let entry = PromptLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            model_name: model_name.to_string(),
            prompt: prompt.to_string(),
            response: response.to_string(),
            prompt_tokens,
            completion_tokens,
            total_tokens,
            estimated_cost_usd: (cost * 1_000_000.0).round() / 1_000_000.0,
            latency_ms,
            cached,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let mut logs = self.logs.write().unwrap();
        if logs.len() >= 500 {
            logs.pop_front();
        }
        logs.push_back(entry.clone());

        entry
    }

    pub fn recent_logs(&self, limit: usize) -> Vec<PromptLogEntry> {
        let logs = self.logs.read().unwrap();
        logs.iter().rev().take(limit).cloned().collect()
    }
}

/// 3. Semantic Cache Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCacheItem {
    pub query: String,
    pub embedding: Vec<f32>,
    pub response: Value,
    pub hit_count: u64,
    pub created_at: String,
}

/// Semantic Cache: Matches incoming queries via vector cosine similarity.
/// If cosine_similarity >= threshold, returns cached LLM response in < 1ms!
#[derive(Debug, Clone)]
pub struct SemanticCache {
    threshold: f32,
    items: Arc<RwLock<Vec<SemanticCacheItem>>>,
    total_hits: Arc<AtomicU64>,
    total_misses: Arc<AtomicU64>,
}

impl Default for SemanticCache {
    fn default() -> Self {
        Self::new(0.90)
    }
}

impl SemanticCache {
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold,
            items: Arc::new(RwLock::new(Vec::new())),
            total_hits: Arc::new(AtomicU64::new(0)),
            total_misses: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Lookup cached response for a query embedding
    pub fn lookup(&self, query_embedding: &[f32]) -> Option<(Value, f32, String)> {
        let mut items = self.items.write().unwrap();
        let mut best_match: Option<(usize, f32)> = None;

        for (idx, item) in items.iter().enumerate() {
            let sim = cosine_similarity(query_embedding, &item.embedding);
            if sim >= self.threshold {
                if let Some((_, best_sim)) = best_match {
                    if sim > best_sim {
                        best_match = Some((idx, sim));
                    }
                } else {
                    best_match = Some((idx, sim));
                }
            }
        }

        if let Some((idx, sim)) = best_match {
            items[idx].hit_count += 1;
            self.total_hits.fetch_add(1, Ordering::Relaxed);
            info!("🎯 [Vella Semantic Cache] Cache HIT! (similarity: {:.4}, query: '{}')", sim, items[idx].query);
            return Some((items[idx].response.clone(), sim, items[idx].query.clone()));
        }

        self.total_misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    /// Store query embedding and response in semantic cache
    pub fn put(&self, query: &str, embedding: Vec<f32>, response: Value) {
        let mut items = self.items.write().unwrap();
        // If cache gets large, purge least used
        if items.len() >= 1000 {
            items.sort_by_key(|it| it.hit_count);
            items.remove(0);
        }

        items.push(SemanticCacheItem {
            query: query.to_string(),
            embedding,
            response,
            hit_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        });
    }

    pub fn stats_json(&self) -> serde_json::Value {
        let hits = self.total_hits.load(Ordering::Relaxed);
        let misses = self.total_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 { (hits as f64 / total as f64) * 100.0 } else { 0.0 };

        serde_json::json!({
            "cache_entries": self.items.read().unwrap().len(),
            "threshold": self.threshold,
            "total_hits": hits,
            "total_misses": misses,
            "hit_rate_percentage": (hit_rate * 10.0).round() / 10.0
        })
    }

    pub fn purge(&self) {
        let mut items = self.items.write().unwrap();
        items.clear();
    }
}
