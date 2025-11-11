#!/usr/bin/env bash
set -euo pipefail

echo "Running RustPoolSimProjectGenesis validation..."

# Detect required tools and report missing ones clearly.
missing_tools=()

check_tool() {
  if ! command -v "$1" &>/dev/null; then
    missing_tools+=("$1")
  fi
}

check_tool cargo
check_tool wasm-pack
check_tool git
check_tool cargo-audit || true # handled explicitly below

if [ ${#missing_tools[@]} -ne 0 ]; then
  echo "Missing required tools:"
  for t in "${missing_tools[@]}"; do
    echo "  - $t"
  done
  echo "Install the missing tooling and re-run this script."
  exit 1
fi

# Ensure we are in the project root (script is in scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." &>/dev/null && pwd)"
cd "$PROJECT_ROOT"

echo "Using PROJECT_ROOT=$PROJECT_ROOT"

echo "1) Generating Cargo.lock (if needed)..."
cargo generate-lockfile

echo "2) Running cargo check..."
cargo check

echo "3) Running cargo fmt --all -- --check..."
cargo fmt --all -- --check

echo "4) Running cargo clippy --all-targets --all-features -- -D warnings..."
cargo clippy --all-targets --all-features -- -D warnings

echo "5) Running cargo test --all-targets..."
cargo test --all-targets

echo "6) Building WASM via wasm-pack (release, web target)..."
wasm-pack build --target web --release --out-dir pkg

WASM_FILE="$(find pkg -maxdepth 1 -name '*.wasm' | head -n 1 || true)"
if [ -z "$WASM_FILE" ]; then
  echo "Error: No .wasm file produced in pkg/. wasm-pack build may have failed."
  exit 1
fi

WASM_SIZE_BYTES=$(wc -c < "$WASM_FILE")
MAX_SIZE_BYTES=1048576

echo "WASM output: $WASM_FILE ($WASM_SIZE_BYTES bytes)"
if [ "$WASM_SIZE_BYTES" -ge "$MAX_SIZE_BYTES" ]; then
  echo "Error: WASM binary size exceeds 1MB budget (${MAX_SIZE_BYTES} bytes)."
  echo "Adjust release/profile or exports to reduce size and re-run."
  exit 1
fi

echo "7) Running cargo audit (if available)..."
if command -v cargo-audit &>/dev/null; then
  cargo audit
else
  echo "cargo-audit is not installed or not on PATH."
  echo "Security policy: cargo-audit remains a REQUIRED check in CI and for contributors."
  echo "Install via: cargo install cargo-audit"
  # Do not treat as success; enforce requirement by failing here to match policy.
  exit 1
fi

echo "All checks PASSED"