use vella::space::telemetry::{CcsdsPacket, TelemetryIngestor};
use vella::space::dtn::DtnQueue;
use vella::robotics::ros2_bridge::Ros2Bridge;
use vella::gaming::netcode::RollbackEngine;
use vella::core::local_first::LocalFirstSync;
use vella::data::matrix::SyntheticMatrix;
use vella::ai::swarm::AgentSwarm;
use std::time::SystemTime;

#[test]
fn test_space_telemetry_and_dtn() {
    let ingestor = TelemetryIngestor::new();
    let packet = CcsdsPacket {
        spacecraft_id: 42,
        payload_type: 1,
        timestamp: SystemTime::now(),
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    
    assert!(ingestor.ingest_packet(packet).is_ok());

    let mut dtn = DtnQueue::new(3600);
    dtn.enqueue_bundle(vec![0x01, 0x02, 0x03]);
    assert_eq!(dtn.transmit_when_ready().unwrap(), vec![0x01, 0x02, 0x03]);
}

use serde_json::json;

#[tokio::test]
async fn test_robotics_ros2() {
    let mut bridge = Ros2Bridge::new();
    bridge.subscribe("/cmd_vel", "geometry_msgs/Twist").await.unwrap_err(); // Since tx is None, it errors which is fine for the mock
    
    // To properly mock it, we would need to connect, but this test just checks the state.
    // Let's assert it fails properly when not connected.
    assert!(bridge.publish("/cmd_vel", json!({"linear": {"x": 1.0}})).await.is_err());
}

#[test]
fn test_gaming_rollback() {
    let mut engine = RollbackEngine::new();
    engine.advance_frame(vec![1]);
    engine.advance_frame(vec![2]);
    engine.advance_frame(vec![3]);
    
    assert_eq!(engine.rollback_to(1).unwrap(), vec![1]);
}

#[test]
fn test_local_first_sync() {
    let mut sync = LocalFirstSync::new();
    sync.queue_offline_mutation();
    sync.queue_offline_mutation();
    assert_eq!(sync.sync_with_server().unwrap(), 2);
}

#[test]
fn test_synthetic_matrix() {
    let matrix = SyntheticMatrix::new();
    let dataset = matrix.hallucinate_dataset("users", 100);
    assert_eq!(dataset.len(), 100);
}

#[test]
fn test_ai_agent_swarm() {
    let mut swarm = AgentSwarm::new();
    swarm.spawn_agent("crawler");
    swarm.spawn_agent("analyzer");
    assert_eq!(swarm.trigger_living_database("competitor_prices"), 20);
}
