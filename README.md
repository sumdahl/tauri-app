# Tauri App

A cross-platform desktop application built with Tauri, SvelteKit, and TypeScript.

## Tech Stack

- **Frontend**: SvelteKit 2 + TypeScript + Svelte 5
- **Backend**: Tauri v2 (Rust)
- **Styling**: Tailwind CSS v4
- **Icons**: Lucide Svelte
- **Forms**: Superforms + Formsnap + Zod
- **UI Components**: bits-ui (shadcn-svelte)
- **Date Handling**: @internationalized/date

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version recommended)
- [Rust](https://www.rust-lang.org/) toolchain
- [pnpm](https://pnpm.io/) (recommended) or npm/yarn
- For Android development: [Android Studio](https://developer.android.com/studio) with Android SDK
- For iOS development: [Xcode](https://developer.apple.com/xcode/) (macOS only)

### Installation

```bash
# Install dependencies
pnpm install
```

### Development

```bash
# Start the development server (Desktop)
pnpm tauri dev

# Start the Android development server
pnpm tauri android dev

# Start the iOS development server
pnpm tauri ios dev
```

This command starts the Vite development server and launches the Tauri application.

### Build

```bash
# Build for production (Desktop)
pnpm tauri build

# Build for Android
pnpm tauri android build

# Build for iOS
pnpm tauri ios build
```

The built application will be in `src-tauri/target/release/bundle`.

## Project Structure

```
tauri-app/
├── src/                    # SvelteKit frontend source
│   ├── lib/               # Components and utilities
│   │   ├── api.ts         # Tauri API wrappers
│   │   └── components/    # UI components
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Tauri/Rust backend source
│   ├── src/               # Rust source files
│   │   ├── lib.rs         # Library entry point
│   │   ├── main.rs        # Application entry point
│   │   ├── commands/      # Tauri command handlers
│   │   ├── error.rs       # Error handling
│   │   └── state.rs       # Application state
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── static/                # Static assets
├── package.json            # Node.js dependencies
└── svelte.config.js       # SvelteKit configuration
```

## Features

- Cross-platform desktop app (Windows, macOS, Linux)
- Mobile support (Android, iOS)
- Native system integration via Tauri
- Hot module replacement during development
- Type-safe communication between frontend and backend
- Application state management
- Logging with env_logger
- Error handling with thiserror
- Form validation with Zod + Superforms


## For UI Components , we use ShadCN-Svelte
[https://www.shadcn-svelte.com/themes]

## For Form , use Formsnap & Superforms
[https://svelte-4.shadcn-svelte.com/docs/components/form]

## License

MIT
