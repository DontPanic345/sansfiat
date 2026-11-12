#!/usr/bin/env bash
# Every check CI runs on the engine, in its order.
set -euo pipefail
cd "$(dirname "$0")"
size=$(wc -c < CLAUDE.md)
if [ "$size" -gt 8192 ]; then echo "engine/CLAUDE.md is $size bytes, over its 8 KB budget" >&2; exit 1; fi
cargo fmt --check
cargo clippy --all-targets -- -D warnings
# A test file runs in under a minute: a proof that needs whole games on the retro maps is a
# report (`sweep --judge`, or an ignored test), run once by the step that needs it.
log=$(mktemp)
cargo test 2>&1 | tee "$log"
slow=$(awk '/Running/ { name = $0 } /finished in/ { t = $NF; sub(/s$/, "", t); if (t + 0 > 60) print t "s:" name }' "$log")
rm -f "$log"
if [ -n "$slow" ]; then echo "test files over a minute:" >&2; echo "$slow" >&2; exit 1; fi
./build-web.sh
./cross-build.sh
node www/check.mjs
node www/play/all.mjs
