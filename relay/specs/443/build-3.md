# #443 build-3 (canvas): the units on the map, judged beside main

Read `relay/specs/443/spec.md` first: the question, the decisions, "The views" and "For the review". Build-1 cut the
figures and masks (`relay/specs/443/atlas.md`, `cells.png`); build-2 put them on the map and gave you the views,
`--units`, `play/units.mjs` and `FIGURE` (`relay/specs/443/easel.md`). Your job is how the units look and read. Don't
change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easel gives you

- `sources.js`: the `actor.*` regions, the `over` banner, the `cloth` polygons and the `grey` op's range. Change any
  recipe; `--check` must still pass, the cells keep their keys, and every other cell keeps main's pixels.
- `layers/units.js`'s `FIGURE`: the figure's height, its minimum screen size, where its feet stand, the stack's
  gap, the outline widths, the numbers' place.
- `shots.mjs`: the 13 unit views, `--diff`, `--units`; `play/units.mjs`.

You may change the easel where the painting needs it (a rule, a table, a check), and say so in your summary. The
spec's decisions hold unless the views show one fails. Then make the smallest change that works and record it as a
decision: for example, the tunic joining the cloth, or a stack's figures overlapping more closely.

## What proves the painting

Shoot every unit view on a main worktree (the branch's `shots.mjs` copied in) and on the branch, `--diff` them
(cameras and digests match), and read them side by side:

- **Shape.** The camp and the scout are different figures at k 18 and at phone k 6, not two blobs of one size.
- **Colour.** In `lineup-*`, red's, blue's and gold's units are told apart on plains and in the wood, at k 35, 18
  and phone k 6. In `medium9-units-play` and `medium9-gold-play` your units read beside the capital's house, ring
  and clans, on plains and in the wood. Gold's banner must not read as the study's ochre.
- **Signals.** The selected unit's white edge and the hungry camp's red edge (`small2-camps-*`) read as clearly as
  main's. Every number in a stack reads. An army with archers still shows its bow (`play/army.mjs`'s raise, or a
  shot of it).
- **Fit.** A lone figure stands in its own hex at k 18 and 35, and doesn't hide the capital's house or the clans
  more than main's shield does. Trees, rivers, borders and icons sit where main has them.
- **Numbers.** `--units` on every view: every unit nearest its own civ (it exits 1 otherwise). Set a bar for the
  tinted pixels per figure at k 18 and at phone k 6 that the views show is enough to read the colour, and hold
  every unit above it. Report each unit's tinted share and pixels.

Show the selected and hungry edges in at least one capture each, selecting a unit in the shot where none is.

## Evidence

- `relay/specs/443/views.md`: per view, what main shows and what the branch shows; the `--diff` and `--units`
  tables; the bar you set and why; `play/units.mjs`'s result; every change to the easel.
- `relay/specs/443/shots/`: main's and the branch's PNGs of `lineup-close`, `lineup-play`, `lineup-edge`,
  `medium9-units-play`, `medium9-gold-close` and `small2-camps-close`, named `<view>-main.png` and
  `<view>-branch.png`, cropped round the units where the full canvas is mostly empty, as 442's were.
- `atlas.md` updated with the final hashes and recipes.

Run `node www/play/all.mjs` and `engine/check.sh` whole once before you finish.
