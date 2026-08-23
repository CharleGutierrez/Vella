use crate::api::handlers::AppState;
use crate::core::error::VellaError;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct MagicLinkRequestPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct MagicLinkVerifyPayload {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackPayload {
    pub provider: String,
    pub oauth_id: String,
    pub email: String,
    pub username: Option<String>,
}

pub async fn google_oauth_url_handler() -> impl IntoResponse {
    let url = crate::auth::OAuthService::get_google_auth_url("google-client-id", "/api/auth/oauth/callback");
    Json(json!({ "success": true, "provider": "google", "auth_url": url }))
}

pub async fn github_oauth_url_handler() -> impl IntoResponse {
    let url = crate::auth::OAuthService::get_github_auth_url("github-client-id", "/api/auth/oauth/callback");
    Json(json!({ "success": true, "provider": "github", "auth_url": url }))
}

pub async fn oauth_callback_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OAuthCallbackPayload>,
) -> Result<impl IntoResponse, VellaError> {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let username = payload.username.unwrap_or_else(|| {
        payload.email.split('@').next().unwrap_or("user").to_string()
    });

    let session = state
        .oauth_service
        .handle_oauth_login(
            &state.auth_service,
            &payload.provider,
            &payload.oauth_id,
            &payload.email,
            &username,
            ip,
            ua,
        )
        .await?;

    let cookie_header = format!(
        "vella_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
        session.token
    );
    let mut res_headers = HeaderMap::new();
    res_headers.insert("Set-Cookie", cookie_header.parse().unwrap());

    Ok((
        StatusCode::OK,
        res_headers,
        Json(json!({ "success": true, "session": session })),
    ))
}

pub async fn magic_link_request_handler(
    State(state): State<AppState>,
    Json(payload): Json<MagicLinkRequestPayload>,
) -> Result<impl IntoResponse, VellaError> {
    if !payload.email.contains('@') {
        return Err(VellaError::Validation("Invalid email address".to_string()));
    }

    let token = state.oauth_service.request_magic_link(&payload.email).await?;

    Ok(Json(json!({
        "success": true,
        "message": format!("Magic login link dispatched for {}", payload.email),
        "preview_token": token,
        "preview_login_url": format!("/api/auth/magic-link/verify?token={}", token)
    })))
}

pub async fn magic_link_verify_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<MagicLinkVerifyPayload>,
) -> Result<impl IntoResponse, VellaError> {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match state.oauth_service.verify_magic_link(&state.auth_service, &payload.token, ip, ua).await? {
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
                Json(json!({ "success": true, "session": session })),
            ))
        }
        None => Err(VellaError::Unauthorized("Invalid or expired magic login link".to_string())),
    }
}
