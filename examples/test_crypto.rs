use vella::web3::crypto::CryptoWallet;

fn main() {
    println!("=== Vella Crypto Test ===");
    
    // 1. Generate new wallet
    println!("Generating new ECDSA secp256k1 keypair...");
    let wallet = CryptoWallet::generate_new();
    
    println!("New Wallet Address: {}", wallet.address);
    println!("Private Key (Hidden): ******");

    // 2. Sign a message
    let message = "Send 10 BTC";
    println!("Message to sign: '{}'", message);
    
    let signature = wallet.sign_message(message);
    println!("Cryptographic Signature (hex): {}", signature);
    
    println!("=== Test Complete ===");
}
