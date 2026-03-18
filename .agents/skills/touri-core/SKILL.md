---
name: touri-core
description: Core architectural and engineering mandates for the Touri project (Tauri + Axum + HTMX). Use this skill when making any structural, backend, or frontend changes to the Touri codebase.
---

# Touri Core Engineering Skill

This skill provides expert guidance for developing a cross-platform application using Tauri, Axum, HTMX, and Tailwind CSS with Server-Side Rendering (SSR).

## 🏗 Domain-Driven Design (DDD) Workflow

Every feature implementation must follow this sequential flow:

1. **Domain Logic (`src/domain`)**: Define entities, value objects, and repository traits. Logic here must be pure Rust and free of infrastructure dependencies.
2. **Infrastructure (`src/infrastructure`)**: Implement repository traits (e.g., SQLite/SQLx) and external services.
3. **Application Layer (`src/application`)**: Create Axum request handlers and services that coordinate domain logic.
4. **Presentation Layer (`src/presentation`)**: Develop HTML templates and Tailwind styles.

## 📜 Mandatory Rules

- **No `unwrap()`**: All fallible operations must return a `Result`. Use the `thiserror` crate for custom error types. **Never use `unwrap()` or `expect()` outside of test files.**
- **Offline First**: HTMX (`htmx.min.js`) and Tailwind CSS must be bundled locally. **No external CDNs.**
- **SSR Priority**: All core state transitions and UI rendering occur in Axum. The frontend is a thin layer driven by HTMX fragments.
- **Minimal JS**: Use HTMX attributes for dynamic interactions. Vanilla JavaScript is only permitted as a fallback.

## 🔍 Validation & Quality Standards

- **Reproduction**: Bug fixes MUST be preceded by a reproduction test case or script that demonstrates the failure.
- **Verification**: Every task is incomplete until verified through unit/integration tests and manual UI checks in the Tauri window.
- **Linter**: Run `cargo clippy` and `cargo check` after any backend change to ensure structural and behavioral correctness.

## 📂 Project References

- **Architecture**: See [ARCHITECTURE.md](references/ARCHITECTURE.md) for detailed system diagrams.
- **Rules**: See [RULES.md](references/RULES.md) for a comprehensive list of project constraints.
