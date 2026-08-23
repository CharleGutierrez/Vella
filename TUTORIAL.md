# 📘 Vella End-to-End Tutorial: From SQLite to PostgreSQL RAG

This step-by-step tutorial guides you through building a complete, production-grade LLM Knowledge Base and E-Commerce application using Vella.

---

## 🎯 What You Will Build
1. **Multi-Database Backend**: Start with SQLite locally, then scale to PostgreSQL with pgvector in production.
2. **AI Knowledge Base**: Document models with 1536-dimensional vector embeddings for Retrieval-Augmented Generation (RAG).
3. **AI Middleware**: Rate-limiting token consumption and enabling semantic caching for sub-millisecond responses.
4. **Realtime React 18 / Vue 3 Frontend**: Connecting frontend hooks to Vella's WebSocket hub.
5. **Headless CMS & Approval Queue**: Managing content status (Draft -> Published) and reviewing sensitive financial mutations.

---

## 🛠️ Step 1: Project Setup

Initialize your Rust binary project:

```bash
cargo new my_vella_app
cd my_vella_app
```

Add Vella to your `Cargo.toml`:

```toml
[dependencies]
vella = "0.1"
tokio = { version = "1.38", features = ["full"] }
serde_json = "1.0"
```

---

## 🦀 Step 2: Define Domain Models (`src/main.rs`)

```rust
use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Define Category Model
    let category_schema = ModelSchema::new("Category")
        .category("E-Commerce")
        .icon("tag")
        .field(Field::string("name").required().unique().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::boolean("is_active").default_value(serde_json::json!(true)))
        .with_timestamps();

    // 2. Define Product Model (with Money, Progress Bar, Approval on discount, and Vector Embedding)
    let product_schema = ModelSchema::new("Product")
        .category("E-Commerce")
        .icon("shopping-bag")
        .field(Field::string("title").required().searchable())
        .field(Field::money("price", "USD").required().filterable(true))
        // Sensitive field requiring manager review
        .field(Field::float("discount_percent").requires_approval())
        .field(Field::progress_bar("stock_quantity", 500.0, "#22c55e"))
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        .field(Field::foreign_key("category_id", "Category"))
        // 1536d vector for visual / semantic similarity
        .field(Field::vector("embedding", 1536))
        .with_timestamps();

    // 3. Define RAG Knowledge Base Document Model
    let doc_schema = ModelSchema::new("Document")
        .category("AI Knowledge Base")
        .icon("cpu")
        .field(Field::string("title").required().searchable())
        .field(Field::markdown("content").required())
        .field(Field::vector("embedding", 1536))
        .field(Field::r#enum("status", vec!["Draft", "Indexed", "Archived"]))
        .with_timestamps();

    // 4. Launch Vella Server
    VellaApp::new()
        .site_name("My AI Store & Docs")
        .bind("0.0.0.0:8080")
        .database("sqlite://app.db?mode=rwc")
        .auto_export_types_to("./frontend/types/vella.d.ts")
        .semantic_cache(true, 0.90)
        .token_rate_limit(100_000)
        .register(category_schema)
        .register(product_schema)
        .register(doc_schema)
        .run()
        .await?;

    Ok(())
}
```

---

## ⚡ Step 3: Run the Server and Access the Headless CMS

```bash
cargo run
```

Open your browser to:
- **Headless CMS & Admin Dashboard**: `http://localhost:8080` (Login: `admin` / `admin`)
- **OpenAPI Swagger UI**: `http://localhost:8080/swagger`
- **TypeScript Definitions**: `http://localhost:8080/api/types/typescript.d.ts`

---

## 🤖 Step 4: Using the Agentic Scaffolder CLI

Generate a new model on the fly using natural language:

```bash
vella generate model Customer --ai "Customer with stripe billing, oauth ID, and avatar image"
```

---

## 🔍 Step 5: Executing Vector Similarity & RAG Queries

### Direct Vector Search (`POST /api/d/document/search-vector`):
```json
{
  "model": "Document",
  "query_vector": [0.05, -0.12, 0.88, ...],
  "top_k": 3,
  "metric": "Cosine"
}
```

### RAG Semantic Query (`POST /api/ai/rag/query`):
```json
{
  "query": "How do I configure vector embeddings in Vella?",
  "model_name": "Document",
  "query_vector": [0.05, -0.12, 0.88, ...]
}
```

---

## 🚀 Step 6: Scaling to PostgreSQL in Production

When moving from local development to production, **do not change any application code**. Simply update your database connection string:

```rust
// In production:
.database("postgres://postgres:password@postgres-cluster.internal:5432/prod_db")
```

Vella will automatically:
1. Enable `pgvector` (`CREATE EXTENSION IF NOT EXISTS vector;`).
2. Map `FieldType::Vector` to `vector(N)`.
3. Create HNSW vector similarity indexes.
4. Scale connection pools to high concurrency.
