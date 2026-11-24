# #445 build-3 (canvas): the places on the map, judged beside main

Read `relay/specs/445/spec.md` first: the question, the inventory, the decisions, "The views" and "For the review".
Build-1 cut the cells and masks (`relay/specs/445/atlas.md`, `cells.png`); build-2 put them on the map and gave you the
views, `--places`, `play/places.mjs` and `PLACE` (`relay/specs/445/easel.md`). Your job is how the places look and read.
Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easel gives you

- `sources.js`: the `place.*` regions (keep polygons, scale, ground point, `floor`) and their masks' `cloth` polygons and
  grey floor. Change any recipe; `--check` must still pass, the cells keep their keys, size and anchor, and every other
  cell keeps main's pixels.
- `layers/places.js`'s `PLACE`: each role's width (`w`), where a place stands in its hex (`feet`), and `from`, the zoom
  below which main's houses draw.
- `shots.mjs` and `shots-views.mjs`: the 10 place views, `--diff`, `--places`; `play/places.mjs`.

You may change the easel where the painting needs it (a rule, a table, a check), and say so in your summary. The spec's
decisions hold unless the views show one fails. Then make the smallest change that works and record it as a decision:
for example, a crowded town's clans drawn a little smaller on a place's tile, or a lower hamlet so 8 clans in front of it
don't read as a heap.

## Variants the art may lack

The goal says to author any missing variant in the study's palette. None is planned (spec, decisions). If the views
show one is needed (a hamlet too tall for a crowded town, a flag that reads as the study's red on a Gold settlement, a
hall whose banners vanish at k 18), build it deterministically in the atlas tool from the study's own pixels (a crop, a
recolour by `ramp` within the study's palette, a mirrored or trimmed group), never from outside art, and add it as a
new key without moving the others. Record each in `atlas.md` and as a decision.

## What proves the painting

Shoot every place view on a main worktree (the branch's `shots.mjs` and `shots-views.mjs` copied in) and on the branch,
`--diff` them (cameras and digests match; `medium9-places-flat` differs in no pixel, and `medium9-places-edge` too while
`from` keeps the places main's at k 6), and read them side by side:

- **Role.** Hamlet, palisade and hall are three different things at k 35 and k 18. The capital reads as the civ's seat:
  the largest place, its banners its civ's. If you lower `PLACE.from` so the places draw at phone k 6, say which zoom it
  lands at, and judge `medium9-places-edge` and `places-lineup-edge` by eye instead of by pixel.
- **Ownership.** Each settlement's flag and capital's banners read as its civ's on Red, Blue and Gold, on plains and in
  the wood. Gold must not read as the study's ochre thatch (#443 met the same: its grey floor went from 90 to 190).
- **The crowd.** The town of 8 at 735 (`medium9-places-*`) and at 2787 (`large8-places-close`), the capital of 9 at 2524
  (`large8-places-play`), and the lineup's three crowded places each show the place and every clan living on it; a
  starving clan's red edge still shows.
- **Beside a unit, a forest and a ridge.** In `places-lineup-close` (Blue's settlement, Red's army on 124, the wood and
  the ridge) and `small2-places-close` (Blue's capital and its scout beside the forest town 505), the unit's figure and
  number stay readable and the trees and peaks draw as on main.
- **Markers.** The town diamond, the gold centre ring, signposts and borders sit where main has them and read over the
  places.
- **Numbers.** `--places` on every view: every place of its tile's role, every owned place nearest its own civ (it exits 1
  otherwise). Set a bar for the tinted pixels per owned place at k 18 that the views show is enough to read the colour,
  hold every owned place above it, and make `--places` exit 1 under it, as #443's `TINT_BAR`. Report each role's tinted
  pixels and share per view.
- **Time.** `--time medium9-places-play` on both trees: the branch's median beside main's. Over 1.5× main's, find why and
  fix it.

## Evidence

- `relay/specs/445/views.md`: per view, what main shows and what the branch shows; the `--diff`, `--places` and `--time`
  tables; the bar you set and why; `play/places.mjs`'s result; every change to the easel and every variant authored.
- `relay/specs/445/shots/`: main's and the branch's PNGs of `places-lineup-close`, `places-lineup-play`,
  `medium9-places-play`, `medium9-places-close`, `small2-places-close`, `large8-places-close` and `large8-places-play`,
  named `<view>-main.png` and `<view>-branch.png`, cropped round the places where the full canvas is mostly empty
  (`relay/specs/444/proto/crops.mjs` is the pattern).
- `atlas.md` updated with the final hashes and recipes.

Run `node www/play/all.mjs` and `engine/check.sh` whole once before you finish.
