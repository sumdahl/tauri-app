# Tauri App

A cross-platform desktop application built with Tauri, SvelteKit, and TypeScript.

## Tech Stack

- **Frontend**: SvelteKit 2 + TypeScript
- **Backend**: Tauri v2 (Rust)
- **Styling**: Tailwind CSS v4
- **Icons**: Lucide Svelte

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version recommended)
- [Rust](https://www.rust-lang.org/) toolchain
- [pnpm](https://pnpm.io/) (recommended) or npm/yarn

### Installation

```bash
# Install dependencies
pnpm install
```

### Development

```bash
# Start the development server
pnpm tauri dev
```

This command starts the Vite development server and launches the Tauri application.

### Build

```bash
# Build for production
pnpm tauri build
```

The built application will be in `src-tauri/target/release/bundle`.

## Project Structure

```
tauri-app/
├── src/                    # SvelteKit frontend source
│   ├── lib/               # Components and utilities
│   │   └── components/    # UI components
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Tauri/Rust backend source
│   ├── src/               # Rust source files
│   │   ├── lib.rs         # Library entry point
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── static/                # Static assets
├── package.json            # Node.js dependencies
└── svelte.config.js       # SvelteKit configuration
```

## Features

- Cross-platform desktop app (Windows, macOS, Linux)
- Native system integration via Tauri
- Hot module replacement during development
- Type-safe communication between frontend and backend


## For UI Components , we use ShadCN-Svelte
[https://www.shadcn-svelte.com/themes]

## For Form , use Formsnap & Superforms
[https://svelte-4.shadcn-svelte.com/docs/components/form]

## License

MIT
