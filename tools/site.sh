#!/usr/bin/env bash
# Usage: tools/site.sh <out>, from the repo root.
set -euo pipefail
out=$(realpath -m "$1")
mkdir -p "$out"

./engine/build-web.sh
cp -r engine/www "$out/engine"
