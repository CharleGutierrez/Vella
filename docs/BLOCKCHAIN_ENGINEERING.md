# Vella Web3 & Blockchain Engineering Manual

Welcome to the definitive guide for using the Vella framework to build next-generation Web3, DeFi, and DePIN applications. Vella provides an embedded Rust suite for cryptographic primitives, L2 sequencing, smart contract interaction, and decentralized physical infrastructure networks.

## Core Cryptography (`crypto.rs`)

Vella provides native, high-performance cryptographic primitives without relying on heavy external clients. 

### Key Generation and Addresses
Using ECDSA and the secp256k1 curve, you can securely generate keypairs. From the public key, Vella derives the standard Keccak256 Ethereum-style addresses.

### Signing Transactions
You can mathematically sign transaction payloads directly in Vella.

```rust
// examples/test_crypto.rs snippet
use vella::web3::crypto::{generate_keypair, sign_payload};

fn main() {
    let (priv_key, pub_key) = generate_keypair();
    let address = pub_key.to_address();
    println!("New Wallet Address: {}", address);

    let payload = b"Transfer 10 ETH";
    let signature = sign_payload(&priv_key, payload);
    println!("Signature: {:?}", signature);
}
```

## Zero-Knowledge Rollups (`sequencer.rs`)

Vella acts as a powerful L2 sequencer out-of-the-box. It batches off-chain transactions and generates genuine `bellman` zk-SNARK proofs to submit to Layer 1.

### Batching & Proving
The sequencer bundles hundreds of state transitions and computes a zero-knowledge proof proving their validity without revealing the underlying data.

```rust
// example snippet
use vella::web3::sequencer::ZkSequencer;

let sequencer = ZkSequencer::new();
sequencer.add_transaction(tx1);
let proof = sequencer.generate_snark_proof();
sequencer.submit_to_l1(proof);
```

## Smart Contracts & RPC Indexing (`compiler.rs` & `indexer.rs`)

Vella bridges the gap between off-chain Rust services and on-chain logic.

### Deploying Contracts
You can compile and deploy smart contracts programmatically via HTTP JSON-RPC to any EVM-compatible network.

### Listening to Events
Use WebSockets to index on-chain events in real-time, allowing your Vella backend to react to blockchain state changes instantly.

```rust
// examples/test_blockchain.rs snippet
use vella::web3::indexer::EventIndexer;
use vella::web3::compiler::ContractDeployer;

#[tokio::main]
async fn main() {
    let deployer = ContractDeployer::new("http://localhost:8545");
    let contract_address = deployer.deploy("MyContract.sol").await.unwrap();

    let indexer = EventIndexer::new("ws://localhost:8546");
    indexer.listen(contract_address, |event| {
        println!("New event received: {:?}", event);
    }).await;
}
```

## Cross-Chain Oracles & DePIN (`oracle.rs` & `depin.rs`)

The future is multi-chain and physical. Vella natively supports Decentralized Physical Infrastructure Networks (DePIN) and cross-chain communications.

### Cross-Chain Bridges
Vella's oracle modules allow you to read state from one chain (e.g., Ethereum) and relay it to another (e.g., Solana), ensuring synchronized states across distinct consensus environments.

### DePIN Token Distribution
Reward IoT hardware nodes based on cryptographic hashes of their uptime and data provision. Vella allows direct token distribution to hardware endpoints.

```rust
use vella::web3::depin::HardwareNodeManager;
use vella::web3::oracle::CrossChainBridge;

let bridge = CrossChainBridge::new(eth_rpc, sol_rpc);
bridge.sync_state().await;

let mut depin = HardwareNodeManager::new();
depin.register_node(node_pub_key);
depin.distribute_rewards(node_pub_key, proof_of_uptime).await;
```
