#!/usr/bin/env bash
# Every check CI runs on the engine, in its order.
set -euo pipefail
cd "$(dirname "$0")"
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -p sansfiat -- sweep --judge
./build-web.sh
./cross-build.sh
node www/check.mjs
