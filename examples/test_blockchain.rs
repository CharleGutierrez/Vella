use vella::web3::depin::DepinGateway;
use vella::web3::indexer::ContractIndexer;
use vella::web3::ipfs::IpfsStorageGateway;
use vella::web3::oracle::CrossChainOracle;
use vella::web3::sequencer::ZkRollupSequencer;
use vella::web3::wallet::EmbeddedWalletManager;
use vella::model::schema::ModelSchema;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Testing Vella Blockchain/Web3 Modules ===\n");

    // 1. Depin Gateway
    let depin = DepinGateway::new("0xTokenContract", 1.5);
    let tx_hash = depin.ingest_sensor_and_reward("0xDeviceWallet", "temperature: 22.5C").await.unwrap();
    println!("Depin reward TX Hash: {}\n", tx_hash);

    // 2. Indexer
    let mut indexer = ContractIndexer::new("https://eth.rpc", 1);
    let schema = ModelSchema::new("Event");
    indexer.bind_contract_to_schema("0xMyContract", "Transfer(address,address,uint256)", &schema).await.unwrap();
    let index_id = indexer.process_incoming_event("0xMyContract", "Transfer payload").await.unwrap();
    println!("Indexed event ID: {}\n", index_id);

    // 3. IPFS Gateway
    let ipfs = IpfsStorageGateway::new("dummy_api_key");
    let cid = ipfs.upload_and_pin("test.txt", b"Hello Vella IPFS").await.unwrap();
    println!("IPFS CID: {}\n", cid);

    // 4. Oracle
    let oracle = CrossChainOracle::new();
    let ccip_hash = oracle.route_cross_chain_message("Ethereum", "Solana", "Move 10 USDC").await.unwrap();
    println!("Cross-Chain Hash: {}\n", ccip_hash);

    // 5. Embedded Wallet Manager
    let wallet = EmbeddedWalletManager::new("https://bundler.rpc", "0xPrivateKey");
    let contract_addr = wallet.provision_wallet_for_user("test@vella.app").await.unwrap();
    println!("Provisioned Wallet Address: {}", contract_addr);
    let sponsor_tx = wallet.sponsor_transaction_gas("Transfer 10 USDC").await.unwrap();
    println!("Sponsor TX Hash: {}\n", sponsor_tx);

    // 6. ZkRollup Sequencer
    let mut sequencer = ZkRollupSequencer::new(1);
    sequencer.queue_offchain_action("Action 1");
    sequencer.queue_offchain_action("Action 2");
    
    // Run the sequencer for a bit to let it process
    let sequencer_handle = tokio::spawn(async move {
        sequencer.run_sequencer_daemon().await;
    });

    sleep(Duration::from_secs(2)).await;
    sequencer_handle.abort();
    
    println!("\n=== Web3 Test Completed Successfully ===");
}
