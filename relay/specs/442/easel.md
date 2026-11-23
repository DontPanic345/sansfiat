# #442 build-2 (easel): the evidence

Run on the branch at build-1's cells. Numbers are as the code stood when this was written; build-3 changes `WOODS` and the cells.

## Counts on the medium world

`node www/check-map.mjs` (medium, seed 3, read before any turn: no fields). Each slot is counted by the rule from its
neighbours and `WOODS`, and matched against the piece `woodPieces` keeps.

| Slots | Count |
| --- | --- |
| `tree.big` (forest slots, deep) | 666 |
| `tree.small` (forest slots in the band, kept by `sparse`) | 168 |
| reed (marsh slots) | 85 |
| clear (river course; no clearing, as there are no fields) | 281 |

Forest tiles on that world: 1 interior, 72 edge, 12 lone. The world's forest has an interior tile, so none is lacking;
the interior is thin there, and the staged `play/woods.mjs` carries the interior proof.

## The 15 new views on the branch

Shot with `node www/shots.mjs --only …` on the branch. Every camera reads back as asked (no clamp, no tile mismatch).
Digests are the game's, from `views.json`. The two turn views lead the first civ for 8 turns; `small2-marsh-fields-close`
has digest `ff55c006fe645483`, the digest the spec recorded for small 2 after 8 turns.

| View | Window | k | Tile | Camera (cx, cy) | Digest |
| --- | --- | --- | --- | --- | --- |
| medium9-woods-close | laptop | 35 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| medium9-woods-play | laptop | 18 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| medium9-woods-near | laptop | 12 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| medium9-woods-far | laptop | 11 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| medium9-woods-edge | phone | 6 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| medium9-woods-flat | phone | 5 | 827 | -0.866, -1.500 | 2c0e18547e9eacd9 |
| small2-lone-close | laptop | 35 | 439 | 0.000, -3.000 | 340af46c1f94e630 |
| small2-lone-play | laptop | 18 | 439 | 0.000, -3.000 | 340af46c1f94e630 |
| small9-river-close | laptop | 35 | 258 | -4.330, 7.500 | 7b49b3ced7d1eb3d |
| small2-delta-near | laptop | 12 | 506 | 6.928, 3.000 | 340af46c1f94e630 |
| small2-delta-far | laptop | 11 | 506 | 6.928, 3.000 | 340af46c1f94e630 |
| small2-delta-edge | phone | 6 | 506 | 6.928, 3.000 | 340af46c1f94e630 |
| small2-delta-flat | phone | 5 | 506 | 6.928, 3.000 | 340af46c1f94e630 |
| medium9-fields-close (8 turns) | laptop | 35 | 828 | 0.000, 0.000 | e5fc275e001a30e2 |
| small2-marsh-fields-close (8 turns) | laptop | 35 | 535 | 7.794, 1.500 | ff55c006fe645483 |

The `-near` and `-far` pair straddle `WOODS.from` (24 device px): at k 12 the cell height is 24 and the woods draw;
at k 11 it is 23 and they don't. The `-edge` and `-flat` pair are cells and flat at phone k 6 and k 5.

## `--cull`

`node www/shots.mjs --cull /tmp/relay442/cull` on the branch: 38 views with an `on`, all identical, exit 0. The
new views' culled and unculled `stats.woods` run from 0 and 0 (far, edge, flat) up to 2152 of 2159 (`-near`,
where the woods are almost all on the canvas).

## `--time` (five static redraws, median of `stats.terrainMs`)

| View | Branch (ms) | Main (ms) | Difference |
| --- | --- | --- | --- |
| medium9-woods-play | 26.1 (25.3–26.9), `stats.woods` 1600 | 24.0 (23.3–24.8) | +2.1 |
| large8-play | 27.4 (26.7–28.0), `stats.woods` 816 | 26.5 (25.5–31.4) | +0.9 |

Main was timed with the branch's `shots.mjs` copied into a worktree of `main` (no `stats.woods`: printed `-`). One
run each, on this runner, so the spread is noisy; both differences sit inside the spec's 3 ms bar, but build-3 should
re-time them before it tunes the numbers.

## Notes for build-3

- `-far` and `-edge` share a tile and camera with `-play`; only k (and the window) differs.
- Re-time the `--time` pair before tuning: one run each on this runner, and the branch's extra 0.9 to 2.1 ms is within the bar but not by much.
