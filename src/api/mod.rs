pub mod filter;
pub mod handlers;
pub mod openapi;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::*;

pub use handlers::AppState;
pub use openapi::{openapi_json_handler, swagger_handler};

/// Build the unified Vella API Router with REST, Vector, AI, Realtime, and Auth sub-modules
pub fn build_api_router(state: AppState) -> Router {
    Router::new()
        // 1. Auth & OAuth endpoints
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/auth/me", get(me_handler))
        .route("/api/auth/oauth/google", get(google_oauth_url_handler))
        .route("/api/auth/oauth/github", get(github_oauth_url_handler))
        .route("/api/auth/oauth/callback", post(oauth_callback_handler))
        .route("/api/auth/magic-link/request", post(magic_link_request_handler))
        .route("/api/auth/magic-link/verify", post(magic_link_verify_handler))
        // 2. Schema & dAPI endpoints
        .route("/api/d/schema", get(schema_handler))
        .route("/api/d/:model", get(list_records_handler).post(create_record_handler))
        .route(
            "/api/d/:model/:id",
            get(get_record_handler)
                .put(update_record_handler)
                .delete(delete_record_handler),
        )
        .route("/api/d/:model/search-vector", post(search_vector_handler))
        // 3. Time-Travel Rollback & Audit Logs
        .route("/api/d/rollback/:log_id", post(rollback_handler))
        .route("/api/d/audit-logs", get(list_audit_logs_handler))
        // 4. Approval Workflow
        .route("/api/d/approvals", get(list_approvals_handler))
        .route("/api/d/approvals/:id/approve", post(approve_handler))
        .route("/api/d/approvals/:id/reject", post(reject_handler))
        // 5. Realtime Sync (WebSocket & SSE)
        .route("/api/realtime/ws", get(realtime_ws_handler))
        .route("/api/realtime/sse", get(realtime_sse_handler))
        // 6. Zero-Config TypeScript Types Export
        .route("/api/types/typescript.d.ts", get(typescript_definitions_handler))
        .route("/api/types/export", post(export_types_handler))
        // 7. AI Tuner, Decision Engine & Agentic Scaffolder
        .route("/api/ai/report", get(ai_report_handler))
        .route("/api/ai/indexes/apply", post(apply_index_handler))
        .route("/api/ai/assess-risk", post(assess_risk_handler))
        .route("/api/ai/generate-model", post(generate_model_handler))
        // 8. RAG & Semantic Cache AI Middleware
        .route("/api/ai/rag/query", post(rag_query_handler))
        .route("/api/ai/cache/stats", get(cache_stats_handler))
        .route("/api/ai/cache/purge", post(cache_purge_handler))
        .route("/api/ai/token-stats", get(token_stats_handler))
        // 9. Health & Resilience Probes
        .route("/health", get(health_check_handler))
        .route("/health/live", get(liveness_probe_handler))
        .route("/health/ready", get(readiness_probe_handler))
        // 10. OpenAPI & Swagger
        .route("/api/openapi.json", get(openapi_json_handler))
        .route("/swagger", get(swagger_handler))
        .with_state(state)
}
