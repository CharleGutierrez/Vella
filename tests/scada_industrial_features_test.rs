use vella::scada::{ScadaDriver, IndustrialProtocol, Isa18Alarm, AlarmState, SwingingDoorCompressor};
use vella::core::TmrVoter;
use vella::ui::hmi::HmiCanvasBuilder;
use vella::ai::AiTuner;
use std::sync::Arc;

#[tokio::test]
async fn test_industrial_protocol_driver() {
    let driver = ScadaDriver::new(IndustrialProtocol::ModbusTcp { 
        ip: "192.168.1.50".to_string(), 
        port: 502 
    });
    
    // Now returns a Result indicating a real connection attempt (which will time out or refuse since IP is fake)
    let res = driver.read_holding_register(40001).await;
    assert!(res.is_err(), "Modbus driver should fail to connect to fake IP");
    let err_msg = res.unwrap_err();
    assert!(err_msg.contains("timed out") || err_msg.contains("refused") || err_msg.contains("unreachable"), "Unexpected error: {}", err_msg);
}

#[test]
fn test_isa18_alarm_state_machine() {
    let mut alarm = Isa18Alarm::new("PUMP_101_TEMP");
    assert_eq!(alarm.state, AlarmState::Normal);
    
    // Sensor breaches safe limit
    alarm.trigger_breach();
    assert_eq!(alarm.state, AlarmState::UnackActive);
    
    // Operator sees it and acknowledges it
    alarm.operator_acknowledge();
    assert_eq!(alarm.state, AlarmState::AckActive);
    
    // Physical temp drops back to normal
    alarm.trigger_clear();
    assert_eq!(alarm.state, AlarmState::Normal);
}

#[test]
fn test_swinging_door_compression() {
    let tuner = Arc::new(AiTuner::new());
    let mut compressor = SwingingDoorCompressor::new(0.5, tuner);
    
    // Initial value is always archived
    assert_eq!(compressor.process_signal(100.0, 50.0), Some(100.0));
    
    // Value barely moves, should be dropped
    assert_eq!(compressor.process_signal(100.2, 50.0), None);
    assert_eq!(compressor.process_signal(100.4, 50.0), None);
    
    // Value breaks deviation geometry, should be archived
    assert_eq!(compressor.process_signal(101.5, 50.0), Some(101.5));
    
    // High disk usage (90%) triggers AI to double the threshold (from 0.5 to 1.0)
    // So moving from 101.5 to 102.0 (diff 0.5) is now dropped!
    assert_eq!(compressor.process_signal(102.0, 90.0), None);
}

#[test]
fn test_triple_modular_redundancy() {
    // All 3 agree
    assert_eq!(TmrVoter::execute_hardware_vote(10, 10, 10).unwrap(), 10);
    
    // Node C diverges
    assert_eq!(TmrVoter::execute_hardware_vote(10, 10, 99).unwrap(), 10);
    
    // All 3 diverge (Critical Failure)
    assert!(TmrVoter::execute_hardware_vote(10, 20, 30).is_err());
}

#[test]
fn test_hmi_canvas_binding() {
    let hmi = HmiCanvasBuilder::new();
    let json_bind = hmi.bind_svg_to_telemetry_tag("tank_svg_element", "TAG_FLUID_LVL");
    
    assert_eq!(json_bind["element_id"], "tank_svg_element");
    assert_eq!(json_bind["animation_type"], "fill_level");
}
