use axum::{response::Html, Router, routing::get};
use tracing::info;

pub fn observability_router() -> Router {
    Router::new().route("/", get(dashboard_html))
}

async fn dashboard_html() -> Html<&'static str> {
    info!("Serving embedded observability dashboard");
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Vella Observability</title>
            <style>
                body { font-family: system-ui, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; }
                .card { background: #1e293b; padding: 1.5rem; border-radius: 0.5rem; margin-bottom: 1rem; }
                h1 { color: #38bdf8; }
            </style>
        </head>
        <body>
            <h1>⚡ Vella OTel Dashboard</h1>
            <div class="card">
                <h3>Live Traces</h3>
                <p>Telemetry metrics and distributed tracing pipeline metrics will appear here.</p>
                <ul id="traces">
                    <li>[200 OK] /api/d/article/search-vector (1.2ms)</li>
                    <li>[503 Service Unavailable] Circuit Breaker Tripped - db_cluster_1</li>
                </ul>
            </div>
            <div class="card">
                <h3>Active Background Jobs</h3>
                <p>No active cron tasks running.</p>
            </div>
        </body>
        </html>
    "#)
}
