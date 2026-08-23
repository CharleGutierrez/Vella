use crate::api::handlers::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tracing::info;

/// WebSocket upgrade handler for bidirectional realtime sync
pub async fn realtime_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    state.realtime_hub.increment_ws();
    info!("🔌 [Vella Realtime] New WebSocket client connected");

    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.realtime_hub.subscribe();

    // Spawn task to forward broadcast messages to WebSocket client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json_str) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json_str)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Task to read incoming client messages (e.g. ping/pong, subscribe filters)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(_) => {}
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    state.realtime_hub.decrement_ws();
    info!("🔌 [Vella Realtime] WebSocket client disconnected");
}
