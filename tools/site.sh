#!/usr/bin/env bash
# Usage: tools/site.sh <out>, from the repo root.
set -euo pipefail
out=$(realpath -m "$1")
mkdir -p "$out"

dirs=()
for f in */index.html; do
  d=${f%/index.html}
  [ "$d" = engine ] && continue
  cp -r "$d" "$out/$d"
  dirs+=("$d")
done

./engine/build-web.sh
cp -r engine/www "$out/engine"
dirs=(engine "${dirs[@]}")

{
  echo '<!doctype html><meta charset="utf-8"><title>Sans Fiat</title><h1>Sans Fiat prototypes</h1><ul>'
  for d in "${dirs[@]}"; do echo "<li><a href=\"$d/\">$d</a></li>"; done
  [ -f merge4/docs/guide.html ] && echo '<li><a href="merge4/docs/guide.html">merge4 guide</a></li>'
  echo '</ul>'
} > "$out/index.html"
