# #441 Coastlines, lakes and the remaining ground types

**Question:** none in the first four scores: no rule moves. This is piece 3 of the illustrated map
(`docs/proposals/map-art.md`). It asks, each against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web`
  is empty, so every digest, `SAVE_VERSION` (27) and every save stay main's without a sweep.
  Selection, pan and zoom are `map/view.js`, which gains the new layer's import, its entry in
  `LAYERS` and a word in `noCull`'s comment, and nothing else.
- **Every ground is art, and water has a shore.** On identical views of the retro maps (same seed,
  camera, zoom and window, shot by `www/shots.mjs`), main's sea, lake, coast, hills, forest, desert and
  marsh are flat placeholder hexes; the branch's are ground cut from the source studies, and every
  edge between water and land carries a bank that turns at the hex corners without a gap or a lump.
  A neck reads as land with water on both sides, a strait as water that stays open, a bay and a
  one-tile pond as water held by a shore, and a river mouth opens through the bank into the water.
  Neighbouring land of different kinds meets in a ragged edge, not a hex outline.
- **Everything else draws where it did, at every zoom.** Rivers, floodplains, passes, borders,
  fields, icons, clans and units sit where main has them and draw over the new ground; mountains
  still stand on their tiles. Below the cell zoom (the phone's k 5) the flat fill still tells sea,
  lake, coast, plains, forest, hills, mountain, desert and marsh apart, in the colours of their art.
  Culled and unculled draws are pixel-identical, and the static draw time (`stats.terrainMs`) stays
  near main's.

The control is main at the branch's base, shot with the same tool (439's and 440's way: a main
worktree, the branch's `shots.mjs` copied in).

## What the piece delivers

1. **New cut ops** (build-1), in `www/assets/cut.mjs`:
   - **fill**: a hex region whose samples read a source rectangle repeated, every other copy mirrored
     so the copies seam, at a scale and an offset (so variants differ). Water has no hex in the source
     (`shores.md`).
   - **ramp**: a recolour after the cut: each pixel's luminance onto a ramp between colours taken as
     the means of named source rectangles, so a ground the source lacks (desert, marsh, forest floor,
     coast, lake water) is built from one it has, its colours from the sheet.
   - **rotate**: a sprite region turned about its ground point, so one straight run of bank serves all
     six edges.
   - **derived cells**: a cell made from another cut cell, masked: the land fringes (below), each
     cropped to its mask.
   - **flat from a rectangle**: a cell may name `flat: [x, y, w, h]`, a source rectangle whose mean
     colour is the cell's `flat` instead of the cut's mean, so a ground whose cell is mostly grass (hills)
     still has its own colour at the flat zoom.
2. **The grounds** (build-1 first cuts, build-3 judged): `terrain.<kind>.0`–`.2` for sea, lake,
   coast, forest, hills, mountain, desert and marsh are cut from the sources (plains already is).
   No key is added for a ground: they are main's keys, at main's rectangles.
3. **The shore and fringe cells** (build-1), new keys after the mountain pieces:
   - `shore.edge.<d>.0`–`.1`, `d` 0–5: the bank across the edge from a land tile to its water
     neighbour in direction `d` (`EDGE_DIR`: E, SE, SW, W, NW, NE), anchored at the edge's midpoint.
   - `shore.in.<c>` and `shore.out.<c>`, `c` 0–5: the bank turning at a hex vertex, anchored at the
     vertex. `in`: the vertex is corner `c` of a water tile whose two other tiles are land (water pokes
     into land). `out`: corner `c` of a land tile whose two other tiles are water (land pokes into
     water). Corner `c` is `geom.js`'s `CORNER[c]`: 0 upper right, 1 lower right, 2 bottom, 3 lower
     left, 4 upper left, 5 top.
   - `fringe.<kind>.<d>.0`–`.1` for each kind in `FRINGE` (below) and `d` 0–5: that kind's ground,
     masked to a ragged strip inside a tile along its edge `d`, anchored at that tile's centre.
4. **The edge pieces and their layer** (build-2): `www/map/edges.js` turns a map into pieces, a pure
   function; `www/map/layers/edges.js` draws them, a static layer right after `terrain` and before
   `ridges`.
5. **The views and checks** (build-2): `www/shots.mjs` gains the shore views below, its chooser the
   shore classes, and `--cull` covers them; `www/check.mjs` and `www/play/edges.mjs` check the pieces
   in CI; `www/check-map.mjs` takes the ridge checks out of `check.mjs` (602 lines) beside the new ones.
6. **The look, judged** (build-3): every ground, the bank and its corners, the fringes and the flat
   colours, on the views beside main.

## How the pieces are chosen

A tile's **kind** is the terrain's name, with water split: terrain 0 is `lake` where `l.lake`, else
`sea`. Water is `sea` and `lake`; land is every other kind. The pieces for a map, built once per map:

- **Shore edges.** Each shared edge between a land tile `i` and a water tile `j` draws one piece,
  `shore.edge.<d>.<min(i, j) % 2>` at the edge's midpoint, `d` the direction from `i` to `j`; except a
  **mouth**, an edge a river crosses into the water (`l.river[i] > 0` and `l.flow[i] === j`), which
  draws none, so the river's line (drawn later, 0.62 of the way into the water tile, as now) runs
  through a gap in the bank.
- **Shore corners.** Each hex vertex whose three tiles are all on the map and include one or two
  water tiles draws one piece at the vertex: with one water tile `t`, `shore.in.<c>`; with two, the
  land tile `t` and `shore.out.<c>`; `c` the vertex's corner index on `t`. A vertex is enumerated once:
  as tile `i`'s corner 1 (with its E and SE neighbours, at which it is E's corner 3 and SE's corner 5)
  or corner 2 (with its SE and SW neighbours: SE's corner 4, SW's corner 0). A corner draws at a mouth
  too: it frames the gap.
- **Fringes.** Each shared edge between two land tiles of different kinds draws one fringe of the
  kind higher in `FRINGE` into the other tile: `fringe.<high>.<d>.<min(i, j) % 2>` at the lower tile's
  centre, `d` the direction from the lower tile to the higher one. `FRINGE`, highest first: desert,
  marsh, forest, hills, mountain, coast. Plains is lowest and never bleeds; a kind not in `FRINGE`
  draws no fringe.
- **Off the map** is nothing: an edge or vertex with an off-map tile draws no piece (the rim is sea
  on every retro map).

**Order:** fringes first (by their kind from lowest in `FRINGE` to highest, then y, x, tile), then
shore edges, then shore corners (each by y, x, tile). All of it is ground: none overhangs a hex by more
than its strip, and mountains (the `ridges` layer, after) stand over it. Each piece draws at the
terrain's scale (drawn height `round(cell.h × H / 148)`, as the ridges are) with its anchor on its point.

**Culling:** a piece draws when its drawn rectangle meets the canvas; `view.noCull` (now named for
both layers) draws every piece, and `shots.mjs --cull` holds the two draws identical. At the flat
zoom the layer draws nothing.

## Shape: Easel and Canvas

The ops, the keys, the piece rules, their order and culling, the views and the checks are settled
here to the rule, and the first cuts only need to be visible and in place: cheap builds make them. How
the shores and grounds look, which is the piece, is judged by eye. No `Shape:` line was given; this is
439's and 440's shape, for the same reason. The easel is two steps, cut by area: the atlas, then the
map.

- **build-1, easel (the atlas):** the four ops; first cuts of the eight grounds; the bank's edge
  cells, its corner cells and the fringe cells; `--check` holding them.
- **build-2, easel (the map):** the piece builder and the layer; the views, the chooser, `--cull`;
  the checks in CI.
- **build-3, canvas:** the look of every ground, the bank, its corners and the fringes; the flat
  colours; the captures on main and the branch, and the evidence.

## Settled here, by running

- **The shores on the retro maps** (`shores.md`): 526–1710 shore edges a map and as many corners,
  half of each kind; 22–39 necks, 22–145 straits, 19–67 sea bays; ponds of one tile on four maps; 3–13
  mouths into the sea and 0–12 into lakes. Water meets every land kind, not only coast (lakes meet
  hills and mountains often), so the bank goes on all of them. Marsh is on small 2 and large 8 only,
  at deltas.
- **The source has no hex of water, no desert, no marsh** (`shores.md`): opaque water only in bands
  (clean blocks about 137×60 at (1270, 832)); a bank of tan blocks about 15–20 px thick, level at x
  1040–1240, y 880–905. So water is filled, the missing grounds are recoloured, and the bank is one
  strip turned six ways.
- **A method that works** (`proto/shore-cut.mjs`, `proto/sea.png`, `proto/bank.png`,
  `proto/patch3x.png`): a sea hex filled from the block at (1270, 832) 137×60, mirrored copies, at the
  plains' scale 74/109; a bank cut from the level run (x 1060–1190, from 22 px above the water line at
  y 902 to 4 px below, keeping tan pixels `r > 0.9 g && r > b + 30`) turned to face each edge
  (`angle = 60 d − 90` degrees from the source's south-facing run), on the land side of the edge. The
  patch reads as grass, a bank and sea. The block's mirror shows as a regular ripple pattern, and the
  bank's ends leave small gaps and overlaps at the corners: those are the canvas's.
- **The flat colours can't all be cell means** (spec-review, measured on `terrain.png`): the two hill
  hexes average `(127, 135, 48)` and `(126, 134, 48)`, the plains hex `(122, 133, 50)`: 6 apart in RGB.
  Taken as the cell's mean, hills would be plains at the flat zoom. Main's nine flat colours are at
  least 28 apart (its closest pair hills and mountain), so that is the bar: **no two of the nine kinds'
  flat colours (sea, lake, coast, plains, forest, hills, mountain, desert, marsh) closer than 28 in
  RGB**. A cell's `flat` may come from a source rectangle (the hill's ochre, say) for this; every flat
  is still a colour of the sheet.
- **The views' cameras** (`shores.md`, `proto/cams.mjs`): each view's tile below, set at its zoom and
  window on main, reads back unclamped.
- **Callers.** `atlas.json` is read by `atlas.js` (by key), `terrain.js` (`terrain.*` by key and
  their `flat`), `ridges.js` (`mountain.*`), `fields.js`, `floodplains.js`, `passes.js`, `icon.js`,
  `units.js` (`overlay.*`, `icon.*`, `unit.*` by key) and `play/art.mjs` (every cell inside the sheet
  and not blank, at least 41). New keys after the mountain pieces move no older rectangle. `bar.js`
  toggles named layers only. `legend.js` has its own swatches (the HUD, out of scope). Nothing in Rust
  reads the atlas. Growing the sheet moves a few dozen pixels of the old views (`440/easel.md`: Chrome
  samples a taller image slightly differently), so no view is held byte-identical to main.

## Decisions I made

- **Shores and fringes are composed per edge and per vertex**, not drawn as a cell per neighbour
  mask: six neighbours make 64 masks for each pair of kinds, while six edge directions and twelve
  corners compose every coastline, necks and ponds included, from a few dozen cells.
- **The bank goes on every land kind against water**, sea and lake alike, marsh included; piece 4
  (woodland and marsh) may soften a delta's edge with reeds.
- **The bank lies on the land side of the edge**, its water face on the edge line, so a water tile
  keeps its whole hex of water and a one-tile strait stays open.
- **A river mouth is a gap in the bank**: no edge piece where a river flows into water; the corners
  either side still draw and frame it. The river's line stays as it is and runs through.
- **Lake water is the sea's water, lighter**, as main's lake is lighter than its sea, so an inland
  lake still reads as a lake.
- **Coast is grass warmed toward the bank's sand**, a band you can see along every shore, but land;
  the bank does the rest. The target (`ui.png`) has grass to the water's edge.
- **Forest ground is the forest floor: grass darker and cooler**, which must read as forest at play
  zoom and at the flat zoom until piece 4 plants the trees.
- **Hills are the source's two hill hexes** (the bottom-left row's first two), cut as the plains are,
  their neighbours' hills patched out.
- **Mountain ground stays grass** unless the canvas finds rockier grass reads better under the peaks.
- **Desert and marsh are built from the grass**: desert recoloured onto the source's sand and ochre,
  marsh darkened with pools of the water. Whether the desert stipple (`overlay.desert-dots`) still draws
  is the canvas's call: deserts must stay telling apart at a glance (`map.md`, On the page).
- **Land meets land in fringes of the higher kind**, `FRINGE` desert, marsh, forest, hills, mountain,
  coast, plains lowest: the strong grounds bleed into the grass, not the grass into them. The order
  and which kinds bleed are a table the canvas may change. Land fringes have no corner pieces; the
  canvas adds them only if a seam shows.
- **The flat fill takes every ground's `flat`**, as plains does now: its cell's mean, or the mean of a
  source rectangle the cell names where the mean is too close to another kind's (hills' starts from its
  mound's ochre). The mountain keeps its peak's grey. No two kinds' flats closer than 28 in RGB, main's
  closest pair.
- **Variants are by tile index**: a ground `i % 3` (as now), an edge piece `min(i, j) % 2`, a corner
  one variant, so the map always looks the same.
- **The layer is `edges`**, between `terrain` and `ridges`: rivers, floodplains, passes and the rest
  draw over the shore, and peaks over both.
- **New keys go after the mountain pieces**, each a cell its build sizes, so every older key keeps its
  rectangle; fringe cells are cropped to their strip, so the sheet grows by rows, not pages.

## Unsettled, and which step holds it

- **How the shores and grounds look** (build-3, the canvas): the water's fill (block, scale, the
  ripples' repeat), the lake's tone, the bank's strip, thickness and placement, the twelve corners
  (gaps, lumps, the bend at a neck and in a pond), the eight grounds' recipes and colours beside the
  grass and the peaks, the fringes' depth and raggedness and `FRINGE`, the desert stipple, and the flat
  colours' contrast (each kind's `flat` rectangle, no pair closer than 28). Proof: the captures of the views below, main beside the branch, read by eye, the
  `--diff` shares and `--cull`, in `relay/specs/441/views.md`.

## Ideas taken

None: no open idea is about the map's art.

## The views

`www/shots.mjs` keeps its 21 views and adds these, each at the pick (`?size=S&seed=N&run=1`), camera on
the tile named: the tile of that class nearest world (0, 0), ties to the lower index, with the classes
as `shores.md` defines them. Laptop is 1280×800, phone 390×844.

| View | Map | Window | k | Camera |
|---|---|---|---|---|
| `small2-neck-close` | small 2 | laptop | 35 | the neck (402) |
| `small2-delta-close` | small 2 | laptop | 35 | the mouth into the sea (506, a delta) |
| `small2-delta-play` | small 2 | laptop | 18 | the mouth into the sea (506) |
| `small6-bay-close` | small 6 | laptop | 35 | the bay (376) |
| `small9-pond-close` | small 9 | laptop | 35 | the pond (341) |
| `small9-lake-play` | small 9 | laptop | 18 | the mouth into a lake (377) |
| `medium8-strait-close` | medium 8 | laptop | 35 | the strait (875) |
| `medium9-neck-play` | medium 9 | laptop | 18 | the neck (1004) |
| `large8-delta-play` | large 8 | laptop | 18 | the marsh (2639) |
| `large8-pond-play` | large 8 | laptop | 18 | the pond (1289) |
| `small2-neck-edge` | small 2 | phone | 6 | the neck: the last zoom that draws cells |
| `small2-neck-flat` | small 2 | phone | 5 | the neck: the flat fill |

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty. `SAVE_VERSION` 27.
- `node www/assets/draw-atlas.mjs` twice: the same sha256 both times and as committed; `--check`
  passes. `atlas.json` has every key main's has, each at main's `x`, `y`, `w`, `h`.
- `node www/check-map.mjs` (in CI, new): on the medium world, every land–water edge has one shore edge piece
  but the mouths, which have none; every vertex with one or two water tiles has one corner piece of the
  right side and corner; every edge between land kinds has one fringe of the higher kind; the order is
  the spec's.
- `node www/play/edges.mjs` (in CI): on a staged map with a neck, a bay, a one-tile pond and desert
  beside plains, the pieces at named places are the spec's, and culled and unculled draws with a shore
  tile just off the canvas are identical.
- A main worktree with the branch's `shots.mjs`: shoot every view on both and `--diff`: the cameras
  match. Run `--cull` on the branch. Read the pairs: every ground is art; the bank follows every shore
  and turns at its corners; the neck reads as land, the strait stays open, the bay and the ponds read
  as water held by a shore, the mouths open through the bank; lakes are lighter than the sea; land
  kinds meet in fringes; rivers, floodplains, passes, borders, fields, icons, clans, units and peaks
  sit where main has them. `small2-neck-edge` and `small2-neck-flat` tell every kind apart at both
  zooms, and `views.md` lists the nine flat colours with their closest pair, at least 28 apart in RGB.
- `stats.terrainMs` beside main's on the play views: the branch's static draw stays within a few ms.
- On the branch page, select a coast tile, a water tile and a unit, pan and zoom: they behave as on
  main.
- The PR carries main's and the branch's captures of `small2-neck-close`, `small2-delta-close`,
  `small6-bay-close`, `small9-lake-play`, `medium9-neck-play` and `small2-neck-flat`, from the files
  build-3 commits under `relay/specs/441/shots/`.
