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

    // --- REAL OAUTH INTEGRATION ---
    // In a real flow, 'oauth_id' would be an 'access_token' or 'code'
    // For Vella's real security mode, we verify the token with the provider
    let (verified_id, verified_email, verified_username) = if payload.oauth_id.starts_with("ya29.") || payload.oauth_id.starts_with("gho_") {
        if payload.provider == "google" {
            if let Some(profile) = crate::auth::oauth_verify::verify_google_token(&payload.oauth_id).await {
                (profile.id, profile.email, profile.name)
            } else {
                return Err(VellaError::Unauthorized("Invalid Google Access Token".to_string()));
            }
        } else if payload.provider == "github" {
             if let Some(profile) = crate::auth::oauth_verify::verify_github_token(&payload.oauth_id).await {
                (profile.id, profile.email, profile.name)
            } else {
                return Err(VellaError::Unauthorized("Invalid GitHub Access Token".to_string()));
            }
        } else {
            (payload.oauth_id.clone(), payload.email.clone(), payload.username.clone().unwrap_or(payload.email.split('@').next().unwrap_or("user").to_string()))
        }
    } else {
        // Fallback for mocked/dev mode
        let username = payload.username.unwrap_or_else(|| {
            payload.email.split('@').next().unwrap_or("user").to_string()
        });
        (payload.oauth_id.clone(), payload.email.clone(), username)
    };

    let session = state
        .oauth_service
        .handle_oauth_login(
            &state.auth_service,
            &payload.provider,
            &verified_id,
            &verified_email,
            &verified_username,
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
