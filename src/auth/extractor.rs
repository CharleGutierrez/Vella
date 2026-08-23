use crate::api::handlers::AppState;
use crate::auth::rbac::AuthUser;
use crate::core::error::VellaError;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};

/// Axum extractor for requiring an authenticated user in a route handler
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub AuthUser);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = VellaError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let user_opt = extract_user_from_parts(parts, state).await?;
        match user_opt {
            Some(u) => Ok(AuthenticatedUser(u)),
            None => Err(VellaError::Unauthorized("Authentication required".to_string())),
        }
    }
}

/// Axum extractor for optionally authenticated routes
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = VellaError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let user_opt = extract_user_from_parts(parts, state).await?;
        Ok(OptionalAuthUser(user_opt))
    }
}

async fn extract_user_from_parts(parts: &Parts, state: &AppState) -> Result<Option<AuthUser>, VellaError> {
    // 1. Check Bearer token or API key
    if let Some(auth_header) = parts.headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                return state.auth_service.validate_session(token).await;
            }
        }
    }

    // 2. Check Cookie
    if let Some(cookie_header) = parts.headers.get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for pair in cookie_str.split(';') {
                let parts: Vec<&str> = pair.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == "vella_session" {
                    return state.auth_service.validate_session(parts[1]).await;
                }
            }
        }
    }

    Ok(None)
}
