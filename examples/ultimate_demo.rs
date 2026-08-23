use vella::prelude::*;
use serde_json::json;

/// Custom Model Hook demonstrating lifecycle interception
struct AuditNotifierHook;

#[async_trait::async_trait]
impl ModelHook for AuditNotifierHook {
    async fn after_create(&self, model: &str, record: &serde_json::Value) -> Result<(), VellaError> {
        if model == "Article" {
            let title = record.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown");
            println!("🔔 [HOOK] A new article titled '{}' was just published!", title);
        }
        Ok(())
    }

    async fn before_update(&self, model: &str, _id: i64, data: &mut serde_json::Value) -> Result<(), VellaError> {
        if model == "User" {
            println!("🔔 [HOOK] Intercepting User update to validate logic...");
            if let Some(obj) = data.as_object_mut() {
                // Example mutation: track last updated timestamp explicitly
                obj.insert("hook_updated_at".to_string(), json!(chrono::Utc::now().to_rfc3339()));
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // =========================================================================
    // 1. Define Models
    // =========================================================================

    // Auth, Billing & OAuth Model
    let user_schema = ModelSchema::new("User")
        .category("CRM & Authentication")
        .icon("users")
        .description("Platform Users with OAuth, Stripe Billing, and Approval Queues")
        .field(Field::string("username").required().searchable())
        .field(Field::email("email").required().unique().searchable())
        .field(Field::password("password_hash"))
        .field(Field::r#enum("role", vec!["Admin", "Manager", "Editor", "Viewer"]).default_value(json!("Viewer")))
        .field(Field::string("stripe_customer_id").unique().help("Stripe Customer ID"))
        .field(Field::r#enum("billing_tier", vec!["Free", "Pro", "Enterprise"]))
        // Sensitive field: any change to balance triggers AI Risk Assessment and Manager Approval Workflow!
        .field(Field::money("balance", "USD").requires_approval().help("Changes require Manager review"))
        .field(Field::boolean("is_active").default_value(json!(true)))
        .field(Field::string("oauth_provider").help("e.g. google, github"))
        .field(Field::string("oauth_id").unique())
        .with_timestamps();

    // Content Management System Model (Headless CMS, Markdown, Realtime)
    let article_schema = ModelSchema::new("Article")
        .category("Content & CMS")
        .icon("file-text")
        .description("Articles managed via Headless CMS with Realtime Sync")
        .field(Field::string("title").required().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::markdown("content").required().help("Markdown formatted body"))
        .field(Field::image("cover_image", "uploads/articles"))
        .field(Field::r#enum("status", vec!["Draft", "InReview", "Published", "Archived"]).filterable(true))
        .field(Field::progress_bar("read_progress", 100.0, "#3b82f6"))
        .field(Field::foreign_key("author_id", "User").help("Author relation"))
        .with_timestamps();

    // AI Knowledge Base Model (RAG, Vector Embeddings, Semantic Cache)
    let doc_schema = ModelSchema::new("KnowledgeDoc")
        .category("AI & RAG")
        .icon("cpu")
        .description("Vectorized Documents for Retrieval-Augmented Generation (RAG)")
        .field(Field::string("title").required().searchable())
        .field(Field::markdown("content").required())
        // Native Vector Support: 1536 dimensions for OpenAI text-embedding-3-small
        .field(Field::vector("embedding", 1536).help("1536d semantic text embedding"))
        .field(Field::r#enum("status", vec!["Draft", "Indexed"]))
        .with_timestamps();

    // =========================================================================
    // 2. Pre-Seed Database (Showcase SQLite vector embedding queries)
    // =========================================================================
    let db_url = "sqlite://vella_ultimate_demo.db?mode=rwc";
    
    // Note: The scale-up promise means replacing the line above with:
    // let db_url = "postgres://user:pass@localhost:5432/vella_prod";
    // and Vella will automatically use pgvector, bigserial, and Postgres pooling!

    let db = vella::db::SqliteDatabase::connect(db_url, 10).await?;
    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await?;
    vella::db::SchemaMigrator::migrate_model(&db.pool, &doc_schema).await?;

    // Seed some vectors if empty
    let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM \"knowledgedocs\"")
        .fetch_one(&db.pool)
        .await
        .unwrap_or((0,));

    if count_row.0 == 0 {
        let sample_docs = vec![
            ("Vella Realtime WebSocket Hub", "Vella automatically broadcasts CREATE, UPDATE, DELETE events to React, Vue, and Angular SDKs over WebSockets.", vec![0.8f32; 1536]),
            ("Semantic Caching Architecture", "Vella caches LLM completions. If a query matches an embedded cache item with cosine similarity >= 0.90, it bypasses the LLM.", vec![0.5f32; 1536]),
            ("AI Scaffolder", "Use `vella generate model User --ai 'A user'` to instantly scaffold models.", vec![0.2f32; 1536]),
        ];

        for (title, content, vec) in sample_docs {
            let mut payload = serde_json::Map::new();
            payload.insert("title".to_string(), json!(title));
            payload.insert("content".to_string(), json!(content));
            payload.insert("embedding".to_string(), json!(vec));
            payload.insert("status".to_string(), json!("Indexed"));
            let _ = db.insert(&doc_schema, &payload).await;
        }
        println!("🌱 Pre-seeded Knowledge Base with Vector Embeddings.");
    }

    println!("\n🚀 Launching Vella Ultimate Demo...");
    println!("Capabilities Loaded:");
    println!(" - ✅ LLM-Native Vector Search & Semantic Cache");
    println!(" - ✅ Headless CMS & Realtime WebSockets");
    println!(" - ✅ Typescript Zero-Config Sync");
    println!(" - ✅ OAuth, Magic Links & RBAC");
    println!(" - ✅ Audit Trails & Time-Travel Rollbacks");
    println!(" - ✅ AI Manager Approval Queues");

    // =========================================================================
    // 3. Launch Vella Engine
    // =========================================================================
    VellaApp::new()
        .site_name("Vella Ultimate Showcase")
        .bind("0.0.0.0:8080")
        .database(db_url)
        // AI Middleware configurations
        .semantic_cache(true, 0.90)
        .token_rate_limit(50_000) // 50k tokens per minute limit
        // Zero-config TypeScript sync
        .auto_export_types_to("./frontend/types/vella.d.ts")
        // Register schemas
        .register(user_schema)
        .register(article_schema)
        .register(doc_schema)
        // Register Custom Hook
        .hook(AuditNotifierHook)
        // Start the server
        .run()
        .await?;

    Ok(())
}
