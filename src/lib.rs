//! # Vella
//!
//! Next-Generation LLM-Native Rust Web Engine & Headless CMS.
//! Bridging PocketBase and Supabase with native vector search (pgvector & sqlite-vec),
//! agentic schema scaffolding, AI middleware (semantic caching, prompt logging, token rate-limiting),
//! zero-config TypeScript sync, realtime WebSocket/SSE, and multi-database scale.

pub mod ai;
pub mod api;
pub mod app;
pub mod audit;
pub mod auth;
pub mod core;
pub mod db;
pub mod model;
pub mod prelude;
pub mod realtime;
pub mod types;
pub mod ui;

pub use ai::{
    cosine_similarity, dot_product, euclidean_distance, AiDecisionEngine, AiScaffolder,
    AiTuner, DistanceMetric, PromptLogEntry, PromptLogger, RiskAssessment, RiskLevel,
    SemanticCache, TokenRateLimiter, VectorSearchQuery, VectorSearchResult,
};
pub use app::{Vella, VellaApp};
pub use core::config::VellaConfig;
pub use core::error::VellaError;
pub use core::events::{EventBus, SystemEvent};
pub use core::hooks::ModelHook;
pub use db::{DatabaseAdapter, DatabaseType, SqlDialect};
pub use model::{Field, FieldType, ModelSchema, SchemaRegistry};
pub use realtime::{RealtimeHub, RealtimeMessage};
pub use types::TypeScriptGenerator;
