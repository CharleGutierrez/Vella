use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::VellaError;
use crate::core::events::SystemEvent;
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use sqlx::Row;
use std::collections::HashMap;

pub async fn list_audit_logs_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, VellaError> {
    let limit = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(50);
    let offset = params.get("offset").and_then(|v| v.parse().ok()).unwrap_or(0);

    let logs = state.audit_service.list_logs(limit, offset).await?;
    Ok(Json(json!({ "success": true, "data": logs })))
}

pub async fn rollback_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(log_id): Path<i64>,
) -> Result<impl IntoResponse, VellaError> {
    let row_opt = sqlx::query("SELECT model_name, record_id FROM _vella_audit_logs WHERE id = ? LIMIT 1")
        .bind(log_id)
        .fetch_optional(&state.pool)
        .await?;

    let (model_name, record_id): (String, i64) = match row_opt {
        Some(r) => (r.try_get("model_name")?, r.try_get("record_id")?),
        None => return Err(VellaError::NotFound("Audit log not found".to_string())),
    };

    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Schema for '{}' not found", model_name)))?;

    let success = state
        .audit_service
        .rollback(
            log_id,
            schema,
            user.as_ref().map(|u| u.id),
            user.as_ref().map(|u| u.username.as_str()),
        )
        .await?;

    if success {
        state.event_bus.publish(SystemEvent::RollbackExecuted {
            log_id,
            model: schema.name.clone(),
            record_id,
        });

        Ok(Json(json!({ "success": true, "message": "Successfully rolled back to snapshot state" })))
    } else {
        Err(VellaError::Validation("Could not perform rollback on snapshot".to_string()))
    }
}
