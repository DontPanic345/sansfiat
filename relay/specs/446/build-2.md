# #446 build-2 (easel): the frame under the bar

Read `relay/specs/446/spec.md` first: "Frame time" under the question, "Settled here" and the decisions. Build-1 left the
harness (`www/shots-probe.mjs`, `--time verify` with a frame column, `--layers`), the controls (`relay/specs/446/proto/control.sh`)
and the baseline (`relay/specs/446/baseline.md`); its summary says how noisy `--time` is. Build-3, the canvas, fixes
selection, popping and legibility after you, on a map that pans within the bar. Your job is speed with no pixel moved.
Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.

## What done is

- **The bar.** On every view of the verify set, the branch's median frame (`terrainMs + overlayMs`, `--time verify`) is at
  most `max(1.5 × pre-art's, 8)` ms, pre-art's from `baseline.md` or a fresh run on `/tmp/preart` beside yours (run both
  in the same session if the runner's speed looks different from the baseline's).
- **No pixel moves.** `--out` on the verify set and on every view with `units`, `clans` or `places`, `--diff` against main
  (`/tmp/main`): cameras and digests match, and 0% differ on every view. `--cull` exits 0. `node www/play/all.mjs` passes.

## Where to look

`--layers` on main: medium 9 at k 18, `edges` 9.0 ms of 16.4; small 2 at k 18, 6.4 of 10.6; small 2 at k 6, 2.6 of 4.7.
`layers/edges.js` walks every shore and fringe piece of the whole map each redraw and computes its rectangle before it culls;
`layers/woods.js` and `layers/ridges.js` do the same with fewer pieces. The terrain layer itself is about pre-art's. At k 6
the overlay half also grew (2.9 → 5.5 ms on small 2): the units, clans and places layers. Read `map/edges.js`,
`layers/{edges,woods,ridges,terrain}.js`, `atlas.js` (`scaled`) and `view.js` (`collectVisible`, `drawStatic`, `drawNow`)
before you choose. Ways that keep every pixel: index the pieces by tile and draw only `view.vis`'s; keep the scaled images and
the pieces' device positions between redraws at the same `k`; skip a layer's work where it draws nothing. Measure each with
`--layers` and keep what pays. A cache that blits a wider static layer on a pan is allowed if every shot stays
byte-identical and `--cull` passes, and it is the larger change: try the cheap ones first.

Keep each layer's draw order and its `stats` counts (`stats.edges`, `stats.woods`, `stats.ridges` as main counts them:
pieces drawn), since `--cull` and the play scripts read them.

## Evidence

`relay/specs/446/frame.md`: `--layers` before and after on main's three worst views; the `--time verify` table (pre-art,
main, branch, the bar, pass or fail per view), with each run's raw output in `frame.txt`; the `--diff` summary; what you
changed in each layer and what each change saved. If a view can't be brought under the bar without moving a pixel, say
which, by how much, and what you tried; build-3 reads it.

Run `engine/check.sh` whole once before you finish.

Build-3 relies on: every view still byte-identical to main's, so its own `--diff` against main shows only its fixes; the
frame within the bar, so its changes are measured against a passing frame.
