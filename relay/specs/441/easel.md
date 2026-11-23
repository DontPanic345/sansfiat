# #441 build-2 (easel): the map, the layer, the views and the checks

## Checks run

- `node www/check-map.mjs` (in `check.sh`, after `check.mjs`): on the medium seed 3 world, the mountain pieces (the
  block moved from `check.mjs`) and the shore and fringe pieces, counted again from the neighbours in the check:
  `ok: edges, medium seed 3: 1531 fringes, 1028 shore edges, 1036 corners, 8 river mouths; the synthetic mouth draws its corners`.
  Mutation: with the mouth exception removed from `map/edges.js`, it exits 1 (`the shore and fringe pieces are the rules' pieces`).
- `node www/play/edges.mjs`: 23 checks pass (named tiles, counts, order, camera unclamped, shore tile 1.0 unit below the
  canvas, `stats.edges` 136 culled of 266, culled and unculled PNGs identical).
- `node www/play/ridges.mjs` and `node www/play/art.mjs`: pass.
- `node www/shots.mjs --cull`: all 23 views with an `on` identical (below).
- `node www/shots.mjs --diff`: cameras match on all 33 views against main (below).
- `engine/check.sh` whole: see the commit's run (result recorded in the summary).

## Views: main against the branch

Main is `f5f4620` in a worktree, its wasm built, with this branch's `shots.mjs` copied in. The branch is shot with
the final layer code. All 33 views: cameras match (`--diff` exit 0). The shares are large because build-1's ground
art replaced every placeholder hex in every view, so they measure the new ground, not the shore layer alone. The
branch's final pixels equal the first branch shoot on all 33 views (0% differ), so the layer's later change is pixel-neutral.

| View | Share of pixels differing |
|---|---|
| small2-close | 67.508% |
| small2-play | 48.717% |
| small2-edge | 19.564% |
| small2-flat | 12.935% |
| small3-play | 55.966% |
| small6-play | 50.734% |
| small9-play | 51.9% |
| medium8-play | 75.054% |
| medium9-play | 76.007% |
| large8-play | 66.825% |
| small9-fork-close | 78.878% |
| small9-fork-play | 49.447% |
| small2-bend-close | 73.077% |
| small2-bend-play | 49.432% |
| medium9-blob-play | 75.731% |
| large8-bend-play | 68.2% |
| large8-fork-play | 67.823% |
| small9-edge-bottom | 31.08% |
| small9-edge-right | 60.166% |
| small9-fork-edge | 20.649% |
| small9-fork-flat | 13.889% |
| small2-neck-close | 59.044% |
| small2-delta-close | 72.63% |
| small2-delta-play | 49.414% |
| small6-bay-close | 69.002% |
| small9-pond-close | 75.195% |
| small9-lake-play | 51.693% |
| medium8-strait-close | 82.283% |
| medium9-neck-play | 77.508% |
| large8-delta-play | 58.013% |
| large8-pond-play | 69.174% |
| small2-neck-edge | 19.515% |
| small2-neck-flat | 12.964% |

## `--cull`: culled and unculled draws

`stats.ridges` and `stats.edges` are the counts drawn (culled, then with `noCull`). Every view is identical.

```
small9-fork-close  tile 469  stats.ridges 76 culled, 133 unculled  stats.edges 784 culled, 1774 unculled  identical
small9-fork-play  tile 469  stats.ridges 125 culled, 133 unculled  stats.edges 1504 culled, 1774 unculled  identical
small2-bend-close  tile 377  stats.ridges 53 culled, 111 unculled  stats.edges 767 culled, 1801 unculled  identical
small2-bend-play  tile 377  stats.ridges 98 culled, 111 unculled  stats.edges 1616 culled, 1801 unculled  identical
medium9-blob-play  tile 777  stats.ridges 229 culled, 293 unculled  stats.edges 2356 culled, 3632 unculled  identical
large8-bend-play  tile 1937  stats.ridges 211 culled, 574 unculled  stats.edges 2544 culled, 6982 unculled  identical
large8-fork-play  tile 1953  stats.ridges 312 culled, 574 unculled  stats.edges 2345 culled, 6982 unculled  identical
small9-edge-bottom  tile 474  stats.ridges 73 culled, 133 unculled  stats.edges 948 culled, 1774 unculled  identical
small9-edge-right  tile 474  stats.ridges 88 culled, 133 unculled  stats.edges 566 culled, 1774 unculled  identical
small9-fork-edge  tile 469  stats.ridges 133 culled, 133 unculled  stats.edges 1774 culled, 1774 unculled  identical
small9-fork-flat  tile 469  stats.ridges 0 culled, 0 unculled  stats.edges 0 culled, 0 unculled  identical
small2-neck-close  tile 402  stats.ridges 85 culled, 111 unculled  stats.edges 711 culled, 1801 unculled  identical
small2-delta-close  tile 506  stats.ridges 52 culled, 111 unculled  stats.edges 703 culled, 1801 unculled  identical
small2-delta-play  tile 506  stats.ridges 98 culled, 111 unculled  stats.edges 1616 culled, 1801 unculled  identical
small6-bay-close  tile 376  stats.ridges 59 culled, 91 unculled  stats.edges 722 culled, 1723 unculled  identical
small9-pond-close  tile 341  stats.ridges 91 culled, 133 unculled  stats.edges 774 culled, 1774 unculled  identical
small9-lake-play  tile 377  stats.ridges 133 culled, 133 unculled  stats.edges 1625 culled, 1774 unculled  identical
medium8-strait-close  tile 875  stats.ridges 52 culled, 231 unculled  stats.edges 799 culled, 3515 unculled  identical
medium9-neck-play  tile 1004  stats.ridges 214 culled, 293 unculled  stats.edges 2347 culled, 3632 unculled  identical
large8-delta-play  tile 2639  stats.ridges 235 culled, 574 unculled  stats.edges 1933 culled, 6982 unculled  identical
large8-pond-play  tile 1289  stats.ridges 139 culled, 574 unculled  stats.edges 2443 culled, 6982 unculled  identical
small2-neck-edge  tile 402  stats.ridges 111 culled, 111 unculled  stats.edges 1801 culled, 1801 unculled  identical
small2-neck-flat  tile 402  stats.ridges 0 culled, 0 unculled  stats.edges 0 culled, 0 unculled  identical
```

## `stats.terrainMs`: the static draw

Medians of nine redraws of the same camera (the views' cameras from `views.json`, laptop window), main against the
branch. The shots' own `terrainMs` is the first draw after load, which includes scaling the atlas cells, so it is
not used here.

| View | Main ms | Branch ms |
|---|---|---|
| small2-play | 4.6 | 5.9 |
| medium9-play | 8.3 | 10.4 |
| large8-play | 10.6 | 19.5 |
| small9-lake-play | 4.5 | 5.8 |
| medium9-neck-play | 8.1 | 10.2 |
| large8-delta-play | 7.3 | 8.8 |

The brief asks for the static draw to stay within a few milliseconds of main's. It does on small 2, medium 9 and the
delta view (+1 to +3 ms). It does not on large 8's play view (+9 ms, nearly double), and this is not closed.

**Cause.** The cost is the number of images drawn, not the walk over the list. On large8-play, 2466 edge pieces are
drawn against 1233 terrain cells, about two per tile. Timed with `drawImage` stubbed on the offscreen layer, the list
walk and lookups cost about 1 ms more than main's (6.5 against 5.6 ms on the delta camera), so drawing from per-tile
lists over `view.vis` would save about 1 ms and leave the draws, so it is not done. The per-key image lookups are read
once per draw (as `terrain.js` does), which changed nothing measurable.

**Not done, for the review.** The fix that would close the gap is one drawImage per tile: the shore and fringe pieces of a
tile composed once per map into a per-tile image (a design call, since the layer is separate and the bank overlaps
neighbours). At the flat zoom the layer draws nothing, so the phone's static cost is unchanged.

## Pieces per class

From `www/map/edges.js` on the two maps (`Sim` layout, seed 2 small and seed 8 large):

| Piece | Small 2 (817 tiles) | Large 8 (3367 tiles) |
|---|---|---|
| `fringe.desert` | 142 | 962 |
| `fringe.marsh` | 22 | 21 |
| `fringe.forest` | 143 | 623 |
| `fringe.hills` | 205 | 854 |
| `fringe.mountain` | 112 | 443 |
| `fringe.coast` | 95 | 684 |
| `shore.edge.<d>.<v>` | 538 | 1685 |
| `shore.in.<c>` | 293 | 927 |
| `shore.out.<c>` | 251 | 783 |
| total | 1801 | 6982 |

These agree with `shores.md`: on small 2, 538 banks and 6 sea mouths make the table's 544 land–water edges, and the
corners are 293 one-water and 251 two-water; on large 8 they are 927 one-water and 783 two-water. The small 2 fringes,
719, are the sum of the table's land-edge row for differing kinds.

## Corrections to the brief

- `www/check.mjs` goes from 602 to 552 lines: its 50-line ridge block moved to `www/check-map.mjs`, which also holds
  the edge block and the synthetic mouth.
- The brief's "a shore tile sits 1.0 world unit beyond the canvas's bottom edge" is checked as the tile's centre at
  canvas height plus `k` device px (one world unit), with the camera read back unclamped.

## Decisions taken in this build

- **Fringe draw order.** "From lowest in FRINGE to highest" is read as the weakest kind first: coast's fringes draw
  before desert's, so a strong ground lies over a weak one. The order is the same in `edges.js` and the check.
- **A fringe goes into the lower kind's tile** (the one lower in `FRINGE`, plains lowest), at its centre, toward the
  higher kind; its variant is `min(i, j) % 2`.
- **A mouth** is read from the land tile only: `river[i] > 0 && flow[i] === j` for the land tile `i` and water `j`.
- **Layer cache.** Pieces are built once per map and cached in a `WeakMap` by `view.nb`, as the ridges are.
- **The synthetic mouth** in `check-map.mjs` is seven tiles: a plains centre with a river flowing east into a sea tile.
  Its two corners at the sea tile are `shore.in.3` and `shore.in.4`, and the neighbours' banks to the sea tile draw.

## Views in the chooser

The classes `neck`, `strait`, `bay`, `pond`, `mouth-sea`, `mouth-lake` and `marsh` are `shores.md`'s definitions, and
the chooser picks the nearest of each class to world (0, 0). The named tile is the nearest of its class on each seed
(checked in Node before the views were shot), and each view's camera is read back unclamped.
