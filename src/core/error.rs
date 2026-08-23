use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Core error type for Vella framework operations.
#[derive(Debug, Error)]
pub enum VellaError {
    #[error("Record or resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("AI Rate Limit Exceeded: {0}")]
    RateLimited(String),

    #[error("Vector operation error: {0}")]
    VectorError(String),

    #[error("Realtime transport error: {0}")]
    RealtimeError(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl VellaError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::VectorError(_) => StatusCode::BAD_REQUEST,
            Self::RealtimeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::Conflict(_) => "CONFLICT",
            Self::RateLimited(_) => "RATE_LIMITED",
            Self::VectorError(_) => "VECTOR_ERROR",
            Self::RealtimeError(_) => "REALTIME_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Internal(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl IntoResponse for VellaError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "success": false,
            "error": {
                "code": self.error_code(),
                "message": self.to_string(),
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for VellaError {
    fn from(err: sqlx::Error) -> Self {
        VellaError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for VellaError {
    fn from(err: serde_json::Error) -> Self {
        VellaError::Validation(err.to_string())
    }
}

impl From<std::io::Error> for VellaError {
    fn from(err: std::io::Error) -> Self {
        VellaError::Internal(err.to_string())
    }
}
