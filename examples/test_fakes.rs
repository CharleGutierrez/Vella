use vella::web3::fhe::FheEngine;
use vella::web3::compiler::ContractDeployer;
use vella::trading::fpga::FpgaCompiler;
use vella::ai::tuner::AiTuner;

#[tokio::main]
async fn main() {
    println!("=== TESTING FULLY HOMOMORPHIC ENCRYPTION ===");
    let fhe = FheEngine::new("test_key");
    let encrypted = fhe.encrypt(10);
    let computed = fhe.compute_ai_inference_on_ciphertext(&encrypted);
    let decrypted = fhe.decrypt(&computed);
    println!("Initial value: 10. After 'Complex AI Neural Network Math': {}\n", decrypted);

    println!("=== TESTING FPGA COMPILER ===");
    let _ = FpgaCompiler::compile_to_verilog("wall_street_destroyer");
    println!("");

    println!("=== TESTING WEB3 SMART CONTRACT COMPILER ===");
    let deployer = ContractDeployer::new("https://eth-mainnet.alchemyapi.io", "0xPrivate");
    let bytecode = deployer.compile_solidity("DeFiGod", "contract DeFiGod {}").unwrap();
    let _ = deployer.deploy_contract(&bytecode).await;
    println!("");

    println!("=== TESTING AI TUNER (SCADA & HFT) ===");
    let tuner = AiTuner::new();
    let _ = tuner.predict_market_volatility_circuit_breaker(50.0, 0.2);
    let _ = tuner.tune_slam_downsample_rate(35.0);
}

