use serde::{Deserialize, Serialize};

/// Global configuration for the Vella LLM-Native engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VellaConfig {
    pub site_name: String,
    pub bind_address: String,
    pub database_url: String,
    pub max_db_connections: u32,
    pub session_duration_days: i64,
    pub enable_cors: bool,
    pub enable_gzip: bool,
    pub auto_export_types: bool,
    pub types_export_path: Option<String>,
    pub enable_semantic_cache: bool,
    pub semantic_cache_threshold: f32,
    pub token_rate_limit_per_minute: u64,
    pub redis_url: Option<String>,
    pub otlp_endpoint: Option<String>,
}

impl Default for VellaConfig {
    fn default() -> Self {
        Self {
            site_name: "Vella".to_string(),
            bind_address: "0.0.0.0:8080".to_string(),
            database_url: "sqlite://vella.db?mode=rwc".to_string(),
            max_db_connections: 25,
            session_duration_days: 7,
            enable_cors: true,
            enable_gzip: true,
            auto_export_types: false,
            types_export_path: None,
            enable_semantic_cache: true,
            semantic_cache_threshold: 0.90,
            token_rate_limit_per_minute: 100_000,
            redis_url: None,
            otlp_endpoint: None,
        }
    }
}

impl VellaConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.site_name = name.into();
        self
    }

    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.bind_address = addr.into();
        self
    }

    pub fn database(mut self, url: impl Into<String>) -> Self {
        self.database_url = url.into();
        self
    }

    pub fn max_db_connections(mut self, count: u32) -> Self {
        self.max_db_connections = count;
        self
    }

    pub fn auto_export_types_to(mut self, path: impl Into<String>) -> Self {
        self.auto_export_types = true;
        self.types_export_path = Some(path.into());
        self
    }

    pub fn semantic_cache(mut self, enabled: bool, threshold: f32) -> Self {
        self.enable_semantic_cache = enabled;
        self.semantic_cache_threshold = threshold;
        self
    }

    pub fn token_rate_limit(mut self, limit_per_minute: u64) -> Self {
        self.token_rate_limit_per_minute = limit_per_minute;
        self
    }

    pub fn with_redis(mut self, url: impl Into<String>) -> Self {
        self.redis_url = Some(url.into());
        self
    }

    pub fn with_opentelemetry(mut self, endpoint: impl Into<String>) -> Self {
        self.otlp_endpoint = Some(endpoint.into());
        self
    }
}
