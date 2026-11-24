# #446 build-2 (easel): the pan under the bar

Read `relay/specs/446/spec.md` first: "Frame time" under the question, "Settled here" and the decisions, then
`relay/specs/446/review.txt`. Build-1 left the harness (`www/shots-probe.mjs`: `--pan`, `--shift`, `--layers`; `--time
verify` in `shots.mjs`), the controls (`relay/specs/446/proto/control.sh`) and the baseline (`relay/specs/446/baseline.md`);
its summary says how noisy the timings are. Build-3, the canvas, fixes selection, popping and legibility after you, on a
map that pans within the bar. Your job is a fast pan that looks like main's. Don't change Rust: nothing under
`engine/core`, `engine/cli` or `engine/web`.

## Why a cache, not culling

On main `panBy` sets `view.dirty`, so every pan frame redraws the whole static layer. `layers/edges.js` costs 8 to 10 ms
of it on medium and large maps, and its cost is the `drawImage` call itself (about 2.7 µs, whatever the piece's size).
At k 6 to 12 on medium 9, 76 to 95% of its pieces are on screen, so culling by tile, caching positions or ImageBitmaps
save under 1.5 ms where `medium9-units-edge` needs about 5 and `medium9-places-edge` about 8 (`review.txt`). A pan moves
the camera by whole css px, so the static layer can be kept and shifted, and only what the pan uncovers drawn.

## What done is

- **The bar.** On every view of the verify set, the branch's median `--pan` frame is at most `max(1.5 × pre-art's, 8)`
  ms, pre-art's from `baseline.md` or a fresh run on `/tmp/preart` beside yours (run both in the same session if the
  runner's speed looks different from the baseline's), and its slowest pan frame is no slower than main's median pan
  frame: a cache that refills in one big redraw now and then stutters. `--time verify` (a full redraw, as a zoom makes)
  is no slower than main's, within the noise build-1 measured.
- **Fresh frames are main's.** `--out` on the verify set and on every view with `units`, `clans` or `places`, `--diff`
  against main (`/tmp/main`): cameras and digests match, and 0% differ on every view. `--cull` exits 0. `node
  www/play/all.mjs` passes.
- **Panned frames are main's, as near as a shift can be.** `--pan DIR` on the branch and on main, the last frames
  compared per view: no more pixels differ, and none by more, than `--shift` gives on main for the same drag (shifting
  main's own layer is exact but for anti-aliasing, except where a drawing is tied to the canvas, not the world; on
  `medium9-units-edge` and `large8-places-play` that is up to 5% near the map's rim). Fewer is fine: anchoring such a
  drawing to the world (a fill or pattern set from the canvas origin) is allowed, if fresh frames stay main's. A seam or
  a stale strip shows here as more.
- **Everything that changes the static layer still redraws it.** A zoom, a resize or dpr change, a static View overlay
  toggled, a static layer going `stale` after a turn, a new map or a load: each is a full redraw, as on main. Add a
  check to `www/play/` (seconds, a built or small map) that pans, then does each of these, and compares the frame with
  a fresh full redraw at the same camera.

## Where to look

`view.js`: `panBy`, `viewChanged`, `drawNow`, `drawStatic`, `collectVisible`, `stepAnim` (a `centreOn` glide moves `k`:
leave it a full redraw), `resize`, `setLayer` (it sets `dirty` for a static layer) and the `stale` check in `setModel`. Then each static layer
(`layers/{terrain,edges,woods,ridges,rivers,floodplains,passes,sources,country,fertility,hidden-costs}.js`, each that exports `static`): what it culls
by (`view.vis`, the canvas rectangle) decides how it draws a strip. Ways to look at: copy the layer onto itself by the
pan's device px and draw the uncovered strips under a clip, with `collectVisible` and the layers' canvas tests taking the
strip's rectangle; or a layer wider than the canvas, drawn ahead of the pan in slices so no frame pays for all of it.
Measure with `--pan` and keep what passes. A pan by a fraction of a device px (a pinch on a high-dpr phone) may fall
back to a full redraw.

`view.js` is 556 lines: the cache goes in a new `map/pan.js` (the kept layer, the shift, the strips, what invalidates
it), with `view.js` calling it, so `view.js` stays under 600. Add `www/play/pan.mjs` for the check above and name both
in `engine/CLAUDE.md` within its byte cap.

Keep each layer's draw order and its `stats` counts on a full redraw (`stats.edges`, `stats.woods`, `stats.ridges`:
pieces drawn), since `--cull` and the play scripts read them; `stats.terrainMs` stays the static layer's time on the
frame, so `--time` and the overlay half still read.

## Evidence

`relay/specs/446/frame.md`: the `--pan` table (pre-art, main, branch median and slowest, the bar, pass or fail per view)
and the `--time verify` table (main, branch), each run's raw output in `frame.txt`; the `--diff` summary; the panned
frames against main's beside `--shift`'s floor, per view; what you built and what each part saved. If a view can't be
brought under the bar, say which, by how much, and what you tried; build-3 reads it.

Run `engine/check.sh` whole once before you finish.

Build-3 relies on: every fresh view byte-identical to main's, so its own `--diff` against main shows only its fixes; the
pan within the bar, so its changes are measured against a passing frame; the overlay layers untouched, since they still
draw every frame.
