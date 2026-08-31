# Changelog

All notable changes to the Vella framework will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- **Ollama Integration** — Full local AI support via [Ollama](https://ollama.com):
  - `LocalLlmEngine` rewritten as a real Ollama HTTP client (`/api/generate`, `/api/chat`, `/api/embed`).
  - `RagEngine` now generates **real embeddings** via Ollama (`nomic-embed-text`, `mxbai-embed-large`, etc.) instead of mock zero-vectors.
  - `RagEngine::generate_answer()` new method for the RAG generation step using any local chat model.
  - `AiScaffolder` gains `OLLAMA_SCAFFOLD_MODEL` env-var support — set it to use a local model (e.g. `qwen2.5-coder`) for schema generation without any cloud API key.
  - `OllamaChatMessage` type exported from `vella::ai` for multi-turn conversation building.
  - `OLLAMA_INTEGRATION.md` — comprehensive integration guide with Docker Compose examples, model recommendations, and troubleshooting.
- Fully automated CI/CD pipelines (Stale bot, Dependabot, Auto-Labeler, Rustdoc GitHub Pages).
- Multi-stage Dockerization (`Dockerfile` and `docker-compose.yml`) for Zero-Setup deployment.
- God-Tier System Architecture documentation (`ARCHITECTURE.md`).
- Cross-platform auto-compiled binary releases for Windows, macOS, and Linux.
- Open source community standards (`CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`).
- `README.zh-CN.md` for massive global reach.
- `CODEOWNERS` mapping for enterprise-grade repository governance.
