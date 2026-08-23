use crate::ai::decision::AiDecisionEngine;
use crate::ai::generator::AiScaffolder;
use crate::ai::vector::VectorSearchQuery;
use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::VellaError;
use crate::core::events::SystemEvent;
use crate::db::adapter::DatabaseAdapter;
use crate::db::DatabaseType;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApplyIndexPayload {
    pub table: String,
    pub column: String,
}

#[derive(Debug, Deserialize)]
pub struct AssessRiskPayload {
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateModelPayload {
    pub name: String,
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct RagQueryPayload {
    pub query: String,
    pub model_name: String,
    pub query_vector: Vec<f32>,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    #[serde(default)]
    pub force_fresh: bool,
}

fn default_top_k() -> usize {
    5
}

pub async fn ai_report_handler(State(state): State<AppState>) -> impl IntoResponse {
    let report = state.ai_tuner.generate_report(&state.registry);
    Json(json!({ "success": true, "report": report }))
}

pub async fn apply_index_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, VellaError> {
    let reviewer = match user {
        Some(ref u) if u.role.is_admin() => u,
        _ => return Err(VellaError::Forbidden("Requires Admin role to apply database indexes".to_string())),
    };

    let table = params
        .get("table")
        .ok_or_else(|| VellaError::Validation("Parameter 'table' is required".to_string()))?;
    let column = params
        .get("column")
        .ok_or_else(|| VellaError::Validation("Parameter 'column' is required".to_string()))?;

    let result = state.ai_tuner.apply_index(&state.pool, table, column).await?;

    Ok(Json(json!({
        "success": true,
        "message": result,
        "applied_by": reviewer.username
    })))
}

pub async fn assess_risk_handler(
    Json(payload): Json<AssessRiskPayload>,
) -> impl IntoResponse {
    let assessment = AiDecisionEngine::assess_approval_risk(
        &payload.field_name,
        payload.old_value.as_deref(),
        &payload.new_value,
    );

    Json(json!({ "success": true, "assessment": assessment }))
}

/// Agentic AI Schema Generator Handler (`POST /api/ai/generate-model`)
pub async fn generate_model_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateModelPayload>,
) -> impl IntoResponse {
    let db_type = DatabaseType::from_url(&state.config.database_url);
    let scaffold_result = AiScaffolder::scaffold(&payload.name, &payload.prompt, db_type);

    Json(json!({
        "success": true,
        "result": scaffold_result
    }))
}

/// RAG & Semantic Cached AI Query Handler (`POST /api/ai/rag/query`)
pub async fn rag_query_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Json(payload): Json<RagQueryPayload>,
) -> Result<impl IntoResponse, VellaError> {
    let start = std::time::Instant::now();
    let user_id = user.as_ref().map(|u| u.id);
    let identifier = user.as_ref().map(|u| u.username.as_str()).unwrap_or("anonymous");

    // 1. Rate Limiting Check
    state.token_limiter.check_and_consume(identifier, 500)?;

    // 2. Check Semantic Cache
    if !payload.force_fresh && state.config.enable_semantic_cache {
        if let Some((cached_response, similarity, cached_q)) = state.semantic_cache.lookup(&payload.query_vector) {
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            state.event_bus.publish(SystemEvent::SemanticCacheHit {
                query: payload.query.clone(),
                similarity,
            });

            return Ok(Json(json!({
                "success": true,
                "cached": true,
                "similarity_score": similarity,
                "matched_query": cached_q,
                "latency_ms": (latency * 100.0).round() / 100.0,
                "response": cached_response
            })));
        }
    }

    // 3. Perform Vector Search retrieval across target model
    let schema = state
        .registry
        .get(&payload.model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", payload.model_name)))?;

    let v_query = VectorSearchQuery {
        model: schema.name.clone(),
        vector_field: "embedding".to_string(),
        query_vector: payload.query_vector.clone(),
        top_k: payload.top_k,
        metric: crate::ai::vector::DistanceMetric::Cosine,
    };

    let context_matches = state.db.search_vectors(schema, &v_query).await?;
    let latency = start.elapsed().as_secs_f64() * 1000.0;

    let response_data = json!({
        "retrieved_context": context_matches,
        "synthesis": format!("RAG synthesis for prompt: '{}' using {} retrieved knowledge fragments.", payload.query, context_matches.len()),
    });

    // 4. Store in Semantic Cache
    if state.config.enable_semantic_cache {
        state.semantic_cache.put(&payload.query, payload.query_vector, response_data.clone());
    }

    // 5. Audit Prompt Log
    let log_entry = state.prompt_logger.log_completion(
        user_id,
        &payload.model_name,
        &payload.query,
        &serde_json::to_string(&response_data).unwrap_or_default(),
        150,
        350,
        latency,
        false,
    );

    state.event_bus.publish(SystemEvent::AiPromptLogged {
        model_used: payload.model_name,
        prompt_tokens: 150,
        completion_tokens: 350,
        latency_ms: latency,
    });

    Ok(Json(json!({
        "success": true,
        "cached": false,
        "latency_ms": (latency * 100.0).round() / 100.0,
        "log_id": log_entry.id,
        "cost_usd": log_entry.estimated_cost_usd,
        "response": response_data
    })))
}

pub async fn cache_stats_handler(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "success": true,
        "semantic_cache": state.semantic_cache.stats_json(),
        "recent_prompts": state.prompt_logger.recent_logs(20)
    }))
}

pub async fn cache_purge_handler(State(state): State<AppState>) -> impl IntoResponse {
    state.semantic_cache.purge();
    Json(json!({ "success": true, "message": "Semantic cache purged" }))
}

pub async fn token_stats_handler(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "success": true,
        "total_tokens_consumed": state.token_limiter.total_tokens_consumed(),
        "total_requests_blocked": state.token_limiter.total_requests_blocked(),
        "rate_limit_per_minute": state.config.token_rate_limit_per_minute
    }))
}
