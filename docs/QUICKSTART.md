# 🚀 Vella Quickstart: Build an AI-App in 5 Minutes

Vella is designed to be ridiculously easy to start while retaining industrial-grade performance. This guide will walk you through building your first AI-native application with Vella.

## 1. Installation

You can install Vella via Cargo or clone the repository directly.

```bash
# Clone the repository
git clone https://github.com/CharleGutierrez/Vella.git
cd Vella

# Build the project
cargo build --release
```

## 2. Setting up the App Engine

Create a new file called `main.rs`. We are going to initialize the `VellaApp` engine.

```rust
use vella::app::VellaApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = VellaApp::new();
    
    // Boot the server
    app.run().await?;
    
    Ok(())
}
```

## 3. Adding the AI Gateway

Let's plug Anthropic's Claude into our Vella engine. We will use the `UnifiedAiGateway` which automatically handles formatting and fallbacks.

```rust
use vella::app::VellaApp;
use vella::ai::{UnifiedAiGateway, AiConfig, AiProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the AI Gateway
    let gateway = UnifiedAiGateway::new();

    // Configure the AI Provider
    let config = AiConfig {
        provider: AiProvider::Anthropic,
        base_url: "https://api.anthropic.com/v1/messages".to_string(),
        api_key: std::env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
        model: "claude-3-5-sonnet-20240620".to_string(),
    };

    // Ask it a question
    let response = gateway.generate(&config, "What is Rust?").await?;
    println!("AI Says: {}", response);
    
    Ok(())
}
```

## 4. Run it!

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cargo run
```

## Next Steps
You just executed a highly optimized LLM query through Vella! In the next tutorials, we will explore:
* **Tool Calling**: Allowing the AI to run local Rust functions.
* **GIS Support**: Saving spatial `Polygon` data into the Database.
* **WASM Edge Pipelines**: Executing data science models in WebAssembly.
