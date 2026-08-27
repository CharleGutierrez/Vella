use tracing::{info, warn};
use std::net::UdpSocket;

pub struct UdpTelemetryListener {
    bind_address: String,
    pub socket: Option<UdpSocket>,
}

impl UdpTelemetryListener {
    pub fn new(bind_address: &str) -> Self {
        info!("Initializing Ultra-Low Latency UDP Telemetry Listener on {}", bind_address);
        Self {
            bind_address: bind_address.to_string(),
            socket: None,
        }
    }

    /// Binds to a UDP socket to receive fire-and-forget telemetry packets at 200mph
    pub fn listen_for_telemetry(&mut self) -> Result<(), String> {
        info!("Binding to UDP Socket: {}", self.bind_address);
        
        let socket = UdpSocket::bind(&self.bind_address).map_err(|e| format!("Failed to bind UDP: {}", e))?;
        // Set a small read timeout so it doesn't block forever if no data
        let _ = socket.set_read_timeout(Some(std::time::Duration::from_millis(500)));
        self.socket = Some(socket);
        
        warn!("UDP Stream active: Bypassing TCP handshakes for maximum throughput.");
        Ok(())
    }
    
    pub fn receive(&self) -> Option<Vec<u8>> {
        if let Some(socket) = &self.socket {
            let mut buf = [0; 1024];
            if let Ok((amt, _src)) = socket.recv_from(&mut buf) {
                return Some(buf[..amt].to_vec());
            }
        }
        None
    }
}
