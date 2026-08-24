use tracing::info;
use serde_json::{json, Value};

pub struct HmiCanvasBuilder;

impl HmiCanvasBuilder {
    pub fn new() -> Self {
        info!("Initializing SCADA HMI 2D Mimic Panel Engine");
        Self
    }

    /// Generates the payload to bind a graphical SVG tank element to a live OPC UA telemetry tag
    pub fn bind_svg_to_telemetry_tag(&self, svg_element_id: &str, scada_tag: &str) -> Value {
        info!("HMI Builder: Binding Graphic '{}' to Live Telemetry Tag '{}'", svg_element_id, scada_tag);
        
        json!({
            "element_id": svg_element_id,
            "data_source": scada_tag,
            "animation_type": "fill_level",
            "color_states": {
                "safe": "#00FF00",
                "warning": "#FFFF00",
                "critical": "#FF0000"
            }
        })
    }
}
