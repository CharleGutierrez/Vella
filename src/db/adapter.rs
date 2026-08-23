use crate::ai::vector::{VectorSearchQuery, VectorSearchResult};
use crate::core::error::VellaError;
use crate::model::ModelSchema;
use async_trait::async_trait;
use serde_json::{Map, Value};

/// Extensible database adapter interface for Vella (SQLite, PostgreSQL, MySQL)
/// with unified native vector similarity search support.
#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn get_by_id(&self, schema: &ModelSchema, id: i64) -> Result<Option<Value>, VellaError>;
    async fn insert(&self, schema: &ModelSchema, payload: &Map<String, Value>) -> Result<Value, VellaError>;
    async fn update(&self, schema: &ModelSchema, id: i64, payload: &Map<String, Value>) -> Result<Option<Value>, VellaError>;
    async fn delete(&self, schema: &ModelSchema, id: i64) -> Result<bool, VellaError>;
    async fn execute_raw(&self, sql: &str) -> Result<(), VellaError>;
    /// Perform vector similarity search across model records
    async fn search_vectors(&self, schema: &ModelSchema, query: &VectorSearchQuery) -> Result<Vec<VectorSearchResult>, VellaError>;
}
