# Vella Framework: Cryptocurrency Engineering Manual

Welcome to the definitive guide for Cryptocurrency Engineers using the **Vella** framework. Vella provides high-performance, robust, and secure components for building modern cryptocurrency exchanges, wallets, and decentralized finance (DeFi) applications.

This manual covers the core cryptographic and financial currency features built into Vella, specifically focusing on wallet security, limit order book matching, and forex/crypto trading pair management.

## 1. Wallet Security and Cryptography (`crypto.rs`)

At the heart of any cryptocurrency system is secure key management and transaction signing. Vella leverages industry-standard cryptography to ensure that funds remain secure.

### ECDSA Secp256k1 Keypairs & Address Derivation
Vella supports the `secp256k1` elliptic curve, the same curve used by Bitcoin and Ethereum, for generating secure asymmetric keypairs.

*   **Private Key Generation**: Cryptographically secure random number generators (CSPRNG) are used to create 256-bit private keys.
*   **Public Key Derivation**: The corresponding public key is derived using elliptic curve multiplication.
*   **Wallet Addresses**: Wallet addresses are typically derived by hashing the public key (e.g., using SHA-256 and RIPEMD-160 for Bitcoin-like addresses, or Keccak-256 for Ethereum-like addresses).

### Mathematical Transaction Signing
When a user wishes to transfer assets (e.g., sending BTC or ETH), they must mathematically prove ownership of the funds by signing the transaction payload with their private key.

*   **Hashing the Payload**: The transaction details (sender, receiver, amount, nonce, fee) are hashed.
*   **ECDSA Signature**: The hash is signed using the `secp256k1` private key, resulting in an `r` and `s` value (and a recovery ID `v`).
*   **Verification**: The network verifies the signature against the sender's public key before accepting the transaction into the mempool.

## 2. Cryptocurrency Exchanges (`matching.rs`)

Building a centralized exchange (CEX) or a decentralized exchange (DEX) order book requires a highly optimized matching engine. Vella includes a high-performance **Price-Time Priority FIFO Limit Order Book**.

### Price-Time Priority Matching
The matching engine orders limit bids (buy orders) and asks (sell orders) based on two rules:
1.  **Price**: Better prices are matched first. The highest bid and the lowest ask sit at the top of the book.
2.  **Time**: If multiple orders share the exact same price, the order that arrived first (FIFO - First In, First Out) is matched first.

### Limit vs. Market Orders
*   **Limit Orders**: Placed at a specific price, adding liquidity to the book (Maker).
*   **Market Orders**: Executed immediately against the best available prices in the order book, removing liquidity (Taker).

## 3. Crypto Trading Pairs and Margins (`forex.rs`)

For platforms offering spot trading and leveraged derivatives, Vella's trading engine seamlessly handles currency pairs and margin requirements.

### Trading Pairs
Vella allows the definition of standard cryptocurrency pairs, such as `BTC/USD`, `ETH/USDT`, or `SOL/BTC`. The engine calculates exchange rates and standardizes quantities.

### Pip Spreads & Margins
*   **Pip Spreads**: The engine can calculate the spread between the highest bid and lowest ask, essential for market makers and platform profitability.
*   **Leveraged Trading**: For margin trading, Vella manages initial margin requirements, maintenance margins, and triggers liquidations when a user's account equity drops below the maintenance threshold.

## 4. Code Examples

Below are explicit Rust code snippets demonstrating how to combine Vella's wallet generation and trading logic to instantly start building a crypto exchange backend.

### Wallet Generation & Transaction Signing
```rust
// test_crypto.rs
use vella::crypto::{KeyPair, Transaction};

fn main() {
    // 1. Generate a secure secp256k1 keypair
    let keypair = KeyPair::generate_secp256k1();
    let public_key = keypair.public_key();
    let private_key = keypair.private_key();
    
    // Derive address (pseudo-code depending on target chain)
    let address = public_key.derive_address();
    println!("New Wallet Address: {}", address);

    // 2. Create a financial transaction payload
    let mut tx = Transaction::new(
        address.clone(),           // Sender
        "0xRecipientAddress...",   // Recipient
        1.5,                       // Amount (e.g., 1.5 BTC)
        0.0001                     // Fee
    );

    // 3. Mathematically sign the transaction
    let signature = tx.sign(&private_key);
    println!("Transaction Signature: {:?}", signature);
    
    // Verify before broadcasting
    assert!(tx.verify(&public_key, &signature));
}
```

### High-Frequency Limit Order Book & Forex Pairs
```rust
// test_hft.rs / test_forex.rs
use vella::matching::{OrderBook, Order, Side};
use vella::forex::{TradingPair, MarginAccount};

fn main() {
    // 1. Initialize a BTC/USD Trading Pair
    let btc_usd = TradingPair::new("BTC", "USD");
    
    // 2. Setup the FIFO Limit Order Book
    let mut order_book = OrderBook::new(btc_usd.clone());

    // 3. Add liquidity (Limit Orders)
    let buy_order = Order::limit(Side::Bid, 50000.0, 2.0); // Buy 2 BTC at $50,000
    let sell_order1 = Order::limit(Side::Ask, 50500.0, 1.0); // Sell 1 BTC at $50,500
    let sell_order2 = Order::limit(Side::Ask, 50500.0, 3.0); // Sell 3 BTC at $50,500
    
    order_book.add_order(buy_order);
    order_book.add_order(sell_order1); // Has time priority over sell_order2
    order_book.add_order(sell_order2);

    println!("Current Spread: ${}", order_book.spread());

    // 4. Execute a Market Order against the book
    // Buy 2.5 BTC immediately at best available prices
    let execution_report = order_book.execute_market(Side::Bid, 2.5);
    
    // It should match 1.0 BTC from sell_order1 and 1.5 BTC from sell_order2
    println!("Execution Report: {:?}", execution_report);
    
    // 5. Margin Management
    let mut margin_account = MarginAccount::new(10000.0); // $10,000 initial equity
    margin_account.open_leveraged_position(btc_usd, 5.0, 50000.0); // 5x leverage
    
    if margin_account.equity() < margin_account.maintenance_margin() {
        margin_account.liquidate();
    }
}
```
