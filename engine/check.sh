#!/usr/bin/env bash
# Every check CI runs on the engine, in its order.
set -euo pipefail
cd "$(dirname "$0")"
size=$(wc -c < CLAUDE.md)
if [ "$size" -gt 8192 ]; then echo "engine/CLAUDE.md is $size bytes, over its 8 KB budget" >&2; exit 1; fi
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
./build-web.sh
./cross-build.sh
node www/check.mjs
node www/play/all.mjs
