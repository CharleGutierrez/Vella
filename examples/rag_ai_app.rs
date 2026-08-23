use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Declare RAG Knowledge Base Document Model with Vector Embedding
    let doc_schema = ModelSchema::new("Document")
        .category("AI Knowledge Base")
        .icon("cpu")
        .description("Vectorized Documents for Retrieval-Augmented Generation (RAG)")
        .field(Field::string("title").required().searchable())
        .field(Field::string("source_url"))
        .field(Field::markdown("content").required().help("Chunk text content"))
        .field(Field::vector("embedding", 1536).help("1536d text embedding"))
        .field(Field::r#enum("status", vec!["Draft", "Indexed", "Archived"]))
        .with_timestamps();

    // 2. Pre-seed sample vector documents
    let db_url = "sqlite://vella_rag.db?mode=rwc";
    let db = vella::db::SqliteDatabase::connect(db_url, 5).await?;
    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await?;
    vella::db::SchemaMigrator::migrate_model(&db.pool, &doc_schema).await?;

    let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM \"documents\"")
        .fetch_one(&db.pool)
        .await
        .unwrap_or((0,));

    if count_row.0 == 0 {
        let sample_docs = vec![
            ("Vella Multi-Database Architecture", "Vella supports SQLite for local simplicity and PostgreSQL pgvector for production scale.", vec![0.1f32; 1536]),
            ("Semantic Caching in Vella", "Vella caches LLM completions using cosine similarity to reduce latency to <1ms.", vec![0.5f32; 1536]),
            ("Zero-Config Frontend Types", "Vella exports .d.ts TypeScript definitions automatically on model changes.", vec![0.9f32; 1536]),
        ];

        for (title, content, vec) in sample_docs {
            let mut payload = serde_json::Map::new();
            payload.insert("title".to_string(), serde_json::json!(title));
            payload.insert("content".to_string(), serde_json::json!(content));
            payload.insert("embedding".to_string(), serde_json::json!(vec));
            payload.insert("status".to_string(), serde_json::json!("Indexed"));
            let _ = db.insert(&doc_schema, &payload).await;
        }
    }

    println!("\n🤖 Launching Vella RAG & LLM-Native Knowledge Engine...");

    // 3. Start Vella Server with AI Middleware Pipeline
    VellaApp::new()
        .site_name("Vella RAG Knowledge Engine")
        .bind("0.0.0.0:8080")
        .database(db_url)
        .semantic_cache(true, 0.88)
        .token_rate_limit(100_000)
        .auto_export_types_to("./frontend/types/vella.d.ts")
        .register(doc_schema)
        .run()
        .await?;

    Ok(())
}
