#!/usr/bin/env bash
# The piece's proof of health: the last digest of each retro map at turn 400, with every civ on the
# AI, and with the scripted lead (--lead), plus the state digest. Run from engine/ after
# `cargo build --release`; diff the output against main's (digests-main.txt).
set -euo pipefail
B=target/release/sansfiat
for m in "small 2" "small 3" "small 6" "small 9" "medium 8" "medium 9" "large 8"; do
  set -- $m
  for f in "" "--lead" "--state"; do
    echo "$1 $2 ${f:-ai}: $($B digest --size $1 --seed $2 --turns 400 $f | tail -1)"
  done
done
