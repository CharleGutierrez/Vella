# ⚡ Vella: The Definitive Master Manual

Welcome to **Vella**, the next-generation LLM-Native Rust Web Engine and Headless CMS. Vella bridges the gap between the zero-config simplicity of PocketBase and the high-concurrency, enterprise scale of Supabase.

This manual covers everything from installation to enterprise deployment, AI middleware configuration, and imaginative edge-case architectures.

---

## 📑 Table of Contents

1. [Introduction & Core Philosophy](#1-introduction--core-philosophy)
2. [Installation & Quickstart](#2-installation--quickstart)
3. [Agentic AI Scaffolder CLI](#3-agentic-ai-scaffolder-cli)
4. [Schema Design & Field Types](#4-schema-design--field-types)
5. [The Headless CMS & Admin Dashboard](#5-the-headless-cms--admin-dashboard)
6. [LLM-Native Features (Vector & RAG)](#6-llm-native-features-vector--rag)
7. [Frictionless Frontend Type Safety & SDKs](#7-frictionless-frontend-type-safety--sdks)
8. [Realtime Synchronization (WebSocket & SSE)](#8-realtime-synchronization-websocket--sse)
9. [Enterprise Governance & Approvals](#9-enterprise-governance--approvals)
10. [Resilience, Scaling, & OpenTelemetry](#10-resilience-scaling--opentelemetry)
11. [Imaginative Use Cases (The Future)](#11-imaginative-use-cases-the-future)

---

## 1. Introduction & Core Philosophy

Vella was designed to solve three modern developer pain points:
1. **The Scale-Up Cliff:** Developers love single-binary SQLite databases for local dev, but hit a wall when they need Kubernetes, horizontal scaling, and PostgreSQL. Vella allows you to switch from SQLite to PostgreSQL natively just by changing a string.
2. **The LLM Integration Nightmare:** Building RAG (Retrieval-Augmented Generation) requires gluing together a vector database, caching layers, and token trackers. Vella makes Vectors, Semantic Caching, and Token Rate Limiting native concepts.
3. **Frontend Sync Issues:** Vella automatically generates strict TypeScript `.d.ts` interfaces and real-time SDKs for React, Vue, and Angular without heavy GraphQL or REST boilerplate.

---

## 2. Installation & Quickstart

### Prerequisites
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Step 1: Create a Project
```bash
cargo new vella_app
cd vella_app
cargo add vella tokio serde_json
```

### Step 2: Write your `src/main.rs`
```rust
use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let article_schema = ModelSchema::new("Article")
        .category("Blog")
        .field(Field::string("title").required().searchable())
        .field(Field::markdown("content").required())
        .with_timestamps();

    VellaApp::new()
        .site_name("My First Vella App")
        .bind("0.0.0.0:8080")
        .database("sqlite://vella.db?mode=rwc")
        .register(article_schema)
        .run()
        .await?;

    Ok(())
}
```

### Step 3: Run
```bash
cargo run
```
Navigate to `http://localhost:8080`. Log in with **`admin` / `admin`**. You now have a working Headless CMS!

---

## 3. Agentic AI Scaffolder CLI

Don't write boilerplate. Use Vella's AI CLI to generate schemas using natural language.

```bash
cargo run --bin vella -- generate model Customer --ai "A customer with Stripe billing, OAuth ID, and an avatar image"
```

**What happens?**
The AI parses your prompt and outputs:
1. **Rust Builder Code** to copy into `main.rs`.
2. **SQL DDL** migrations (Postgres or SQLite).
3. **TypeScript Definitions** for your frontend.

---

## 4. Schema Design & Field Types

Vella models are defined using a fluent builder pattern.

### Available Field Types
*   **Basic:** `Field::string()`, `Field::integer()`, `Field::float()`, `Field::boolean()`, `Field::datetime()`.
*   **Specialized:** `Field::email()`, `Field::password()`, `Field::json()`, `Field::image()`, `Field::file()`.
*   **Rich UI:** `Field::markdown()`, `Field::html()`, `Field::progress_bar()`, `Field::money()`, `Field::r#enum()`.
*   **Relational & AI:** `Field::foreign_key()`, `Field::vector("embedding", 1536)`.

### Field Modifiers
Modifiers enforce database constraints and UI behavior:
```rust
Field::string("slug")
    .required()
    .unique()
    .searchable()
    .read_only()
    .requires_approval() // Triggers Manager Approval Queue
    .help("URL friendly identifier")
```

---

## 5. The Headless CMS & Admin Dashboard

The moment your app compiles, you get a **Glassmorphic React SPA** at `/admin`.
*   **Content Management:** Rich text editors, boolean toggles, and status badges.
*   **Vector Playground:** A dedicated tab to test cosine similarity searches against your database.
*   **AI Tuner Dashboard:** View p50/p99 query latencies and 1-click apply AI-recommended B-Tree/HNSW indexes.
*   **Audit Trail:** View exact JSON diffs of who changed what, and click **"Restore Snapshot"** to time-travel rollbacks.

---

## 6. LLM-Native Features (Vector & RAG)

### Vector Similarity Search
When using `.database("postgres://...")`, Vella automatically installs `pgvector` and creates HNSW indexes. When using SQLite, Vella uses a blazing-fast in-memory SIMD partition cache.

**Query via API:**
```http
POST /api/d/article/search-vector
{
  "query_vector": [0.05, -0.12, 0.88, ...],
  "top_k": 5,
  "metric": "Cosine"
}
```

### Semantic Caching (<1ms LLM Responses)
Enable Semantic Caching in your configuration to bypass OpenAI/Anthropic entirely for repeated queries.
```rust
VellaApp::new()
    .semantic_cache(true, 0.90) // 90% cosine similarity threshold
```

### Token Rate Limiting & Prompt Logging
Protect your billing limits and audit all LLM usage.
```rust
VellaApp::new()
    .token_rate_limit(50_000) // 50k tokens per minute
```
View token consumption and exact prompt payloads in the Admin UI.

---

## 7. Frictionless Frontend Type Safety & SDKs

Vella eliminates the need for manual type tracking.

### Zero-Config Types Export
```rust
VellaApp::new()
    .auto_export_types_to("./frontend/types/vella.d.ts")
```
Every time the server starts, it writes perfect TypeScript interfaces to your frontend folder.

### Frontend SDKs
Vella serves auto-generated SDKs directly:
*   **React:** `http://localhost:8080/api/sdk/react.ts`
*   **Vue 3:** `http://localhost:8080/api/sdk/vue.ts`
*   **Angular 17:** `http://localhost:8080/api/sdk/angular.ts`

---

## 8. Realtime Synchronization (WebSocket & SSE)

Never write a `.setInterval()` polling loop again.

Using the React SDK:
```tsx
import { useVellaQuery, useRealtimeSubscription } from './api/sdk/react';

function LiveDashboard() {
  const { data: articles, refetch } = useVellaQuery('Article');

  // Any creation, update, delete, or rollback triggers a refetch instantly
  useRealtimeSubscription('models:article', (event) => {
    console.log("Mutation detected!", event);
    refetch();
  });

  return <div>{/* Render Articles */}</div>;
}
```

---

## 9. Enterprise Governance & Approvals

For FinTech, Healthcare, or sensitive CRMs, you cannot let editors change data unilaterally.

### The Two-Person Rule
```rust
Field::money("balance", "USD").requires_approval()
```
If a user edits the balance, the change is **quarantined**.
The AI Decision Engine assesses the change:
*   *Is it a 500% increase?* -> **Critical Risk**.
*   *Is it a minor 2% adjustment?* -> **Low Risk**.

Managers view the queue in the Admin UI and click "Approve" to commit the change.

---

## 10. Resilience, Scaling, & OpenTelemetry

Vella is built for zero-downtime environments.

### The Self-Healing Pipeline
1.  **Panic Recovery:** If a custom route panics, the HTTP request is safely caught, returning a `500 JSON` error while the server stays alive.
2.  **Circuit Breaker:** Prevents cascading database failures under extreme load.
3.  **Watchdog:** Continuously pings the database and initiates exponential backoff reconnects if the network drops.

### Multi-Node Enterprise Scaling
To scale Vella across Kubernetes clusters, enable the **Redis Backplane** and **OpenTelemetry**:
```rust
VellaApp::new()
    .with_redis("redis://redis.internal:6379")
    .with_opentelemetry("http://otel-collector:4317")
```
WebSockets will now sync seamlessly across hundreds of Vella nodes globally.

---

## 11. Imaginative Use Cases (The Future)

What can you build with Vella?

*   **Global Autonomous AI Support Agent:** Use Vella's Semantic Cache and pgvector. When a user asks a question, Vella checks the cache; if $\ge 0.95$ similar, it responds in $<1ms$. If not, it does a Vector Search on KnowledgeDocs, synthesizes via OpenAI, and Vella logs the tokens and caches the result for the next user.
*   **Collaborative Realtime Canvas:** Use Vella's WebSockets and Rust's raw speed to build a Figma/Notion clone. Every keystroke is saved to SQLite locally, synced to the server, and broadcast to all collaborators.
*   **Edge-Deployed Analytics Engine:** Compile Vella's pure-Rust parsing and validation core to `wasm32-wasi`. Deploy it on Cloudflare Workers to validate and rate-limit million-RPS data streams before piping them to a central Supabase cluster.
*   **Zero-Trust Financial Ledger:** Use the Two-Person Approval workflow for all transactions, backed by the immutable Audit Trail. If a rogue admin deletes a ledger entry, use 1-Click Time Travel to revert the database snapshot instantly.

---

**Vella is more than a framework; it is an intelligent, self-healing foundation for the next decade of web software.**
