use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;
use serde_json::Value;

/// Ultra-low latency Key-Value store mapped directly in memory for ML Feature Lookups
#[derive(Clone)]
pub struct FeatureStore {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl FeatureStore {
    pub fn new() -> Self {
        info!("Initializing In-Memory MLOps Feature Store");
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Nightly background jobs push heavy aggregations here
    pub fn push_feature(&self, user_id: &str, feature_name: &str, value: Value) {
        let mut db = self.store.write().unwrap();
        let key = format!("{}:{}", user_id, feature_name);
        db.insert(key, value);
    }

    /// Live inference endpoints pull this in < 1ms
    pub fn get_feature(&self, user_id: &str, feature_name: &str) -> Option<Value> {
        let db = self.store.read().unwrap();
        let key = format!("{}:{}", user_id, feature_name);
        db.get(&key).cloned()
    }
}
