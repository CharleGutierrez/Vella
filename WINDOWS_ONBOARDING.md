# 🪟 Vella: Windows Onboarding Guide

Setting up **Vella** on a brand-new Windows 10 or Windows 11 computer is incredibly straightforward. Because Vella is built on Rust and utilizes SQLite by default, you **do not** need to install heavy dependencies like Docker, WSL (Windows Subsystem for Linux), PostgreSQL, or Redis to get started locally.

Here is the complete, step-by-step guide to going from a factory-fresh Windows PC to running your first AI-native Vella backend.

---

### Phase 1: Prepare the Windows Environment
A new Windows machine needs the Rust compiler and Git. The easiest way to install these natively is by using Windows Package Manager (`winget`) via PowerShell.

1. **Open PowerShell as Administrator** (Press the Windows Key, type `PowerShell`, right-click, and select "Run as Administrator").
2. **Install Git & C++ Build Tools:** Rust requires Microsoft C++ build tools to compile code. Run this command:
   ```powershell
   winget install --id Git.Git -e --source winget
   winget install --id Microsoft.VisualStudio.2022.BuildTools --force
   ```
3. **Install Rust** by running the official Rustup installer:
   ```powershell
   winget install --id Rustlang.Rustup -e --source winget
   ```
   *(Note: A black terminal window may pop up asking for installation options. Just press `1` and hit `Enter` for the default installation).*
4. **Restart your computer** (or close and reopen PowerShell) so Windows recognizes the newly installed `cargo` commands in your system PATH.

---

### Phase 2: Create Your First Vella Project
Now that your PC is ready, let's create the backend. Open a normal PowerShell window and run:

1. **Create a new Rust application:**
   ```powershell
   cargo new my_vella_backend
   cd my_vella_backend
   ```
2. **Add Vella to your project dependencies:**
   Open the `Cargo.toml` file in Notepad, VS Code, or your preferred editor, and add these lines under `[dependencies]`:
   ```toml
   [dependencies]
   vella = { git = "https://github.com/CharleGutierrez/Vella.git", branch = "main" }
   tokio = { version = "1.38", features = ["full"] }
   ```

---

### Phase 3: Write Your Engine Code
Open the `src/main.rs` file and replace everything inside it with the Vella quickstart code. This will dynamically create a local SQLite database and launch the Headless CMS automatically.

```rust
use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    // 1. Define your Database Schema natively in Rust
    let article_schema = ModelSchema::new("Article")
        .field(Field::string("title").required().searchable())
        .field(Field::string("content"))
        .field(Field::r#enum("status", vec!["Draft", "Published"]))
        .with_timestamps();

    // 2. Launch the Vella Engine!
    VellaApp::new()
        .site_name("My New Windows App")
        .bind("127.0.0.1:8080")
        // Uses a local SQLite file - no Postgres or Docker installation required!
        .database("sqlite://local_dev.db?mode=rwc") 
        .register(article_schema)
        .run()
        .await?;

    Ok(())
}
```

---

### Phase 4: Run and Experience Vella
1. Go back to your PowerShell window and compile the engine:
   ```powershell
   cargo run
   ```
   *Note: Because this is a fresh Windows PC, the first compile might take 1–2 minutes as Rust downloads and compiles the Axum web framework and OpenTelemetry crates. Every run after this will be nearly instant.*

2. **Access the Headless CMS:**
   Once the terminal outputs `Vella Engine listening on 127.0.0.1:8080`, open Microsoft Edge, Chrome, or Firefox and navigate to:
   👉 **`http://127.0.0.1:8080`**

You will instantly see the Vella Admin SPA (Single Page Application). From there, you can view your dynamically generated `Article` table, add records, and begin interacting with Vella's REST APIs and Vector endpoints natively on Windows!
