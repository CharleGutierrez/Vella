pub mod decision;
pub mod generator;
pub mod middleware;
pub mod stats;
pub mod tuner;
pub mod vector;
pub mod chunking;
pub mod local_llm;
pub mod registry;
pub mod gpu;
pub mod vision;
pub mod gateway;

pub use chunking::DocumentSplitter;
pub use local_llm::LocalLlmEngine;
pub use registry::ModelRegistry;
pub use gpu::HardwareAccelerator;
pub use gateway::{UnifiedAiGateway, AiConfig, AiProvider};
pub use decision::{AiDecisionEngine, RiskAssessment, RiskLevel};
pub use generator::{AiScaffolder, GeneratedScaffoldResult};
pub use middleware::{PromptLogEntry, PromptLogger, SemanticCache, TokenRateLimiter};
pub use stats::{SlowQueryLog, WorkloadStats};
pub use tuner::{AiTuner, AiTunerReport, IndexRecommendation};
pub use vector::{
    cosine_similarity, dot_product, euclidean_distance, format_pgvector_literal,
    parse_vector_from_json, DistanceMetric, VectorSearchQuery, VectorSearchResult,
};
