use crate::ai::decision::AiDecisionEngine;
use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::VellaError;
use crate::core::events::SystemEvent;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use sqlx::Row;

pub async fn list_approvals_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, VellaError> {
    let raw_approvals = state.approval_service.list_pending().await?;
    let enriched: Vec<serde_json::Value> = raw_approvals.into_iter().map(|app| {
        let risk = AiDecisionEngine::assess_approval_risk(
            &app.field_name,
            app.old_value.as_deref(),
            &app.new_value,
        );
        json!({
            "id": app.id,
            "model_name": app.model_name,
            "record_id": app.record_id,
            "field_name": app.field_name,
            "old_value": app.old_value,
            "new_value": app.new_value,
            "status": app.status,
            "requested_by_username": app.requested_by_username,
            "created_at": app.created_at,
            "ai_risk": risk,
        })
    }).collect();

    Ok(Json(json!({ "success": true, "data": enriched })))
}

pub async fn approve_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, VellaError> {
    let reviewer = match user {
        Some(ref u) if u.role.can_approve() => u,
        _ => return Err(VellaError::Forbidden("Requires Manager or Admin role".to_string())),
    };

    let row_opt = sqlx::query("SELECT model_name FROM _vella_approvals WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;

    let model_name: String = match row_opt {
        Some(r) => r.try_get("model_name")?,
        None => return Err(VellaError::NotFound("Approval request not found".to_string())),
    };

    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let approved = state
        .approval_service
        .approve(id, reviewer.id, &reviewer.username, &schema.table_name)
        .await?;

    if approved {
        state.event_bus.publish(SystemEvent::ApprovalResolved {
            approval_id: id,
            approved: true,
        });

        Ok(Json(json!({ "success": true, "message": "Change approved and applied to record" })))
    } else {
        Err(VellaError::Validation("Approval is no longer pending".to_string()))
    }
}

pub async fn reject_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, VellaError> {
    let reviewer = match user {
        Some(ref u) if u.role.can_approve() => u,
        _ => return Err(VellaError::Forbidden("Requires Manager or Admin role".to_string())),
    };

    let rejected = state
        .approval_service
        .reject(id, reviewer.id, &reviewer.username)
        .await?;

    if rejected {
        state.event_bus.publish(SystemEvent::ApprovalResolved {
            approval_id: id,
            approved: false,
        });

        Ok(Json(json!({ "success": true, "message": "Change request rejected" })))
    } else {
        Err(VellaError::Validation("Approval is no longer pending".to_string()))
    }
}
