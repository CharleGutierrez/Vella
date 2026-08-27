# Vella: Nuclear Reactor Telemetry Dashboard Tutorial

Welcome to the comprehensive guide for building Nuclear Reactor Telemetry Dashboards using the Vella framework. This manual is designed for Software Engineers focused on IoT telemetry, dashboarding, and state machine simulations. 

> [!WARNING]  
> **Safety Notice:** This manual focuses exclusively on software engineering, network patterns, and simulated state machines. It does not contain real-world reactor physics, thermohydraulic equations, or actionable engineering calculations. It is intended for building software simulations and UI dashboards only.

## Table of Contents
1. [Introduction](#introduction)
2. [Reactor Telemetry Simulation](#reactor-telemetry-simulation)
3. [Threshold Safety Logic](#threshold-safety-logic)
4. [High-Frequency Data Ingestion](#high-frequency-data-ingestion)
5. [Code Examples](#code-examples)

---

## 1. Introduction

When building a telemetry dashboard for a simulated nuclear reactor, you need a robust way to generate mock data, evaluate safety thresholds, and ingest high-frequency sensor readings. Vella provides a suite of tools for these specific tasks, allowing you to focus on building responsive and reliable UI dashboards.

## 2. Reactor Telemetry Simulation

To test your dashboard, you need realistic but safe simulated data. Vella includes `src/scada/simulation.rs` to help you build basic software state machines that mock variables like "Core Temperature" and "Coolant Pressure" as they naturally rise and fall over time.

A typical simulation state machine will have:
- **State Variables:** `core_temperature`, `coolant_pressure`, `radiation_level`.
- **Control Variables:** `cooling_rods_active`, `coolant_pump_speed`.
- **Update Loop:** A tick function that slightly alters the state variables based on the control variables to simulate physical processes (e.g., temperature rises if cooling rods are inactive).

## 3. Threshold Safety Logic

Your dashboard and backend should monitor incoming telemetry and react to dangerous conditions. This is done through basic threshold evaluations. 

For example, if the core temperature exceeds a specific safe limit, the system should automatically trigger safety mechanisms within the simulation.

```rust
// Example of basic threshold logic
if state.core_temperature > 1000.0 {
    state.cooling_rods_active = true;
    log::warn!("Core temperature exceeded 1000C. Activating cooling rods.");
}
```

## 4. High-Frequency Data Ingestion

Simulated radiation sensors and other IoT devices can generate a massive amount of data. To handle this high-frequency ingestion, Vella utilizes UDP for low-latency, connectionless data transfer. 

You can use Vella's `src/net/udp.rs` module to set up a UDP listener that rapidly ingests incoming telemetry packets, deserializes them, and updates the shared simulation state or forwards them to the dashboard backend.

## 5. Code Examples

Below is a complete, simplified example demonstrating how to initialize a mock SCADA state machine and start a UDP listener to ingest high-frequency sensor data.

### Initializing the Simulation and Ingestion

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use vella::scada::simulation::{ReactorState, SimulationTick};
use vella::net::udp::UdpTelemetryListener;

fn main() -> std::io::Result<()> {
    // 1. Initialize the mock SCADA state machine
    let state = Arc::new(Mutex::new(ReactorState {
        core_temperature: 800.0,
        coolant_pressure: 15.0,
        cooling_rods_active: false,
    }));

    let simulation_state = Arc::clone(&state);

    // 2. Start the simulation loop in a separate thread
    thread::spawn(move || {
        loop {
            let mut s = simulation_state.lock().unwrap();
            
            // Basic tick logic (mocking temperature changes)
            if !s.cooling_rods_active {
                s.core_temperature += 2.5; // Temp rises
            } else {
                s.core_temperature -= 5.0; // Temp drops rapidly with rods
            }

            // Threshold Safety Logic
            if s.core_temperature > 1000.0 {
                s.cooling_rods_active = true;
                println!("ALERT: Temperature critical! Cooling rods deployed.");
            } else if s.core_temperature < 800.0 {
                s.cooling_rods_active = false;
            }

            // Drop lock before sleeping
            drop(s);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // 3. Start high-frequency UDP data ingestion
    let listener_state = Arc::clone(&state);
    let address = "127.0.0.1:8080";
    
    println!("Starting UDP Telemetry Listener on {}", address);
    let listener = UdpTelemetryListener::bind(address)?;
    
    listener.start(move |packet| {
        // Handle incoming high-frequency data (e.g., radiation sensors)
        // Here we just log the reception in the mock
        println!("Received telemetry packet: {:?}", packet);
        
        // Example: Update state based on packet
        // let mut s = listener_state.lock().unwrap();
        // s.update_from_packet(packet);
    });

    Ok(())
}
```
