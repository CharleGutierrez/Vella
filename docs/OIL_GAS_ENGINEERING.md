# Vella Framework: Oil & Gas Engineering Manual

Welcome to the **Vella Framework** manual specifically designed for Oil & Gas Engineers. The Vella framework provides high-performance, low-latency tools perfect for the energy sector, allowing engineers to build robust SCADA (Supervisory Control and Data Acquisition) systems, process telemetry, and implement autonomous safety mechanisms.

This guide will walk you through applying Vella to real-world energy sector challenges.

---

## 1. Refinery & Pipeline Telemetry Simulation

In modern refineries and pipeline networks, monitoring temperature and pressure across vast distances is critical. Using Vella's `src/scada/simulation.rs`, you can easily model and monitor these parameters across drilling rigs or refinery pipelines.

The simulation engine allows you to generate synthetic telemetry data that mimics real-world conditions, helping you test your control systems before deployment. You can define entities like "Pump Station Alpha" or "Pipeline Segment 4" and stream their simulated metrics into your processing pipeline.

## 2. Threshold Logic & Relief Valves

Safety is paramount in the energy sector. Vella's PID-style state machine autonomously evaluates critical metrics to prevent catastrophic failures. 

For example, you can configure the system to monitor pressure build-ups. If a pipeline segment's pressure crosses a critical threshold (e.g., 150 PSI), the state machine programmatically triggers safety protocols, such as opening relief valves or initiating cooling loops, without requiring human intervention.

## 3. Remote Rig Data Ingestion

Offshore rigs and remote drilling sites often suffer from intermittent and high-latency network connections. In these environments, traditional TCP connections (with their required handshakes) can fail or cause unacceptable delays.

Vella solves this with its ultra-low latency UDP listener (`src/net/udp.rs`). By deploying this module, you can ingest fire-and-forget telemetry from remote sensors. This ensures that critical data packets (like sudden pressure spikes) are received and processed as quickly as possible, even in degraded network conditions.

---

## 4. Code Examples

Below are concrete Rust code examples demonstrating how to use Vella for common Oil & Gas scenarios.

### Example 1: Pipeline Pressure Monitoring

This example demonstrates how to set up a SCADA simulation to monitor pipeline pressure and trigger a relief valve if it exceeds 150 PSI.

```rust
use vella::scada::simulation::{Simulator, Metric};
use vella::control::Threshold;

fn main() {
    let mut simulator = Simulator::new();
    
    // Simulate a pipeline pressure sensor
    simulator.add_sensor("Pipeline_Segment_4", Metric::Pressure);
    
    // Define the safety threshold
    let relief_valve_trigger = Threshold::new(150.0);
    
    simulator.on_update(|data| {
        if data.sensor == "Pipeline_Segment_4" && relief_valve_trigger.is_exceeded(data.value) {
            println!("CRITICAL: Pressure exceeded 150 PSI! Triggering relief valve...");
            // Trigger relief valve logic here
        } else {
            println!("Pipeline_Segment_4 Pressure: {} PSI - Nominal", data.value);
        }
    });
    
    simulator.run();
}
```

### Example 2: Drill Bit Temperature and Remote Ingestion

This example sets up a UDP listener to ingest drill bit temperature data from a remote offshore rig.

```rust
use vella::net::udp::UdpListener;

fn main() -> std::io::Result<()> {
    // Bind UDP listener for remote telemetry ingestion
    let mut listener = UdpListener::bind("0.0.0.0:8080")?;
    
    println!("Listening for remote rig telemetry on port 8080...");
    
    listener.on_packet_received(|packet| {
        // Parse the incoming fire-and-forget telemetry packet
        let telemetry_data = String::from_utf8_lossy(&packet.payload);
        
        // Example payload: "Drill_Bit_1:Temp:210F"
        if telemetry_data.contains("Temp") {
            let temp_val: f64 = extract_temperature(&telemetry_data);
            if temp_val > 250.0 {
                println!("WARNING: Drill bit temperature too high: {}F", temp_val);
                // Initiate cooling loop logic
            }
        }
    });
    
    listener.start();
    Ok(())
}

fn extract_temperature(data: &str) -> f64 {
    // Dummy parsing logic
    210.0
}
```

These examples can be adapted and expanded to build out a complete, high-performance monitoring and control system for your Oil & Gas infrastructure.
