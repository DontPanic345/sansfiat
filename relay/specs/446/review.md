# #446 review

Main is still `ab15ef9`, the merge base: no merge needed. `git diff origin/main --stat -- engine/core engine/cli engine/web` is
empty, so every digest and `SAVE_VERSION` are main's and no sweep is needed: the judge's scores can't move.

## Read cold

- `map/pan.js` + `view.js`: the `dirty` setter sets `full`; only `panBy` goes through `viewChanged(true)`. `shifted` falls back to
  a whole redraw on a zoom, resize, dpr change, off-whole-px or over-half-canvas shift. Strips are drawn under a clip, with
  `collectVisible` and the edge/woods/ridge culls narrowed to the strip, then `collectVisible` restored. `drawStatic` fills the
  background with `fillRect`, which the clip holds to the strip. Correct as read.
- `units.js`: a stack centres on its tile within `span` 1.6, a lone figure at most `slotMax` 0.6 right; numbers over a stack's
  figures by row. `clans.js`: yard rows narrow with the hex. `places.js`, `units.js`: dark and civ rims from `atlas.outline`
  (cached by name, colour, height and width). Culling by each layer's reach; trails by `onCanvas`.
- Zoom, `tileAt`, the click cycle, `CLAN.from`/`PLACE.from` untouched.

## The drag nobody ran (`proto/drag.mjs`)

Medium 9, led, laptop. At each k: 40 random whole-px drags, then 10 drags, an end turn (space), 10 drags; then 10 drags, fertility
on, 10 drags, fertility off, 5 drags. Each end state against a fresh full redraw at the same camera:

| k | 40 drags: layer / frame px differ | > 40 in a channel | end turn mid-pan | overlay toggled mid-pan |
|---|---|---|---|---|
| 6 | 0.087% / 0.070% | 0.001% | 1 whole redraw, 0.005% | 2 whole redraws, 0.006% |
| 12 | 0.168% / 0.130% | 0 | 1 whole redraw, 0.007% | 2 whole redraws, 0.016% |
| 18 | 0.104% / 0.085% | 0 | 1 whole redraw, 0.063% | 2 whole redraws, 0.035% |

Anti-aliasing at strip seams only; no stale strip, no visible seam.

## Pan frame (median / slowest ms, `baseline-pan-*.txt`, `after-pan.txt`)

| view | pre-art | main | branch |
|---|---|---|---|
| medium9-play k18 | 13.4 / 46.3 | 24.5 / 56.7 | 4.5 / 32.5 |
| large8-play k18 | 17.2 / 68.8 | 27.2 / 60.8 | 5.7 / 36.1 |
| medium9-places-play k18 | 7.0 / 50.9 | 14.6 / 49.1 | 3.5 / 32.3 |
| medium9-woods-near k12 | 8.0 / 42.8 | 20.1 / 49.7 | 3.8 / 27.8 |
| large8-places-close k35 | 4.8 / 62.9 | 7.7 / 52.8 | 4.3 / 42.0 |
| lineup-close k35 | 5.3 / 19.4 | 5.0 / 17.7 | 2.2 / 12.8 |
| small2-edge k6 | 3.8 / 4.4 | 8.3 / 9.2 | 2.2 / 5.4 |
| medium9-units-flat k5 | 6.1 / 7.1 | 6.0 / 7.2 | 1.6 / 2.7 |

Every verify view's branch median is under its bar and under the pre-art median. The slowest-frame clause as the spec wrote it
(slowest ≤ main's median) fails on 16 views: two ~30-50 ms stalls per run in the overlay half, present on pre-art and main too.
The branch's slowest is below pre-art's slowest on 19 of 21 views (small2-edge 5.4 vs 4.4 and medium9-woods-edge are within 1 ms).

## Left as nits

- lineup-close: Gold's stack numbers stand just under Red's "6.0" in the hex above; readable, close.
- The capital's hall under an army is still the busiest spot (build-3 says so).
