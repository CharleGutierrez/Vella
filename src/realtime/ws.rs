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
    info!("?? [Vella Realtime] New WebSocket client connected");

    let (mut sender, mut receiver) = socket.split();
    
    // Subscribe to BOTH standard Realtime JSON messages AND raw SystemEvents (for binary CRDTs)
    let mut json_rx = state.realtime_hub.subscribe();
    let mut raw_rx = state.event_bus.subscribe();

    // Spawn task to forward broadcast messages to WebSocket client
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg) = json_rx.recv() => {
                    if let Ok(json_str) = serde_json::to_string(&msg) {
                        if sender.send(Message::Text(json_str)).await.is_err() {
                            break;
                        }
                    }
                }
                Ok(raw_msg) = raw_rx.recv() => {
                    if let crate::core::events::SystemEvent::CrdtSyncMessage { room: _, data } = raw_msg {
                        // Forward Yjs binary updates natively
                        if sender.send(Message::Binary(data)).await.is_err() {
                            break;
                        }
                    }
                }
                else => break,
            }
        }
    });

    let event_bus = state.event_bus.clone();
    
    // Task to read incoming client messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(_) => {}
                Message::Binary(data) => {
                    // When a Yjs client sends an update, broadcast it to all other clients via the event bus!
                    event_bus.publish(crate::core::events::SystemEvent::CrdtSyncMessage {
                        room: "global".to_string(),
                        data,
                    });
                }
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
    info!("?? [Vella Realtime] WebSocket client disconnected");
}
