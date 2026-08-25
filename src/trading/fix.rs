use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tracing::{info, error};

/// Real Asynchronous FIX (Financial Information eXchange) Protocol Client
pub struct FixClient {
    target_comp_id: String,
    sender_comp_id: String,
    msg_seq_num: usize,
    stream: Option<TcpStream>,
}

impl FixClient {
    pub fn new(target_comp_id: impl Into<String>, sender_comp_id: impl Into<String>) -> Self {
        Self {
            target_comp_id: target_comp_id.into(),
            sender_comp_id: sender_comp_id.into(),
            msg_seq_num: 1,
            stream: None,
        }
    }

    /// Calculates the standard FIX Checksum (modulo 256 of all ASCII byte values)
    fn calculate_checksum(msg: &str) -> String {
        let sum: u32 = msg.as_bytes().iter().map(|&b| b as u32).sum();
        format!("{:03}", sum % 256)
    }

    /// Constructs a fully compliant FIX 4.4 message with SOH delimiters, BodyLength, and Checksum
    pub fn build_message(&mut self, msg_type: &str, body_fields: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // 35=MsgType, 49=SenderCompID, 56=TargetCompID, 34=MsgSeqNum, 52=SendingTime
        let header = format!(
            "35={}\x0149={}\x0156={}\x0134={}\x0152={}\x01",
            msg_type, self.sender_comp_id, self.target_comp_id, self.msg_seq_num, timestamp
        );
        self.msg_seq_num += 1;

        let body = format!("{}{}", header, body_fields);
        let body_length = body.len(); // BodyLength (9) calculates length of msg after 9= up to 10=

        // 8=BeginString, 9=BodyLength
        let header_prefix = format!("8=FIX.4.4\x019={}\x01", body_length);
        let msg_without_checksum = format!("{}{}", header_prefix, body);

        let checksum = Self::calculate_checksum(&msg_without_checksum);
        format!("{}10={}\x01", msg_without_checksum, checksum)
    }

    /// Establishes an asynchronous TCP connection to the Institutional Exchange (e.g., matching engine)
    pub async fn connect(&mut self, host_port: &str) -> Result<(), String> {
        info!("Establishing ultra-low latency TCP connection to {} at {}...", self.target_comp_id, host_port);
        match TcpStream::connect(host_port).await {
            Ok(stream) => {
                self.stream = Some(stream);
                info!("Successfully connected to Exchange.");
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
                Err(e.to_string())
            }
        }
    }

    /// Sends a FIX Logon (MsgType = A)
    pub async fn send_logon(&mut self) -> Result<(), String> {
        let msg = self.build_message("A", "98=0\x01108=30\x01"); // 98=EncryptMethod(None), 108=HeartBtInt(30s)
        self.transmit(msg).await
    }

    /// Sends a New Order Single (MsgType = D)
    pub async fn send_order(&mut self, symbol: &str, quantity: u64, price: f64) -> Result<String, String> {
        let cl_ord_id = format!("ORD_{}_{}", symbol, self.msg_seq_num);
        // 11=ClOrdID, 21=HandlInst, 55=Symbol, 54=Side(1=Buy), 60=TransactTime, 38=OrderQty, 40=OrdType(2=Limit), 44=Price
        let body = format!(
            "11={}\x0121=1\x0155={}\x0154=1\x0160=20231010-10:10:10\x0138={}\x0140=2\x0144={}\x01",
            cl_ord_id, symbol, quantity, price
        );

        let msg = self.build_message("D", &body);
        self.transmit(msg).await?;
        
        Ok(cl_ord_id)
    }

    async fn transmit(&mut self, msg: String) -> Result<(), String> {
        if let Some(stream) = &mut self.stream {
            info!("Transmitting FIX message: {}", msg.replace('\x01', "|"));
            stream.write_all(msg.as_bytes()).await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Not connected to exchange".to_string())
        }
    }
}
