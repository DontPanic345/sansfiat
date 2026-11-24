#!/usr/bin/env bash
# The two controls for #446, beside the branch:
#   /tmp/preart  the page before the art track (0fd7835, the commit before #448) on today's wasm: the same worlds, the old map
#   /tmp/main    main at the branch's base
# Each gets the branch's www/pkg, shots.mjs and shots-views.mjs (and any other shots-*.mjs), so the same views shoot on all three.
# Run from the repo root after engine/build-web.sh (or check.sh) has built engine/www/pkg.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
base=${BASE:-$(git merge-base HEAD origin/main)}
for t in preart:0fd7835 main:$base; do
  dir=/tmp/${t%%:*}; rev=${t#*:}
  [ -d "$dir" ] || git worktree add -f "$dir" "$rev" >/dev/null
  rm -rf "$dir/engine/www/pkg"; cp -r "$root/engine/www/pkg" "$dir/engine/www/"
  cp "$root"/engine/www/shots*.mjs "$dir/engine/www/"
done
echo "controls ready: /tmp/preart (0fd7835 www), /tmp/main ($base)"
