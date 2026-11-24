# #441 build-2 (easel): the map — the edge pieces, their layer, the views and the checks

Read `relay/specs/441/spec.md` first ("How the pieces are chosen" is your rule, word for word),
`relay/specs/441/shores.md` (the classes and the view tiles) and build-1's summary and
`relay/specs/441/atlas.md` (the keys and their anchors). Build-3, the canvas, judges how the shore looks
and may change what you build; your job is machinery that is exact. No Rust changes: nothing under
`engine/core`, `engine/cli` or `engine/web`. No atlas changes either: `www/assets/` is build-1's.

Read before you edit: `www/map/ridges.js` and `www/map/layers/ridges.js` (the pattern you follow),
`www/map/layers/terrain.js` (`cellHeight`, `isFlat`), `www/map/view.js` (`LAYERS`, `nb`, `noCull`,
`stats`), `www/map/geom.js` (`CORNER`, `EDGE_DIR`), `www/shots.mjs`, `www/check.mjs` (its ridge block
near the end), `www/play/ridges.mjs`, `www/play-stage.mjs` (`~` sea, `=` lake, `:` desert, `.` plains),
`relay/specs/441/proto/shore.mjs` (the classes).

## 1. The pieces: `www/map/edges.js`

A pure module (no DOM), as `map/ridges.js` is:

```js
/** The fringe order, highest first: a kind bleeds into its lower neighbours. Plains is lowest and absent. */
export const FRINGE = ['desert', 'marsh', 'forest', 'hills', 'mountain', 'coast'];
/** A tile's kind: the terrain's name, terrain 0 split into 'sea' and 'lake'. */
export function kindOf(i, { terrain, lake, names })
/** The shore and fringe pieces of a map, in draw order: [{ key, x, y, tile, other, kind }]. */
export function edgePieces({ terrain, lake, river, flow, nb, px, py, names })
```

`terrain`, `lake`, `river` and `flow` are the layout's arrays (`view.l`), `nb` the view's six neighbours a
tile (−1 off the map, `EDGE_DIR` order), `names` the terrain index's names (`view.names`). `kind` is
`'fringe'`, `'edge'` or `'corner'`; `x, y` the piece's point in world units (a fringe: the lower tile's
centre; an edge: the shared edge's midpoint; a corner: `px[t] + CORNER[c][0], py[t] + CORNER[c][1]`);
`tile` the tile it is drawn for (the lower tile, the land tile, the odd tile) and `other` the neighbour
(an edge's or fringe's) or −1. The keys, the mouth rule, the vertex enumeration and corner indices
(corner 1 of `i` is E's corner 3 and SE's corner 5; corner 2 is SE's corner 4 and SW's corner 0), the
variants and the order are the spec's.

## 2. The layer: `www/map/layers/edges.js`

A static layer, `name = 'edges'`, in `LAYERS` right after `terrain` and before `ridges`. `view.js` is
607 lines: its only changes are the import, the `LAYERS` entry, and `noCull`'s comment naming both
layers. Like `layers/ridges.js`, it:

- draws nothing where `isFlat(cellHeight(view), view)`;
- builds the pieces once per map, cached in a `WeakMap` keyed by `view.nb`;
- draws each piece with `drawImage` of `atlas.scaled(key, round(cell.h × H / 148))`, whole device px, its
  anchor (scaled) on the piece's point;
- skips a piece whose drawn rectangle misses the canvas, unless `view.noCull`;
- sets `view.stats.edges` (pieces drawn) and nothing else in `stats`.

If a draw is slow on large 8 (`stats.terrainMs` well above main's), draw from per-tile lists over
`view.vis` instead of the whole list, as long as `--cull` still holds; say which in `easel.md`.

## 3. The views: `www/shots.mjs`

Add the spec's twelve shore views to `VIEWS`, keeping the 21 there. The chooser (`CAMERA`, in the page,
still from `game.map` alone so the file copies onto main) gains the classes `neck`, `strait`, `bay`,
`pond`, `mouth-sea`, `mouth-lake` and `marsh`, defined as `shores.md` defines them (port
`proto/shore.mjs`; off the map counts as water). Each view names its `tile`, and the shots must find
`shores.md`'s: 402, 506, 376, 341, 377, 875, 1004, 2639, 1289. `--cull` shoots every view with an `on`
(mountain and shore), with `noCull` off and on, and fails unless the PNGs are byte-identical, printing
`stats.ridges` and `stats.edges` for both.

## 4. Checks in CI

- **`www/check-map.mjs`**, new, run by `check.sh` after `www/check.mjs`: move `check.mjs`'s ridge
  block here (it is 602 lines), building its own medium world through `Sim` as `check.mjs` does, and
  add the edges block: on that world, counted straight from the neighbours in the check itself, every
  land–water edge has one `edge` piece with the spec's key except the mouths, which have none; every
  vertex with one or two water tiles (all three on the map) has one `corner` piece with the right side
  and corner index; every land edge between kinds has one `fringe` of the higher kind; and the list
  is in the spec's order. A synthetic seven-tile layout (a land tile, a river on it flowing into a
  water neighbour) checks the mouth has no edge piece and its two corners still draw. Seconds.
- **`www/play/edges.mjs`**, run by `play/all.mjs`: a staged map (radius 8) with a neck (a land tile
  between sea and a lake), a bay, a one-tile pond in plains, and desert beside plains. It checks the
  pieces at named tiles (keys, kinds, counts) against the spec's rules, and, with the camera put so a
  pond's shore tile sits 1.0 world unit beyond the canvas's bottom edge (assert the camera wasn't
  clamped), that culled and unculled draws give identical canvas PNGs. Seconds.
- `play/ridges.mjs` and `play/art.mjs` still pass.

## Done

1. Main (a worktree at the branch's base, its wasm built, this `shots.mjs` copied in) and the branch,
   every view, `--diff`: cameras match. `--cull` on the branch passes.
2. `relay/specs/441/easel.md`: the `--diff` table (all 33 views), the `--cull` output with
   `stats.ridges` and `stats.edges`, `stats.terrainMs` beside main's on the laptop play views
   (`small2-play`, `medium9-play`, `large8-play`, `small9-lake-play`, `medium9-neck-play`,
   `large8-delta-play`), each class's piece count on small 2 and large 8, and any correction to this
   brief.
3. `engine/CLAUDE.md` (8 KB budget): a short line for `map/edges.js`, the edges layer and
   `check-map.mjs`.

Run `engine/check.sh` once at the end. Commit as `#441 build-2: the shore layer`.
