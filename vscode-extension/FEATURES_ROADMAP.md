# Vella VS Code Extension - Features Roadmap

This document outlines the product roadmap for the Vella VS Code extension, covering both essential quality-of-life improvements and advanced, modern capabilities to enhance developer productivity.

## 🌟 Essential Features

The foundation of the Vella VS Code extension. These features ensure a smooth and integrated developer experience for day-to-day tasks.

*   **Vella Backend Server Management:**
    *   Start, stop, and restart the Vella backend server directly from the VS Code status bar or command palette.
    *   Integrated output channel for viewing server logs.
*   **Auto-Sync Frontend SDKs:**
    *   Integrated file-watchers that automatically detect changes to backend schemas and instantly re-generate and sync the corresponding frontend SDKs (TypeScript, Python, etc.).
*   **Intelligent Snippets & Auto-completion:**
    *   Rich snippets for rapid `ModelSchema` creation, API route definitions, and configuration files.
    *   Context-aware auto-completion for Vella-specific Rust macros and traits.
*   **Basic Rust Error Parsing & Diagnostics:**
    *   Enhanced parsing of Rust compiler errors specifically tailored to Vella's architecture, providing clearer, actionable diagnostics and quick-fix suggestions within the editor.

## 🚀 Modern & Advanced Features

Pushing the boundaries of what an IDE extension can do, these features transform VS Code into a comprehensive control center for Vella projects.

*   **Visual Schema Builder (Prisma Studio-like):**
    *   An interactive GUI panel allowing developers to drag-and-drop database models, define relationships visually, and instantly auto-generate the underlying Rust backend code (`ModelSchema` structs, migrations).
*   **Live Telemetry & Analytics Dashboard:**
    *   A rich Webview panel for visualizing real-time data streams directly in the editor.
    *   Pre-built widget templates for monitoring HFT (High-Frequency Trading) Limit Order Books, SCADA IoT metrics, and system performance telemetry.
*   **Web3 Control Center:**
    *   A dedicated panel for decentralized operations without leaving the editor.
    *   Features include:
        *   Generating and managing ECDSA key pairs.
        *   Viewing, pinning, and managing IPFS CIDs.
        *   Compiling and deploying Smart Contracts directly to testnets/mainnets.
*   **AI Scaffolder Chat (Gemini Integration):**
    *   Integration of Vella's `gemini_scaffolder.rs` into a persistent VS Code sidebar chat.
    *   Developers can prompt the AI using natural language (e.g., "Build an ERP inventory management module" or "Scaffold a Forex trading bot logic") and the AI will generate, explain, and insert the required Rust and configuration code directly into the workspace.
