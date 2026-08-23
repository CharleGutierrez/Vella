use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::any::Any;
use std::sync::atomic::{AtomicU64, Ordering};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::error;

static PANIC_RECOVERIES: AtomicU64 = AtomicU64::new(0);

pub fn total_panic_recoveries() -> u64 {
    PANIC_RECOVERIES.load(Ordering::Relaxed)
}

/// Creates a custom panic recovery layer that intercepts unhandled panics,
/// isolates the fault, returns a clean JSON 500, and keeps the server 100% resilient.
pub fn panic_recovery_layer() -> CatchPanicLayer<fn(Box<dyn Any + Send + 'static>) -> Response> {
    CatchPanicLayer::custom(handle_panic)
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    PANIC_RECOVERIES.fetch_add(1, Ordering::Relaxed);

    let msg = if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown internal panic".to_string()
    };

    error!("🛡️ [Vella Self-Healing Panic Recovery] Intercepted panic: '{}'. Isolated successfully without process crash.", msg);

    let body = Json(json!({
        "success": false,
        "error": {
            "code": "PANIC_RECOVERED",
            "message": "An internal error occurred, but Vella isolated the failure and maintained uptime.",
            "recovered": true
        }
    }));

    (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
}
