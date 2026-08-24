# Project Map - vella

## Project purpose
Next-Generation LLM-Native Rust Web Engine & Headless CMS. Bridging PocketBase and Supabase with native vector search, agentic schema scaffolding, AI middleware, zero-config TypeScript sync, realtime WebSocket/SSE, and multi-database scale.

## Stack
- Framework: Axum with Tower middleware
- Database: sqlx (SQLite, PostgreSQL, MySQL)
- Realtime: WebSocket (ws) and SSE (tokio-stream)
- Observability: OpenTelemetry, tracing-subscriber
- Caching/State: Redis

## Structure
- `src/` — core engine
  - `src/main.rs` — CLI entry point
  - `src/lib.rs` — library root
  - `src/app.rs` — main application server and routing setup
  - `src/ai/` — AI engine (vector search, tuner, middleware, RAG capabilities)
  - `src/api/` — Axum HTTP handlers (CRUD, auth, realtime, OpenAPI, filters)
  - `src/audit/` — audit logging and approval workflow engine
  - `src/auth/` — RBAC, OAuth, cryptographic token extractors
  - `src/core/` — config, hooks, resilience (watchdog, circuit breakers), WASM
  - `src/db/` — database adapters, migrator, generic query builder
  - `src/model/` — dynamic schema definitions, field validation, registry
  - `src/realtime/` — SSE and WebSocket hub managers
  - `src/ui/` — built-in admin UI, SDK generators (React, Vue, Angular)
- `examples/` — showcase applications
  - `examples/demo.rs` — basic CMS demo
  - `examples/todo_app.rs` — realtime todo app
  - `examples/rag_ai_app.rs` — AI RAG application
  - `examples/ultimate_demo.rs` — comprehensive feature showcase

## Relationships
- `src/main.rs` builds and runs `src/app.rs`.
- `src/api/` handlers use `src/auth/` for RBAC extraction and validation.
- `src/api/` dynamically generates queries via `src/db/` against `src/model/` schemas.
- `src/app.rs` initializes `src/realtime/hub.rs` to broadcast events from `src/core/events.rs`.
- `src/api/handlers/ai.rs` processes vector data using `src/ai/vector.rs` and `src/ai/middleware.rs`.
