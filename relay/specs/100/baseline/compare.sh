#!/usr/bin/env bash
# Compares the current build with the baselines taken from main before piece #100 (see README.md):
# every turn's digest of the four watching runs, the led run's state digest, and the retro sweep.
# Run from anywhere. Prints "ok" or "DIFFERS" per item and exits 1 if any differs.
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
engine="$here/../../../../engine"
cd "$engine" && cargo build --release -p sansfiat 2>&1 | tail -1
bin="$engine/target/release/sansfiat"
out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
failed=0
check() { # name, command...
  name=$1; shift
  if "$@" > "$out/got.txt" 2> "$out/err.txt" && diff -q "$out/got.txt" "$here/$name" > /dev/null; then
    echo "ok       $name"
  else
    echo "DIFFERS  $name"; diff "$out/got.txt" "$here/$name" | head -3; head -2 "$out/err.txt"; failed=1
  fi
}
check seed1-r16-t400.txt "$bin" digest --seed 1 --radius 16 --turns 400
check seed7-r16-t400.txt "$bin" digest --seed 7 --radius 16 --turns 400
check seed4-r20-t400.txt "$bin" digest --seed 4 --radius 20 --turns 400
check seed3-r23-t150.txt "$bin" digest --seed 3 --radius 23 --turns 150
check seed2-r16-t400-lead.state.txt "$bin" digest --seed 2 --radius 16 --turns 400 --lead --state
check retro-sweep.txt "$bin" sweep --after 200 --maps retro
# Not compared after build-4: the led run's plain digest moves with the log's shape.
exit $failed
