# 📖 Vella: The Comprehensive Reference Manual

Welcome to the definitive guide for **Vella** — the ultra-fast, AI-native Headless CMS and industrial backend engine. This manual covers every feature, command, and configuration necessary to adapt Vella to your local development, enterprise edge, or factory-floor environment.

---

## Table of Contents
1. [Core Philosophy](#1-core-philosophy)
2. [CLI & Environment Setup](#2-cli--environment-setup)
3. [The Schema Builder (ORM)](#3-the-schema-builder-orm)
4. [The Unified AI Gateway](#4-the-unified-ai-gateway)
5. [Real-Time Data (WebSockets & SSE)](#5-real-time-data-websockets--sse)
6. [Industrial & SCADA Extensions](#6-industrial--scada-extensions)
7. [WASM Compute Engine](#7-wasm-compute-engine)
8. [Security, Roles, & Resilience](#8-security-roles--resilience)
9. [Deployment & Production](#9-deployment--production)

---

## 1. Core Philosophy
Vella replaces the standard stack (Express/Django + Postgres + Redis + OpenAI SDK) with a **single, highly-concurrent Rust binary**.
* **Memory Safety:** Written in Rust, immune to null-pointer dereferences and data races.
* **Edge-Native:** Compiles down to a tiny, fast-booting binary suitable for AWS Lambda, Cloudflare Workers, or IoT Raspberry Pis.
* **AI-First:** LLMs are treated as first-class citizens, not bolted-on SDKs.

---

## 2. CLI & Environment Setup

### Environment Variables
Create a `.env` file in the root of your project. Vella automatically loads these on boot.

```env
# Database
DATABASE_URL=postgres://user:pass@localhost:5432/vella
DATABASE_MAX_CONNECTIONS=100

# AI Keys
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-...
GEMINI_API_KEY=AIza...

# Network
PORT=8080
HOST=0.0.0.0

# Telemetry
RUST_LOG=info,vella=debug
ENABLE_1000HZ_IPC=true
```

### Commands
Vella leverages Cargo for execution and building.

* `cargo run`: Boots the development server.
* `cargo run --release`: Compiles the highly-optimized production binary.
* `cargo test`: Executes the 60+ internal unit and integration tests (including AI mocks and SCADA logic).

---

## 3. The Schema Builder (ORM)

Vella uses a fluent builder pattern to dynamically generate your database tables, REST APIs, and TypeScript frontend SDKs at runtime.

### Defining a Model
```rust
use vella::model::{Schema, Field, FieldType};

let mut user_schema = Schema::new("Users")
    .description("Customer user accounts")
    .field(Field {
        name: "email".to_string(),
        field_type: FieldType::String,
        unique: true,
        ..Default::default()
    })
    .with_timestamps();
```

### GIS & Spatial Fields
Vella supports native PostGIS integration.

```rust
.field(Field {
    name: "delivery_zone".to_string(),
    field_type: FieldType::Polygon,
    spatial_indexed: true, // Auto-generates GiST index
    ..Default::default()
})
```
*Supported Spatial Types:* `FieldType::Point`, `FieldType::Polygon`, `FieldType::Geometry`.

---

## 4. The Unified AI Gateway

Vella abstracts away all LLM proprietary formats.

### Initializing the Gateway
```rust
use vella::ai::{UnifiedAiGateway, AiConfig, AiProvider, AiRequest, AiMessage};
let gateway = UnifiedAiGateway::new();
```

### Standard Chat Generation
```rust
let config = AiConfig {
    provider: AiProvider::Anthropic,
    base_url: "https://api.anthropic.com/v1/messages".to_string(),
    api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
    model: "claude-3-5-sonnet-20240620".to_string(),
};

let response = gateway.generate(&config, "Write a Haiku about Rust.").await?;
```

### The AI Circuit Breaker (Failover Routing)
Never let an API outage crash your app.

```rust
let primary = config; // Claude
let backup = AiConfig { provider: AiProvider::OllamaLocal, ... }; // Local Llama3

// If Anthropic is down, Vella instantly hits your local server
let response = gateway.generate_with_fallback(&primary, &backup, "Hello").await;
```

---

## 5. Real-Time Data (WebSockets & SSE)

Vella natively broadcasts database mutations (INSERT, UPDATE, DELETE) to listening frontends.

### Connecting via Frontend
Vella auto-generates a WebSocket endpoint at `/api/realtime`.
```javascript
const socket = new WebSocket('ws://localhost:8080/api/realtime');
socket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (data.action === "INSERT" && data.collection === "Users") {
        console.log("New User joined:", data.record);
    }
};
```

---

## 6. Industrial & SCADA Extensions

If you are running Vella on a factory floor or racecar:

* **1000Hz Telemetry:** Vella spins up a dedicated, non-blocking Tokio thread that listens on UDP for high-frequency data packets.
* **Swinging Door Compression:** Enable this in your `.env` to silently compress time-series sensor data by up to 90% before writing it to Postgres.
* **ISA-18.2 Alarms:** Vella tracks sensor states (Normal, Unacknowledged, Acknowledged, Cleared) directly in memory for HMI dashboards.

---

## 7. WASM Compute Engine

Upload Python scripts (compiled via Pyodide/WASM) into Vella to execute custom data pipelines without network latency.

1. Compile your script to `pipeline.wasm`.
2. Place it in `/vella/wasm/`.
3. Vella maps the data from Apache Arrow directly into the WASM memory buffer, executes the logic, and returns the result to the HTTP router.

---

## 8. Security, Roles, & Resilience

* **Row-Level Security (RLS):** Policies are evaluated at the AST level before SQL generation.
* **Rate Clamping:** Built-in DDoS protection throttles abusive IPs via a highly concurrent `parking_lot` memory lock.
* **SQL Injection Immunity:** All queries use prepared statements with strict parameterization; raw strings are never executed.

---

## 9. Deployment & Production

Vella compiles down to a single binary.

### Dockerizing Vella
```dockerfile
FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/vella /usr/local/bin/vella
CMD ["vella"]
```

### Systemd Configuration (Linux)
If running directly on a Linux server without Docker:

```ini
[Unit]
Description=Vella AI Engine
After=network.target postgresql.service

[Service]
ExecStart=/usr/local/bin/vella
Restart=always
User=vella
EnvironmentFile=/etc/vella/.env

[Install]
WantedBy=multi-user.target
```
