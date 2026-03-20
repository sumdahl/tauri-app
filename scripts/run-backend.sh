#!/usr/bin/env bash
set -euo pipefail

backend_bin="./src-tauri/target/debug/server"

if [[ "${FORCE_BACKEND_BUILD:-0}" == "1" || ! -x "$backend_bin" ]]; then
  CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build --manifest-path src-tauri/Cargo.toml --bin server
fi

exec "$backend_bin"
