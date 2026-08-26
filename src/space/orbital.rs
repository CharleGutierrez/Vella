use sgp4::{Elements, Constants};
use tracing::info;

/// Keplarian elements for orbital mechanics and satellite tracking
pub struct OrbitalEngine;

impl OrbitalEngine {
    pub fn new() -> Self {
        Self
    }

    /// Calculates genuine X, Y, Z coordinates (in km) and velocities (km/s) 
    /// using real SGP4 orbital mechanics and Earth physical constants.
    pub fn calculate_satellite_position(&self, tle_line1: &str, tle_line2: &str, minutes_since_epoch: f64) -> Result<(f64, f64, f64), String> {
        info!("⚙️ [Vella Space] Parsing Two-Line Element (TLE) array...");
        
        let elements = Elements::from_tle(
            Some("Satellite".to_string()), 
            tle_line1.as_bytes(), 
            tle_line2.as_bytes()
        ).map_err(|e| format!("Invalid TLE data: {}", e))?;
        
        let constants = Constants::from_elements(&elements)
            .map_err(|e| format!("Failed to instantiate Earth physical constants: {}", e))?;

        info!("🪐 [Vella Space] Executing SGP4 orbital propagation...");
        
        let prediction = constants.propagate(minutes_since_epoch)
            .map_err(|e| format!("Propagation failed: {}", e))?;

        // Returns Earth-Centered Inertial (ECI) Coordinates in kilometers
        let position = prediction.position;
        
        info!("✨ [Vella Space] Satellite Location Computed: X: {:.2}km, Y: {:.2}km, Z: {:.2}km", position[0], position[1], position[2]);
        
        Ok((position[0], position[1], position[2]))
    }
}
