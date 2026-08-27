use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ScadaSimulation {
    pub temperature: f64,
    pub pressure: f64,
    pub cooling_active: bool,
    pub release_valve_open: bool,
}

impl ScadaSimulation {
    pub fn new() -> Self {
        Self {
            temperature: 50.0,
            pressure: 100.0,
            cooling_active: false,
            release_valve_open: false,
        }
    }

    pub fn tick(&mut self) {
        // Natural increase if not cooled
        if !self.cooling_active {
            self.temperature += 15.0;
        } else {
            self.temperature -= 10.0;
        }

        if !self.release_valve_open {
            self.pressure += 20.0;
        } else {
            self.pressure -= 25.0;
        }

        // Safety logic (Threshold based loop)
        if self.temperature > 80.0 && !self.cooling_active {
            self.cooling_active = true;
            println!("[SCADA] WARNING: High Temperature {:.2}C. Activating Cooling System.", self.temperature);
        } else if self.temperature < 40.0 && self.cooling_active {
            self.cooling_active = false;
            println!("[SCADA] INFO: Temperature normalized {:.2}C. Deactivating Cooling System.", self.temperature);
        }

        if self.pressure > 150.0 && !self.release_valve_open {
            self.release_valve_open = true;
            println!("[SCADA] CRITICAL: High Pressure {:.2} PSI. Opening Release Valve.", self.pressure);
        } else if self.pressure < 90.0 && self.release_valve_open {
            self.release_valve_open = false;
            println!("[SCADA] INFO: Pressure normalized {:.2} PSI. Closing Release Valve.", self.pressure);
        }

        println!("[SCADA] Status | Temp: {:.2} (Cooling: {}) | Pressure: {:.2} (Valve: {})",
            self.temperature, self.cooling_active, self.pressure, self.release_valve_open);
    }
}
