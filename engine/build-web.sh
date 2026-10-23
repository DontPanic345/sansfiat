#!/usr/bin/env bash
# Builds the wasm front end into www/pkg. Needs: rustup target add wasm32-unknown-unknown;
# cargo install wasm-bindgen-cli --version 0.2.128 (must match web/Cargo.toml).
set -euo pipefail
cd "$(dirname "$0")"
cargo build -p web --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir www/pkg target/wasm32-unknown-unknown/release/web.wasm
