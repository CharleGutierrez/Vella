use crate::api::handlers::AppState;
use crate::core::error::VellaError;
use crate::types::TypeScriptGenerator;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct ExportTypesPayload {
    pub output_path: Option<String>,
}

/// Serve TypeScript definitions directly via HTTP
pub async fn typescript_definitions_handler(
    State(state): State<AppState>,
) -> Response {
    let dts = TypeScriptGenerator::generate_full_definitions(&state.registry);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        dts,
    )
        .into_response()
}

/// Trigger automatic TypeScript export to filesystem
pub async fn export_types_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExportTypesPayload>,
) -> Result<impl IntoResponse, VellaError> {
    let target_path = payload
        .output_path
        .or_else(|| state.config.types_export_path.clone())
        .unwrap_or_else(|| "./frontend/types/vella.d.ts".to_string());

    TypeScriptGenerator::export_to_file(&target_path, &state.registry)
        .map_err(|e| VellaError::Internal(format!("Failed to write types to {}: {}", target_path, e)))?;

    Ok(Json(json!({
        "success": true,
        "exported_to": target_path,
        "models_count": state.registry.len()
    })))
}
