use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::api::handlers::AppState;
use crate::web3::compiler::ContractDeployer;

#[derive(Deserialize)]
pub struct DeployRequest {
    pub contract_name: String,
    pub source_code: String,
    pub network_rpc: Option<String>,
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub contract_address: String,
    pub bytecode_len: usize,
    pub status: String,
}

pub async fn deploy_contract_handler(
    State(_state): State<AppState>,
    axum::Json(payload): axum::Json<DeployRequest>,
) -> Json<Value> {
    let rpc = payload.network_rpc.unwrap_or_else(|| "https://eth-mainnet.g.alchemy.com/v2/mock".to_string());
    
    // In a real scenario, private key might come from state/config. Using a dummy one for simulation.
    let deployer = ContractDeployer::new(rpc, "0xDummyPrivateKey");
    
    match deployer.compile_solidity(&payload.contract_name, &payload.source_code) {
        Ok(bytecode) => {
            match deployer.deploy_contract(&bytecode).await {
                Ok(address) => {
                    Json(json!(DeployResponse {
                        contract_address: address,
                        bytecode_len: bytecode.len(),
                        status: "deployed".to_string(),
                    }))
                },
                Err(e) => Json(json!({ "error": format!("Deployment failed: {}", e) }))
            }
        },
        Err(e) => Json(json!({ "error": format!("Compilation failed: {}", e) }))
    }
}
