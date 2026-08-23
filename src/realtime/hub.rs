use crate::core::events::{EventBus, SystemEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};
use futures_util::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMessage {
    pub topic: String,
    pub event: String,
    pub payload: Value,
    pub timestamp: String,
}

/// Central Realtime Hub: Manages WebSocket & SSE client streams and distributes events
#[derive(Debug, Clone)]
pub struct RealtimeHub {
    broadcaster: broadcast::Sender<RealtimeMessage>,
    active_ws_clients: Arc<AtomicU64>,
    active_sse_clients: Arc<AtomicU64>,
}

impl Default for RealtimeHub {
    fn default() -> Self {
        Self::new(2048)
    }
}

impl RealtimeHub {
    pub fn new(capacity: usize) -> Self {
        let (broadcaster, _) = broadcast::channel(capacity);
        Self {
            broadcaster,
            active_ws_clients: Arc::new(AtomicU64::new(0)),
            active_sse_clients: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start listening to system event bus and forward events to realtime subscribers
    pub fn start_event_bridge(&self, event_bus: &EventBus, redis_url: Option<String>) {
        let mut rx = event_bus.subscribe();
        let tx = self.broadcaster.clone();
        let tx_redis = self.broadcaster.clone();

        if let Some(url) = redis_url {
            // Enterprise Multi-Node Redis Backplane
            tokio::spawn(async move {
                match redis::Client::open(url) {
                    Ok(client) => {
                        let mut pub_conn = client.get_multiplexed_async_connection().await.unwrap();
                        let mut pubsub = client.get_async_pubsub().await.unwrap();
                        if pubsub.subscribe("vella_realtime_events").await.is_ok() {
                            info!("🌍 [Vella Enterprise] Connected to Redis Pub/Sub Backplane for Multi-Node Realtime");

                            // Task 1: Listen to local EventBus -> Publish to Redis
                            tokio::spawn(async move {
                                while let Ok(event) = rx.recv().await {
                                    if let Some(msg) = Self::system_event_to_message(event) {
                                        let json_str = serde_json::to_string(&msg).unwrap_or_default();
                                        let _: redis::RedisResult<()> = redis::cmd("PUBLISH")
                                            .arg("vella_realtime_events")
                                            .arg(json_str)
                                            .query_async(&mut pub_conn)
                                            .await;
                                    }
                                }
                            });

                            // Task 2: Listen to Redis -> Broadcast to local WebSockets
                            let mut stream = pubsub.on_message();
                            while let Some(msg) = stream.next().await {
                                if let Ok(payload) = msg.get_payload::<String>() {
                                    if let Ok(rm) = serde_json::from_str::<RealtimeMessage>(&payload) {
                                        let _ = tx_redis.send(rm);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => error!("❌ Failed to connect to Redis Backplane: {}", e),
                }
            });
        } else {
            // Standard Single-Node Local Broadcaster
            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    if let Some(msg) = Self::system_event_to_message(event) {
                        let _ = tx.send(msg);
                    }
                }
            });
            info!("📡 [Vella Realtime Hub] Realtime event bridge active (Single-Node Local Mode)");
        }
    }

    fn system_event_to_message(event: SystemEvent) -> Option<RealtimeMessage> {
        match event {
            SystemEvent::RecordCreated { ref model, id, ref data } => Some(RealtimeMessage {
                topic: format!("models:{}", model.to_lowercase()),
                event: "CREATE".to_string(),
                payload: serde_json::json!({ "model": model, "id": id, "data": data }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::RecordUpdated { ref model, id, ref changes } => Some(RealtimeMessage {
                topic: format!("models:{}", model.to_lowercase()),
                event: "UPDATE".to_string(),
                payload: serde_json::json!({ "model": model, "id": id, "changes": changes }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::RecordDeleted { ref model, id } => Some(RealtimeMessage {
                topic: format!("models:{}", model.to_lowercase()),
                event: "DELETE".to_string(),
                payload: serde_json::json!({ "model": model, "id": id }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::ApprovalRequested { approval_id, ref model, record_id } => Some(RealtimeMessage {
                topic: "approvals".to_string(),
                event: "REQUEST".to_string(),
                payload: serde_json::json!({ "approval_id": approval_id, "model": model, "record_id": record_id }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::ApprovalResolved { approval_id, approved } => Some(RealtimeMessage {
                topic: "approvals".to_string(),
                event: "RESOLVE".to_string(),
                payload: serde_json::json!({ "approval_id": approval_id, "approved": approved }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::RollbackExecuted { log_id, ref model, record_id } => Some(RealtimeMessage {
                topic: format!("models:{}", model.to_lowercase()),
                event: "ROLLBACK".to_string(),
                payload: serde_json::json!({ "log_id": log_id, "model": model, "record_id": record_id }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            SystemEvent::AiPromptLogged { ref model_used, prompt_tokens, completion_tokens, latency_ms } => Some(RealtimeMessage {
                topic: "ai:telemetry".to_string(),
                event: "PROMPT_LOGGED".to_string(),
                payload: serde_json::json!({ "model": model_used, "prompt_tokens": prompt_tokens, "completion_tokens": completion_tokens, "latency_ms": latency_ms }),
                timestamp: chrono::Utc::now().to_rfc3339(),
            }),
            _ => None,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RealtimeMessage> {
        self.broadcaster.subscribe()
    }

    pub fn increment_ws(&self) {
        self.active_ws_clients.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_ws(&self) {
        self.active_ws_clients.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn increment_sse(&self) {
        self.active_sse_clients.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_sse(&self) {
        self.active_sse_clients.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn stats_json(&self) -> Value {
        serde_json::json!({
            "active_websocket_connections": self.active_ws_clients.load(Ordering::Relaxed),
            "active_sse_streams": self.active_sse_clients.load(Ordering::Relaxed),
            "realtime_engine": "Native Tokio Broadcast"
        })
    }
}
