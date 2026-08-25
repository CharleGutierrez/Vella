# 🧠 Unified AI Gateway

Vella's AI Gateway is a native, highly concurrent abstraction layer over the world's most popular LLM providers. Instead of importing 5 different SDKs to support OpenAI, Anthropic, and Google, Vella unifies them into a single Rust interface.

## Supported Providers
* `AiProvider::OpenAI`
* `AiProvider::Anthropic` (Claude 3.x)
* `AiProvider::Gemini`
* `AiProvider::DeepSeek`
* `AiProvider::Grok`
* `AiProvider::OllamaLocal` (Offline Qwen, Llama3)

## Advanced Capabilities

### 1. Tool Calling (Function Calling)
You can pass a JSON schema of tools into the `AiRequest`. This allows the LLM to trigger local Rust functions (e.g., executing a database query) before responding.

### 2. Multimodal Vision
The `AiMessage` struct supports an `image_url` property. If you pass an image, Vella automatically maps the payload to the Vision endpoints for Claude 3.5 Sonnet, GPT-4o, or Gemini 1.5 Pro.

### 3. Server-Sent Event Streaming
For real-time chat applications, Vella supports asynchronous token streaming via `generate_stream()`. This pipes tokens directly from the LLM into Vella's Realtime WebSocket engine.

## The Circuit Breaker (Failover)
Enterprise AI applications cannot afford downtime when an API rate-limits you.

```rust
let response = gateway.generate_with_fallback(&claude, &grok, "Prompt").await;
```
If Claude throws an `HTTP 429 Too Many Requests` or `HTTP 500 Internal Server Error`, Vella intercepts the crash and invisibly routes the prompt to the backup provider (Grok) without dropping the user's WebSocket connection.
