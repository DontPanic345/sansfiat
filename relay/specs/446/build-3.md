# #446 build-3 (canvas): the illustrated map, fixed and judged beside the pre-art page

Read `relay/specs/446/spec.md` first: the question and its bars, the controls, the decisions, the verify set and "For the
review". Build-1 left the harness (`www/shots-probe.mjs`: `--hits`, `--pop`, `--overlays`, `--layers`, `--pan`, `--shift`; `--time verify`; the
`VERIFY` set and the `POPS` cases in `shots-views.mjs`; `onScreen` honouring `noCull`) and the baseline
(`relay/specs/446/baseline.md`). Build-2 made a pan shift the static layer and brought it under the bar, fresh frames unmoved (`relay/specs/446/frame.md`).
Your job is what the player sees and clicks. Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.
Keep selection, zoom and the flat switch as main has them (`tileAt`, the click cycle, `MAX_K`, `minK`, `CLAN.from`,
`PLACE.from`): the fixes are in the drawing.

## What you fix

- **Selection.** `--hits` exits 0: every unit, clan and place drawn has its box's centre on its own tile. On main a stack's
  second unit stands in the next hex (`units.js`'s `FIGURE.gap` and `pad`; #443's notes suggest a tighter stack or the
  numbers above the figures) and a crowded town's outer clans spill out (`clans.js`'s ring on a place's tile). Keep a
  stack's numbers readable and never overlapping (`play/units.mjs` checks it), and a crowd's every clan visible
  (`play/places.mjs`, `--clans`).
- **Popping.** `--pop` exits 0. Each layer that culls by `view.onScreen` culls by the reach of what it draws: give
  `onScreen` a margin argument, or cull on the drawn rectangle as `edges.js` does. `--cull` exits 0 too.
- **Legibility**, judged on the verify set's captures, pre-art beside main beside the branch, at close, play, between and
  strategic zoom. On main Gold's scout and army on Gold's own land at k 18 are smudges where the pre-art page had clear
  shields, and the capital's hall in its gold ring is a blob (`shots/spec-medium9-places-play-*.png`). Find every such case
  in the set (each civ's units on its own land and on a rival's, a capital in its ring, a crowded town, a settlement beside
  an army) and make it read: the cloth's grey floor or share, a unit's outline or edge, a figure's size, the order of the
  ring and the hall, the ring's own width. Read each View overlay (`--overlays` on the branch beside main's and pre-art's)
  and fix one the art buries. Any recolour or variant is built in the atlas tool from the studies' pixels, as #445's pennant
  was, with `--check` passing and every other cell keeping its pixels.
- **The unit bound at k 5** (the open idea, taken): `--units` exits 0, `medium9-units-flat`'s scout 715 among them, with
  `TINT_BAR[5]` held at 40. Tune `FIGURE` and the cloth. If a 14 px banner can't read as its civ, say so with the numbers
  and what you tried; don't lower the bar.

Every shots check exits 0 on the branch at the end: `--hits`, `--pop`, `--units`, `--clans`, `--places`, `--cull`. You may
change the easel where the painting needs it (a check, a view, a case), and say so in your summary. A spec decision holds
unless the captures show it fails; then make the smallest change that works and record it as a decision.

## What proves it

- `--out` on the verify set on all three trees (`proto/control.sh`), `--diff` branch against main and against pre-art:
  cameras and digests match. Read the triples by eye; say per view what pre-art, main and the branch show.
- `--pan` on the branch and pre-art after your changes: every view within the bar still; `--time verify` no slower than main's. A static layer you change draws a pan's strips as build-2's do.
- `--hits` and `--pop` before (`baseline.md`) and after.
- `node www/play/all.mjs` passes.

## Evidence

- `relay/specs/446/views.md`: per verify view, what each tree shows; the `--hits`, `--pop`, `--pan`, `--units`,
  `--clans`, `--places` and `--overlays` tables before and after; every change to a layer's table, the atlas or the easel,
  and why.
- `relay/specs/446/shots/`: `<view>-preart.png`, `<view>-main.png` and `<view>-branch.png` for `medium9-play`,
  `medium9-places-play`, `large8-places-close`, `lineup-close`, `small2-edge` and `medium9-units-flat`, cropped round what
  they show where the canvas is mostly empty (`relay/specs/444/proto/crops.mjs` is the pattern). Delete the spec step's
  `spec-*.png` once yours are in.
- `atlas.md` with the new hashes and recipes, if the atlas changed.

Run `engine/check.sh` whole once before you finish.
