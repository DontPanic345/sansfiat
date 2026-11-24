# #446 baseline (build-1): the harness on three trees

Measured on this runner (4 cores, headless Chrome, software canvas), by `engine/www/shots.mjs` and the new
`engine/www/shots-probe.mjs`. The trees: **branch** `relay/446` (draws as main: build-1 changed no drawing), **main**
`ab15ef9` at `/tmp/main`, and **pre-art** `0fd7835`'s page on today's `www/pkg` at `/tmp/preart`, both made by
`proto/control.sh` with the branch's `shots*.mjs`. Raw output of each command is the `baseline-*.txt` beside this file.
Views are the 21 in `VERIFY` (`shots-views.mjs`). Timing noise is shown where it was measured; the numbers below are one
run each unless marked.

## Checks

- `engine/check.sh` whole, after the last edit passed, exit 0 (log in the run, not committed: `cargo fmt`, `clippy -D warnings`, the tests, `build-web`, `cross-build`, `check.mjs`, `check-map.mjs`, `draw-atlas --check`, and `play/all.mjs`, last run 11:49 UTC after the final edit).
- `engine/CLAUDE.md` is 8,191 bytes (budget 8,192): the probes are folded into the `shots` rows.

## 1. `--out` and `--diff`

- `--out` on the verify set, all three trees: exit 0 each (`baseline-out-*.txt`, `views.json` in `/tmp/t446/out-*`).
- **branch against main: 0% differ on all 21 views, exit 0**: no drawing changed, the cameras and digests match.
- **branch against pre-art: cameras and digests match (exit 0)**; the shares that differ are in `baseline-diff-preart.txt`
  and run from 16% (small 2 flat) to 97% (large 8 play): the pre-art page is the map before the art.

## 2. `--pan`: the frame a player makes most

Twelve `panBy` steps of a whole-px drag ending on the view's camera; each `drawNow` timed whole. The drag is (x, x/2) with
x from (24, 22, …, 2), the first size (forwards, then reversed) whose start lets the twelve steps land on the camera
exactly; it's the **pan drag** the shift and the proof below use. Bar: `max(1.5 × pre-art median, 8 ms)`.

| View | k | pre-art | main | branch 1 | branch 2 | bar | main ÷ pre-art | branch ÷ pre-art | over the bar: main / b1 / b2 | slowest frame, main / b1 |
|---|---|---|---|---|---|---|---|---|---|---|
| small2-close | 35 | 6.7 | 10.6 | 10.8 | 10.0 | 10.1 | 1.58 | 1.61 | over / over / - | 44.9 / 44.6 |
| medium9-places-close | 35 | 4.0 | 7.1 | 7.7 | 8.7 | 8.0 | 1.77 | 1.93 | - / - / over | 37.4 / 37.3 |
| large8-places-close | 35 | 4.8 | 7.7 | 7.7 | 8.3 | 8.0 | 1.60 | 1.60 | - / - / over | 52.8 / 53.5 |
| medium9-clans-close | 35 | 4.0 | 7.0 | 6.7 | 6.9 | 8.0 | 1.75 | 1.68 | - / - / - | 33.6 / 32.7 |
| lineup-close | 35 | 5.3 | 5.0 | 4.7 | 5.0 | 8.0 | 0.94 | 0.89 | - / - / - | 17.7 / 18.0 |
| small2-play | 18 | 9.1 | 15.6 | 15.0 | 15.8 | 13.6 | 1.71 | 1.65 | over / over / over | 50.8 / 51.1 |
| medium9-play | 18 | 13.4 | 24.5 | 23.5 | 23.3 | 20.1 | 1.83 | 1.75 | over / over / over | 56.7 / 56.4 |
| large8-play | 18 | 17.2 | 27.2 | 25.8 | 26.5 | 25.8 | 1.58 | 1.50 | over / over / over | 60.8 / 63.6 |
| medium9-woods-play | 18 | 12.8 | 23.3 | 22.4 | 23.0 | 19.2 | 1.82 | 1.75 | over / over / over | 56.8 / 55.5 |
| medium9-places-play | 18 | 7.0 | 14.6 | 13.6 | 14.0 | 10.5 | 2.09 | 1.94 | over / over / over | 49.1 / 48.0 |
| large8-places-play | 18 | 9.8 | 17.0 | 16.8 | 16.6 | 14.7 | 1.73 | 1.71 | over / over / over | 50.6 / 47.1 |
| medium9-units-play | 18 | 8.0 | 15.0 | 14.7 | 15.4 | 12.0 | 1.88 | 1.84 | over / over / over | 50.8 / 50.5 |
| medium9-clans-play | 18 | 8.0 | 15.9 | 16.2 | 15.0 | 12.0 | 1.99 | 2.02 | over / over / over | 49.9 / 50.6 |
| medium9-woods-near | 12 | 8.0 | 20.1 | 21.2 | 19.5 | 12.0 | 2.51 | 2.65 | over / over / over | 49.7 / 52.3 |
| medium9-woods-far | 11 | 8.8 | 19.2 | 20.2 | 18.0 | 13.2 | 2.18 | 2.30 | over / over / over | 47.3 / 47.8 |
| small2-edge | 6 | 3.8 | 8.3 | 9.0 | 8.3 | 8.0 | 2.18 | 2.37 | over / over / over | 9.2 / 9.7 |
| medium9-woods-edge | 6 | 5.0 | 13.0 | 13.3 | 13.0 | 8.0 | 2.60 | 2.66 | over / over / over | 17.3 / 16.6 |
| medium9-units-edge | 6 | 5.5 | 14.8 | 15.0 | 13.3 | 8.3 | 2.69 | 2.73 | over / over / over | 17.7 / 18.2 |
| medium9-places-edge | 6 | 6.0 | 15.2 | 15.4 | 14.7 | 9.0 | 2.53 | 2.57 | over / over / over | 74.5 / 73.9 |
| small2-flat | 5 | 3.8 | 3.8 | 4.4 | 4.1 | 8.0 | 1.00 | 1.16 | - / - / - | 4.5 / 5.6 |
| medium9-units-flat | 5 | 6.1 | 6.0 | 6.4 | 6.0 | 9.1 | 0.98 | 1.05 | - / - / - | 7.2 / 7.3 |

- **Over the bar:** main 15 of 21, branch run 1 15 of 21, branch run 2 16 of 21. The branch draws as main, so the
  branch's frame is main's, and build-2 has the whole gap to close: 14 to 16 views, the largest at medium 9 play k 18
  (main 24.5 against a bar of 20.1) and the strategic edges (14.8 against 8.3 at `medium9-units-edge`).
- **Noise, two runs on the branch:** the medians differ by up to 12% (`medium9-places-close` 7.7 and 8.7 ms,
  `medium9-woods-far` 20.2 and 18.0, `medium9-units-edge` 15.0 and 13.3). Build-2 should count a view as passing or
  failing only outside that band.
- **The slowest-frame clause doesn't hold on main itself.** Spec: the slowest pan frame no slower than main's median.
  Main's slowest frame is 1.1 to 6.9 times its own median (the run doesn't record which frame; a first-frame cost is the likely one); branch run 1's slowest is above main's median on all 21 views. The clause as written
  can't pass on main, so it can't be the bar. **Flagged for the planner and the review:** a slowest-frame bar needs a
  rule about the first frame or a bound on main's own spread (e.g. twice main's median).
- **The last frame, byte for byte:** on main, each view's last pan frame is byte-identical to its `--out` PNG (21 of 21,
  `cmp`), and its camera matches `views.json`. The camera is exact on 17 of 21 views; on `medium9-woods-near`,
  `small2-edge`, `medium9-woods-edge` and `small2-flat` the twelve float steps land up to 6e-15 world units off, and the
  PNG is still identical. The probe prints `OFF the view camera` for those.
- **Pan frame = `--time`'s frame on main.** `medium9-play` main: the pan median is 24.5 ms against 23.2 for `--time`'s
  frame median. So the pan frame and the redraw measure the same thing on main; the pan doesn't hide a cost.

Pan PNGs (`/tmp/t446/pan-main/`, `pan.json`) are the input build-2 compares with: remake them on `/tmp/main` with
`--pan /tmp/t446/pan-main`.

## 3. `--time verify`: the full redraw (what a zoom costs)

Median of five redraws, ms: terrain (T), overlay (O) and frame (T + O, the median of the five sums).

| View | pre-art T / O / frame | main T / O / frame | branch T / O / frame | branch frame ÷ main frame |
|---|---|---|---|---|
| small2-close | 1.7 / 5.8 / 7.6 | 2.6 / 7.2 / 10.2 | 2.8 / 7.3 / 10.1 | 0.99 |
| medium9-places-close | 0.6 / 3.3 / 3.9 | 1.9 / 4.7 / 6.9 | 2.3 / 5.0 / 7.4 | 1.07 |
| large8-places-close | 1.4 / 4.3 / 5.7 | 2.8 / 5.4 / 8.7 | 2.3 / 5.0 / 7.2 | 0.83 |
| medium9-clans-close | 0.7 / 3.3 / 4.1 | 2.3 / 5.0 / 7.4 | 2.1 / 4.8 / 6.9 | 0.93 |
| lineup-close | 0.9 / 3.1 / 4.2 | 1.4 / 3.9 / 5.2 | 1.6 / 3.9 / 5.8 | 1.12 |
| small2-play | 4.0 / 5.4 / 8.9 | 6.2 / 10.7 / 17.5 | 6.5 / 9.8 / 16.5 | 0.94 |
| medium9-play | 6.3 / 6.4 / 12.4 | 18.9 / 4.2 / 23.2 | 20.1 / 4.3 / 24.5 | 1.06 |
| large8-play | 7.9 / 8.4 / 16.3 | 22.4 / 7.6 / 30.4 | 21.4 / 8.1 / 29.9 | 0.98 |
| medium9-woods-play | 6.6 / 6.6 / 13.5 | 20.5 / 4.9 / 25.3 | 20.3 / 4.7 / 25.4 | 1.00 |
| medium9-places-play | 3.5 / 4.3 / 7.7 | 6.6 / 8.8 / 15.4 | 7.0 / 8.4 / 15.3 | 0.99 |
| large8-places-play | 3.9 / 5.3 / 9.2 | 7.1 / 10.0 / 17.3 | 7.1 / 9.7 / 16.8 | 0.97 |
| medium9-units-play | 2.6 / 3.9 / 6.5 | 5.3 / 8.1 / 13.4 | 6.0 / 7.9 / 13.4 | 1.00 |
| medium9-clans-play | 3.9 / 4.0 / 7.9 | 7.1 / 8.6 / 15.9 | 7.2 / 8.4 / 15.7 | 0.99 |
| medium9-woods-near | 2.3 / 5.8 / 8.3 | 15.0 / 5.6 / 20.9 | 17.5 / 6.0 / 23.5 | 1.12 |
| medium9-woods-far | 2.3 / 5.6 / 8.4 | 14.2 / 4.3 / 18.4 | 14.5 / 4.3 / 18.5 | 1.01 |
| small2-edge | 1.5 / 2.9 / 4.5 | 3.8 / 5.4 / 9.2 | 4.3 / 5.6 / 9.9 | 1.08 |
| medium9-woods-edge | 3.2 / 3.9 / 7.1 | 12.9 / 3.7 / 16.8 | 12.6 / 3.5 / 16.1 | 0.96 |
| medium9-units-edge | 1.8 / 3.0 / 4.8 | 10.2 / 2.2 / 12.4 | 10.8 / 2.3 / 13.1 | 1.06 |
| medium9-places-edge | 2.3 / 3.5 / 5.7 | 11.8 / 3.8 / 15.6 | 17.5 / 5.4 / 23.0 | 1.47 |
| small2-flat | 1.1 / 3.5 / 4.7 | 0.8 / 3.2 / 4.0 | 1.0 / 3.5 / 4.3 | 1.07 |
| medium9-units-flat | 1.7 / 4.2 / 5.9 | 1.7 / 4.3 / 6.2 | 1.7 / 4.5 / 6.2 | 1.00 |

- Branch against main on the same code: the frame ratio runs 0.83 to 1.47 (`medium9-places-edge`: main 15.6 and branch 23.0 ms in one run, the same code; the full redraw's noise floor on this runner is wide). A zoom's
  bar in build-2 and build-3 needs a repeat run to be read against.
- Pre-art against main: main's frame is 1.0 (flat views) to 2.7 times pre-art's (edges and woods), the spec's picture.

## 4. `--shift` on main: the shift floor

The static layer at the pan drag's start, shifted by the drag's device px, against the static layer at its end (full
redraws), over the overlap (`baseline-shift-main.txt`):

| View | k | shift (device px) | overlap px | differ | by > 40 |
|---|---|---|---|---|---|
| small2-close | 35 | -96,-48 | 841824 | 0.050% | 0.000% |
| medium9-places-close | 35 | 144,72 | 487344 | 0.109% | 0.000% |
| large8-places-close | 35 | -288,-144 | 354144 | 0.142% | 0.000% |
| medium9-clans-close | 35 | 288,144 | 354144 | 14.754% | 0.069% |
| lineup-close | 35 | 240,120 | 664560 | 0.005% | 0.000% |
| small2-play | 18 | 120,60 | 810840 | 0.002% | 0.000% |
| medium9-play | 18 | -216,-108 | 692664 | 0.019% | 0.000% |
| large8-play | 18 | 216,108 | 692664 | 0.238% | 0.001% |
| medium9-woods-play | 18 | -120,-60 | 810840 | 3.069% | 0.598% |
| medium9-places-play | 18 | -264,-132 | 374904 | 0.049% | 0.000% |
| large8-places-play | 18 | -264,-132 | 374904 | 15.707% | 3.026% |
| medium9-units-play | 18 | 288,144 | 354144 | 0.053% | 0.000% |
| medium9-clans-play | 18 | 192,96 | 440640 | 1.272% | 0.204% |
| medium9-woods-near | 12 | 288,144 | 610080 | 0.040% | 0.000% |
| medium9-woods-far | 11 | 48,24 | 905520 | 2.458% | 0.061% |
| small2-edge | 6 | 168,84 | 155844 | 0.058% | 0.000% |
| medium9-woods-edge | 6 | 240,120 | 99900 | 0.014% | 0.000% |
| medium9-units-edge | 6 | 240,120 | 48600 | 0.428% | 0.012% |
| medium9-places-edge | 6 | 72,36 | 129744 | 0.118% | 0.000% |
| small2-flat | 5 | -144,-72 | 175644 | 0.602% | 0.000% |
| medium9-units-flat | 5 | 192,96 | 68904 | 8.100% | 1.506% |

- **These floors are larger than the spec's.** Spec-review measured 0 to 0.05% on its drags. On the pan drags, which
  are up to 288 px, 15 views are under 1%, and six are over: `medium9-clans-close` 14.8%, `large8-places-play`
  15.7%, `medium9-woods-play` 3.1%, `medium9-woods-far` 2.5%, `medium9-units-flat` 8.1%, `medium9-clans-play` 1.3%. The likely cause: main's
  static layers cull by what is on the canvas, so pieces that come into view at the leading edge are missing from the
  start layer; a shifted cache can't draw them. Not verified pixel by pixel. **Build-2 has to read this table, not the
  spec's sentence: its floor is per drag, and it should be measured with `noCull` at the start if the cache is to be
  held to the shift alone.**

## 5. `--layers`: each static layer alone, median ms

| View | k | pre-art terrain | main terrain | main edges | main woods | main ridges |
|---|---|---|---|---|---|---|
| small2-close | 35 | 1.6 | 1.5 | 1.0 | - | - |
| medium9-places-close | 35 | 0.6 | 0.7 | 0.7 | 0.6 | - |
| large8-places-close | 35 | 0.8 | 1.1 | 1.3 | - | - |
| medium9-clans-close | 35 | 0.6 | 0.6 | 0.7 | 0.3 | - |
| lineup-close | 35 | 1.0 | 1.0 | 0.5 | - | - |
| small2-play | 18 | 4.1 | 4.6 | 5.3 | 0.4 | - |
| medium9-play | 18 | 7.4 | 7.0 | 9.3 | 1.0 | 0.7 |
| large8-play | 18 | 8.1 | 8.0 | 10.9 | 0.7 | 0.4 |
| medium9-woods-play | 18 | 7.3 | 7.8 | 9.5 | 1.0 | 0.4 |
| medium9-places-play | 18 | 4.3 | 3.4 | 7.8 | 1.0 | - |
| large8-places-play | 18 | 3.9 | 5.2 | 3.9 | 0.5 | 0.4 |
| medium9-units-play | 18 | 3.0 | 2.6 | 2.9 | 0.7 | - |
| medium9-clans-play | 18 | 4.1 | 4.2 | 2.1 | 0.8 | - |
| medium9-woods-near | 12 | 3.8 | 2.6 | 8.8 | 1.3 | 0.6 |
| medium9-woods-far | 11 | 5.0 | 3.8 | 8.9 | - | 0.4 |
| small2-edge | 6 | 2.6 | 2.1 | 3.0 | - | - |
| medium9-woods-edge | 6 | 3.8 | 2.6 | 8.4 | - | 0.4 |
| medium9-units-edge | 6 | 2.7 | 1.6 | 7.5 | - | 0.4 |
| medium9-places-edge | 6 | 5.3 | 4.3 | 8.1 | - | 0.4 |
| small2-flat | 5 | 1.1 | 0.9 | - | - | - |
| medium9-units-flat | 5 | 2.4 | 1.6 | - | - | - |

Pre-art has no edges layer; its terrain is 0.6 to 8.1 ms against main's terrain-plus-edges. Edges are the cost the
spec named: 7.5 to 10.9 ms on the medium and large play and edge views of main.

## 6. `--hits` and `--pop` on the branch

**Hits: exit 1, 42 misses on 21 views** (`baseline-hits-branch.txt`). The spec's `hits.txt` lists 8 misses on 6 views
(its summary says 7 on 5), and every one reproduces here: `lineup-close` (unit 18, camp 126 → 142), `lineup-play`,
`clans-lineup-close` (unit 29, 126 → 142), `medium9-units-play` (unit 596, 1510 → 1539), `medium9-places-close` (clans 679
and 720 of the town on 735 → 691 and 736), `large8-places-close` (clans 1777 and 1911 of the town on 2787). This run
also covers every view with units, clans or places, so it adds misses the spec didn't check: `medium9-places-play`,
`large8-places-play`, the Gold and stacked-unit views, the flat and edge views, and the clan lineups. None on
`small2-close`, `medium9-clans-close`, `small2-play`, `large8-play`, `medium9-woods-play` or `medium9-clans-play`.
Build-3's bar is zero.

**Pop: exit 1, 5 placements draw differently with culling off** (`baseline-pop-branch.txt`). The spec's `pop.txt`
numbers reproduce exactly: `lineup-close` 126 at 90 px left 500 px and at 130 px left 20 px; `large8-places-close`
2787 at 90 px right 13 px; `places-lineup-close` 47 and 140 on no side. Two placements are new, both at 130 px left:
`medium9-stack-close` 547 (28 px) and `medium9-clans-close` 734 (16 px). The spec measured only the first set.

## 7. `--overlays`: each View layer that starts off, alone, over three views

Share of pixels each layer moves by more than 48 in a channel (`baseline-overlays-*.txt`):

| View, layer | pre-art | main | branch |
|---|---|---|---|
| medium9-play fertility | 45.8% | 20.3% | 20.3% |
| medium9-play country | 1% | 1% | 1% |
| medium9-play hidden-costs | 7.9% | 7.9% | 7.9% |
| medium9-play territories | 6% | 12.9% | 12.9% |
| medium9-play worn | 0% | 0% | 0% |
| medium9-play festival-routes | 1.5% | 1.3% | 1.3% |
| medium9-places-close fertility | 54.9% | 27% | 27% |
| medium9-places-close country | 1.4% | 1.5% | 1.5% |
| medium9-places-close hidden-costs | 12.1% | 11.7% | 11.7% |
| medium9-places-close territories | 7.8% | 13.7% | 13.7% |
| medium9-places-close worn | 0% | 0% | 0% |
| medium9-places-close festival-routes | 1.8% | 1.5% | 1.5% |
| small2-edge fertility | 8.3% | 3.9% | 3.9% |
| small2-edge country | 0.7% | 0.7% | 0.7% |
| small2-edge hidden-costs | 0.3% | 0.3% | 0.3% |
| small2-edge territories | 3% | 4.1% | 4.1% |
| small2-edge worn | 0% | 0% | 0% |
| small2-edge festival-routes | 1.7% | 1.5% | 1.5% |

Main and the branch are identical. The pre-art page's fertility reads more strongly (45.8% against 20.3% at
`medium9-play`), since the art changed what fertility sits over; the territories read more on main than on pre-art.

## 8. `--units`, `--clans`, `--places`, `--cull` on the branch

- `--clans`: exit 0 (`baseline-clans-branch.txt`); every clan is nearest its own civ and at its bar.
- `--places`: exit 0 (`baseline-places-branch.txt`).
- `--units`: exit 0 (`baseline-units-branch.txt`), all 54 unit rows at their civ and bar, after one fix below.
  Main's `--units` (`baseline-units-main.txt`): exit 0 with every row at its bar too, `medium9-units-flat` included, so the spec's claim that main exits 1 there doesn't reproduce (the drawing is the same; the spec has been corrected).
- `--cull`: **exit 1** on the branch (`baseline-cull-branch.txt`): 10 close views with an `on` layer each draw 18 to 20
  pixels differently with the culling off. Main's `--cull` is exit 0 with 38 views identical
  (`baseline-cull-main.txt`). The cause is the one change in `view.js`: `onScreen` now honours `noCull`, so the
  80-px margin that hides some objects from the cull check is visible to it. The new sparse differences are real
  off-canvas pops from the margin. Build-3 has to make `--cull` exit 0 (its bar).

## Changes this step made to the views

- `small2-camps-selected` (not in the verify set) selected tile 527, which no longer holds a unit after eight turns
  (no unit stands there now; the world has moved since #443). `--units` and `--hits` both failed on it
  on main as well. It now stands on the camp on tile 257 (camera and selection), and `--hits`/`--units` pass it.
  The camp's hunger was not checked.

## For build-2 and build-3

- The bars that can be measured here: `--pan` (within the noise band above), `--time verify` (noisy), `--hits` (zero
  misses), `--pop` (zero placements), `--cull` (exit 0), `--units`/`--clans`/`--places` (exit 0).
- The slowest-frame clause of the pan bar needs a decision, see above.
- The shift floor is per drag, see above.
