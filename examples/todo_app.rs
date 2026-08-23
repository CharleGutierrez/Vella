use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Declare the Todo Model Schema
    let todo_schema = ModelSchema::new("Todo")
        .category("Productivity")
        .icon("check-square")
        .description("Multi-Frontend Task Items with Realtime Sync")
        .field(Field::string("title").required().searchable())
        .field(Field::string("category").searchable().filterable(true))
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]).filterable(true))
        .field(Field::boolean("is_completed").default_value(serde_json::json!(false)).filterable(true))
        .field(Field::progress_bar("progress", 100.0, "#10b981").filterable(true))
        .field(Field::html("description"))
        .with_timestamps();

    // 2. Pre-seed initial tasks if database is newly initialized
    let db_url = "sqlite://vella_todo.db?mode=rwc";
    let db = vella::db::SqliteDatabase::connect(db_url, 5).await?;
    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await?;
    vella::db::SchemaMigrator::migrate_model(&db.pool, &todo_schema).await?;

    let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM \"todos\"")
        .fetch_one(&db.pool)
        .await
        .unwrap_or((0,));

    if count_row.0 == 0 {
        let initial_tasks = vec![
            ("Design LLM-Native architecture with Vella", "Vella Core", "Critical", 100, true),
            ("Connect pgvector & SQLite vector similarity engine", "Vector DB", "High", 100, true),
            ("Build Agentic Scaffolder CLI for model generation", "AI Scaffolder", "High", 100, true),
            ("Implement Semantic Caching (<1ms RAG response)", "AI Middleware", "High", 100, true),
            ("Deploy WebSocket & SSE Realtime Synchronization", "Realtime", "High", 100, true),
            ("Generate Zero-Config TypeScript .d.ts Definitions", "Frontend DX", "High", 100, true),
        ];

        for (title, cat, prio, prog, comp) in initial_tasks {
            let mut payload = serde_json::Map::new();
            payload.insert("title".to_string(), serde_json::json!(title));
            payload.insert("category".to_string(), serde_json::json!(cat));
            payload.insert("priority".to_string(), serde_json::json!(prio));
            payload.insert("progress".to_string(), serde_json::json!(prog));
            payload.insert("is_completed".to_string(), serde_json::json!(comp));
            let _ = db.insert(&todo_schema, &payload).await;
        }
    }

    println!("\n⚡ Launching Vella Multi-Framework Realtime Todo Application...");

    // 3. Start Vella Server
    VellaApp::new()
        .site_name("Vella Realtime Task Hub")
        .bind("0.0.0.0:8080")
        .database(db_url)
        .register(todo_schema)
        .run()
        .await?;

    Ok(())
}
