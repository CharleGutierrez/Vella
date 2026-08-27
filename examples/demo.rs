use vella::government::voting::ZkVotingEngine;
use vella::medicine::genomics::GenomicsEngine;
use vella::scada::alarms::Isa18Alarm;
use vella::trading::fix::FixEngine;
use vella::web3::fhe::FheEngine;
use vella::quantum::q_bits::QuantumEngine;

#[tokio::main]
async fn main() {
    println!("\n=== Testing Voting ===");
    let voting = ZkVotingEngine::new("ELEC_2026");
    let _ = voting.cast_anonymous_vote("proof", "cand_1");

    println!("\n=== Testing Genomics ===");
    let genomics = GenomicsEngine::new("hg38");
    let _ = genomics.align_and_detect_mutations("ATCGATCG");

    println!("\n=== Testing SCADA ===");
    let mut alarm = Isa18Alarm::new("REACTOR_TEMP");
    alarm.trigger_breach();
    alarm.operator_acknowledge();
    alarm.trigger_clear();

    println!("\n=== Testing Trading (FIX) ===");
    let fix = FixEngine::new("NASDAQ", "VELLA_FUND");
    let _ = fix.send_order("AAPL", 100, 150.5).await;

    println!("\n=== Testing Web3 FHE ===");
    let fhe = FheEngine::new("secret_key");
    let enc = fhe.encrypt(42);
    let res = fhe.compute_ai_inference_on_ciphertext(&enc);
    let _ = fhe.decrypt(&res);
    
    println!("\n=== Testing Quantum Entanglement ===");
    let q = QuantumEngine::new(256);
    let _ = q.entangle_and_factorize("SHA256_RSA2048_HASH");
}
