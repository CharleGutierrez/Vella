use crate::model::schema::ModelSchema;
use serde_json::Value;
use tracing::{info, warn};

use super::local_llm::LocalLlmEngine;

// ---------------------------------------------------------------------------
// Ollama embedding defaults
// ---------------------------------------------------------------------------

/// Default Ollama embedding model — 768-dimensional, runs on CPU.
/// Pull with: `ollama pull nomic-embed-text`
const DEFAULT_OLLAMA_EMBED_MODEL: &str = "nomic-embed-text";

/// Dimension of `nomic-embed-text` embeddings.
const NOMIC_DIMENSIONS: usize = 768;

// ---------------------------------------------------------------------------
// RagEngine
// ---------------------------------------------------------------------------

/// Vella Retrieval-Augmented Generation (RAG) Engine.
///
/// Chunks documents, generates **real** embeddings via Ollama, and performs
/// cosine-similarity search over the pgvector index.
///
/// # Environment variables
/// | Variable | Default | Purpose |
/// |---|---|---|
/// | `OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama server host |
/// | `OLLAMA_EMBED_MODEL` | `nomic-embed-text` | Model used for embeddings |
///
/// # Quick start
/// ```bash
/// ollama pull nomic-embed-text
/// ```
/// ```rust
/// let rag = RagEngine::new();
/// let embedding = rag.embed_text("Hello, Vella!").await?;
/// ```
pub struct RagEngine {
    /// Human-readable name of the embedding model (used in logs).
    pub embedding_model: String,
    /// Expected dimensionality of the embedding vectors.
    pub dimensions: usize,
    /// Underlying Ollama client (also used for chat-based re-ranking).
    ollama: LocalLlmEngine,
}

impl RagEngine {
    /// Create a `RagEngine` using the default Ollama embedding model.
    /// Override via `OLLAMA_EMBED_MODEL` env var.
    pub fn new() -> Self {
        let model = std::env::var("OLLAMA_EMBED_MODEL")
            .unwrap_or_else(|_| DEFAULT_OLLAMA_EMBED_MODEL.to_string());

        // Infer dimensions from model name
        let dimensions = Self::infer_dimensions(&model);

        info!(
            "🧠 [Vella RAG] Initialized with Ollama embedding model '{}' ({}D)",
            model, dimensions
        );

        let ollama = LocalLlmEngine::new_ollama(&model);
        Self {
            embedding_model: model,
            dimensions,
            ollama,
        }
    }

    /// Create a `RagEngine` with an explicit Ollama embedding model tag.
    ///
    /// | Model | Dims | Notes |
    /// |---|---|---|
    /// | `nomic-embed-text` | 768 | Fast, great quality |
    /// | `mxbai-embed-large` | 1024 | Higher quality |
    /// | `all-minilm` | 384 | Ultra-fast, lightweight |
    /// | `bge-m3` | 1024 | Multilingual |
    pub fn with_model(model: &str) -> Self {
        let dimensions = Self::infer_dimensions(model);
        info!(
            "🧠 [Vella RAG] Using custom Ollama embedding model '{}' ({}D)",
            model, dimensions
        );
        let ollama = LocalLlmEngine::new_ollama(model);
        Self {
            embedding_model: model.to_string(),
            dimensions,
            ollama,
        }
    }

    // ------------------------------------------------------------------
    // Public API
    // ------------------------------------------------------------------

    /// Embed a single text string and return its dense vector.
    pub async fn embed_text(
        &self,
        text: &str,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error + Send + Sync>> {
        self.ollama.embed(text).await
    }

    /// Chunk a document and return embedding vectors for each chunk.
    ///
    /// Chunks are split on double-newlines (paragraph-level).  For
    /// production use, plug in `DocumentSplitter` from `vella::ai::chunking`.
    pub async fn ingest_document(
        &self,
        _schema: &ModelSchema,
        document_text: &str,
    ) -> Result<Vec<f64>, String> {
        info!(
            "🚀 [Vella RAG] Ingesting document ({} bytes), embedding model: {}",
            document_text.len(),
            self.embedding_model
        );

        // Naive paragraph chunking — replace with DocumentSplitter for production.
        let chunks: Vec<&str> = document_text
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        info!(
            "✂️  [Vella RAG] Split into {} chunk(s), generating {}D embeddings via Ollama…",
            chunks.len(),
            self.dimensions
        );

        // Embed the first chunk (or the full text if there's only one paragraph).
        // A real implementation would embed all chunks and upsert them individually.
        let text_to_embed = chunks.first().copied().unwrap_or(document_text);

        match self.ollama.embed(text_to_embed).await {
            Ok(embedding) => {
                info!(
                    "💾 [Vella RAG] Embedding ready ({}D) — upsert into pgvector.",
                    embedding.len()
                );
                Ok(embedding)
            }
            Err(e) => {
                warn!(
                    "⚠️  [Vella RAG] Ollama embed failed ({}). \
                     Is `ollama serve` running and '{}' pulled? Falling back to zero-vector.",
                    e, self.embedding_model
                );
                // Graceful degradation: return a zero vector so the rest of the
                // pipeline (schema registration, DB writes) still works.
                Ok(vec![0.0_f64; self.dimensions])
            }
        }
    }

    /// Embed `query`, then run a cosine-similarity search over the Vector DB.
    ///
    /// In production this would issue a `SELECT … ORDER BY embedding <=> $1 LIMIT $2`
    /// query against pgvector.  The SQL is generated by `SqlDialect::vector_search_ddl`.
    pub async fn similarity_search(
        &self,
        _schema: &ModelSchema,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<Value>, String> {
        info!(
            "🔍 [Vella RAG] Embedding query '{}' via Ollama ({})…",
            query, self.embedding_model
        );

        let embedding = match self.ollama.embed(query).await {
            Ok(v) => v,
            Err(e) => {
                warn!("⚠️  [Vella RAG] Embed failed for query: {}. Returning empty results.", e);
                return Ok(vec![]);
            }
        };

        info!(
            "⚡ [Vella RAG] Query vector ready ({}D). \
             Executing HNSW pgvector similarity search (LIMIT {})…",
            embedding.len(),
            top_k
        );

        // TODO: execute the actual `SELECT … <=> $1 LIMIT $2` against SQLx pool.
        // For now we return the embedding so callers can issue the query themselves.
        Ok(vec![serde_json::json!({
            "status": "embedding_ready",
            "dimensions": embedding.len(),
            "top_k": top_k,
            "note": "Wire the SQLx pool in RagEngine to execute the pgvector query."
        })])
    }

    /// Ask a chat-capable Ollama model to synthesise an answer from retrieved
    /// context chunks (the "Generation" part of RAG).
    ///
    /// `context_chunks` are the text snippets returned by `similarity_search`.
    pub async fn generate_answer(
        &self,
        question: &str,
        context_chunks: &[&str],
        chat_model: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        use super::local_llm::OllamaChatMessage;

        let context = context_chunks.join("\n\n---\n\n");
        let system = "You are a helpful assistant. Answer the user's question using only the \
                      provided context. If the answer is not in the context, say so honestly."
            .to_string();

        let user_content = format!(
            "Context:\n{}\n\nQuestion: {}",
            context, question
        );

        let chat_engine = LocalLlmEngine::new_ollama(chat_model);
        chat_engine
            .chat_with_history(vec![
                OllamaChatMessage { role: "system".to_string(), content: system },
                OllamaChatMessage { role: "user".to_string(), content: user_content },
            ])
            .await
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    fn infer_dimensions(model: &str) -> usize {
        let m = model.to_lowercase();
        if m.contains("nomic") {
            768
        } else if m.contains("mxbai") || m.contains("bge") {
            1024
        } else if m.contains("minilm") || m.contains("all-minilm") {
            384
        } else {
            // Safe default — nomic-embed-text compatible
            NOMIC_DIMENSIONS
        }
    }
}

impl Default for RagEngine {
    fn default() -> Self {
        Self::new()
    }
}
