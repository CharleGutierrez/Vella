use vella::net::UdpTelemetryListener;
use vella::net::SharedMemoryRingBuffer;
use vella::db::TimeSeriesAdapter;
use vella::core::RtosIsolator;
use vella::compute::MpiClusterManager;
use vella::ai::AiTuner;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

#[test]
fn test_udp_telemetry_listener() {
    let listener = UdpTelemetryListener::new("0.0.0.0:8000");
    let result = listener.listen_for_telemetry();
    
    assert!(result.is_ok(), "UDP Listener failed to initialize");
}

#[test]
fn test_timeseries_downsampling() {
    let tuner = Arc::new(AiTuner::new());
    let ts = TimeSeriesAdapter::new("TimescaleDB", tuner);
    
    // Normal latency (10ms) uses base 100ms bucket
    let query_fast = ts.query_downsampled_bucket("tire_temp_fl", 100, 10);
    assert!(query_fast.contains("time_bucket('100 milliseconds'"), "Missing normal time_bucket interval");
    
    // High latency (250ms) triggers AI to widen bucket to 500ms
    let query_slow = ts.query_downsampled_bucket("tire_temp_fl", 100, 250);
    assert!(query_slow.contains("time_bucket('500 milliseconds'"), "AI failed to stretch bucket interval");
}

#[test]
fn test_rtos_hard_realtime_isolation() {
    let executed = Arc::new(AtomicBool::new(false));
    let executed_clone = executed.clone();
    
    RtosIsolator::spawn_hard_realtime_task("Brake_By_Wire_Controller", move || {
        executed_clone.store(true, Ordering::SeqCst);
    });
    
    // Give the raw thread a tiny moment to spin
    thread::sleep(Duration::from_millis(10));
    
    assert!(executed.load(Ordering::SeqCst), "Hard real-time OS thread failed to execute");
}

#[test]
fn test_mpi_cfd_cluster_synchronization() {
    let mpi = MpiClusterManager::new(1024);
    let result = mpi.execute_cfd_simulation("front_wing_mesh_v2");
    
    assert_eq!(result.unwrap(), "CFD Simulation Converged", "MPI cluster failed to converge CFD mesh");
}

#[test]
fn test_1000hz_ipc_shared_memory() {
    let ipc = SharedMemoryRingBuffer::new();
    
    // Simulating writing a physics frame (e.g. suspension load)
    ipc.write_physics_frame(123456789);
    
    let read_frame = ipc.read_latest_frame();
    assert_eq!(read_frame, 123456789, "IPC Ring Buffer corrupted memory frame");
}
