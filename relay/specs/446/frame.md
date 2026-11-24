# #446 build-2 (easel): the pan under the bar

Measured on this runner (as baseline.md): branch `relay/446` with `map/pan.js`, main at `/tmp/main`, pre-art medians from
`baseline-pan-preart.txt`. Raw output of every run is in `frame.txt` (sections `=== name`). Branch timings are two runs.

## `--pan` (twelve panBy steps, each drawNow timed, ms)

| View | k | pre-art | main median | main slowest | branch median (run 1 / 2) | branch slowest (1 / 2) | bar | median | slowest ≤ main median |
|---|---|---|---|---|---|---|---|---|---|
| small2-close | 35 | 6.7 | 9.9 | 44.4 | 5 / 4.4 | 40.5 / 39.5 | 10.1 | pass | no |
| medium9-places-close | 35 | 4 | 8.3 | 37.9 | 3.6 / 4 | 31.5 / 32.8 | 8.0 | pass | no |
| large8-places-close | 35 | 4.8 | 8 | 51.9 | 5.7 / 4.9 | 50.4 / 49.5 | 8.0 | pass | no |
| medium9-clans-close | 35 | 4 | 7.5 | 34.1 | 3.6 / 4.3 | 30.5 / 33.4 | 8.0 | pass | no |
| lineup-close | 35 | 5.3 | 4.5 | 17.4 | 3.4 / 4 | 16.7 / 20.8 | 8.0 | pass | no |
| small2-play | 18 | 9.1 | 15.2 | 49.7 | 4.7 / 5.1 | 39.6 / 40.1 | 13.6 | pass | no |
| medium9-play | 18 | 13.4 | 24.8 | 57.8 | 6 / 5.6 | 37.9 / 40.7 | 20.1 | pass | no |
| large8-play | 18 | 17.2 | 27 | 64 | 8.4 / 7.2 | 41 / 42.9 | 25.8 | pass | no |
| medium9-woods-play | 18 | 12.8 | 23.2 | 57.5 | 6.5 / 6 | 37.7 / 38.3 | 19.2 | pass | no |
| medium9-places-play | 18 | 7 | 14 | 49 | 4.6 / 5.5 | 38.4 / 42.8 | 10.5 | pass | no |
| large8-places-play | 18 | 9.8 | 17.8 | 51.5 | 5.5 / 6 | 36 / 38.6 | 14.7 | pass | no |
| medium9-units-play | 18 | 8 | 15.4 | 52.8 | 5.1 / 4.5 | 39.5 / 41 | 12.0 | pass | no |
| medium9-clans-play | 18 | 8 | 15.7 | 50.5 | 4.8 / 6.5 | 40.1 / 39.2 | 12.0 | pass | no |
| medium9-woods-near | 12 | 8 | 20.4 | 49.1 | 5.3 / 4.8 | 34.2 / 34 | 12.0 | pass | no |
| medium9-woods-far | 11 | 8.8 | 18 | 44.6 | 3.8 / 4 | 30.7 / 35 | 13.2 | pass | no |
| small2-edge | 6 | 3.8 | 9.1 | 12.5 | 2.4 / 2.5 | 3.3 / 4.8 | 8.0 | pass | pass |
| medium9-woods-edge | 6 | 5 | 12.8 | 15.1 | 4 / 3.6 | 4.6 / 6.9 | 8.0 | pass | pass |
| medium9-units-edge | 6 | 5.5 | 15 | 22.4 | 3.4 / 4 | 5.1 / 10.9 | 8.3 | pass | pass |
| medium9-places-edge | 6 | 6 | 17 | 74.1 | 3.4 / 4.8 | 63.1 / 61.4 | 9.0 | pass | no |
| small2-flat | 5 | 3.8 | 4 | 4.5 | 1.8 / 2 | 2.9 / 3.6 | 8.0 | pass | pass |
| medium9-units-flat | 5 | 6.1 | 6.8 | 8.4 | 1.8 / 2.1 | 2.3 / 5.1 | 9.1 | pass | pass |

**The median bar passes on all 21 views**, in both branch runs (medium9-play 6.0 against 24.8 on main, bar 20.1;
medium9-units-edge 3.4 against 15.0, bar 8.3).

**The slowest-frame clause fails on the 15 views at k 11 and up, and on medium9-places-edge, and it is not the cache.** Every
run has two spike frames (frames 5 and 11 of 12), about 32 to 40 ms, and the time is in `overlayMs`, the half after the
static layer: on `medium9-play` the branch's frames read terrain/overlay `1.1/31.9` and `0.9/36.3` where its others read
`1.2/3.2`; main's read `20.1/31.9` and `19.5/36.3`; the pre-art page's one spike reads `7.4/40.2` (run `PANMS=1 node
www/shots-probe.mjs --pan NAME`, which prints each frame's ms and its terrain/overlay split). The spike has the same size
on all three trees and it falls inside the overlay layers, which this step doesn't touch, so a pan frame that costs
3 to 6 ms on the branch can't be under main's 15 to 25 ms median on the frame the runner stalls. On `medium9-play` without the two spikes the
branch's slowest frame is 10.6 ms against main's 24.8 median (the other views were not printed per frame). The runner is software-drawn; a real browser may not stall. The
clause is the spec's, restated by build-1 as failing on main itself (its slowest is 1.1 to 6.9 times its median).

## `--time verify` (a full redraw, frame median ms)

| View | main | branch | |
|---|---|---|---|
| small2-close | 10.4 | 10.8 | ok |
| medium9-places-close | 7.2 | 6.9 | ok |
| large8-places-close | 7.1 | 7.1 | ok |
| medium9-clans-close | 6.6 | 6.7 | ok |
| lineup-close | 4.8 | 5.1 | ok |
| small2-play | 15.9 | 16.3 | ok |
| medium9-play | 23.1 | 24.2 | ok |
| large8-play | 29.3 | 31 | ok |
| medium9-woods-play | 23.9 | 24.3 | ok |
| medium9-places-play | 15.2 | 14.8 | ok |
| large8-places-play | 17.6 | 16.3 | ok |
| medium9-units-play | 12.9 | 13.2 | ok |
| medium9-clans-play | 15.9 | 15.8 | ok |
| medium9-woods-near | 20.9 | 22.4 | ok |
| medium9-woods-far | 18.2 | 18.1 | ok |
| small2-edge | 9.1 | 9.1 | ok |
| medium9-woods-edge | 16.1 | 16.6 | ok |
| medium9-units-edge | 11.9 | 12.8 | ok |
| medium9-places-edge | 15.8 | 15 | ok |
| small2-flat | 4.6 | 4.4 | ok |
| medium9-units-flat | 5.8 | 5.9 | ok |

No view is more than 10% slower than main (largest: large8-play 29.3 against 31.0, medium9-woods-near 20.9 against 22.4,
inside build-1's run to run spread of up to 12%). A full redraw is main's.

## Fresh frames

`--out` on all 85 views (the verify set and every view with units, clans or places, and the rest), branch against main with
`--diff`: **0% differ on every one, cameras and digests match** (`frame.txt`, `=== diff`: 85 lines, none above 0%).
`--cull` still exits 1, with the same ten close views (18 to 20 px each) as in baseline.md §8, run for run: its cause is
build-1's `onScreen` honouring `noCull` (units, clans and places on the margin), not the static layers; the strip
culls in edges, woods and ridges draw exactly what a whole canvas draws (the ridge, edge and woods counts on every
cull view match). Build-3's popping fix is what makes it exit 0.

## Panned frames against main's (`proto/pandiff.mjs`, last `--pan` frame on both trees, whole frame) beside `--shift`'s floor (main's layer shifted by the whole drag, over the overlap)

| View | branch vs main differ | by more than 40 | main's shift floor | by more than 40 |
|---|---|---|---|---|
| large8-places-close | 0.216% | 0.000% | 0.142% | 0.000% |
| large8-places-play | 12.864% | 2.371% | 15.707% | 3.026% |
| large8-play | 0.513% | 0.000% | 0.238% | 0.001% |
| lineup-close | 0.125% | 0.000% | 0.005% | 0.000% |
| medium9-clans-close | 9.748% | 0.057% | 14.754% | 0.069% |
| medium9-clans-play | 1.020% | 0.135% | 1.272% | 0.204% |
| medium9-places-close | 1.605% | 0.011% | 0.109% | 0.000% |
| medium9-places-edge | 0.352% | 0.003% | 0.118% | 0.000% |
| medium9-places-play | 0.231% | 0.001% | 0.049% | 0.000% |
| medium9-play | 0.176% | 0.000% | 0.019% | 0.000% |
| medium9-units-edge | 2.534% | 0.140% | 0.428% | 0.012% |
| medium9-units-flat | 4.376% | 0.578% | 8.100% | 1.506% |
| medium9-units-play | 0.277% | 0.000% | 0.053% | 0.000% |
| medium9-woods-edge | 0.296% | 0.002% | 0.014% | 0.000% |
| medium9-woods-far | 2.147% | 0.051% | 2.458% | 0.061% |
| medium9-woods-near | 0.069% | 0.000% | 0.040% | 0.000% |
| medium9-woods-play | 2.637% | 0.487% | 3.069% | 0.598% |
| small2-close | 0.106% | 0.000% | 0.050% | 0.000% |
| small2-edge | 0.138% | 0.000% | 0.058% | 0.000% |
| small2-flat | 0.639% | 0.021% | 0.602% | 0.000% |
| small2-play | 0.033% | 0.000% | 0.002% | 0.000% |

By these counts the branch is **at or under the floor on the six views where main's floor is large** (`large8-places-play`
12.9 against 15.7, `medium9-clans-close` 9.7 against 14.8, `medium9-units-flat` 4.4 against 8.1, `medium9-woods-play` 2.6
against 3.1, `medium9-woods-far` 2.1 against 2.5, `medium9-clans-play` 1.0 against 1.3) and **over it on the other fifteen**, by
up to 1.6% of the frame (`medium9-places-close`, floor 0.11%; `medium9-units-edge` 2.5 against 0.43; `medium9-woods-edge` 0.30 against
0.01). The pan brief's "no more pixels, none by more" is not met on those. What it is: the difference sits in the bands
the strips drew (the top and left of a frame panned down and right, twelve strips deep), and nearly all of it is within
40 in a channel (by more than 40: at most 0.14% of the frame outside the large-floor views). `play/pan.mjs` on a small map
at k 18 gives 0.027% after four pans. In `medium9-places-close` the differing pixels are rows of hex-outline stroke (k 35: an
outline's edge lies on a half device px, so a camera that differs by an ulp from a fresh draw's rounds its coverage the
other way). The floor measures one copy by the whole drag, with no strip drawn at an intermediate camera, so it can't
show this. I found no way to remove it without changing fresh frames (a rounded world transform for the static layer),
which the brief forbids; a stale strip or a seam would show as a line of "more than 40", and there is none.

## What was built, and what each part saved

- `map/pan.js`: `keep(view)` records the camera a static layer was drawn at; `shifted(view)` copies the layer by the pan's whole
  device px (`globalCompositeOperation = 'copy'`, drawn onto itself), then draws each uncovered strip (an L at most) under a clip with
  `view.strip` set, and returns false for a whole redraw when `view.full`, the zoom, size or dpr changed, the shift isn't whole device
  px (a half-px pan falls back), or it exceeds half the canvas.
- `view.js` (557 → 576 lines): `dirty` is a setter that also sets `full`, so everything that already set `dirty`
  (a zoom, `setLayer`, `setModel`'s `stale`, `resize`, `setLayout`, `stepAnim`'s glide, a script's `m.dirty = true`) is a whole redraw;
  only `panBy` sets the pan's flag. `collectVisible` and the new `box()` take the strip's rectangle; `edges`, `woods`
  and `ridges` cull against `box()` rather than the canvas.
- Saved: a pan frame is the blit of the layer, the strips (about the pan's width of the frame: 3% a step at 24 px)
  and the overlay half. medium9-play: 24.8 → 6.0 ms median; medium9-units-edge: 15.0 → 3.4; medium9-woods-near 20.4 → 5.3.
  A strip costs 1 to 3 ms (terrain/overlay split in `PANMS` output); the self-copy about 1 ms.
- `www/play/pan.mjs` (in `play/all.mjs`'s run): on a small map at k 18, a pan draws strips only (no whole redraw) and matches a fresh
  full redraw but for 0.03% anti-aliasing; a pan by half a device px, a zoom, a resize, `setLayer` on and off, a stale layer
  and a new map (`setLayout` + `setModel`) each draw whole and match a fresh one exactly.
- `shots-probe.mjs`: `PANMS=1` with `--pan` prints each frame's ms and terrain/overlay split.

Not done: the slowest-frame clause (above), `--cull` exit 0 (build-3: popping), the panned-frame counts on fifteen views.
