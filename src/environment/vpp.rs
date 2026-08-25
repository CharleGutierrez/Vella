/// Vella Virtual Power Plant (VPP)
/// Aggregates decentralized home batteries and solar panels to stabilize the city grid.
pub struct VirtualPowerPlant {
    city_grid_api: String,
}

impl VirtualPowerPlant {
    pub fn new(city_api: impl Into<String>) -> Self {
        Self {
            city_grid_api: city_api.into(),
        }
    }

    /// Evaluates current city energy demand and automatically buys/sells decentralized battery power
    pub fn execute_grid_arbitrage(&self, grid_demand_megawatts: f64, local_solar_output_megawatts: f64) {
        println!("⚡ [Vella VPP] Monitoring Smart Grid via {}...", self.city_grid_api);
        
        if grid_demand_megawatts > 500.0 && local_solar_output_megawatts > 50.0 {
            println!("🔥 [Vella VPP] CRITICAL GRID STRAIN DETECTED. Preventing blackout.");
            println!("🔋 [Vella VPP] Discharging {} MW from decentralized home batteries to city grid.", local_solar_output_megawatts);
        } else if grid_demand_megawatts < 100.0 {
            println!("☀️ [Vella VPP] Grid demand is low. Charging home batteries using excess renewable solar energy.");
        } else {
            println!("⚖️ [Vella VPP] Grid is stable. Holding battery reserves.");
        }
    }
}
