#!/usr/bin/env bash
# Usage: tools/pages.sh <out>, from a clean checkout of main. Needs GH_TOKEN.
set -euo pipefail
out=$(realpath -m "$1")
site=$(mktemp)
cp tools/site.sh "$site"
home=$(git rev-parse HEAD)

tools/site.sh "$out"
main=$(find "$out" -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | sort | jq -Rsc 'split("\n") | map(select(. != ""))')

rows=$(mktemp)
gh pr list --state open --limit 100 \
  --json number,title,url,headRefName,isCrossRepository,statusCheckRollup \
  --jq '.[] | select(.isCrossRepository | not)' >"$rows.prs"
while read -r pr; do
  b=$(jq -r .headRefName <<<"$pr")
  dest="$out/b/$b"
  mkdir -p "$dest"
  build=ok
  if git fetch -q origin "+refs/heads/$b:refs/remotes/origin/$b" &&
     git checkout -qf --detach "origin/$b" &&
     git clean -qffdx -e engine/target; then
    script=tools/site.sh
    [ -f "$script" ] || script="$site"
    bash "$script" "$dest" >"$dest/build.log" 2>&1 || build=failed
    commit=$(git log -1 --format='%h%x1f%cs%x1f%s' | jq -Rc 'split("\u001f") | {sha: .[0], date: .[1], subject: .[2]}')
  else
    build=failed
    commit='{}'
  fi
  folders=$(find "$dest" -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | sort | jq -Rsc 'split("\n") | map(select(. != ""))')
  jq -c --arg build "$build" --argjson commit "$commit" --argjson folders "$folders" \
    '. + {build: $build, commit: $commit, folders: $folders}' <<<"$pr" >>"$rows"
done <"$rows.prs"
git checkout -qf "$home"

jq -rs --argjson main "$main" '
  def ci: [.statusCheckRollup[] | (.conclusion // .state // "") | ascii_upcase] as $s
    | if ($s | length) == 0 then "none"
      elif any($s[]; IN("FAILURE", "ERROR", "CANCELLED", "TIMED_OUT", "ACTION_REQUIRED", "STARTUP_FAILURE")) then "failing"
      elif any($s[]; IN("", "PENDING", "EXPECTED")) then "pending"
      else "passing" end;
  "<!doctype html><meta charset=\"utf-8\"><title>Sans Fiat</title>",
  "<style>body{font-family:system-ui,sans-serif;margin:2rem}td,th{padding:.25rem .75rem;text-align:left}.failed,.failing{color:#b00}.pending{color:#a60}</style>",
  "<h1>Sans Fiat</h1><h2>main</h2><ul>",
  ($main[] | select(. != "b") | "<li><a href=\"\(@html)/\">\(@html)</a></li>"),
  "<li><a href=\"merge4/docs/guide.html\">merge4 guide</a></li></ul>",
  "<h2>Open pull requests</h2>",
  if length == 0 then "<p>None.</p>" else
    "<table><tr><th>Branch</th><th>PR</th><th>CI</th><th>Last commit</th><th>Play</th></tr>",
    (.[] | "b/\(.headRefName)" as $p |
      "<tr><td>\(.headRefName | @html)</td>",
      "<td><a href=\"\(.url | @html)\">#\(.number) \(.title | @html)</a></td>",
      "<td class=\"\(ci)\">\(ci)</td>",
      "<td>\(.commit.sha // "" | @html) \(.commit.date // "" | @html) \(.commit.subject // "" | @html)</td>",
      "<td>\(if .build == "failed" then "<span class=\"failed\">build failed</span> " else "" end)",
      (.folders[] | "<a href=\"\($p | @html)/\(@html)/\">\(@html)</a> "),
      "<a href=\"\($p | @html)/build.log\">log</a></td></tr>"),
    "</table>"
  end
' "$rows" >"$out/index.html"
