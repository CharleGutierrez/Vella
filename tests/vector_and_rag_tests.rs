use serde_json::json;
use vella::ai::vector::{cosine_similarity, dot_product, euclidean_distance, DistanceMetric, VectorSearchQuery};
use vella::db::DatabaseAdapter;
use vella::prelude::*;

#[test]
fn test_vector_math_operations() {
    // 1. Identical vectors -> cosine similarity = 1.0
    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![1.0, 2.0, 3.0];
    let sim = cosine_similarity(&v1, &v2);
    assert!((sim - 1.0).abs() < 1e-5);

    // 2. Orthogonal vectors -> cosine similarity = 0.0
    let v_ortho1 = vec![1.0, 0.0];
    let v_ortho2 = vec![0.0, 1.0];
    let sim_ortho = cosine_similarity(&v_ortho1, &v_ortho2);
    assert!(sim_ortho.abs() < 1e-5);

    // 3. Opposite vectors -> cosine similarity = -1.0
    let v_opp1 = vec![1.0, 2.0];
    let v_opp2 = vec![-1.0, -2.0];
    let sim_opp = cosine_similarity(&v_opp1, &v_opp2);
    assert!((sim_opp - (-1.0)).abs() < 1e-5);

    // 4. Dot Product & Euclidean Distance
    let dot = dot_product(&v1, &v2);
    assert_eq!(dot, 14.0);

    let euc = euclidean_distance(&v_ortho1, &v_ortho2);
    assert!((euc - (2.0f32).sqrt()).abs() < 1e-5);
}

#[tokio::main]
#[test]
async fn test_vector_similarity_search_in_database() {
    let db_file = format!("/tmp/vella_test_vector_search_{}.db", rand::random::<u64>());
    let db = vella::db::SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();

    let doc_schema = ModelSchema::new("Doc")
        .field(Field::string("title").required())
        .field(Field::vector("embedding", 3))
        .with_timestamps();

    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    vella::db::SchemaMigrator::migrate_model(&db.pool, &doc_schema).await.unwrap();

    // Insert 3 documents with known embeddings
    let docs = vec![
        ("Rust Programming", vec![0.9f32, 0.1, 0.0]),
        ("WebAssembly Edge", vec![0.85f32, 0.15, 0.05]),
        ("Cooking Italian Pasta", vec![0.0f32, 0.9, 0.8]),
    ];

    for (title, vec) in docs {
        let mut map = serde_json::Map::new();
        map.insert("title".to_string(), json!(title));
        map.insert("embedding".to_string(), json!(vec));
        db.insert(&doc_schema, &map).await.unwrap();
    }

    // Query for Rust-like vector
    let query = VectorSearchQuery {
        model: "Doc".to_string(),
        vector_field: "embedding".to_string(),
        query_vector: vec![0.95, 0.05, 0.0],
        top_k: 2,
        metric: DistanceMetric::Cosine,
    };

    let results = db.search_vectors(&doc_schema, &query).await.unwrap();
    assert_eq!(results.len(), 2);
    // Closest match should be "Rust Programming"
    assert_eq!(results[0].record.get("title").unwrap().as_str().unwrap(), "Rust Programming");
    assert!(results[0].score > 0.99);

    // Second match should be "WebAssembly Edge"
    assert_eq!(results[1].record.get("title").unwrap().as_str().unwrap(), "WebAssembly Edge");
}

#[test]
fn test_semantic_cache_instant_lookup() {
    let cache = SemanticCache::new(0.90);
    let prompt_vec = vec![0.8, 0.5, 0.1];

    // Put item in cache
    cache.put(
        "How to scale SQLite to Postgres in Vella?",
        prompt_vec.clone(),
        json!({ "answer": "Change the connection string from sqlite:// to postgres://" }),
    );

    // 1. Exact match lookup
    let hit = cache.lookup(&prompt_vec);
    assert!(hit.is_some());
    let (res, sim, matched_q) = hit.unwrap();
    assert!((sim - 1.0).abs() < 1e-5);
    assert!(matched_q.contains("How to scale"));
    assert!(res.get("answer").is_some());

    // 2. Near match lookup (similarity ~0.998)
    let near_vec = vec![0.81, 0.49, 0.1];
    let near_hit = cache.lookup(&near_vec);
    assert!(near_hit.is_some());

    // 3. Distant query (should miss)
    let distant_vec = vec![0.0, 0.0, 1.0];
    let miss = cache.lookup(&distant_vec);
    assert!(miss.is_none());

    let stats = cache.stats_json();
    assert_eq!(stats.get("total_hits").unwrap().as_u64().unwrap(), 2);
    assert_eq!(stats.get("total_misses").unwrap().as_u64().unwrap(), 1);
}

#[test]
fn test_token_rate_limiter_and_prompt_logger() {
    let limiter = TokenRateLimiter::new(1000); // 1000 tokens / minute

    // 1. First consume 600 tokens (allowed)
    let res1 = limiter.check_and_consume("user_alice", 600);
    assert!(res1.is_ok());

    // 2. Consume 300 tokens (allowed, total 900)
    let res2 = limiter.check_and_consume("user_alice", 300);
    assert!(res2.is_ok());

    // 3. Consume 200 tokens (exceeds 1000 -> blocked)
    let res3 = limiter.check_and_consume("user_alice", 200);
    assert!(res3.is_err());
    assert_eq!(limiter.total_requests_blocked(), 1);

    // Prompt Logger
    let logger = PromptLogger::default();
    let entry = logger.log_completion(
        Some(1),
        "gpt-4o",
        "Explain RAG",
        "RAG retrieves relevant knowledge embeddings...",
        100,
        250,
        45.2,
        false,
    );

    assert_eq!(entry.total_tokens, 350);
    assert!(entry.estimated_cost_usd > 0.0);
    assert_eq!(logger.recent_logs(5).len(), 1);
}
