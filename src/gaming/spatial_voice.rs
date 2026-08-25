/// Spatial Voice Chat (WebRTC Audio Routing via Geospatial coords)
pub struct SpatialVoiceRouter {
    max_audible_distance: f32,
}

impl SpatialVoiceRouter {
    pub fn new(max_audible_distance: f32) -> Self {
        Self { max_audible_distance }
    }

    pub fn calculate_attenuation(&self, distance: f32) -> f32 {
        if distance > self.max_audible_distance {
            0.0
        } else {
            1.0 - (distance / self.max_audible_distance)
        }
    }
}
