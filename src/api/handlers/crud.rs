use crate::ai::vector::VectorSearchQuery;
use crate::api::filter::QueryOptions;
use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::VellaError;
use crate::core::events::SystemEvent;
use crate::db::adapter::DatabaseAdapter;
use crate::db::sqlite::SqliteDatabase;
use crate::model::validator::FieldValidator;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::{json, Map, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn schema_handler(State(state): State<AppState>) -> impl IntoResponse {
    let schemas = state.registry.all();
    Json(json!({ "success": true, "schemas": schemas }))
}

pub async fn list_records_handler(
    State(state): State<AppState>,
    Path(model_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let q_opts = QueryOptions::parse(&params);
    let (select_sql, select_params, count_sql, count_params) = q_opts.build_sql(schema);

    let start_time = std::time::Instant::now();

    // 1. Get total count
    let mut count_query = sqlx::query(&count_sql);
    for p in count_params {
        count_query = SqliteDatabase::bind_json_value(count_query, &p);
    }
    let count_row = count_query.fetch_one(&state.pool).await?;
    let total_count: i64 = count_row.try_get("total").unwrap_or(0);

    // 2. Select records
    let mut select_query = sqlx::query(&select_sql);
    for p in select_params {
        select_query = SqliteDatabase::bind_json_value(select_query, &p);
    }

    let rows = select_query.fetch_all(&state.pool).await?;
    let duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    // Telemetry to AI Tuner
    let filtered_fields: Vec<&str> = q_opts.filters.iter().map(|f| f.field_name.as_str()).collect();
    state.ai_tuner.record_query_pattern(&schema.name, &schema.table_name, &filtered_fields, duration_ms);

    let data: Vec<Value> = rows.iter().map(SqliteDatabase::row_to_json).collect();

    Ok(Json(json!({
        "success": true,
        "total": total_count,
        "limit": q_opts.limit,
        "offset": q_opts.offset,
        "data": data
    })))
}

pub async fn get_record_handler(
    State(state): State<AppState>,
    Path((model_name, id)): Path<(String, i64)>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let record = state
        .db
        .get_by_id(schema, id)
        .await?
        .ok_or_else(|| VellaError::NotFound(format!("Record #{} not found in '{}'", id, model_name)))?;

    Ok(Json(json!({ "success": true, "data": record })))
}

pub async fn create_record_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(model_name): Path<String>,
    Json(mut payload): Json<Value>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    // Run before_create hooks
    for hook in state.hooks.iter() {
        hook.before_create(&schema.name, &mut payload).await?;
    }

    let obj = match payload {
        Value::Object(map) => map,
        _ => return Err(VellaError::Validation("Payload must be a JSON object".to_string())),
    };

    // Validate fields
    for field in &schema.fields {
        FieldValidator::validate_field(field, obj.get(&field.name))?;
    }

    let created = state.db.insert(schema, &obj).await?;
    let record_id = created.get("id").and_then(|v| v.as_i64()).unwrap_or(0);

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            &schema.name,
            record_id,
            "CREATE",
            user.as_ref().map(|u| u.id),
            user.as_ref().map(|u| u.username.as_str()),
            &Value::Object(obj),
            &created,
            None,
        )
        .await;

    // Publish event for Realtime WebSocket & SSE clients
    state.event_bus.publish(SystemEvent::RecordCreated {
        model: schema.name.clone(),
        id: record_id,
        data: created.clone(),
    });

    // Run after_create hooks
    for hook in state.hooks.iter() {
        hook.after_create(&schema.name, &created).await?;
    }

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": created }))))
}

pub async fn update_record_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path((model_name, id)): Path<(String, i64)>,
    Json(mut payload): Json<Value>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let old_record = state
        .db
        .get_by_id(schema, id)
        .await?
        .ok_or_else(|| VellaError::NotFound(format!("Record #{} not found in '{}'", id, model_name)))?;

    // Run before_update hooks
    for hook in state.hooks.iter() {
        hook.before_update(&schema.name, id, &mut payload).await?;
    }

    let obj = match payload {
        Value::Object(map) => map,
        _ => return Err(VellaError::Validation("Payload must be a JSON object".to_string())),
    };

    let mut pending_approvals = Vec::new();
    let mut fields_to_update = Map::new();

    for (k, v) in obj {
        if let Some(field) = schema.get_field(&k) {
            FieldValidator::validate_field(field, Some(&v))?;

            if field.requires_approval && user.as_ref().map(|u| !u.role.can_approve()).unwrap_or(true) {
                let old_val = old_record.get(&k).map(|ov| ov.to_string());
                let new_val = v.to_string();
                let _ = state
                    .approval_service
                    .create_approval(
                        &schema.name,
                        id,
                        &k,
                        old_val.as_deref(),
                        &new_val,
                        user.as_ref().map(|u| u.id),
                        user.as_ref().map(|u| u.username.as_str()),
                    )
                    .await;
                pending_approvals.push(k);
            } else {
                fields_to_update.insert(k, v);
            }
        }
    }

    let updated_record = state
        .db
        .update(schema, id, &fields_to_update)
        .await?
        .ok_or_else(|| VellaError::NotFound("Failed to update record".to_string()))?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            &schema.name,
            id,
            "UPDATE",
            user.as_ref().map(|u| u.id),
            user.as_ref().map(|u| u.username.as_str()),
            &Value::Object(fields_to_update.clone()),
            &old_record,
            None,
        )
        .await;

    // Publish event for Realtime subscribers
    state.event_bus.publish(SystemEvent::RecordUpdated {
        model: schema.name.clone(),
        id,
        changes: Value::Object(fields_to_update),
    });

    // Run after_update hooks
    for hook in state.hooks.iter() {
        hook.after_update(&schema.name, id, &updated_record).await?;
    }

    let mut res_json = json!({ "success": true, "data": updated_record });
    if !pending_approvals.is_empty() {
        res_json["pending_approval_fields"] = json!(pending_approvals);
        res_json["message"] = json!("Some fields were queued for approval.");
    }

    Ok(Json(res_json))
}

pub async fn delete_record_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path((model_name, id)): Path<(String, i64)>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let old_record = state
        .db
        .get_by_id(schema, id)
        .await?
        .ok_or_else(|| VellaError::NotFound(format!("Record #{} not found in '{}'", id, model_name)))?;

    // Run before_delete hooks
    for hook in state.hooks.iter() {
        hook.before_delete(&schema.name, id).await?;
    }

    state.db.delete(schema, id).await?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            &schema.name,
            id,
            "DELETE",
            user.as_ref().map(|u| u.id),
            user.as_ref().map(|u| u.username.as_str()),
            &Value::Null,
            &old_record,
            None,
        )
        .await;

    // Publish event for Realtime subscribers
    state.event_bus.publish(SystemEvent::RecordDeleted {
        model: schema.name.clone(),
        id,
    });

    // Run after_delete hooks
    for hook in state.hooks.iter() {
        hook.after_delete(&schema.name, id, &old_record).await?;
    }

    Ok(Json(json!({ "success": true, "message": "Record deleted successfully" })))
}

/// Vector Similarity Search Endpoint (`POST /api/d/:model/search-vector`)
pub async fn search_vector_handler(
    State(state): State<AppState>,
    Path(model_name): Path<String>,
    Json(query): Json<VectorSearchQuery>,
) -> Result<impl IntoResponse, VellaError> {
    let schema = state
        .registry
        .get(&model_name)
        .ok_or_else(|| VellaError::NotFound(format!("Model '{}' not found", model_name)))?;

    let results = state.db.search_vectors(schema, &query).await?;

    Ok(Json(json!({
        "success": true,
        "model": schema.name,
        "count": results.len(),
        "top_k": query.top_k,
        "metric": query.metric,
        "results": results
    })))
}
