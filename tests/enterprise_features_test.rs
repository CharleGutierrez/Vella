use std::sync::Arc;
use vella::storage::{StorageManager, StorageConfig};
use vella::jobs::JobQueue;
use vella::scripting::ScriptEngine;
use vella::ai::chunking::DocumentSplitter;
use vella::ai::local_llm::LocalLlmEngine;
use vella::ai::tuner::AiTuner;
use vella::db::rls::RlsPolicy;
use vella::db::relational::RelationalQueryBuilder;
use vella::realtime::wal::WalTailer;
use bytes::Bytes;

#[tokio::test]
async fn test_storage_manager_memory() {
    let tuner = Arc::new(AiTuner::new());
    let storage = StorageManager::new(StorageConfig::Memory, tuner);
    let test_data = Bytes::from("vella_test_asset_data");
    
    let upload_res = storage.upload("assets/test.txt", test_data.clone()).await;
    assert!(upload_res.is_ok(), "Upload to memory storage failed");
    
    // Pass high access count to trigger AI memory promotion logic
    let download_res = storage.smart_download("assets/test.txt", 1500).await.unwrap();
    assert_eq!(download_res, test_data, "Downloaded data does not match uploaded data");
}

#[tokio::test]
async fn test_scripting_engine() {
    let engine = ScriptEngine::new();
    
    let script = r#"
        let tax_rate = 0.20;
        let price = 100.0;
        price + (price * tax_rate)
    "#;
    
    let ast = engine.compile(script).expect("Failed to compile script");
    let result = engine.execute(&ast, "{}").expect("Failed to execute script");
    
    let final_value: f64 = result.cast();
    assert_eq!(final_value, 120.0, "Script calculation yielded wrong result");
}

#[test]
fn test_document_splitter_rag() {
    let splitter = DocumentSplitter::new();
    
    // High code density text should trigger AI to use larger chunks (1024) instead of 512
    let code_dense_text = "```rust\nfn main() {}\n```\n".repeat(10);
    let chunks = splitter.chunk_text_semantically(&code_dense_text);
    
    assert!(chunks.len() >= 1, "Document was not split properly");
}

#[tokio::test]
async fn test_local_llm_scaffolding() {
    let llm = LocalLlmEngine::new("./models/phi-3-mini.gguf");
    let result = llm.generate_schema_ddl("Create a product table").await.unwrap();
    
    assert!(result.contains("CREATE TABLE"));
}

#[test]
fn test_rls_policy_mutation() {
    let policy = RlsPolicy::new("articles", "tenant_id");
    let mutated = policy.apply_to_query("SELECT * FROM articles", "tenant_abc");
    assert_eq!(mutated, "SELECT * FROM articles WHERE tenant_id = 'tenant_abc'");
}

#[test]
fn test_relational_query_builder() {
    let tuner = Arc::new(AiTuner::new());
    let builder = RelationalQueryBuilder::new(tuner);
    
    let base = "articles";
    let expands = vec!["author.company"];
    
    // Passing 100ms latency triggers the AI Index Recommendation
    let query = builder.build_expansion_query(base, &expands, 100);
    assert!(query.contains("LEFT JOIN author"));
}

#[tokio::test]
async fn test_wal_tailer_initialization() {
    let tailer = WalTailer::new("postgres://localhost/vella");
    assert_eq!(tailer.connection_string, "postgres://localhost/vella");
}

