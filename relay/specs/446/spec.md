# #446 The illustrated map, verified alongside play

**Question:** none in the first four scores, because no rule moves. This is piece 8 of the illustrated map
(`docs/proposals/map-art.md`), the last. Pieces 1 to 7 are on main. This piece checks the whole map as a player meets it and
fixes what the art made worse. It asks each of these against two controls:

- **Health: nothing but the page moved.** `git diff main --stat -- engine/core engine/cli engine/web` is empty, so every
  digest, `SAVE_VERSION` and every save stay as main's without a sweep. Every view records the game's digest, and it
  matches across all three trees.
- **Frame time.** On main every pan redraws the static layer (`panBy` sets `view.dirty`), so a pan frame costs
  `stats.terrainMs + stats.overlayMs`. On main a play-zoom frame of a medium or large map costs 1.8× the pre-art page's
  (medium 9: 24.4 ms against 13.3), and at k 6 to 12 on medium 9, 2 to 2.7× (`time.txt`, `review.txt`): 14 of the 21
  verify views are over the bar below. `layers/edges.js`, the shores and fringes, takes 8 to 10 ms of it, and its cost
  is the draw call, about 2.7 µs each whatever the piece's size, with 76 to 95% of its pieces on screen at k 6 to 12
  (`review.txt`). So no culling makes a full redraw pass: a pan has to stop redrawing what it only moves. **The pan
  frame** is what `--pan` measures (build-1): twelve `panBy` steps of whole css px ending on the view's camera, each
  `drawNow` timed whole. **Bar:** on every view of the verify set, the branch's median pan frame is at most 1.5× the
  pre-art page's or 8 ms, whichever is larger, and its slowest pan frame is no slower than main's median pan frame.
  *Build-1 correction:* main's own slowest pan frame is 1.1 to 6.9 times its median (baseline.md §2), so that slowest-frame
  clause fails on main itself and can't be the bar as written; the planner must restate it before build-2 is judged.
  A full redraw (a zoom, `--time`) is no slower than main's.
- **Selection.** A click picks the tile under the pointer (`tileAt`), then cycles what is on it. On main a stacked unit's
  second figure stands in the next hex, and a crowded town's outer clans spill out of theirs, so a click on what you see
  picks another tile (`hits.txt`: 7 misses on 5 views). **Bar:** every unit, clan and place drawn on a verify or place
  view has its drawn box's centre on its own tile.
- **Map-edge popping.** Units, clans, places, centres and markers cull by `onScreen` (the tile's centre within 80 css px
  of the canvas). A unit stack's second figure reaches 2.25 world units right of its tile, past that margin at k 35, so it
  pops in as its tile crosses 80 px (`pop.txt`: 500 px on the lineup's stack, 13 px on large 8's town of 8). **Bar:** every
  pop case draws the same pixels with culling on and off.
- **Legibility.** On main, at play zoom on its own civ's land, Gold's scout and army are smudges where the pre-art page had
  clear shields, and the capital's hall inside its gold centre ring is a brown blob (`shots/spec-medium9-places-play-*.png`).
  The View overlays (territories, fertility, country, hidden costs, worn, festival routes) read on both
  (`overlays.txt`). **Bar:** by eye on the verify set at close, play and strategic zoom: every unit, place and clan reads,
  in its civ's colour, as well as on the pre-art page, and every overlay reads over the art; and `--units`, `--clans` and
  `--places` exit 0 on the branch. *Build-1 correction:* `--units` exits 0 on main too (baseline.md §8); the claim that main's exits 1 on `medium9-units-flat` doesn't reproduce, so the flat-zoom read is judged by eye.
- **Zoom and selection behaviour stay.** `MAX_K`, `minK`, the flat-zoom switch (`isFlat`, `CLAN.from`, `PLACE.from`),
  `tileAt`, the click cycle and the panels are main's. The fixes are in the drawing.

**The controls.** Main at the branch's base, and **the pre-art page**: `0fd7835` (the commit before #448, the first art
piece) with today's `www/pkg`. The engine's API only gained fields since then, so the old page draws today's worlds: on
every view tried, its digest is main's and the cameras match (`shots.mjs --diff` exit 0, spec step). Both get the
branch's `shots*.mjs` copied in. `proto/control.sh` makes both trees under `/tmp`.

## Settled here, by running

All on this runner, on main `ab15ef9` and the pre-art control.

- **The control works.** `shots.mjs --out` on `small2-play`, `medium9-play`, `small2-flat`, `medium9-places-play`,
  `large8-places-play`, `lineup-close`, `places-lineup-close`, `clans-lineup-close` and `medium9-clans-play` on both:
  digests and cameras match (staged views too).
- **Frame time** (`time.txt`, `--time`): pre-art against main, median frame (terrain + overlay ms). The other 13
  verify views are in `review.txt` (spec-review): eight more over the bar, worst `medium9-units-edge` 13.3 against 4.9 and
  `medium9-woods-near` 21.7 against 8.1.

  | View | pre-art | main | ratio |
  |---|---|---|---|
  | small2-play (k 18) | 9.5 | 15.8 | 1.7 |
  | medium9-play (k 18) | 13.3 | 24.4 | 1.8 |
  | large8-play (k 18) | 17.7 | 28.1 | 1.6 |
  | medium9-woods-play (k 18) | 13.7 | 25.0 | 1.8 |
  | medium9-places-play (k 18) | 8.0 | 14.0 | 1.8 |
  | small2-close (k 35) | 7.1 | 9.9 | 1.4 |
  | small2-edge (phone k 6) | 4.5 | 9.3 | 2.1 |
  | small2-flat (phone k 5) | 4.3 | 4.3 | 1.0 |
  | medium9-units-edge (phone k 6) | 4.9 | 13.3 | 2.7 |
  | medium9-woods-near (k 12) | 8.1 | 21.7 | 2.7 |

- **Where the frame goes** (`proto/layers-tail.mjs`, static layers drawn alone, median ms): medium 9 at k 18, pre-art
  terrain 7.6; main terrain 6.1, edges 9.0, woods 0.9, ridges 0.4. Small 2 at k 18: edges 6.4 of 10.6. Small 2 at k 6:
  edges 2.6 of 4.7. `edges.js` walks every piece of the whole map each redraw and computes its rectangle before culling.
  The overlay half grows at k 6 too (2.9 → 5.5 ms on small 2): the units, clans and places layers. With a pan that
  only moves the static layer, every verify view's frame is its overlay half plus a blit, and every overlay half is
  under its view's bar (largest: `large8-places-play` 10.4 against 15.3).
- **A shifted static layer is main's frame, nearly** (spec-review, `review.txt`): main's static layer at a camera,
  shifted by a pan of whole css px, against main's full redraw at the new camera: 0 to 0.05% of pixels differ, by 2 to
  23 in a channel (anti-aliasing), except where a drawing is tied to the canvas rather than the world:
  `medium9-units-edge` panned (100, −50) differs in 5% (terrain and edges near the map's rim, up to 182), and
  `large8-places-play` panned (24, 12) in 1.1% (up to 141). The cache build-2 makes is held to what shifting main's own
  layer gives, so it adds no seam.
- **Selection** (`hits.txt`, `proto/hits-tail.mjs`): misses on main: `lineup-close`, `lineup-play`, `clans-lineup-close`
  (Red's camp on 126, the stack's second unit, picks 142), `medium9-units-play` (camp 596, 1510 → 1539),
  `medium9-places-close` (clans 679 and 720 of the town of 8 on 735), `large8-places-close` (clans 1777 and 1911 of the
  town on 2787). None on `medium9-stack-close`, `places-lineup-close`, `medium9-play`, `large8-play`.
- **Popping** (`pop.txt`, `proto/pop-tail.mjs`, with `onScreen` made to honour `noCull` as a scratch edit): the lineup's
  stack on 126 at 90 px past the left edge, 500 px; at 130 px, 20 px; large 8's town of 8 at 90 px past the right, 13 px.
  The capital of 3 on 47: none.
- **Overlays** (`overlays.txt`, `proto/overlays-tail.mjs`): each read over the art on `medium9-play`.

## Shape: Easel and Canvas

The harness, the controls and the bars are settled here. The frame time has an exact bar and an exact proof, but its
method is new to the code (a static layer that a pan shifts and tops up): its own step, before the canvas, so the canvas
judges on a map that pans as it should. How the selection, popping and legibility fixes look is judged by eye on real
maps: one canvas.

- **build-1, easel (the harness):** `www/shots-probe.mjs` with `--hits`, `--pop` and `--overlays`; the verify set and the
  pop cases in `shots-views.mjs`; `onScreen` honouring `noCull`; `--time` over a list or the verify set; `--pan`; the
  baseline on all three trees in `baseline.md`.
- **build-2, easel (the frame):** a pan that shifts the static layer and draws only what it uncovers; every verify
  view's pan frame under the bar; every fresh view's PNG byte-identical to main's, and every panned one within main's own
  shift.
- **build-3, canvas:** selection, popping and legibility fixed and judged; the unit bound at k 5; every shots check
  exiting 0; the captures, pre-art, main and branch; `views.md`.

## The verify set

`VERIFY` in `shots-views.mjs`, a list of existing view names (no new camera): representative retro maps at close, play and
strategic zoom.

| Zoom | Views |
|---|---|
| close (laptop, k 35) | `small2-close`, `medium9-places-close`, `large8-places-close`, `medium9-clans-close`, `lineup-close` |
| play (laptop, k 18) | `small2-play`, `medium9-play`, `large8-play`, `medium9-woods-play`, `medium9-places-play`, `large8-places-play`, `medium9-units-play`, `medium9-clans-play` |
| between (laptop, k 12, 11) | `medium9-woods-near`, `medium9-woods-far` |
| strategic (phone, k 6, 5) | `small2-edge`, `medium9-woods-edge`, `medium9-units-edge`, `medium9-places-edge`, `small2-flat`, `medium9-units-flat` |

**The pop cases**, `POPS` in `shots-views.mjs`: `{ view, tile }` pairs, each put 90 and 130 css px past each of the four
edges (eight placements): `lineup-close` 126 (a two-unit stack), `lineup-play` 126, `places-lineup-close` 47 (a capital
that is a town), `places-lineup-close` 140 (a settlement of 3 with Red's army beside it), `large8-places-close` 2787 (a
town of 8), `medium9-stack-close` 547 (the raised stack), `medium9-clans-close` 734. Build-1 may add a case that shows a
pop; none may be dropped.

## Decisions I made

- **The pre-art control is 0fd7835's page on today's wasm.** The same worlds, the old map: the only way to compare
  identical views before the art. Its panels lack the later goods panels, so it judges the map, not the panels.
- **The bars are against the pre-art page.** A regression is what the art track made worse; main is the second control,
  so the PR shows what this piece changed too.
- **The frame bar is 1.5× the pre-art frame, or 8 ms, on a pan.** 1.5× is the bar #440 to #445 held their own steps
  to; the 8 ms floor keeps a 4 ms flat frame from failing on noise. The runner draws in software; the ratio is what
  carries to a real browser. The bar is on the pan, the frame a player makes most and where the art's cost shows as
  lag; a zoom redraws everything and stays at main's cost, since every piece on screen at k 6 to 12 has to be drawn
  once (`review.txt`). A cache that tops up in big refills would pass the median and stutter, so the slowest pan frame
  is barred at main's median.
- **Selection stays by tile.** The design's click is on a tile, and the piece keeps selection behaviour. So what is drawn
  for a tile stays in its hex: a stack drawn tighter, or its numbers moved, a crowd's ring drawn within its hex. A click
  that hit-tests sprites is a change to selection, not a fix.
- **A pop is fixed by culling on what a layer draws.** Each layer that culls by `onScreen` culls by the reach of its own
  drawing (a margin per layer, or its drawn rectangle as `edges.js` does). The 80 px stays for layers that fit in it.
- **The flat zoom keeps main's marks.** `CLAN.from` and `PLACE.from` stay 28 css px; the canvas may change a mark's size
  or colour for legibility, not where the switch falls.
- **The unit bound at k 5 holds** (the open idea, taken). Strategic zoom is where a player scans for armies; a banner that
  reads as another civ there is the legibility this piece fixes. The canvas makes `medium9-units-flat` pass by tuning
  `FIGURE` and the cloth; a scout at 14 px that can't be made to read is recorded with its numbers.
- **No new art.** Fixes come from the atlas tool and the layers' tables; a recolour or a variant is built from the
  studies' pixels in the atlas tool, as #445's pennant was.
- **Out of scope:** the HUD and the panels, new mechanics, the event feed and tooltip (`map.md`, "On the page").

## Unsettled, and which step holds it

- **How a pan gets under the bar** (build-2): the cache's shape, when it refills, how each static layer draws a strip.
  Proof: `--pan` on the verify set on the branch and the pre-art control, every view within the bar; `--time` no slower
  than main's; `--out` on the verify set and every place, clan and unit view, `--diff` against main: 0% on each;
  `--pan`'s final frames against main's fresh frames at the same cameras: no more pixels differ than main's own shifted
  layer gives.
- **How a stack and a crowd stay in their hex, what each layer's cull margin is, and what makes a unit, place and clan read
  at every zoom** (build-3). Proof: `--hits` and `--pop` exit 0; `--units`, `--clans`, `--places`, `--cull` exit 0; the
  verify set's captures on all three trees read side by side; `--pan` still within the bar after the canvas.

## Ideas taken

- **The unit bound at the flat zoom** (#443 build-2): build-3 holds `TINT_BAR[5]` and makes the banner read.

## For the review

Main has moved under the piece if `ab15ef9` is not the merge base: rebase, rebuild `www/pkg`, and run
`proto/control.sh` again. Then:

- `git diff main --stat -- engine/core engine/cli engine/web` is empty and `SAVE_VERSION` is main's.
- `node www/play/all.mjs` passes (CI).
- On the branch: `--hits`, `--pop`, `--units`, `--clans`, `--places`, `--cull` each exit 0.
- `--out` on the verify set on all three trees, `--diff` branch against main and against pre-art: cameras and digests
  match. `--pan` on the verify set on the branch and the pre-art control: every view within the bar; `--time` on the
  branch no slower than main's; the table in the PR.
- Read the pairs of the verify set (pre-art, main, branch) at each zoom: units, places and clans read in their civ's colour
  at least as well as on the pre-art page; Gold's on Gold's land; the capital inside its ring; the overlays from
  `--overlays` read over the art; the trees, ridges, shores, rivers, trails and borders are main's.
- On the branch page (`play.mjs`), on medium 9: click a stacked unit's each figure, a crowded town's outer clan, a capital
  and a settlement, and check each selects what was clicked; pan a stack across the left edge at k 35 and see it slide in
  whole; drag the map about at k 6, 12 and 18 and see no seam or stale strip, then end a turn and toggle a View overlay
  mid-pan and see the map redraw; zoom across k 5 to 35 and see the flat switch where main has it.
- The PR carries pre-art, main and branch captures of `medium9-play`, `medium9-places-play`, `large8-places-close`,
  `lineup-close`, `small2-edge` and `medium9-units-flat` from build-3's `shots/`, and the `--pan`, `--hits` and `--pop`
  tables before and after.
