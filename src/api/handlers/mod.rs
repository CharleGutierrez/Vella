pub mod ai;
pub mod approval;
pub mod audit;
pub mod auth;
pub mod crud;
pub mod health;
pub mod oauth;
pub mod realtime;
pub mod types;

pub use ai::*;
pub use approval::*;
pub use audit::*;
pub use auth::*;
pub use crud::*;
pub use health::*;
pub use oauth::*;
pub use realtime::*;
pub use types::*;

use crate::ai::middleware::{PromptLogger, SemanticCache, TokenRateLimiter};
use crate::ai::tuner::AiTuner;
use crate::audit::{ApprovalService, AuditService};
use crate::auth::oauth::OAuthService;
use crate::auth::service::AuthService;
use crate::core::config::VellaConfig;
use crate::core::events::EventBus;
use crate::core::hooks::ModelHook;
use crate::core::resilience::{CircuitBreaker, SystemWatchdog};
use crate::db::SqliteDatabase;
use crate::model::SchemaRegistry;
use crate::realtime::RealtimeHub;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

/// Shared application state for all HTTP route handlers in Vella
#[derive(Clone)]
pub struct AppState {
    pub db: SqliteDatabase,
    pub pool: Pool<Sqlite>,
    pub config: Arc<VellaConfig>,
    pub registry: SchemaRegistry,
    pub auth_service: Arc<AuthService>,
    pub oauth_service: Arc<OAuthService>,
    pub audit_service: Arc<AuditService>,
    pub approval_service: Arc<ApprovalService>,
    pub event_bus: Arc<EventBus>,
    pub realtime_hub: Arc<RealtimeHub>,
    pub hooks: Arc<Vec<Box<dyn ModelHook>>>,
    pub watchdog: Arc<SystemWatchdog>,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub ai_tuner: Arc<AiTuner>,
    pub token_limiter: Arc<TokenRateLimiter>,
    pub prompt_logger: Arc<PromptLogger>,
    pub semantic_cache: Arc<SemanticCache>,
}
