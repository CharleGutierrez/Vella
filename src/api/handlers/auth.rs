use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::VellaError;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, VellaError> {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match state.auth_service.login(&payload.username, &payload.password, ip, ua).await? {
        Some(session) => {
            let cookie_header = format!(
                "vella_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
                session.token
            );
            let mut res_headers = HeaderMap::new();
            res_headers.insert("Set-Cookie", cookie_header.parse().unwrap());

            Ok((
                StatusCode::OK,
                res_headers,
                Json(json!({
                    "success": true,
                    "session": session
                })),
            ))
        }
        None => Err(VellaError::Unauthorized("Invalid username/email or password".to_string())),
    }
}

pub async fn logout_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, VellaError> {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let _ = state.auth_service.logout(&auth_str[7..]).await;
            }
        }
    }
    let cookie_header = "vella_session=; Path=/; HttpOnly; Max-Age=0";
    let mut res_headers = HeaderMap::new();
    res_headers.insert("Set-Cookie", cookie_header.parse().unwrap());

    Ok((
        StatusCode::OK,
        res_headers,
        Json(json!({ "success": true })),
    ))
}

pub async fn me_handler(
    OptionalAuthUser(user_opt): OptionalAuthUser,
) -> Result<impl IntoResponse, VellaError> {
    match user_opt {
        Some(user) => Ok(Json(json!({ "success": true, "user": user }))),
        None => Err(VellaError::Unauthorized("Not authenticated".to_string())),
    }
}
