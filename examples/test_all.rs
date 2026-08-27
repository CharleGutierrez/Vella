use vella::prelude::*;
use vella::web3::fhe::FheEngine;
use vella::web3::compiler::ContractDeployer;
use vella::trading::fpga::FpgaCompiler;
use vella::ai::tuner::AiTuner;
use vella::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use serde_json::json;
use vella::ui::react_sdk::generate_react_sdk;
use vella::ai::vector::{VectorSearchQuery, DistanceMetric};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== 1. TESTING REACT SDK GENERATION ===");
    let sdk = generate_react_sdk("http://localhost:8080");
    println!("Generated SDK Length: {} characters.", sdk.len());
    println!("SDK Preview: {:.100}...\n", sdk.replace("\n", " "));

    println!("=== 2. TESTING SQLITE CRUD & VECTOR SEARCH ===");
    let doc_schema = ModelSchema::new("Doc")
        .field(Field::string("title"))
        .field(Field::vector("embedding", 3));

    let db = SqliteDatabase::connect("sqlite::memory:", 1).await?;
    SchemaMigrator::migrate_model(&db.pool, &doc_schema).await?;
    
    // Insert
    let mut payload = serde_json::Map::new();
    payload.insert("title".to_string(), json!("Real Data"));
    payload.insert("embedding".to_string(), json!(vec![1.0, 0.0, 0.0]));
    let inserted = db.insert(&doc_schema, &payload).await?;
    println!("Inserted Record: {}", inserted);

    // Vector Search
    let query = VectorSearchQuery {
        model: "Doc".to_string(),
        query_vector: vec![0.9, 0.1, 0.0],
        vector_field: "embedding".to_string(),
        top_k: 1,
        metric: DistanceMetric::Cosine,
    };
    let search_results = db.search_vectors(&doc_schema, &query).await?;
    println!("Vector Search Results (Cosine Similarity):");
    for res in search_results {
        println!(" - ID: {}, Score: {:.4}, Record: {}", res.id, res.score, res.record);
    }
    println!("");

    println!("=== 3. TESTING FHE ENCRYPTION (SATIRE) ===");
    let fhe = FheEngine::new("test");
    let enc = fhe.encrypt(10);
    let computed = fhe.compute_ai_inference_on_ciphertext(&enc);
    println!("10 encrypted + AI Math = {}\n", fhe.decrypt(&computed));

    println!("=== 4. TESTING FPGA (SATIRE) ===");
    let _ = FpgaCompiler::compile_to_verilog("algo");
    println!("");

    println!("=== 5. TESTING WEB3 (SATIRE) ===");
    let deployer = ContractDeployer::new("http://test", "key");
    let bytecode = deployer.compile_solidity("Token", "contract Token {}").unwrap_or_default();
    let _ = deployer.deploy_contract(&bytecode).await;
    println!("");
    
    println!("=== 6. TESTING AI TUNER ===");
    let tuner = AiTuner::new();
    let comp = tuner.tune_compression_deviation(1.5, 50.0);
    let dtn = tuner.tune_dtn_latency_tolerance(5.0, 5000);
    let slam = tuner.tune_slam_downsample_rate(15.0);
    let cb = tuner.predict_market_volatility_circuit_breaker(60.0, 0.3);
    println!("AI Tuner results:");
    println!("  Compression Deviation: {}", comp);
    println!("  DTN Latency Tolerance: {}", dtn);
    println!("  SLAM Downsample Rate: {}", slam);
    println!("  Circuit Breaker Tripped: {}", cb);
    println!("");

    Ok(())
}

