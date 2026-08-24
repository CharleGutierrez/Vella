# 🐧 Vella: Linux Onboarding Guide

Setting up **Vella** on a brand-new Linux machine (Ubuntu, Debian, Fedora, Arch, etc.) is the most performant way to run the engine. Because Vella is built on Rust, it compiles to raw native machine code, taking full advantage of Linux's advanced epoll networking and process schedulers.

Just like on Windows, you **do not** need Docker, Kubernetes, or a heavy PostgreSQL installation to get started locally.

Here is the complete, step-by-step guide to going from a fresh Linux install to running your first AI-native Vella backend.

---

### Phase 1: Prepare the Linux Environment
A new Linux machine needs basic C/C++ compilation tools, OpenSSL (for secure API requests), and the Rust compiler.

1. **Open your Terminal**.
2. **Install Build Essentials and Git:**
   *If you are on Ubuntu/Debian/Linux Mint:*
   ```bash
   sudo apt update
   sudo apt install -y build-essential curl git pkg-config libssl-dev
   ```
   *If you are on Fedora/RHEL:*
   ```bash
   sudo dnf groupinstall "Development Tools"
   sudo dnf install curl git pkgconf-pkg-config openssl-devel
   ```
   *If you are on Arch Linux:*
   ```bash
   sudo pacman -Syu base-devel curl git
   ```

3. **Install Rust** by running the official Rustup script:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   *(When prompted, simply press `1` and hit `Enter` for the default installation).*
4. **Refresh your terminal** to load the new `cargo` commands:
   ```bash
   source "$HOME/.cargo/env"
   ```

---

### Phase 2: Create Your First Vella Project
Now that your Linux machine is ready, let's create the backend. 

1. **Create a new Rust application:**
   ```bash
   cargo new my_vella_backend
   cd my_vella_backend
   ```
2. **Add Vella to your project dependencies:**
   Open the `Cargo.toml` file in `nano`, `vim`, or VS Code, and add these lines under `[dependencies]`:
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
        .site_name("My New Linux App")
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
1. Go back to your terminal and compile the engine:
   ```bash
   cargo run
   ```
   *Note: Because this is a fresh Linux install, the first compile might take 1–2 minutes as Rust downloads and compiles the Axum web framework and OpenTelemetry crates. Every run after this will be nearly instant.*

2. **Access the Headless CMS:**
   Once the terminal outputs `Vella Engine listening on 127.0.0.1:8080`, open your web browser (Firefox, Chrome, or Brave) and navigate to:
   👉 **`http://127.0.0.1:8080`**

You will instantly see the Vella Admin SPA (Single Page Application). From there, you can view your dynamically generated `Article` table, add records, and begin interacting with Vella's REST APIs and Vector endpoints natively on Linux!
