# Ollama Integration Guide

> Run Vella's AI features **100% locally** — no API keys, no cloud costs, full privacy.

Vella's `UnifiedAiGateway`, `LocalLlmEngine`, `RagEngine`, and `AiScaffolder` all support
[Ollama](https://ollama.com) as a first-class local AI backend.

---

## Prerequisites

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Start the server
ollama serve
```

---

## Recommended Models

| Purpose | Model | Pull Command | Notes |
|---|---|---|---|
| Chat / Agent | `llama3.2` | `ollama pull llama3.2` | Fast, great general quality |
| Schema / Code gen | `qwen2.5-coder` | `ollama pull qwen2.5-coder` | Best for Vella scaffolding |
| Tool calling | `qwen2.5:7b` | `ollama pull qwen2.5:7b` | Required for agent tool loops |
| Embeddings (RAG) | `nomic-embed-text` | `ollama pull nomic-embed-text` | 768-dim, CPU-friendly |
| Embeddings (high-q) | `mxbai-embed-large` | `ollama pull mxbai-embed-large` | 1024-dim |
| Multilingual embed | `bge-m3` | `ollama pull bge-m3` | 1024-dim, 100+ languages |

---

## Environment Variables

| Variable | Default | Purpose |
|---|---|---|
| `OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama server host (change for remote Pi, NAS, etc.) |
| `OLLAMA_EMBED_MODEL` | `nomic-embed-text` | Embedding model used by `RagEngine` |
| `OLLAMA_SCAFFOLD_MODEL` | *(unset)* | Enables Ollama-powered schema scaffolding |

---

## 1. AI Scaffolder CLI

Generate a complete model schema, SQL DDL, and TypeScript types from a natural-language prompt using a local Ollama model:

```bash
# Pull a code-capable model
ollama pull qwen2.5-coder

# Generate a schema (Ollama will be used because GEMINI_API_KEY is not set)
OLLAMA_SCAFFOLD_MODEL=qwen2.5-coder cargo run -- generate model Post \
  --ai "A blog post with title, markdown body, author, tags, view count, and SEO slug" \
  --database sqlite
```

**Priority order for schema generation:**

```
1. GEMINI_API_KEY set  →  Google Gemini (cloud)
2. OLLAMA_SCAFFOLD_MODEL set  →  Local Ollama model
3. (neither)  →  Rule-based offline mock
```

---

## 2. Chat & Generation in Rust

### Single-turn completion

```rust
use vella::ai::LocalLlmEngine;

let engine = LocalLlmEngine::new_ollama("llama3.2");
let reply = engine.generate("Explain what a vector database is in 2 sentences.").await?;
println!("{}", reply);
```

### Multi-turn chat

```rust
use vella::ai::{LocalLlmEngine, OllamaChatMessage};

let engine = LocalLlmEngine::new_ollama("llama3.2");

let reply = engine.chat_with_history(vec![
    OllamaChatMessage { role: "system".to_string(),  content: "You are a Rust expert.".to_string() },
    OllamaChatMessage { role: "user".to_string(),    content: "What is a lifetime?".to_string() },
]).await?;

println!("{}", reply);
```

### Builder-style chaining

```rust
// Point at a remote Ollama server (e.g. a Raspberry Pi on your LAN)
let engine = LocalLlmEngine::new_ollama("mistral")
    .with_base_url("http://192.168.1.42:11434");
```

---

## 3. RAG — Retrieval-Augmented Generation

```rust
use vella::ai::RagEngine;

// Uses OLLAMA_EMBED_MODEL env var, defaults to nomic-embed-text
let rag = RagEngine::new();

// --- Ingest ---
let embedding = rag.ingest_document(&schema, "Your document text here...").await?;
// embedding is a Vec<f64> — serialise to JSON and upsert via DatabaseAdapter::insert()

// --- Search (wired to SqliteDatabase::search_vectors) ---
let results = rag.similarity_search(&db, &schema, "What is Vella?", 5).await?;
for hit in results {
    println!("score={:.4}  record={}", hit.score, hit.record);
}

// --- Full RAG pipeline in one call ---
// Embeds question → searches DB → retrieves top-k records → generates answer
let answer = rag.ask(
    &db,          // SqliteDatabase
    &schema,      // ModelSchema with an "embedding" vector field
    "What is Vella used for?",
    "content",    // the text field to use as LLM context
    5,            // top-k results
    "llama3.2",   // any Ollama chat model
).await?;
println!("{}", answer);

// --- Manual: generate answer from retrieved chunks ---
let answer = rag.generate_answer(
    "What is Vella used for?",
    &["Vella is a Rust web framework...", "It supports RAG and vector search..."],
    "llama3.2",
).await?;
println!("{}", answer);
```

### Use a different embedding model

```rust
// High-quality 1024-dim embeddings
let rag = RagEngine::with_model("mxbai-embed-large");

// Lightweight 384-dim (fastest on CPU)
let rag = RagEngine::with_model("all-minilm");
```

---

## 4. Unified AI Gateway (Ollama as a Provider)

The `UnifiedAiGateway` treats Ollama as an OpenAI-compatible endpoint:

```rust
use vella::ai::{AiConfig, AiProvider, UnifiedAiGateway};

let config = AiConfig {
    provider:  AiProvider::OllamaLocal,
    base_url:  "http://localhost:11434/v1/chat/completions".to_string(),
    api_key:   "ollama".to_string(), // key is ignored by Ollama
    model:     "llama3.2".to_string(),
};

let gateway = UnifiedAiGateway::new();

// Simple generation
let reply = gateway.generate(&config, "What is a blockchain?").await?;

// Autonomous agent loop with tool calling (requires a tool-capable model)
let agent_config = AiConfig { model: "qwen2.5:7b".to_string(), ..config };
let result = gateway.run_autonomous_agent(&agent_config, "Search for Rust 2024 news").await?;

// High-availability: Ollama primary, OpenAI fallback
let openai_config = AiConfig {
    provider: AiProvider::OpenAI,
    base_url: "https://api.openai.com/v1/chat/completions".to_string(),
    api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
    model: "gpt-4o-mini".to_string(),
};
let reply = gateway.generate_with_fallback(&config, &openai_config, "Hello!").await;
```

---

## 5. List Available Local Models

```rust
let engine = LocalLlmEngine::new_ollama("any");
let models = engine.list_models().await?;
for m in models {
    println!("{}", m);
}
```

Or via the Ollama CLI:

```bash
ollama list
```

---

## 6. Docker Compose with Ollama

Add Ollama as a service alongside Vella:

```yaml
# docker-compose.yml
services:
  vella:
    build: .
    ports:
      - "8080:8080"
    environment:
      - OLLAMA_BASE_URL=http://ollama:11434
      - OLLAMA_EMBED_MODEL=nomic-embed-text
      - OLLAMA_SCAFFOLD_MODEL=qwen2.5-coder
    depends_on:
      - ollama

  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
    # Uncomment for GPU support:
    # deploy:
    #   resources:
    #     reservations:
    #       devices:
    #         - capabilities: [gpu]

volumes:
  ollama_data:
```

```bash
docker compose up -d
# Pull models into the running container
docker exec -it vella-ollama-1 ollama pull nomic-embed-text
docker exec -it vella-ollama-1 ollama pull llama3.2
```

---

## Troubleshooting

| Error | Cause | Fix |
|---|---|---|
| `Connection refused` | Ollama not running | Run `ollama serve` |
| `model not found` | Model not pulled | `ollama pull <model>` |
| `/api/embed returned 404` | Old Ollama version | `ollama update` (needs ≥ 0.3.0) |
| Empty embeddings | Wrong endpoint | Ensure `OLLAMA_BASE_URL` has no trailing slash |
| Tool calls not working | Model doesn't support tools | Switch to `qwen2.5`, `llama3.1`, or `hermes3` |
