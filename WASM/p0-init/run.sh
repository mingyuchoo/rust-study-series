#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "==> Building Rust WASM..."
cd "$SCRIPT_DIR/Rust"
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/deps/*.wasm ../Node/rust.wasm
echo "==> WASM file copied to Node/rust.wasm"

echo "==> Running Node.js..."
cd "$SCRIPT_DIR/Node"
npm run start
