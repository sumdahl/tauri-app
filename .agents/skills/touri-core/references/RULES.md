# Project Rules & Guidelines

All agents and developers working on this project must adhere to the following mandates:

## 🦀 Rust Coding Standards
1. **Strict Error Handling**: Use of `unwrap()` or `expect()` is strictly forbidden in production code. Use `Result` and handle errors gracefully using custom error types (e.g., via `thiserror`).
2. **Domain Integrity**: Domain logic in `src/domain` must not depend on external infrastructure (like databases or Axum directly). Use traits (repository pattern) to abstract infrastructure.
3. **Async first**: All handlers and I/O operations should utilize `tokio` and `async/await`.

## 🌐 Frontend & Rendering
1. **HTMX over JavaScript**: Dynamic interactions must use HTMX attributes (`hx-get`, `hx-post`, `hx-swap`, etc.) where possible. Vanilla JavaScript is a last resort.
2. **Local Assets ONLY**: Do not use CDNs for HTMX or Tailwind. All libraries must be bundled locally and served from the application to support full offline functionality.
3. **Tailwind Processing**: Use the official Tailwind CLI/build process. Do not use the JIT script.
4. **Server-Side Rendering (SSR)**: The backend (Axum) is responsible for rendering all UI components as HTML fragments.

## 📦 Tauri Integration
1. **Cross-Platform Focus**: Always consider how changes affect desktop (Linux, macOS, Windows) and mobile (Android, iOS) distributions.
2. **Minimal Tauri Bridge**: Use Tauri's inter-process communication (IPC) primarily for system-level APIs. Keep core application state and logic in the Axum layer.
