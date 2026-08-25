use std::time::SystemTime;

/// Spacecraft Telemetry Packet based on CCSDS standards
pub struct CcsdsPacket {
    pub spacecraft_id: u16,
    pub payload_type: u8,
    pub timestamp: SystemTime,
    pub data: Vec<u8>,
}

pub struct TelemetryIngestor;

impl TelemetryIngestor {
    pub fn new() -> Self {
        Self
    }

    pub fn ingest_packet(&self, packet: CcsdsPacket) -> Result<(), String> {
        // Mock parsing and validation of spacecraft telemetry
        if packet.data.is_empty() {
            return Err("Empty telemetry payload".to_string());
        }
        Ok(())
    }
}
