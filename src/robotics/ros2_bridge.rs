use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{error, info};

/// Native ROS2 (Robot Operating System) Bridge via rosbridge v2 protocol
pub struct Ros2Bridge {
    active_topics: HashSet<String>,
    tx_channel: Option<mpsc::Sender<Message>>,
}

impl Default for Ros2Bridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Ros2Bridge {
    pub fn new() -> Self {
        Self {
            active_topics: HashSet::new(),
            tx_channel: None,
        }
    }

    /// Connects to a rosbridge_server (e.g., ws://localhost:9090)
    pub async fn connect(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Connecting to ROS2 rosbridge at {}...", url);
        let (ws_stream, _) = connect_async(url).await?;
        info!("Successfully connected to ROS2.");

        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        self.tx_channel = Some(tx);

        // Background write task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("Error sending to ROS2 bridge: {}", e);
                    break;
                }
            }
        });

        // Background read task
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    // Here you would parse the incoming ROS2 message
                    info!("Received from ROS2: {}", text);
                }
            }
        });

        Ok(())
    }

    /// Subscribes to a ROS2 topic
    pub async fn subscribe(&mut self, topic: &str, msg_type: &str) -> Result<(), String> {
        self.active_topics.insert(topic.to_string());
        
        if let Some(tx) = &self.tx_channel {
            let msg = json!({
                "op": "subscribe",
                "topic": topic,
                "type": msg_type
            });
            tx.send(Message::Text(msg.to_string().into())).await.map_err(|e| e.to_string())?;
            info!("Subscribed to ROS2 topic: {}", topic);
            Ok(())
        } else {
            Err("Not connected to ROS2 bridge".to_string())
        }
    }

    /// Publishes a JSON payload to a ROS2 topic
    pub async fn publish(&self, topic: &str, payload: Value) -> Result<(), String> {
        if !self.active_topics.contains(topic) {
            return Err("Topic not registered".to_string());
        }

        if let Some(tx) = &self.tx_channel {
            let msg = json!({
                "op": "publish",
                "topic": topic,
                "msg": payload
            });
            tx.send(Message::Text(msg.to_string().into())).await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Not connected to ROS2 bridge".to_string())
        }
    }
}
