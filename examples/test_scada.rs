use vella::scada::simulation::ScadaSimulation;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Vella SCADA Simulation: Oil/Gas Refinery & Nuclear Reactor ---");
    let mut sim = ScadaSimulation::new();

    for i in 1..=10 {
        println!("\nTick {}", i);
        sim.tick();
        thread::sleep(Duration::from_millis(100));
    }

    println!("\nSimulation completed successfully.");
}
