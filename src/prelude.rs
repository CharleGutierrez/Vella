pub use crate::ai::{
    cosine_similarity, dot_product, euclidean_distance, AiDecisionEngine, AiScaffolder,
    AiTuner, DistanceMetric, PromptLogEntry, PromptLogger, RiskAssessment, RiskLevel,
    SemanticCache, TokenRateLimiter, VectorSearchQuery, VectorSearchResult,
};
pub use crate::app::{Vella, VellaApp};
pub use crate::auth::{AuthUser, AuthenticatedUser, OptionalAuthUser, Role, Session};
pub use crate::core::config::VellaConfig;
pub use crate::core::error::VellaError;
pub use crate::core::events::{EventBus, SystemEvent};
pub use crate::core::hooks::ModelHook;
pub use crate::db::{DatabaseAdapter, DatabaseType, SqlDialect};
pub use crate::model::{Field, FieldType, ModelSchema, SchemaRegistry};
pub use crate::realtime::{RealtimeHub, RealtimeMessage};
pub use crate::types::TypeScriptGenerator;
