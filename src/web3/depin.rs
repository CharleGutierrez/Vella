/// Vella DePIN (Decentralized Physical Infrastructure Network) Gateway
/// Bridges Vella's SCADA/IoT pipelines to blockchain token economies.
pub struct DepinGateway {
    token_contract_address: String,
    reward_rate_per_packet: f64,
}

impl DepinGateway {
    pub fn new(token_contract: impl Into<String>, reward_rate: f64) -> Self {
        Self {
            token_contract_address: token_contract.into(),
            reward_rate_per_packet: reward_rate,
        }
    }

    /// Receives a sensor packet (e.g. Weather, Bandwidth, Location) and triggers an on-chain crypto reward
    pub async fn ingest_sensor_and_reward(&self, device_wallet: &str, sensor_payload: &str) -> Result<String, String> {
        println!("📡 [Vella DePIN] Received physical sensor telemetry: {}", sensor_payload);
        println!("💰 [Vella DePIN] Minting {} protocol tokens to Device Wallet: {}...", self.reward_rate_per_packet, device_wallet);
        
        // Mock token reward transaction
        let tx_hash = format!("0xDepinRewardTx{}", device_wallet.len());
        Ok(tx_hash)
    }
}
