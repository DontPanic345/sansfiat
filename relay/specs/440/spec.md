# #440 Mountains join into ranges across hexes

**Question:** none in the first four scores: no rule moves. This is piece 2 of the illustrated map
(`docs/proposals/map-art.md`). It asks, each against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web`
  is empty, so every digest, `SAVE_VERSION` (27) and every save stay main's without a sweep.
  Selection, pan and zoom are `map/view.js`, which gains the layer's import, its entry in `LAYERS` and a `noCull = false` field, and nothing else.
- **A mountain range reads as one range.** On identical views of the retro maps (same seed, camera,
  zoom and window, shot by `www/shots.mjs`), main's mountains are grey hexes; the branch's are peaks
  standing on their tiles, joined to their mountain neighbours by ridges, so a chain reads as a chain:
  a lone peak alone, a chain's ends as caps, a straight run, a bend and a fork each as what they are.
  Southern pieces overlap northern ones. A peak whose tile is just off the canvas still shows the part
  of it that reaches on: culled and unculled draws are pixel-identical.
- **Everything else draws where it did.** Rivers, passes, borders, fields, icons, clans and units sit
  where main has them and draw over the mountains, and the static draw time (`stats.terrainMs`) stays
  near main's.

The control is main at the branch's base, shot with the same tool (439's way: a main worktree, the
branch's `shots.mjs` copied in).

## What the piece delivers

1. **Sprite regions** (build-1). `www/assets/cut.mjs` cuts a region that is a source rectangle, not a
   hex: a keep-polygon, a ground key that clears grass, the source's own alpha, area-resampled at a
   scale into the cell with the region's ground point on the cell's anchor. Hex regions are unchanged.
2. **The mountain cells** (build-1 keys, build-2 art). New atlas keys after the icons, each 192×128
   with its anchor at (96, 112), the ground point:
   - `mountain.lone.0`–`.1`: a peak with no mountain neighbour.
   - `mountain.peak.0`–`.2`: the peak on a tile in a range.
   - `mountain.ridge.e.0`–`.1`, `mountain.ridge.se.0`–`.1`, `mountain.ridge.sw.0`–`.1`: the ridge
     across one edge between two mountains, anchored at the edge's midpoint (`sw` is `se` mirrored).
   - `terrain.mountain.0`–`.2`, the ground under the peak, cut as the plains are.
3. **The ridge layer** (build-1). `www/map/ridges.js` turns the map into pieces, a pure function;
   `www/map/layers/ridges.js` draws them, a static layer right after `terrain`.
4. **The views** (build-1). `www/shots.mjs` gains the mountain views below and a culling check.
5. **The look, judged** (build-2): which source pieces fill each key, the keys' sizes and anchors,
   whether ends or forks want pieces of their own, all on the views beside main.

## How the pieces are chosen

A mountain tile's class comes from its six neighbours: how many are mountain and, for two, how far
apart. The pieces for a map, built once per map:

- Each mountain tile draws one peak at its centre: `mountain.lone.<i % 2>` with no mountain
  neighbour, else `mountain.peak.<i % 3>`.
- Each pair of neighbouring mountains draws one ridge at the midpoint of the edge they share, once per
  edge: from tile `i` towards its E (direction 0), SE (1) and SW (2) neighbour `j`, the key
  `mountain.ridge.<e|se|sw>.<min(i, j) % 2>`.

So an end cap is a peak with one ridge, a straight run a peak with two opposite, a bend two that turn,
a fork three or more, and a lone peak none; a pass, which is hills, breaks the chain into two caps.
The world's rim counts as not mountain.

**Depth:** pieces draw in order of their ground point's world y, then x; at the same y a ridge draws
before a peak, so an E–W ridge sits behind the two peaks it joins. A piece's drawn rectangle comes from
its cell's size and anchor at the terrain's scale (the drawn height is `round(cell.h × H / 148)`, `H`
the terrain cell's height in device px, so every cell shares the plains' px per world unit).

**Culling:** a piece draws when its drawn rectangle meets the canvas, whatever its tile's distance
from the screen; the layer keeps its own list (a few hundred pieces even on large) and doesn't read
`view.vis`, whose 1.3-unit margin would drop a peak whose tile is below the bottom edge.

## Shape: Easel and Canvas

The pieces, their order, the culling and the views are settled to the pixel and a cheap model builds
them; how the range looks, which is the piece, is judged by eye. No `Shape:` line was given; this is
439's shape, for the same reason.

- **build-1, easel:** sprite regions in the cutter; the new keys with placeholders; the peaks cut from
  `proto/peak.mjs`'s region so the canvas starts from a working range; the piece builder and the ridge
  layer; mountain ground and flat colour; the views and the culling check; a check in CI.
- **build-2, canvas:** the ridges' art, the peaks', the lone peaks', sizes and anchors; the captures on
  main and the branch, and the evidence.

## Settled here, by running

- **The edge views' cameras** (spec-review, `ranges.md`): `small9-edge-bottom` at k 18 puts its camera
  unclamped (cy −20.58 inside ±25.3). `small9-edge-right` can't be at k 18: the laptop canvas is 71
  world units wide there and the small map 58, so the pan clamp holds cx ≥ −28.9 and the right edge at
  x ≥ 6.7, and small 9's easternmost mountain is at x 6.93. At k 35 its camera (cx −14.76) is
  unclamped, which also proves the edge at the second zoom. `shots.mjs` throws when the camera it gets
  isn't the one it asked for, so a clamp can't pass silently.

- **The mountains on the retro maps** (`ranges.md`): 42–295 a map; forks on five of seven, a 4+ blob on
  five, gentle bends on all, sharp bends on five; no mountain on any rim; passes are hills. Tile index
  order is not row order.
- **A peak cut that works** (`ranges.md`, `proto/peak.mjs`, `proto/peak3x.png`): rectangle,
  keep-polygon and ground key, at the plains' scale, give a clean peak with its foot's boulders and no
  grass. The source's range is one opaque sprite, so no piece can come from its alpha alone.
- **The view tiles** (`ranges.md`): each class's tile nearest world (0, 0) on small 2 and 9, medium 9
  and large 8, all well inside a k 18 view.
- **Callers.** `atlas.json` is read by `atlas.js` (by key), `terrain.js` (`terrain.*` by key),
  `fields.js`, `passes.js`, `icon.js`, `units.js` and the like (`overlay.*`, `icon.*`, `unit.*` by key)
  and `play/art.mjs` (every cell inside the sheet and not blank, at least 41). New keys after the icons
  move no existing rectangle. `bar.js` toggles named layers only; a new layer adds no button. Nothing
  in Rust reads the atlas.

## Decisions I made

- **Pieces are a peak per tile and a ridge per shared edge**, not a sprite per neighbour mask: six
  neighbours make 64 masks, while a peak and three edge directions (two by a mirror) compose every one,
  forks and blobs included, from a handful of cells. An end cap, a bend and a fork are what the
  composition makes; the canvas may add pieces of their own where the composition reads wrong.
- **The overhang draws in a static layer right after `terrain`.** Rivers, floodplains, passes, fields,
  borders, clans, icons and units all draw over the peaks, so nothing the player reads hides behind a
  mountain, and the peaks cost a redraw only when the view moves. The hex outline (k ≥ 14) stays in
  `terrain` and so sits under the peaks.
- **A mountain tile's ground is grass**, cut as `terrain.plains.0`–`.2` are, until piece 3 gives
  mountain ground: the target (`ui.png`) stands its peaks on grass.
- **At the flat zoom a mountain fills with `mountain.peak.0`'s `flat`** (the rock's mean), not the
  grass's, so a range still reads as a grey line on a phone's far zoom. No sprites draw there.
- **Picking stays by hex.** A click on the part of a peak over its northern neighbour picks that
  neighbour, as on main.
- **"Map-edge culling" is the canvas's edge.** The world's rim is sea on all seven maps, so no piece
  can be cut there; what can go wrong is a peak popping in as its tile scrolls on. The proof is that a
  culled draw equals an unculled one.
- **New keys go after the icons**, each 192×128 anchored at (96, 112), so every existing key keeps its
  rectangle; the canvas may change these keys' sizes and anchors, never the older ones'.
- **A ridge's variant is `min(i, j) % 2`** of the two tiles it joins, so the map always looks the same.
- **Unit scale for all cells**: a sprite is drawn at the plains cell's px per world unit; a region
  carries its own source scale, starting at the plains' 74/109.

## Unsettled, and which step holds it

- **How a range looks** (build-2, the canvas): which parts of the range in `terrain.png` fill the
  ridges and the lone peaks; whether the end caps and forks read without pieces of their own; the
  pieces' sizes and anchors; how much a peak may rise into the tile north of it before it hides what
  sits there; how the grey of the peaks sits beside placeholder hills and forest. Proof: the captures
  of the views below, main beside the branch, read by eye, the `--diff` shares, and the culling check,
  in `relay/specs/440/views.md`.

## Ideas taken

None: no open idea is about the map's art.

## The views

`www/shots.mjs` keeps its ten plains views and adds these, each at the pick (`?size=S&seed=N&run=1`),
camera on the tile named (chosen as `ranges.md` chose it: the mountain tile of that class nearest world
(0, 0), ties to the lower index). Laptop is 1280×800, phone 390×844.

| View | Map | Window | k | Camera |
|---|---|---|---|---|
| `small9-fork-close` | small 9 | laptop | 35 | the fork (469) |
| `small9-fork-play` | small 9 | laptop | 18 | the fork (469) |
| `small2-bend-close` | small 2 | laptop | 35 | the bend (377) |
| `small2-bend-play` | small 2 | laptop | 18 | the bend (377) |
| `medium9-blob-play` | medium 9 | laptop | 18 | the 4+ tile (777) |
| `large8-bend-play` | large 8 | laptop | 18 | the bend (1937) |
| `large8-fork-play` | large 8 | laptop | 18 | the fork (1953) |
| `small9-edge-bottom` | small 9 | laptop | 18 | the bend (474) 1.0 world unit below the canvas's bottom edge |
| `small9-edge-right` | small 9 | laptop | 35 | the bend (474) 0.8 world units beyond the right edge |
| `small9-fork-edge` | small 9 | phone | 6 | the fork: the last zoom that draws cells |
| `small9-fork-flat` | small 9 | phone | 5 | the fork: the flat fill |

`--cull` shoots each mountain view twice, the second with the ridge layer's culling off
(`game.map.noCull = true`), and fails unless the two PNGs are byte-identical.

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty. `SAVE_VERSION` 27.
- `node www/assets/draw-atlas.mjs` twice: the same sha256 both times and as committed; `--check`
  passes. `atlas.json` has every key main's has, each at main's `x`, `y`, `w`, `h`.
- `node www/check.mjs` (in CI): the piece builder's classes on the medium world match a count made
  straight from the neighbours, and every shared mountain edge has exactly one ridge.
- `node www/play/ridges.mjs` (in CI): on a staged map with a lone peak, a cap, a straight run, a bend
  and a fork, the pieces are those named under "How the pieces are chosen", in depth order, and a
  culled and an unculled draw with a peak's tile below the canvas are identical.
- A main worktree with the branch's `shots.mjs`: shoot every view on both, `--diff`: the cameras
  match. Run `--cull` on the branch. Read the pairs: on the branch the fork, the bends and the blob
  read as ranges, caps end them, southern peaks overlap northern ones, and rivers, passes, borders,
  fields, icons, clans and units sit where main has them. In `small9-edge-bottom` (k 18) the peak reaches up
  onto the canvas, and in `small9-edge-right` (k 35) its flank reaches in. `small9-fork-edge` and `small9-fork-flat` show the range grey at both zooms.
- `stats.terrainMs` beside main's on the play views: the branch's static draw stays within a few ms.
- On the branch page, select a mountain tile, the tile north of a peak and a unit, pan and zoom: they
  behave as on main.
- The PR carries main's and the branch's captures of `small9-fork-close`, `small9-fork-play`,
  `small2-bend-play`, `medium9-blob-play` and `small9-edge-bottom`, from the files build-2 commits under
  `relay/specs/440/shots/`.
