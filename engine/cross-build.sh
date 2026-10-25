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

# "seed radius turns [lead]": the medium map runs fewer turns so the check stays fast; `lead` plays the
# scripted game (picks a civ, makes, moves and disbands units) so the builds are compared through orders.
failed=0
for pair in "1 16 400" "7 16 400" "4 20 400" "3 23 150" "2 16 400 lead"; do
  set -- $pair
  seed=$1 radius=$2 turns=$3 lead=${4:-}
  flag=""
  [ -n "$lead" ] && flag="--lead"
  target/release/sansfiat digest --seed "$seed" --radius "$radius" --turns "$turns" $flag > "$out/native.txt"
  node cross-build.mjs "$seed" "$radius" "$turns" $lead > "$out/wasm.txt"
  if diff "$out/native.txt" "$out/wasm.txt" > "$out/diff.txt"; then
    echo "seed $seed radius $radius ${lead:-watching}: $(wc -l < "$out/native.txt") digests agree"
  else
    first=$(sed -n '1s/[,acd].*//p' "$out/diff.txt")
    echo "seed $seed radius $radius ${lead:-watching}: the builds part at turn $((first - 1)); first differing lines (native <, wasm >):"
    head -n 6 "$out/diff.txt"
    failed=1
  fi
done
exit $failed
