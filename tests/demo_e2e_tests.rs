use serde_json::json;
use vella::db::DatabaseAdapter;
use vella::prelude::*;

#[tokio::main]
#[test]
async fn test_ecommerce_demo_full_lifecycle() {
    let db_file = format!("/tmp/vella_test_ecommerce_demo_{}.db", rand::random::<u64>());
    let db = vella::db::SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();

    let cat_schema = ModelSchema::new("Category")
        .field(Field::string("name").required().unique())
        .with_timestamps();

    let prod_schema = ModelSchema::new("Product")
        .field(Field::string("title").required().searchable())
        .field(Field::money("price", "USD").required())
        .field(Field::vector("embedding", 3))
        .field(Field::r#enum("status", vec!["Draft", "Published"]))
        .with_timestamps();

    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    vella::db::SchemaMigrator::migrate_model(&db.pool, &cat_schema).await.unwrap();
    vella::db::SchemaMigrator::migrate_model(&db.pool, &prod_schema).await.unwrap();

    // 1. Insert Category
    let mut cat_map = serde_json::Map::new();
    cat_map.insert("name".to_string(), json!("Electronics"));
    let created_cat = db.insert(&cat_schema, &cat_map).await.unwrap();
    assert!(created_cat.get("id").is_some());

    // 2. Insert Product with vector embedding
    let mut prod_map = serde_json::Map::new();
    prod_map.insert("title".to_string(), json!("Smart Noise Cancelling Headphones"));
    prod_map.insert("price".to_string(), json!(249.99));
    prod_map.insert("embedding".to_string(), json!(vec![0.1f32, 0.8, 0.5]));
    prod_map.insert("status".to_string(), json!("Published"));
    let created_prod = db.insert(&prod_schema, &prod_map).await.unwrap();
    let prod_id = created_prod.get("id").unwrap().as_i64().unwrap();

    // 3. Query by vector similarity
    let v_query = VectorSearchQuery {
        model: "Product".to_string(),
        vector_field: "embedding".to_string(),
        query_vector: vec![0.15, 0.75, 0.48],
        top_k: 1,
        metric: DistanceMetric::Cosine,
    };
    let vector_matches = db.search_vectors(&prod_schema, &v_query).await.unwrap();
    assert_eq!(vector_matches.len(), 1);
    assert_eq!(vector_matches[0].id, prod_id);
    assert!(vector_matches[0].score > 0.98);
}
