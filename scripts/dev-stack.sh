#!/usr/bin/env bash
set -euo pipefail

backend_pid=""
backend_address="${BACKEND_ADDRESS:-127.0.0.1:3000}"
backend_host="${backend_address%:*}"
backend_port="${backend_address##*:}"

if [[ "$backend_host" == "0.0.0.0" || "$backend_host" == "::" ]]; then
  backend_host="127.0.0.1"
fi

backend_health_url="http://${backend_host}:${backend_port}/health"

cleanup() {
  if [[ -n "$backend_pid" ]]; then
    kill "$backend_pid" >/dev/null 2>&1 || true
  fi
}

trap cleanup EXIT INT TERM

wait_for_backend() {
  local attempt

  for attempt in $(seq 1 120); do
    if curl -fsS "$backend_health_url" >/dev/null 2>&1; then
      return 0
    fi

    if [[ -n "$backend_pid" ]] && ! kill -0 "$backend_pid" >/dev/null 2>&1; then
      wait "$backend_pid"
    fi

    sleep 0.5
  done

  echo "Backend did not become ready at ${backend_health_url}" >&2
  return 1
}

if ! curl -fsS "$backend_health_url" >/dev/null 2>&1; then
  ./scripts/run-backend.sh &
  backend_pid=$!
  wait_for_backend
fi

exec ./node_modules/.bin/vite dev
