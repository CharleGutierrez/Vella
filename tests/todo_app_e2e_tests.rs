use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;
use vella::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use vella::prelude::*;

/// Comprehensive End-to-End Functional Test Suite for the Multi-Framework Todo Application
#[tokio::test]
async fn test_todo_app_complete_lifecycle_and_showcase() {
    let db_file = format!("/tmp/vella_todo_e2e_{}.db", rand::random::<u64>());
    let db_url = format!("sqlite://{}?mode=rwc", db_file);

    // 1. Declare Todo Model Schema
    let todo_schema = ModelSchema::new("Todo")
        .category("Productivity")
        .field(Field::string("title").required().searchable())
        .field(Field::string("category").searchable().filterable(true))
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]).filterable(true))
        .field(Field::boolean("is_completed").default_value(json!(false)).filterable(true))
        .field(Field::progress_bar("progress", 100.0, "#10b981").filterable(true))
        .with_timestamps();

    // 2. Initialize Database & Seed Tasks
    let db = SqliteDatabase::connect(&db_url, 5).await.unwrap();
    SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &todo_schema).await.unwrap();

    let mut t1 = serde_json::Map::new();
    t1.insert("title".to_string(), json!("Build sub-millisecond Rust backend with Vella"));
    t1.insert("category".to_string(), json!("Rust Core"));
    t1.insert("priority".to_string(), json!("Critical"));
    t1.insert("progress".to_string(), json!(100.0));
    t1.insert("is_completed".to_string(), json!(true));
    let _task1 = db.insert(&todo_schema, &t1).await.unwrap();

    let mut t2 = serde_json::Map::new();
    t2.insert("title".to_string(), json!("Connect React 18 Hooks & Realtime Sync"));
    t2.insert("category".to_string(), json!("React 18"));
    t2.insert("priority".to_string(), json!("High"));
    t2.insert("progress".to_string(), json!(50.0));
    t2.insert("is_completed".to_string(), json!(false));
    let _task2 = db.insert(&todo_schema, &t2).await.unwrap();

    // 3. Build Full Application Router
    let (app, _) = VellaApp::new()
        .site_name("Vella Task Hub")
        .database(&db_url)
        .register(todo_schema.clone())
        .build_router()
        .await
        .unwrap();

    // TEST 1: Verify HTML Delivery of Showcase Page (/todos and /showcase)
    let req = Request::builder().uri("/todos").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // TEST 2: Query All Todos (GET /api/d/todo)
    let req = Request::builder().uri("/api/d/todo?$limit=100&$order=-created_at").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(json_res["success"].as_bool().unwrap());
    assert_eq!(json_res["total"].as_i64().unwrap(), 2);

    // TEST 3: Query Filter by is_completed=true
    let req = Request::builder().uri("/api/d/todo?is_completed=true").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json_res["total"].as_i64().unwrap(), 1);
    assert_eq!(json_res["data"][0]["title"].as_str().unwrap(), "Build sub-millisecond Rust backend with Vella");

    // TEST 4: Create a New Todo Task (POST /api/d/todo)
    let create_payload = json!({
        "title": "Deploy Vella on Bare Metal Cluster",
        "category": "DevOps",
        "priority": "Critical",
        "progress": 0.0,
        "is_completed": false
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/todo")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(create_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let new_task_id = json_res["data"]["id"].as_i64().unwrap();
    assert_eq!(json_res["data"]["title"].as_str().unwrap(), "Deploy Vella on Bare Metal Cluster");

    // TEST 5: Update the Task (PUT /api/d/todo/:id)
    let update_payload = json!({
        "progress": 100.0,
        "is_completed": true
    });
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/api/d/todo/{}", new_task_id))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(update_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // TEST 6: Delete the Task (DELETE /api/d/todo/:id)
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/api/d/todo/{}", new_task_id))
        .body(axum::body::Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
