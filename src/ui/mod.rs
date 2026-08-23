pub mod angular_sdk;
pub mod assets;
pub mod react_sdk;
pub mod todo_showcase;
pub mod vue_sdk;

use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct UiConfig {
    pub site_name: String,
    pub base_url: String,
}

pub async fn admin_ui_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        assets::admin_react_spa_html(&config.site_name),
    )
        .into_response()
}

pub async fn todo_showcase_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        todo_showcase::todo_showcase_html(&config.site_name),
    )
        .into_response()
}

pub async fn react_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = react_sdk::generate_react_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}

pub async fn vue_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = vue_sdk::generate_vue_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}

pub async fn angular_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = angular_sdk::generate_angular_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}
