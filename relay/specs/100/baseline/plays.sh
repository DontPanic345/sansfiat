#!/usr/bin/env bash
# Runs the five page plays (engine/www/play-*.mjs) on the current build and diffs each one's stdout
# and exit code with main's before piece #100 (plays/<name>.txt, plays/exit.txt). Needs engine/www/pkg
# (./build-web.sh). Screenshots go to the directory given (default /tmp/plays-100); open them.
# Prints "ok" or "DIFFERS" per play and exits 1 if any differs.
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
engine="$here/../../../../engine"
shots="${1:-/tmp/plays-100}"
out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
failed=0
for p in camp scout hold lean read; do
  node "$engine/www/play-$p.mjs" "$shots/$p" > "$out/$p.txt" 2> "$out/$p.err" &
  echo $! > "$out/$p.pid"
done
for p in camp scout hold lean read; do
  wait "$(cat "$out/$p.pid")"; code=$?
  want=$(awk -v p="$p" '$1 == p { print $2 }' "$here/plays/exit.txt")
  if [ "$code" = "$want" ] && diff -q "$out/$p.txt" "$here/plays/$p.txt" > /dev/null; then
    echo "ok       play-$p (exit $code)"
  else
    echo "DIFFERS  play-$p (exit $code, main $want)"; diff "$out/$p.txt" "$here/plays/$p.txt" | head -5; tail -3 "$out/$p.err"; failed=1
  fi
done
exit $failed
