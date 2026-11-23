# #442 Woodland and marsh

**Question:** none in the first four scores, because no rule moves. This is piece 4 of the illustrated map
(`docs/proposals/map-art.md`). It asks each of these against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web`
  is empty, so every digest, `SAVE_VERSION` (27) and every save stay as main's without a sweep. A view
  shot after turns records the game's digest, and it matches main's on the same view. Selection, pan and
  zoom stay as they are: `map/view.js` gains the new layer and one line in `setModel`, and its input
  handling moves out to `map/input.js` without changing.
- **A forest is a wood, and a marsh has reeds.** Shoot identical views of the retro maps on main and on
  the branch: the same seed, camera, zoom and window, shot by `www/shots.mjs`. On main, a forest is a
  dark floor with no trees. On the branch, trees cut from the source studies stand on every forest tile.
  A forest's interior reads as dense wood, and toward open land it thins to small, scattered trees. A
  lone wooded hex is a clump with saplings round it, not a hex-shaped block. On main, a marsh is dark
  grass with pinprick pools (`441/notes.md`). On the branch, it is reeds built from the source palette
  over wet ground with water you can see. Its edge on the sea is reeds, not the tan bank.
- **Rivers and fields stay visible.** On a forest or marsh tile, the trees and reeds leave the river's
  course clear. On a tile with fields, they leave the clearing the furrows draw in. Fields lie on forest
  often (20 of 38 field tiles on medium 9 after 8 turns: `proto/fields.txt`), so a worked wood shows as
  a clearing. Rivers, floodplains, passes, borders, fields, icons, clans and units still draw over the
  wood, where main draws them.
- **The zoom boundary holds.** Trees and reeds stop drawing below a cell height (`WOODS.from`). Just
  below it, and at the flat zoom (phone k 5), the ground alone still tells forest and marsh from their
  neighbours, with no pair of the nine flat colours closer than 28 in RGB (441's bar). Culled and
  unculled draws are pixel-identical. The woods add at most 3 ms to the static draw (`stats.terrainMs`,
  median of five redraws) on `medium9-woods-play` and `large8-play`.

The control is main at the branch's base, shot with the same tool: a main worktree with the branch's
`shots.mjs` copied in, as 439 to 441 did.

## What the piece delivers

1. **Tree and reed cells** (build-1), new keys after the fringe cells, each a sprite with its ground
   point at the anchor:
   - `tree.big.0`–`.2`: a full tree, cut from the source's tree groups.
   - `tree.small.0`–`.2`: a sapling or small tree, cut from the right group's thinning tail.
   - `reed.0`–`.2`: a reed clump. The source has no reeds, so a reed clump is the plains hex's yellow
     tuft, stretched tall and recoloured onto the source's reed colours (its dark tree green and its
     tuft straw).

   To make these, `cut.mjs` gains `scaleY`, a sprite region's own vertical scale, and `sources.js`
   gains a `tuft` key that keeps a tuft's blades and clears the grass.
2. **The wood pieces** (build-2): `www/map/woods.js` is a pure function that turns a map and its field
   tiles into tree and reed pieces. `www/map/layers/woods.js` draws them as a static layer, right after
   `edges` and before `ridges`. The layer exports `stale(model, view)`, which `view.setModel` calls so
   the layer redraws when the field tiles on forest and marsh change.
3. **Marsh shores** (build-2): in `map/edges.js`, an edge between a marsh tile and water draws no bank,
   and a vertex whose land tiles are all marsh draws no corner.
4. **The views and checks** (build-2): `www/shots.mjs` gains the views below. They need a `turns`
   option (lead the first civ, end N turns) and the wood classes in the chooser, and `--cull` covers
   them. `www/check-map.mjs` and `www/play/woods.mjs` check the pieces in CI. `view.js`'s
   `attach()` and `pinchState()` move to `www/map/input.js`, because `view.js` is 608 lines and this
   piece adds to it.
5. **The look, judged** (build-3): the trees, the reeds, the slots and their thinning, the clearings,
   `WOODS.from`, the forest and marsh grounds (the marsh's water larger), the flats, and the time,
   all on the views beside main.

## How the pieces are chosen

All in world units: a hex has circumradius 1 and apothem √3/2. Every number is in `WOODS`, a table at
the top of `map/woods.js` with a `note` per number. Build-2 starts it at the values below, and build-3
tunes it.

- **Slots.** A tree tile and a reed tile each have a fixed set of slots: a triangular lattice through
  the hex's centre (spacing `gap`: 0.42 for trees, 0.36 for reeds; rows `gap·√3/2` apart, every other
  row offset by `gap/2`), kept where every edge is at least `inset` (0.1) away. Each slot is moved by
  up to `jitter` (0.07) in x and y. Its variant is also drawn from a hash of the tile index and the
  slot index alone, so a map always looks the same.
- **Trees** (forest tiles). For each slot, take its nearest edge `d` and its depth `s` from that edge.
  An edge is *open* when the neighbour across it is not forest or is off the map.
  - If edge `d` is open and `s < band` (0.32), the slot is sparse. It holds a tree only when its hash is
    under `sparse` (0.4), and then a `tree.small`.
  - Otherwise the slot holds a `tree.big`.

  So an interior tile is all big trees. An edge tile thins toward its open sides. A lone tile is a
  clump of big trees in its middle with a few saplings round it.
- **Reeds** (marsh tiles). Every slot holds a `reed.<v>`.
- **Clear ground.** A slot holds nothing when either of these holds:
  - It lies within `river` (0.22) of the tile's river course: the segment from its centre toward its
    flow tile's centre, when the tile is a river (`river > 0`, `flow ≥ 0`), and the segment from each
    neighbour that flows into it to its centre. Each segment is clipped at the hex edge.
  - The tile has fields (any clan's plot this turn) and the slot lies in the clearing: `|x| < field.x`
    (0.6) and `field.top < y < field.bottom` (0.0 to 0.8), relative to the centre. That is where
    `fields.js` draws its furrows.
- **Order:** by y, then x, then tile, then slot. A tree in front covers one behind, across tiles too.
  Each piece draws at the terrain's scale: drawn height `round(cell.h × H / 148)`, as the ridges and
  edges do. Its anchor goes on the slot.
- **Zoom:** nothing draws while the cell height `H` is under `WOODS.from` (24 device px, which is k 12
  at dpr 1) or the terrain is flat.
- **Culling:** a piece draws when its drawn rectangle meets the canvas. `view.noCull` draws every piece,
  and `shots.mjs --cull` holds the two draws identical.

## Shape: Easel and Canvas

The cells' keys, the piece rules, their order, culling, the staleness hook, the marsh shore, the views
and the checks are all settled here to the rule. The first cuts and starting numbers only need to be
visible and in place, so cheap builds can make them. How the wood and the marsh look is the piece, and
it is judged by eye on real maps, so it belongs to one canvas. No `Shape:` line was given. This is 439's
to 441's shape, for the same reason, with the easel cut by area as 441's was: the atlas, then the map.

- **build-1, easel (the atlas):** `scaleY`, the `tuft` key, and first cuts of the nine tree and reed
  cells, with `--check` holding them.
- **build-2, easel (the map):** the piece builder, the layer, the staleness hook and the marsh shore.
  Also `view.js`'s input split, the views (`turns`, the chooser, `--cull`) and the checks in CI.
- **build-3, canvas:** how the woods and the marsh look, `WOODS`'s numbers, the two grounds, the flats,
  the time, the captures on main and the branch, and the evidence.

## Settled here, by running

- **The woods on the retro maps** (`proto/woods.mjs`, `proto/woods.json`):
  - 33 to 213 forest tiles a map, in 11 to 46 clusters, the biggest 15 to 20 tiles.
  - Mostly edge tiles. Interior tiles (six forest neighbours) are 1 a map on the small and medium 8
    maps, and 10 and 8 on medium 9 and large 8.
  - 2 to 13 lone tiles a map.
  - 1 to 40 forest tiles carry a river.
  - Forest meets coast most, then plains, hills and mountain.
  - Marsh is on small 2 (8 tiles) and large 8 (9 tiles) only, at deltas. It meets the sea on 16 and
    18 edges, and half of it carries a river.

  So the sparse edge is what most forest looks like. The dense interior is the rare case, best seen on
  medium 9, and the lone hex is on every map.
- **Fields lie on forest and marsh** (`proto/fields.mjs`, `proto/fields.txt`). Leading the first civ,
  forest holds 3 to 5 field tiles on small 2 by turn +12, 13 to 20 on medium 9 and 25 to 33 on large 8.
  Marsh holds 4 to 8 on small 2 and large 8. No field exists at the end of the pre-game, where the 41
  views stand, so the field views run 8 turns. The game is the same each time: small 2 run twice gave
  digest `ff55c006fe645483` both times.
- **The source** (`terrain.png`, the middle row) has three tree groups on transparent ground: a dense
  clump at x 40–280, a long band at x 330–990, and a group that thins out to the right at x 1050–1480.
  - The right group's tail has isolated small trees: about (1400–1428, 563–606), with its trunk base
    near (1412, 603), and (1435–1470, 541–603), base near (1452, 600). There is a mid tree at about
    (1338–1386, 530–606).
  - The front trees of the left clump are whole in front of the others, with bases along y 600–640
    (one at about (130–192, 555–640), base (160, 637)).
  - There are no reeds. The plains hex's yellow tufts (around (500–540, 810–830) and (560–590,
    790–806)) are the nearest thing.
- **The views' cameras** (`proto/cams.mjs`): each new view's tile, set at its zoom and window on main, reads
  back unclamped. The laptop's least zoom is k 7.47, so `WOODS.from` can't move the `-far` view below that.
- **The time** (measured on main, medium 9 at k 18): the static draw is 17–19 ms over five redraws,
  with 1133 cells, 2425 edge pieces and 248 ridge pieces. 1500 small `drawImage`s on the layer take
  about 1 ms. Twelve slots on each of about 150 visible forest tiles fits a 3 ms bar.
- **Callers.** `atlas.json` is read by key: by `atlas.js`, `terrain.js`, `edges.js`, `ridges.js`,
  `fields.js`, `floodplains.js`, `passes.js`, `icon.js` and `units.js`. `play/art.mjs` checks that
  every cell lies inside the sheet and is not blank. New keys after the fringe cells move no older
  rectangle. No toggle names `edges` or `ridges`, and the woods layer is the same: on from the start,
  outside `OFF`. `legend.js` (the HUD) is out of scope. No Rust reads the atlas. `fields.js` draws a
  tile's furrows at `y + 0.18` to `y + 0.62`, half-length 0.5, which is where the clearing comes from.
  `rivers.js` draws centre to centre, 0.62 into water.

## Decisions I made

- **Trees stand only on forest tiles, reeds only on marsh.** Woodland blends into its neighbours by
  thinning on its own side of the edge, and 441's forest fringe still bleeds the floor across. A plains
  tile beside a wood keeps clear ground for its fields, clans and tooltip. Crowns may still rise over
  the edge into the tile above, as peaks do.
- **A wood thins by distance to an open edge.** Slots near a non-forest edge are sparse and small, so
  interior, edge and lone tiles all come from one rule, and a forest of any shape gets a ragged margin.
  The alternative was a cell per neighbour mask (64 of them).
- **Single trees in slots, sorted by depth**, not one composed clump per tile. A forest reads as one
  wood across hexes, not as hex-shaped clumps. Trees overlap correctly across tiles, and a river or a
  clearing can be cut out tree by tree.
- **A worked wood is a clearing.** A forest or marsh tile with fields this turn leaves the furrow
  band empty, so the furrows show on cleared ground, not on the canopy. This makes the woods layer
  depend on the turn's plots. It stays static: it redraws only when the field tiles on forest or marsh
  change (`stale`), not every frame.
- **A river runs through the wood in the open.** Slots within 0.22 of the river's course are empty.
  The river layer still draws over everything, as now.
- **The woods layer sits after `edges` and before `ridges`**, so peaks stand over trees, and rivers,
  fields and the rest draw over both.
- **Trees stop below a cell height (`WOODS.from`)**, not at the flat zoom. A tree a few pixels high is
  noise, and it costs a draw. Below that height the forest ground and its flat colour carry the forest.
  The canvas sets the height.
- **The marsh's sea and lake edges are reeds, not the bank.** 441 left this to this piece. A delta is
  soft ground running into water, and a tan wall there reads as coast. A vertex where only marsh meets
  the water draws no corner. Where marsh and other land share a shore vertex, the corner still draws.
- **Reeds are built from the source's tuft and colours.** That means a tuft stretched tall, between the
  tree's dark green and the tuft's straw. No colour comes from outside the sheet.
- **The marsh's water gets larger** (`441/notes.md`), so a delta reads as wet at play zoom. How large is
  the canvas's call.
- **Forest ground may lighten under the trees**, toward the source's grass under its trees, if the
  canvas finds it reads better. Its flat must still be at least 28 from every other kind's.
- **`view.js`'s input handling moves to `map/input.js`**, because `view.js` is over 600 lines and this
  piece adds to it. `attach()` and `pinchState()` move as they are.

## Unsettled, and which step holds it

- **How the woods and the marsh look** (build-3, the canvas). This covers:
  - the tree cuts (which trees, scale, anchors) and the reed recipe;
  - `WOODS`: `gap`, `inset`, `jitter`, `band`, `sparse`, `river`, the clearing, and `from`;
  - whether a third tree size or a clump cell helps the interior read as dense;
  - the forest and marsh grounds, with the marsh's water larger;
  - the flats, with no pair under 28 RGB;
  - the time, within 3 ms.

  Proof: the captures of the views below, main beside the branch, read by eye, plus the `--diff` shares,
  `--cull`, the five-redraw timings and the flat pairs, in `relay/specs/442/views.md`.

## Ideas taken

None were listed. From `441/notes.md` this piece takes the marsh note (reeds and larger water). It
leaves the per-tile composite and the sea's speckle.

## The views

`www/shots.mjs` keeps its 33 views and adds the ones below. Each opens at its pick
(`?size=S&seed=N&run=1`) with the camera on the tile named. That tile is the one of its class nearest
world (0, 0), ties to the lower index. The classes:

- `interior`: a forest tile with six forest neighbours.
- `lone`: a forest tile with none.
- `forest-river`: a forest tile that is a river (`river > 0`, `flow ≥ 0`) or that a river flows into.
- `forest-fields` and `marsh-fields`: such a tile with a clan's plot.
- `marsh`: as now.

A view with `turns: N` leads the first civ after the pre-game and ends N turns before setting the
camera, and records `game.digest()` in `views.json`. Laptop is 1280×800 and phone is 390×844.

| View | Map | Window | k | Camera |
|---|---|---|---|---|
| `medium9-woods-close` | medium 9 | laptop | 35 | the interior (827) |
| `medium9-woods-play` | medium 9 | laptop | 18 | the interior (827) |
| `medium9-woods-near` | medium 9 | laptop | 12 | the interior (827): the last zoom with trees |
| `medium9-woods-far` | medium 9 | laptop | 11 | the interior (827): the first zoom without |
| `medium9-woods-edge` | medium 9 | phone | 6 | the interior (827): the last zoom of cells |
| `medium9-woods-flat` | medium 9 | phone | 5 | the interior (827): the flat fill |
| `small2-lone-close` | small 2 | laptop | 35 | the lone tile (439) |
| `small2-lone-play` | small 2 | laptop | 18 | the lone tile (439) |
| `small9-river-close` | small 9 | laptop | 35 | the forest river (258) |
| `small2-delta-near` | small 2 | laptop | 12 | the marsh (506) |
| `small2-delta-far` | small 2 | laptop | 11 | the marsh (506) |
| `small2-delta-edge` | small 2 | phone | 6 | the marsh (506) |
| `small2-delta-flat` | small 2 | phone | 5 | the marsh (506) |
| `medium9-fields-close` | medium 9, 8 turns | laptop | 35 | the forest with fields (828) |
| `small2-marsh-fields-close` | small 2, 8 turns | laptop | 35 | the marsh with fields (535) |

If build-3 moves `WOODS.from`, it moves the `-near` and `-far` views' k to straddle the new value.
`small2-delta-close` and `-play` and `large8-delta-play` already show the marsh at close and play
zoom.

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty, and `SAVE_VERSION` is 27. On
  both trees, the turns views' `digest` in `views.json` is the same.
- Run `node www/assets/draw-atlas.mjs` twice: the sha256 is the same both times and the same as
  committed, and `--check` passes. `atlas.json` has every key main's has, each at main's `x`, `y`, `w`
  and `h`.
- `node www/check-map.mjs` (in CI) checks, on the medium world, that:
  - every forest slot holds the piece the rule says;
  - no piece lies within `WOODS.river` of a river course;
  - marsh shores draw no bank;
  - on a synthetic layout with fields, the clearing is empty;
  - the order is the spec's.
- `node www/play/woods.mjs` (in CI), on a staged map, checks a seven-tile wood (its centre interior), a
  lone forest tile, and marsh on the sea. The pieces at named tiles are the spec's, there is no bank on
  the marsh's sea edges, and culled and unculled draws with a wood just off the canvas are identical.
- Make a main worktree with the branch's `shots.mjs`, shoot every view on both and `--diff`: the
  cameras match. Run `--cull` on the branch. Then read the pairs:
  - the medium 9 interior reads as dense wood that thins at its margins;
  - the small 2 lone hex is a clump with saplings;
  - the small 9 river runs through the trees in the open;
  - the fields views show furrows in a clearing;
  - the delta reads as reeds and water, with no tan bank on its sea edge;
  - peaks, rivers, borders, icons, clans and units sit where main has them;
  - at `-near` and `-far`, trees appear and go with no other change, and forest and marsh still read;
  - at `-edge` and `-flat`, every kind is told apart.

  `views.md` lists the nine flats with their closest pair, at least 28 apart.
- On `medium9-woods-play` and `large8-play`, compare the five-redraw median of `stats.terrainMs` with
  main's: the branch is at most 3 ms more.
- On the branch page, select a forest tile, a marsh tile and a unit in a wood, then pan and zoom across
  `WOODS.from`. They behave as on main.
- The PR carries main's and the branch's captures of `medium9-woods-close`, `small2-lone-close`,
  `small9-river-close`, `small2-delta-close`, `medium9-fields-close`, `medium9-woods-near` and
  `medium9-woods-far`, from the files build-3 commits under `relay/specs/442/shots/`.
