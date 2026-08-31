use crate::ai::vector::{DistanceMetric, VectorSearchQuery, VectorSearchResult};
use crate::db::SqliteDatabase;
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
/// Chunks documents, generates **real** embeddings via Ollama, and executes
/// cosine-similarity search against the Vella in-memory vector index
/// (backed by [`SqliteDatabase::search_vectors`]).
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
/// let results = rag.similarity_search(&db, &schema, "What is Vella?", 5).await?;
/// ```
pub struct RagEngine {
    /// Human-readable name of the embedding model (used in logs).
    pub embedding_model: String,
    /// Expected dimensionality of the embedding vectors.
    pub dimensions: usize,
    /// Underlying Ollama client (also used for chat-based generation).
    ollama: LocalLlmEngine,
}

impl RagEngine {
    /// Create a `RagEngine` using the default Ollama embedding model.
    /// Override via `OLLAMA_EMBED_MODEL` env var.
    pub fn new() -> Self {
        let model = std::env::var("OLLAMA_EMBED_MODEL")
            .unwrap_or_else(|_| DEFAULT_OLLAMA_EMBED_MODEL.to_string());

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
    // Embedding
    // ------------------------------------------------------------------

    /// Embed a single text string and return its dense vector.
    pub async fn embed_text(
        &self,
        text: &str,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error + Send + Sync>> {
        self.ollama.embed(text).await
    }

    // ------------------------------------------------------------------
    // Ingest
    // ------------------------------------------------------------------

    /// Chunk a document and return an embedding vector for the first chunk.
    ///
    /// In production, pass all chunk vectors to `DatabaseAdapter::insert()`
    /// with the `embedding` field serialised as a JSON array.
    ///
    /// # Graceful degradation
    /// If Ollama is unavailable, returns a zero-vector so schema registration
    /// and DB writes can still succeed.
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

        let text_to_embed = chunks.first().copied().unwrap_or(document_text);

        match self.ollama.embed(text_to_embed).await {
            Ok(embedding) => {
                info!(
                    "💾 [Vella RAG] Embedding ready ({}D) — upsert into DB.",
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
                Ok(vec![0.0_f64; self.dimensions])
            }
        }
    }

    // ------------------------------------------------------------------
    // Similarity search — wired to SqliteDatabase::search_vectors
    // ------------------------------------------------------------------

    /// Embed `query` via Ollama, then run cosine-similarity search against
    /// the Vella in-memory vector index powered by [`SqliteDatabase`].
    ///
    /// The results are full JSON records sorted by descending similarity score.
    ///
    /// ```rust
    /// let rag = RagEngine::new();
    /// let hits = rag.similarity_search(&db, &schema, "What is Vella?", 5).await?;
    /// for hit in hits {
    ///     println!("score={:.4} record={}", hit.score, hit.record);
    /// }
    /// ```
    pub async fn similarity_search(
        &self,
        db: &SqliteDatabase,
        schema: &ModelSchema,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<VectorSearchResult>, String> {
        info!(
            "🔍 [Vella RAG] Embedding query '{}' via Ollama ({})…",
            query, self.embedding_model
        );

        // 1. Embed the query
        let embedding_f64 = match self.ollama.embed(query).await {
            Ok(v) => v,
            Err(e) => {
                warn!(
                    "⚠️  [Vella RAG] Embed failed for query: {}. Returning empty results.",
                    e
                );
                return Ok(vec![]);
            }
        };

        // Convert f64 → f32 for VectorSearchQuery (SqliteDatabase uses f32 internally)
        let query_vector: Vec<f32> = embedding_f64.iter().map(|&v| v as f32).collect();

        info!(
            "⚡ [Vella RAG] Query vector ready ({}D). \
             Executing in-memory HNSW cosine search (LIMIT {})…",
            query_vector.len(),
            top_k
        );

        // 2. Build the search query
        let vsq = VectorSearchQuery {
            model: self.embedding_model.clone(),
            vector_field: "embedding".to_string(),
            query_vector,
            top_k,
            metric: DistanceMetric::Cosine,
        };

        // 3. Delegate to SqliteDatabase's built-in vector search engine
        use crate::db::adapter::DatabaseAdapter;
        db.search_vectors(schema, &vsq)
            .await
            .map_err(|e| format!("Vector search failed: {}", e))
    }

    /// Convenience wrapper that returns plain `Vec<Value>` (records only, no scores).
    /// Useful when you only need the content for the generation step.
    pub async fn search_records(
        &self,
        db: &SqliteDatabase,
        schema: &ModelSchema,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<Value>, String> {
        let results = self.similarity_search(db, schema, query, top_k).await?;
        Ok(results.into_iter().map(|r| r.record).collect())
    }

    // ------------------------------------------------------------------
    // Generation (the "G" in RAG)
    // ------------------------------------------------------------------

    /// Ask a chat-capable Ollama model to synthesise an answer from retrieved
    /// context chunks.
    ///
    /// `context_chunks` are the text snippets returned from your DB records.
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

        let user_content = format!("Context:\n{}\n\nQuestion: {}", context, question);

        let chat_engine = LocalLlmEngine::new_ollama(chat_model);
        chat_engine
            .chat_with_history(vec![
                OllamaChatMessage { role: "system".to_string(), content: system },
                OllamaChatMessage { role: "user".to_string(), content: user_content },
            ])
            .await
    }

    /// Full RAG pipeline in one call:
    /// 1. Embed `question` → similarity search → retrieve top-k records
    /// 2. Extract `text_field` from each record as context
    /// 3. Ask `chat_model` to generate an answer grounded in that context
    pub async fn ask(
        &self,
        db: &SqliteDatabase,
        schema: &ModelSchema,
        question: &str,
        text_field: &str,
        top_k: usize,
        chat_model: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("🤖 [Vella RAG] ask() pipeline: search → retrieve → generate");

        let records = self
            .search_records(db, schema, question, top_k)
            .await
            .unwrap_or_default();

        let chunks: Vec<&str> = records
            .iter()
            .filter_map(|r| r[text_field].as_str())
            .collect();

        if chunks.is_empty() {
            info!("⚠️  [Vella RAG] No matching records found — asking LLM without context.");
            let engine = LocalLlmEngine::new_ollama(chat_model);
            return engine.chat(question).await;
        }

        info!(
            "📚 [Vella RAG] Retrieved {} context chunk(s). Generating answer…",
            chunks.len()
        );

        self.generate_answer(question, &chunks, chat_model).await
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
            NOMIC_DIMENSIONS
        }
    }
}

impl Default for RagEngine {
    fn default() -> Self {
        Self::new()
    }
}
