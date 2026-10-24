#!/usr/bin/env bash
# Checks that the native and wasm builds give the same world: the digest after every turn of a few
# seeds, from `sansfiat digest` and from the wasm build under Node. Needs ./build-web.sh run first.
set -euo pipefail
cd "$(dirname "$0")"

if [ ! -f www/pkg/web_bg.wasm ]; then
  echo "www/pkg/web_bg.wasm is missing: run ./build-web.sh first" >&2
  exit 2
fi

cargo build --release -p sansfiat
out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT

turns=400
failed=0
for pair in "1 16" "7 16" "4 20"; do
  set -- $pair
  seed=$1 radius=$2
  target/release/sansfiat digest --seed "$seed" --radius "$radius" --turns "$turns" > "$out/native.txt"
  node cross-build.mjs "$seed" "$radius" "$turns" > "$out/wasm.txt"
  if diff "$out/native.txt" "$out/wasm.txt" > "$out/diff.txt"; then
    echo "seed $seed radius $radius: $(wc -l < "$out/native.txt") digests agree"
  else
    first=$(sed -n '1s/[,acd].*//p' "$out/diff.txt")
    echo "seed $seed radius $radius: the builds part at turn $((first - 1)); first differing lines (native <, wasm >):"
    head -n 6 "$out/diff.txt"
    failed=1
  fi
done
exit $failed
