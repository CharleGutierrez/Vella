use k256::ecdsa::{SigningKey, VerifyingKey, signature::Signer, Signature};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use sha3::{Digest, Keccak256};
use rand::rngs::OsRng;
use hex;

pub struct CryptoWallet {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
    pub address: String,
}

impl CryptoWallet {
    /// Generate a real ECDSA secp256k1 private key and derive its public key and address.
    pub fn generate_new() -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = *private_key.verifying_key();
        
        let encoded_point = public_key.to_encoded_point(false);
        let public_key_bytes = encoded_point.as_bytes();
        let public_key_uncompressed = &public_key_bytes[1..];
        
        let mut addr_hasher = Keccak256::new();
        addr_hasher.update(public_key_uncompressed);
        let address_hash = addr_hasher.finalize();
        let address_bytes = &address_hash[12..];
        
        let address = format!("0x{}", hex::encode(address_bytes));
        
        Self {
            private_key,
            public_key,
            address,
        }
    }

    /// Sign a payload (message) with the private key to produce a verifiable signature.
    pub fn sign_message(&self, message: &str) -> String {
        let mut hasher = Keccak256::new();
        hasher.update(message.as_bytes());
        let digest = hasher.finalize();

        let signature: Signature = self.private_key.sign(&digest);
        let sig_bytes = signature.to_bytes();
        format!("0x{}", hex::encode(sig_bytes))
    }
}
