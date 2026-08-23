use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tower::ServiceExt;
use vella::prelude::*;

#[tokio::main]
#[test]
async fn test_crypto_password_hashing() {
    let raw = "supersecret123";
    let hash = vella::auth::Crypto::hash_password(raw);

    assert!(hash.starts_with("$s2$"));
    assert!(vella::auth::Crypto::verify_password(raw, &hash));
    assert!(!vella::auth::Crypto::verify_password("wrongpassword", &hash));
}

#[tokio::main]
#[test]
async fn test_multi_db_dialects() {
    let schema = ModelSchema::new("Article")
        .field(Field::string("title").required())
        .field(Field::vector("embedding", 1536))
        .field(Field::boolean("is_published"))
        .with_timestamps();

    let sqlite_ddl = SqlDialect::create_table_ddl(DatabaseType::Sqlite, &schema);
    assert!(sqlite_ddl.contains("CREATE TABLE IF NOT EXISTS \"articles\""));
    assert!(sqlite_ddl.contains("\"id\" INTEGER PRIMARY KEY AUTOINCREMENT"));

    let pg_ddl = SqlDialect::create_table_ddl(DatabaseType::Postgres, &schema);
    assert!(pg_ddl.contains("CREATE EXTENSION IF NOT EXISTS vector;"));
    assert!(pg_ddl.contains("\"embedding\" vector(1536)"));
    assert!(pg_ddl.contains("CREATE INDEX IF NOT EXISTS idx_articles_embedding_hnsw"));

    let mysql_ddl = SqlDialect::create_table_ddl(DatabaseType::MySql, &schema);
    assert!(mysql_ddl.contains("`id` BIGINT AUTO_INCREMENT PRIMARY KEY"));
}

struct TestHook {
    call_count: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl ModelHook for TestHook {
    async fn before_create(&self, model: &str, data: &mut serde_json::Value) -> Result<(), VellaError> {
        self.call_count.fetch_add(1, Ordering::Relaxed);
        if model == "Product" {
            if let Some(obj) = data.as_object_mut() {
                obj.insert("hook_injected".to_string(), json!(true));
            }
        }
        Ok(())
    }
}

#[tokio::main]
#[test]
async fn test_database_crud_and_audit() {
    let db_file = format!("/tmp/vella_test_crud_audit_{}.db", rand::random::<u64>());
    let db_url = format!("sqlite://{}?mode=rwc", db_file);
    let db = vella::db::SqliteDatabase::connect(&db_url, 5).await.unwrap();

    let product_schema = ModelSchema::new("Product")
        .field(Field::string("name").required().searchable())
        .field(Field::money("price", "USD").required())
        .field(Field::boolean("hook_injected").default_value(json!(false)))
        .with_timestamps();

    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    vella::db::SchemaMigrator::migrate_model(&db.pool, &product_schema).await.unwrap();

    let hook_calls = Arc::new(AtomicUsize::new(0));
    let test_hook = TestHook {
        call_count: hook_calls.clone(),
    };

    let (router, _) = VellaApp::new()
        .database(db_url)
        .register(product_schema.clone())
        .hook(test_hook)
        .build_router()
        .await
        .unwrap();

    // 1. Create product
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/product")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "name": "Mechanical Keyboard", "price": 149.99 }).to_string()))
        .unwrap();

    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    assert_eq!(hook_calls.load(Ordering::Relaxed), 1);

    // 2. Query product
    let req = Request::builder()
        .method("GET")
        .uri("/api/d/product?$search=Keyboard")
        .body(Body::empty())
        .unwrap();

    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::main]
#[test]
async fn test_approval_workflow() {
    let db_file = format!("/tmp/vella_test_approvals_{}.db", rand::random::<u64>());
    let db = vella::db::SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();

    let approval_svc = vella::audit::ApprovalService::new(db.pool.clone());
    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

    let app_id = approval_svc
        .create_approval("Product", 101, "price", Some("100.00"), "40.00", Some(1), Some("junior_dev"))
        .await
        .unwrap();

    assert!(app_id > 0);

    let pending = approval_svc.list_pending().await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].field_name, "price");

    let rejected = approval_svc.reject(app_id, 999, "lead_admin").await.unwrap();
    assert!(rejected);

    let pending_after = approval_svc.list_pending().await.unwrap();
    assert_eq!(pending_after.len(), 0);
}
