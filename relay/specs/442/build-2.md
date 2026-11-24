# #442 build-2 (easel): the wood pieces, the woods layer, the marsh shore, the views and the checks

Read `relay/specs/442/spec.md` first. Its "How the pieces are chosen" and "The views" are your rules,
word for word. Build-1 has cut `tree.big.0`–`.2`, `tree.small.0`–`.2` and `reed.0`–`.2`, each with its
ground point at the anchor; its summary and `relay/specs/442/atlas.md` say what it left. Build-3, the
canvas, tunes `WOODS` and the cells by eye. Your job is rules that are exact, numbers that run, and
checks that hold the rules whatever numbers build-3 picks. Don't change Rust: nothing under
`engine/core`, `engine/cli` or `engine/web`. Leave `www/assets/` alone, since it is build-1's and
build-3's.

Read these before you edit:

- `www/map/{view.js,edges.js,ridges.js,geom.js}`, `www/map/layers/{edges.js,ridges.js,terrain.js,fields.js,rivers.js}`;
- `www/shots.mjs`, `www/check-map.mjs`, `www/play/{edges.mjs,ridges.mjs}`, `www/play-stage.mjs`,
  `www/play/all.mjs`;
- `relay/specs/442/proto/{woods.mjs,fields.mjs}` (the classes and the turns loop).

`layers/ridges.js` and `layers/edges.js` are the pattern for the layer: pieces built once per map,
scaled to the terrain, culled by their drawn rectangle.

## 1. `www/map/woods.js` (new, pure)

- `WOODS` is the table of numbers, each with a `note`: `gap` 0.42 for trees and 0.36 for reeds,
  `inset` 0.1, `jitter` 0.07, `band` 0.32, `sparse` 0.4, `river` 0.22, the clearing `{ x: 0.6, top:
  0.0, bottom: 0.8 }`, and `from` 24 (device px of cell height). Keep it in one table so build-3 tunes
  one place.
- `slotsOf(gap)` returns the lattice points of the spec, which are the same for every tile.
- `woodPieces({ terrain, river, flow, nb, px, py, forest, marsh, fields })` returns
  `{ key, x, y, tile, slot, kind }`, with `kind` either `'tree'` or `'reed'`. `fields` is a `Set` of
  tile indices with a plot. The pieces follow the spec's rules: the depth from the nearest open edge,
  the sparse band, the river course, the clearing, and the order (y, x, tile, slot). The hash
  is a pure integer function of `(tile, slot, salt)`, so the jitter in x, the jitter in y, the sparse
  draw and the variant are four independent draws. `Math.random` is never used. Every test reads
  the moved (jittered) point, which is the piece's `x, y`.
- `riverSegments(i, …)` returns the tile's river course: the segment from its centre toward its flow
  tile (`river[i] > 0` and `flow[i] >= 0`), and from each neighbour `j` with `flow[j] === i` and
  `river[j] > 0` to its centre, clipped at the hex edge. Export it, so the checks use the same course.

## 2. `www/map/layers/woods.js` (new) and `view.js`

- The layer is a static layer named `woods`, in `LAYERS` right after `edges`. It draws nothing when
  `isFlat(H, view)` or `H < WOODS.from`. It draws each piece's cell at the terrain's scale with its
  anchor on the slot, and culls by the drawn rectangle unless `view.noCull`, as `layers/edges.js`
  does. It counts what it draws in `view.stats.woods`.
- The fields are the plots of `model.v.clans` that lie on a forest or marsh tile. The pieces are
  cached per map (`view.nb`) and per fields key (the sorted tile list, joined). The static layers
  first draw with `model` null (`setLayout` clears it, then `resize()` draws): that is no fields.
- `export function stale(model, view)` works out the fields key, keeps it on the view, and is true when
  it differs from the key it kept before. The layer draws from the kept key. Keep the key there, not
  only when the layer draws, so a turn below `WOODS.from` doesn't mark the static layer dirty every turn.
  In `MapView.setModel`, add one line:
  `if (LAYERS.some((l) => l.static && l.stale?.(model, this))) this.dirty = true;`.
  Put the word `woods` beside `noCull` in its comment, as 441 did for `edges`.
- **Split `view.js` first**, because it is 608 lines. Move `attach()` and `pinchState()`, unchanged,
  to `www/map/input.js` as `attach(view)` and `pinchState(view)`. `MapView.attach()` stays as a
  one-line call, so callers don't change. `node www/play/all.mjs` must pass before and after, with
  the same results for `camera.mjs`, `select.mjs` and `pick.mjs`.

## 3. Marsh shores, in `www/map/edges.js`

- An edge piece isn't drawn where the land tile is marsh.
- A corner piece isn't drawn when every land tile at its vertex is marsh.
- Mouths and everything else stay as they are.
- Update `check-map.mjs`'s independent count (`expectedPieces`) and `play/edges.mjs` to match, and add
  a marsh tile on the staged sea there with its expectations.

## 4. The views, in `www/shots.mjs`

- Add the spec's 15 views with their tiles. The chooser gains the classes `interior`, `lone`,
  `forest-river`, `forest-fields` and `marsh-fields`, as the spec defines them.
- `turns: N`: after the pre-game wait, press `Lead ` (the first civ), wait for phase `main`, and end N
  turns with `Space`, waiting each time for `body.dataset.turn` to change (`proto/fields.mjs` does
  this). Then set the camera. Record `digest: game.digest()` in the view's `views.json` record. `--diff`
  fails when two digests differ, as it does for cameras.
- `--cull` covers every view with an `on`, which includes the new views, and prints `stats.woods`
  culled and unculled beside the ridges and edges.
- `--time NAME,…`: for each named view, five static redraws (`dirty = true; drawNow()`), printing
  each `stats.terrainMs` and the median. Build-3 and the review use it against a main worktree, so it
  must copy onto main alone, like the rest of the file. On main there is no `stats.woods`: print `-`.
- Shoot every new view on the branch once to prove the cameras: each view's tile is the spec's and
  the camera isn't clamped. If one clamps (a phone view near the rim, say), move only that view's
  `on` or `k`, and record why in `easel.md`.

## 5. The checks

- **`www/check-map.mjs`** (in CI) takes the medium world and counts the woods again from the
  neighbours, with its own copy of the rule, not by calling `woodPieces` for the answer:
  - each forest slot holds `tree.big`, `tree.small` (sparse and kept) or nothing, as the rule says;
  - each marsh slot holds a reed;
  - no piece lies within `WOODS.river` of `riverSegments` of its tile;
  - the order is right.

  Then, on a synthetic layout (as its seven-tile mouth case does), it checks a forest tile with fields
  and a river: no piece is in the clearing or on the course, and the same tile without fields has
  pieces in the clearing. Check that the medium world's forest has interior, edge and lone tiles, or
  say which it lacks.
- **`www/play/woods.mjs`** (new, in CI through `play/all.mjs`) stages a radius-8 map on the
  `play/edges.mjs` pattern. It holds a seven-tile wood (a forest tile and its six neighbours, `f`), a
  lone forest tile, and a strip of marsh (`%`) on the sea. It checks that:
  - the interior tile has only `tree.big` pieces;
  - the lone tile has big trees in its middle and none beyond `band` but small ones;
  - the marsh holds reeds and draws no bank on its sea edges;
  - with the camera put so the wood's southern tile sits 1.0 world unit below the canvas, the culled
    and unculled draws are identical PNGs, and the culled `stats.woods` counts the pieces whose
    rectangle meets the canvas.
- Add the new files to `engine/CLAUDE.md`'s map line.
- Run `engine/check.sh` once, before you finish.

Write `relay/specs/442/easel.md` with the counts on the medium world (tree, sparse, reed and cleared
slots), the 15 views' tiles and cameras, and one `--time` run of `medium9-woods-play` and `large8-play`
on the branch at build-1's cells.

## What later steps rely on

Build-3 changes `WOODS`'s numbers and the cells, and may add a key, such as a third tree size or a
clump. Your checks must read `WOODS`, not copy its numbers, so they hold whatever it picks. If build-3
moves `WOODS.from`, it moves the `-near` and `-far` views' k. The review runs `--diff`, `--cull` and
`--time` on both trees.

## Size

About 12 files and 650 lines: `map/woods.js` (about 120), `map/layers/woods.js` (about 70),
`map/input.js` (about 95, moved), `map/view.js` (−90, +3), `map/edges.js` (+6), `shots.mjs` (235, plus
about 110), `check-map.mjs` (187, plus about 110), `play/woods.mjs` (about 150), `play/edges.mjs` (plus
about 15), `engine/CLAUDE.md`, and `easel.md`. It waits on 15 views shot once: minutes, not a model's
worth.
