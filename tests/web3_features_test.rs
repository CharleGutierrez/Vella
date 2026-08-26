use vella::web3::indexer::ContractIndexer;
use vella::web3::ipfs::IpfsStorageGateway;
use vella::web3::wallet::EmbeddedWalletManager;
use vella::web3::sequencer::ZkRollupSequencer;
use vella::ai::tuner::AiTuner;
use std::time::Duration;

#[tokio::test]
async fn test_web3_ipfs_storage_gateway() {
    let gateway = IpfsStorageGateway::new("fake_api_key");
    
    // Simulate uploading a file to IPFS
    let file_bytes = b"Hello Decentralized World!";
    let result = gateway.upload_and_pin("hello.txt", file_bytes).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().starts_with("ipfs://"));
}

#[tokio::test]
async fn test_web3_embedded_wallet_paymaster() {
    let wallet_manager = EmbeddedWalletManager::new("http://rpc", "0xprivatekey");
    
    let wallet_address = wallet_manager.provision_wallet_for_user("satoshi@vella.dev").await.unwrap();
    assert!(wallet_address.starts_with("0x"));

    let tx_hash = wallet_manager.sponsor_transaction_gas("UserAction:MintNFT").await.unwrap();
    assert!(tx_hash.starts_with("0x"));
}

#[test]
fn test_ai_tuner_zk_rollup_gas_optimization() {
    let tuner = AiTuner::new();
    let base_interval = 10; // 10 seconds

    // Test low gas (Tighten interval)
    let low_gas_interval = tuner.tune_zk_rollup_batch_interval(10.0, base_interval);
    assert_eq!(low_gas_interval, Duration::from_secs(5));

    // Test high gas (Stretch interval to save money)
    let high_gas_interval = tuner.tune_zk_rollup_batch_interval(100.0, base_interval);
    assert_eq!(high_gas_interval, Duration::from_secs(40));

    // Test normal gas
    let normal_gas_interval = tuner.tune_zk_rollup_batch_interval(30.0, base_interval);
    assert_eq!(normal_gas_interval, Duration::from_secs(10));
}

#[test]
fn test_ai_tuner_paymaster_bot_detection() {
    let tuner = AiTuner::new();

    // Normal user (Safe)
    let safe = tuner.predict_gas_sponsorship_viability(10, 0.1);
    assert!(safe);

    // High velocity spammer (Unsafe)
    let spammer = tuner.predict_gas_sponsorship_viability(150, 0.1);
    assert!(!spammer);

    // Probable Bot (Unsafe)
    let bot = tuner.predict_gas_sponsorship_viability(2, 0.95);
    assert!(!bot);
}

use vella::web3::compiler::ContractDeployer;
use vella::web3::fhe::FheEngine;
use vella::web3::depin::DepinGateway;
use vella::web3::oracle::CrossChainOracle;

#[tokio::test]
async fn test_web3_smart_contract_compiler() {
    let deployer = ContractDeployer::new("http://ethereum-rpc", "0xprivatekey");
    let bytecode = deployer.compile_solidity("Token", "contract Token {}").unwrap();
    assert!(!bytecode.is_empty());
    
    let address = deployer.deploy_contract(&bytecode).await.unwrap();
    assert!(address.starts_with("0xVellaDeployedContract"));
}

#[test]
fn test_web3_fhe_engine() {
    let fhe = FheEngine::new("secret_fhe_key");
    let encrypted = fhe.encrypt("My Private Medical Record");
    assert!(!encrypted.is_empty());

    let ai_result = fhe.compute_ai_inference_on_ciphertext(&encrypted);
    let plaintext = fhe.decrypt(&ai_result);
    assert_eq!(plaintext, "FHE_COMPUTED_RESULT_OK");
}

#[tokio::test]
async fn test_web3_depin_gateway() {
    let depin = DepinGateway::new("0xTokenContract", 5.0);
    let tx_hash = depin.ingest_sensor_and_reward("0xDeviceWallet", "TEMP: 24C").await.unwrap();
    assert!(tx_hash.starts_with("0xDepinRewardTx"));
}

#[tokio::test]
async fn test_web3_cross_chain_oracle() {
    let oracle = CrossChainOracle::new();
    let tx_hash = oracle.route_cross_chain_message("Solana", "Ethereum", "Mint 100 NFTs").await.unwrap();
    assert!(tx_hash.starts_with("0xCrossChainTx"));
    
    // Test invalid chain
    let fail = oracle.route_cross_chain_message("Dogecoin", "Ethereum", "Hello").await;
    assert!(fail.is_err());
}
