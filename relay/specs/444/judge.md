# #444 judge

No rule moved: `git diff origin/main --stat -- engine/core engine/cli engine/web` is empty after merging main (1c3f69b),
so `sansfiat sweep --judge` on the branch is main's sweep, game for game (same binary, same digests). No sweep was run:
`judge --compare` of a report against itself is zero in every metric.

| Score | main | branch | floor |
|---|---|---|---|
| Standing | = | = | – |
| Doubt | = | = | – |
| Decisions | = | = | – |
| Motion | = | = | – |
| Health | = (SAVE_VERSION unchanged, digests match on every clan view, `diff.txt`) | = | – |

Verdict: **same**. The piece's question is in the page, not the scores: `views.md`, `diff.txt`, `clans.txt`, `shots/`.
