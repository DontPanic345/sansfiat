# #440 build-1 (easel): sprite regions, the mountain keys, the ridge layer and the views

Read `relay/specs/440/spec.md` first (the decisions, how pieces are chosen, the views) and
`relay/specs/440/ranges.md` (the measurements and the peak cut that works). Build-2, the canvas,
judges how the range looks and may change what you build; your job is machinery that is exact. No
Rust changes: nothing under `engine/core`, `engine/cli` or `engine/web` is touched.

Read before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js}`, `www/atlas.js`,
`www/map/view.js` (`LAYERS`, `drawStatic`, `setWorld`, `collectVisible`, `nb`, `stats`),
`www/map/layers/terrain.js`, `www/shots.mjs`, `www/check.mjs`, `www/play/art.mjs`, `www/play/all.mjs`,
`www/play-stage.mjs` (`grid`, `marks`, `stage`; `^` is mountain), and `relay/specs/440/proto/peak.mjs`.

## 1. Sprite regions in `www/assets/cut.mjs`

A region is a hex region (as now, unchanged to the pixel) or a sprite region:

```js
'peak.left': {
  file: 'terrain',
  rect: [110, 140, 280, 180],                 // x, y, w, h in source px
  keep: [[110,140],[300,140],[300,150],[385,258],[372,300],[110,312]],  // polygon, source px
  key: 'ground',                              // clears grass, tufts and hill (below)
  ground: [262, 292],                         // the source point that lands on the cell's anchor
  scale: 74 / 109,                            // cell px per source px: the plains' scale
  note: "the range's left peak, with its foot's boulders",
}
```

- A cell sample maps to source `ground + (cell − anchor) / scale`; it counts only inside `rect`
  and `keep` and where the key doesn't clear it. 4×4 samples a pixel; colour is the source-alpha-weighted
  mean of the kept samples, alpha the kept samples' source alpha summed over all 16 (a cleared sample
  counts 0, so the foot is antialiased). Port
  `proto/peak.mjs` and match its output: `mountain.peak.0` cut from `peak.left` into 192×128 at anchor
  (96, 112) equals `proto/peak.png` pixel for pixel, or say in `easel.md` why not and by how much.
- The key `'ground'` clears a pixel when `g > r + 6 && g > b + 25` or `b < 0.6 g`. Keys are a named
  table in `cut.mjs` so the canvas can add one; their thresholds live with them, each with a note.
- `patches` and `mirror` work on sprite regions as on hex ones. `flat` is computed as now (mean of
  the cell's alpha-255 pixels); a cut with none throws.
- A sample outside the source throws, as now.

## 2. The mountain keys in `www/assets/placeholders.js`

Append to `layout()`, after the icons' row, so no existing rectangle moves (check: every main key keeps
its `x`, `y`, `w`, `h`):

| Keys | Size | Anchor |
|---|---|---|
| `mountain.lone.0`–`.1` | 192×128 | (96, 112) |
| `mountain.peak.0`–`.2` | 192×128 | (96, 112) |
| `mountain.ridge.e.0`–`.1`, `mountain.ridge.se.0`–`.1`, `mountain.ridge.sw.0`–`.1` | 192×128 | (96, 112) |

`layout()` cells may carry `ax`, `ay`; `anchorOf` takes, in order, the `CELLS` entry's anchor, the
layout cell's, then the centre (mirrored by `mirror` as now). The placeholder for each is drawn in
`TERRAIN_COLOR.mountain` with a dark edge, standing on the anchor's line: a peak a tall triangle about
110 wide and 90 high; a ridge two low triangles along its edge's direction (horizontal for `e`, down to
the right for `se`, down to the left for `sw`), about 120 long and 50 high. They are what the map shows
until the canvas cuts the art, so they only need to be visible and in the right place.

## 3. The cells in `www/assets/sources.js`

- Region `peak.left` as above; `mountain.peak.0` and `.2` its cut, `.1` mirrored; `mountain.lone.0`
  its cut, `.1` mirrored.
- `terrain.mountain.0`–`.2`: the plains' recipe (`.0` `plains.clean`, `.1` `plains.grass`, `.2`
  `plains.clean` mirrored).
- The ridges stay placeholders: the canvas cuts them.

`draw-atlas.mjs` and `--check` need no change beyond what the new keys and regions bring; `--check`
still holds every cut cell and every key to `layout()`.

## 4. The pieces: `www/map/ridges.js`

A pure module (no DOM; `www/check.mjs` imports it under Node):

```js
/** The ridge pieces of a map, in draw order. */
export function ridgePieces({ terrain, nb, px, py, mountain }) → [{ key, x, y, tile, kind }]
/** A mountain tile's class: 'lone' | 'end' | 'straight' | 'bend' | 'sharp' | 'fork' (3 or more). */
export function ridgeClass(i, { terrain, nb, mountain })
```

`terrain` is the layout's terrain array, `nb` the view's `Int32Array` of six neighbours a tile (−1
off the map, in `EDGE_DIR`'s order: E, SE, SW, W, NW, NE), `mountain` the terrain index. The rules are
the spec's "How the pieces are chosen": a peak per mountain tile at its centre (`kind` its class), a
ridge per shared mountain edge at the edge's midpoint from directions 0–2 (`kind` `'ridge'`), keys and
variants as the spec gives them. Sorted by `y`, then ridges before peaks, then `x`, then `tile`.

## 5. The layer: `www/map/layers/ridges.js`

A static layer, `name = 'ridges'`, inserted in `LAYERS` right after `terrain`. `view.js` is 605 lines:
its only changes are the import, the `LAYERS` entry and a `noCull = false` field; everything else lives
in `map/layers/ridges.js` (and the pure rules in `map/ridges.js`). It:

- Draws nothing where the terrain is a flat fill (`terrain.js` exports how it decides: share the
  height `H` and the flat test from there, don't copy them).
- Builds the pieces once per map, cached in the layer module in a `WeakMap` keyed by `view.nb` (which
  `setLayout` makes new for each map), and draws each
  in order with `drawImage` of `atlas.scaled(key, round(cell.h × H / 148))`, whole device pixels like
  the terrain, its anchor (scaled) on the piece's point.
- Skips a piece whose drawn rectangle misses the canvas, unless `view.noCull`.
- Sets `view.stats.ridges` (pieces drawn) and nothing else in `stats`.

In `terrain.js`, the flat fill for `mountain` takes `mountain.peak.0`'s `flat`, falling back as now
(`terrain.mountain.0`'s `flat`, then `TERRAIN_COLOR`), so a placeholder peak, which has no `flat`, fills
as main does; every other kind keeps what it has.

## 6. The views in `www/shots.mjs`

Add the spec's eleven mountain views to `VIEWS` (keep the ten plains views as they are). A view names
its camera: `on: 'plains'` (the present rule, the default), or a mountain class (`'fork'`, `'bend'`,
`'4+'`) with the tile chosen as `ranges.md` chose it, and an optional `edge: { side: 'bottom' | 'right',
by }`, which moves the camera so the tile's centre sits `by` world units past that canvas edge. The
chooser runs in the page from `game.map` alone (copy `proto/nearest.mjs`'s rule; don't import
`ridges.js`, so `shots.mjs` still copies alone onto main). `views.json` records the tile; the tiles must
be `ranges.md`'s (469, 377, 777, 1937, 1953, 474). `edge` uses the canvas's CSS size (`m.w / 2 / k`,
`m.h / 2 / k`). `small9-edge-right` is at k 35: at k 18 the pan clamp can't put any small-9 mountain past
the right edge (spec, "Settled here"). Every view, plains ones too, throws when the camera after
`viewChanged()` isn't the `cx`, `cy` and `k` it asked for (`clampView` moved it), as it now does for `k`.

`--cull DIR` shoots each mountain view twice in one page, the second after `game.map.noCull = true;
game.map.dirty = true` (or whatever makes the static layer redraw), and exits 1 unless the PNGs are
byte-identical, printing each view's `stats.ridges` for both.

## 7. Checks in CI

- `www/check.mjs`: on its medium world, `ridgeClass` over every mountain tile matches a count made
  straight from the neighbours in the check itself, every shared mountain edge has exactly one ridge
  piece, and the pieces are sorted. Seconds.
- `www/play/ridges.mjs`, run by `play/all.mjs`: a staged map (`play-stage.mjs`) with a lone mountain,
  a straight run of three ending in caps, a bend and a fork, on plains in sea. It checks the pieces'
  keys and kinds against the spec's rules, the draw order, and, with the camera put so a peak's tile
  sits 1.0 world unit below the canvas (asserting the camera wasn't clamped: pick the zoom and the
  tile so it isn't), that `stats.ridges` counts it and a culled and an unculled draw
  give identical canvas PNGs. Seconds.
- `play/art.mjs` still passes (every cell inside the sheet and not blank).

## Done

1. `node www/assets/draw-atlas.mjs` twice: identical sha256; `--check` passes, and fails when a byte
   of `mountain.peak.0` in a scratch copy changes.
2. With `CELLS`' mountain entries and the ridge layer removed in a scratch edit, the ten plains views
   shoot byte-identical to the same views shot at the commit this step starts from (nothing else
   moved); reverted.
3. Main (a worktree at the branch's base, its wasm built, this `shots.mjs` copied in) and the branch,
   every view, `--diff`: cameras match. `--cull` on the branch passes.
4. `relay/specs/440/easel.md`: the sha256 lines, the proto comparison, the `--diff` table, the `--cull`
   output with `stats.ridges`, `stats.terrainMs` beside main's on the five laptop play views, and any
   correction to this brief.
5. `engine/CLAUDE.md` (it has an 8 KB budget `check.sh` enforces): a short line for `map/ridges.js`, the ridges layer and `shots.mjs --cull`.

Run `engine/check.sh` once at the end. Commit as `#440 build-1: the ridge layer`.
