pub mod decision;
pub mod generator;
pub mod middleware;
pub mod stats;
pub mod tuner;
pub mod vector;

pub use decision::{AiDecisionEngine, RiskAssessment, RiskLevel};
pub use generator::{AiScaffolder, GeneratedScaffoldResult};
pub use middleware::{PromptLogEntry, PromptLogger, SemanticCache, TokenRateLimiter};
pub use stats::{SlowQueryLog, WorkloadStats};
pub use tuner::{AiTuner, AiTunerReport, IndexRecommendation};
pub use vector::{
    cosine_similarity, dot_product, euclidean_distance, format_pgvector_literal,
    parse_vector_from_json, DistanceMetric, VectorSearchQuery, VectorSearchResult,
};
