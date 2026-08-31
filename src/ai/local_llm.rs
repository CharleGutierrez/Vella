use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info, warn};

// ---------------------------------------------------------------------------
// Ollama native API types
// ---------------------------------------------------------------------------

/// Request payload for Ollama /api/generate (raw completion).
#[derive(Debug, Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Value>,
}

/// Response payload from Ollama /api/generate.
#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    #[allow(dead_code)]
    done: bool,
}

/// Request payload for Ollama /api/chat (multi-turn).
#[derive(Debug, Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaChatMessage {
    pub role: String,
    pub content: String,
}

/// Response from Ollama /api/chat.
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaChatMessage,
    #[allow(dead_code)]
    done: bool,
}

/// Request payload for Ollama /api/embed (embeddings).
#[derive(Debug, Serialize)]
struct OllamaEmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

/// Response from Ollama /api/embed.
#[derive(Debug, Deserialize)]
struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f64>>,
}

// ---------------------------------------------------------------------------
// LocalLlmEngine — wraps the Ollama REST API
// ---------------------------------------------------------------------------

/// A real Ollama client that talks to a running `ollama serve` instance.
///
/// # Quick start
/// ```rust
/// let engine = LocalLlmEngine::new_ollama("llama3.2");
/// let reply = engine.chat("Why is Rust memory-safe?").await?;
/// ```
pub struct LocalLlmEngine {
    /// Ollama model tag, e.g. `"llama3.2"`, `"qwen2.5-coder"`, `"mistral"`.
    pub model: String,
    /// Base URL of the Ollama server (default: `http://localhost:11434`).
    pub base_url: String,
    client: reqwest::Client,
}

impl LocalLlmEngine {
    // ------------------------------------------------------------------
    // Constructors
    // ------------------------------------------------------------------

    /// Connect to a local Ollama server with the given model.
    /// Falls back to `OLLAMA_BASE_URL` env var, then `http://localhost:11434`.
    pub fn new_ollama(model: &str) -> Self {
        let base_url = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        info!("🦙 Initializing Vella LocalLlmEngine → Ollama at {} (model: {})", base_url, model);
        Self {
            model: model.to_string(),
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Legacy constructor kept for backwards compatibility.
    /// `model_path` is now treated as a model tag (e.g. `"llama3.2"`).
    pub fn new(model_path: &str) -> Self {
        warn!(
            "LocalLlmEngine::new() is deprecated — use LocalLlmEngine::new_ollama(). \
             Treating '{}' as an Ollama model tag.",
            model_path
        );
        Self::new_ollama(model_path)
    }

    // ------------------------------------------------------------------
    // Core generation helpers
    // ------------------------------------------------------------------

    fn generate_url(&self) -> String {
        format!("{}/api/generate", self.base_url)
    }

    fn chat_url(&self) -> String {
        format!("{}/api/chat", self.base_url)
    }

    fn embed_url(&self) -> String {
        format!("{}/api/embed", self.base_url)
    }

    /// Single-turn raw completion via `/api/generate`.
    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("🦙 [Ollama] generate() → model={} prompt_len={}", self.model, prompt.len());

        let body = OllamaGenerateRequest {
            model: &self.model,
            prompt,
            stream: false,
            options: None,
        };

        let res = self
            .client
            .post(&self.generate_url())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("🦙 [Ollama] Connection failed (is `ollama serve` running?): {}", e);
                e
            })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Ollama /api/generate returned {}: {}", status, text).into());
        }

        let parsed: OllamaGenerateResponse = res.json().await?;
        info!("🦙 [Ollama] generate() done, response_len={}", parsed.response.len());
        Ok(parsed.response)
    }

    /// Multi-turn chat via `/api/chat`.
    pub async fn chat(
        &self,
        user_message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.chat_with_history(
            vec![OllamaChatMessage {
                role: "user".to_string(),
                content: user_message.to_string(),
            }],
        )
        .await
    }

    /// Multi-turn chat with full conversation history.
    pub async fn chat_with_history(
        &self,
        messages: Vec<OllamaChatMessage>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "🦙 [Ollama] chat_with_history() → model={} turns={}",
            self.model,
            messages.len()
        );

        let body = OllamaChatRequest {
            model: &self.model,
            messages,
            stream: false,
            options: None,
        };

        let res = self
            .client
            .post(&self.chat_url())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("🦙 [Ollama] Connection failed (is `ollama serve` running?): {}", e);
                e
            })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Ollama /api/chat returned {}: {}", status, text).into());
        }

        let parsed: OllamaChatResponse = res.json().await?;
        Ok(parsed.message.content)
    }

    // ------------------------------------------------------------------
    // Schema & DDL generation (preserves old API surface)
    // ------------------------------------------------------------------

    /// Generate SQL DDL for a schema from a natural-language prompt.
    /// Uses Ollama instead of the previous mock.
    pub async fn generate_schema_ddl(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("🦙 [Ollama] generate_schema_ddl() prompt: {}", prompt);

        let full_prompt = format!(
            "You are a senior database architect. \
             Generate only the SQL DDL (CREATE TABLE statement) for the following requirement. \
             Respond with raw SQL only, no explanation, no markdown fences.\n\n\
             Requirement: {}",
            prompt
        );

        self.generate(&full_prompt).await
    }

    // ------------------------------------------------------------------
    // Embeddings
    // ------------------------------------------------------------------

    /// Generate a dense embedding vector using an Ollama embedding model.
    ///
    /// Recommended model: `nomic-embed-text` (768-dim) or `mxbai-embed-large` (1024-dim).
    ///
    /// ```bash
    /// ollama pull nomic-embed-text
    /// ```
    pub async fn embed(
        &self,
        text: &str,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error + Send + Sync>> {
        info!("🦙 [Ollama] embed() → model={} text_len={}", self.model, text.len());

        let body = OllamaEmbedRequest {
            model: &self.model,
            input: text,
        };

        let res = self
            .client
            .post(&self.embed_url())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("🦙 [Ollama] Embed connection failed: {}", e);
                e
            })?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Ollama /api/embed returned {}: {}", status, text).into());
        }

        let parsed: OllamaEmbedResponse = res.json().await?;
        let embedding = parsed
            .embeddings
            .into_iter()
            .next()
            .ok_or("Ollama returned empty embeddings array")?;

        info!("🦙 [Ollama] embed() done, dims={}", embedding.len());
        Ok(embedding)
    }

    // ------------------------------------------------------------------
    // Utility
    // ------------------------------------------------------------------

    /// List all models available in the local Ollama instance.
    pub async fn list_models(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/tags", self.base_url);
        let res: Value = self.client.get(&url).send().await?.json().await?;
        let names = res["models"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        Ok(names)
    }

    /// Returns a builder-style clone with a different model tag.
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Returns a builder-style clone pointing at a different Ollama host.
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }
}
