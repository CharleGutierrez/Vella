# Vella Framework: SCADA & IoT Hardware Engineering Manual

Welcome to the **Vella SCADA & IoT Hardware Engineering Manual**. This detailed tutorial is written specifically for hardware engineers, systems integrators, and industrial IoT developers who need to utilize Vella’s ultra-low latency ingestion, autonomous state-machine processing, and AI-driven telemetry tuning.

---

## 1. Industrial Telemetry Simulation (`src/scada/simulation.rs`)

In critical industrial environments (e.g., Oil & Gas refineries, Nuclear facility cooling loops), state management must be deterministic and instantaneous. Vella implements a highly optimized **PID-style state machine** in `src/scada/simulation.rs`.

### How It Works:
- **Autonomous Monitoring:** The simulation module continuously loops over hardware telemetry, focusing on high-stakes metrics like **Temperature** and **Pressure**.
- **Threshold-Based Relief Valves:** When pressure or temperature readings exceed safe operational thresholds, the state machine bypasses standard control loops and immediately triggers relief valves.
- **PID-Style Feedback:** The control loop mimics Proportional-Integral-Derivative (PID) controllers to ease the system back into safe states without oscillating wildly.

---

## 2. Ultra-Low Latency UDP Ingestion (`src/net/udp.rs`)

When monitoring high-frequency hardware metrics—such as Formula One engine RPMs or real-time Tire Temperatures—TCP handshakes introduce unacceptable latency. 

Vella solves this with its `src/net/udp.rs` module, built entirely around raw `UdpSocket` buffering.

### Key Features:
- **Zero-Handshake Ingestion:** Bypasses TCP connection overhead, dropping straight to datagram packet processing.
- **High-Throughput Ring Buffers:** Reads network packets directly into pre-allocated memory buffers, avoiding expensive memory allocation per packet.
- **Instantaneous Processing:** Ideal for capturing bursts of sensor data up to 1000+ Hz, ensuring you never miss a critical anomaly.

---

## 3. AI Tuner Hardware Integration (`src/ai/tuner.rs`)

Modern SCADA systems shouldn't be statically tuned. Vella’s `src/ai/tuner.rs` introduces dynamic edge intelligence by directly monitoring the host hardware and adjusting parameters on the fly.

### Dynamic Adjustments:
- **Hardware Telemetry:** Utilizing the `sysinfo` crate, Vella tracks the physical host’s **CPU load, RAM usage, and Disk I/O**.
- **Adaptive Thresholds:** If the edge device experiences high CPU load or memory pressure, the AI tuner automatically throttles non-critical telemetry ingestion rates or widens safe-state tolerance bands to preserve system stability and prioritize relief valve logic.

---

## 4. Code Examples

To get started immediately, you can leverage Vella's built-in examples. Feel free to copy, paste, and run these in your test environments.

### Example A: Running the SCADA Telemetry State Machine
**File:** `examples/test_scada.rs`
```rust
use vella::scada::simulation::{ScadaSystem, SensorData};
use vella::ai::tuner::AiTuner;

fn main() {
    println!("Initializing SCADA Telemetry Simulation...");
    let mut system = ScadaSystem::new();
    let tuner = AiTuner::new();

    // Simulate incoming readings from a refinery cooling loop
    let current_reading = SensorData {
        temperature: 125.5, // Celsius
        pressure: 3450.0,   // PSI
    };

    // The tuner adjusts thresholds based on host machine CPU/RAM load
    let dynamic_threshold = tuner.calculate_safe_thresholds();
    
    // The PID-style state machine evaluates the reading
    if let Some(action) = system.evaluate(current_reading, dynamic_threshold) {
        println!("ACTION TRIGGERED: {:?}", action);
        // E.g., triggers relief valve if pressure > 3400 PSI
    } else {
        println!("System stable.");
    }
}
```

### Example B: High-Frequency UDP F1 Ingestion
**File:** `examples/test_f1.rs`
```rust
use vella::net::udp::UdpTelemetryServer;
use std::net::SocketAddr;

fn main() -> std::io::Result<()> {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("Starting Ultra-Low Latency UDP Ingestion on {}", addr);
    
    let mut server = UdpTelemetryServer::bind(addr)?;
    
    // Listen for high-frequency packets (e.g., Engine RPM, Tire Temps)
    server.listen(|packet| {
        // Fast-path parsing directly from the buffer
        let rpm = u32::from_be_bytes(packet[0..4].try_into().unwrap());
        let tire_temp = f32::from_be_bytes(packet[4..8].try_into().unwrap());
        
        println!("F1 Telemetry -> RPM: {}, Tire Temp: {:.1}C", rpm, tire_temp);
    })
}
```

---

*This document is maintained by the Vella Core Engineering Team. For questions or support on custom hardware integration, please refer to the internal SCADA support channel.*
