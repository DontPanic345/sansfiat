# #444 build-3 (canvas): the clans on the map, judged beside main

Read `relay/specs/444/spec.md` first: the question, the inventory, the decisions, "The views" and "For the review".
Build-1 cut the cells and masks (`relay/specs/444/atlas.md`, `cells.png`); build-2 put them on the map and gave you
the views, `--clans`, `play/clans.mjs` and `CLAN` (`relay/specs/444/easel.md`). Your job is how the clans look and
read. Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easel gives you

- `sources.js`: the `clan.*` regions (keep polygons, scale, ground point, `floor`) and their cells' `cloth` polygons
  and grey floor. Change any recipe; `--check` must still pass, the cells keep their keys, size and anchor, and every
  other cell keeps main's pixels.
- `layers/clans.js`'s `CLAN`: the size by people (`per`, `max`, `minPx`), where a lone clan stands (`feet`), the shared
  home's ring and cap (`ring`, `shared`), the starving edge's width, and `from`, the zoom below which main's circles
  draw.
- `shots.mjs`: the 12 clan views, `--diff`, `--clans`; `play/clans.mjs`.

You may change the easel where the painting needs it (a rule, a table, a check), and say so in your summary. The
spec's decisions hold unless the views show one fails. Then make the smallest change that works and record it as a
decision: for example, the donkey joining the walkers so they read as a household moving, or the hearth's second
tent so it reads as more than a tent.

## What proves the painting

Shoot every clan view on a main worktree (the branch's `shots.mjs` copied in) and on the branch, `--diff` them
(cameras and digests match; `medium9-clans-flat` differs in no pixel), and read them side by side:

- **Shape.** Tent, hearth and walkers are three different things at k 35 and k 18; tent and hearth at phone k 6. At k 6 `minPx` sets every clan's height
  (10 css px, 1.67 world units, over `max`), so every clan there is the same size and a little wider than its hex:
  judge that crowd on `medium9-clans-edge` (185 clans). If they don't read at k 6, raise `CLAN.from` to a cell height in device px below which main's circles draw, and say
  which zoom it lands at. The walkers must read as people going somewhere, not as one more tent.
- **Colour.** In `clans-lineup-*`, red's, blue's, gold's and none's clans are told apart on plains and in the wood,
  at k 35, 18 and phone k 6. A clan of no civ reads as pale, not as the study's own hide or as Gold. Gold's cloth must
  not read as the study's ochre (#443 met the same: its grey floor went from 90 to 190). The leaning clans read as
  unaligned clans with a pull, not as a civ's own.
- **Signals.** The starving clans' red edge (`small2-clans-*`, the lineup) reads as clearly as main's ring, on Red's
  own land too, where a red-dyed tent sits inside a red border. Size still tells a big clan from a small one.
- **Co-location.** The shared homes show every clan: the town at 734 in `medium9-clans-*` (two hearths and two tents
  beside the capital, its ring and the town mark), the mixed home at 503 in `small2-clans-close`, the lineup's four.
  A clan beside a unit (the lineup's tile 126, Gold's at 780) leaves the unit's figure and number readable.
- **Fit.** A lone clan stands inside its hex at k 18 and 35. It hides the terrain, the trails and the tamed-grain
  square no more than main's circle did. Trees, rivers, borders, icons, the capital's house, towns and units sit where
  main has them.
- **Numbers.** `--clans` on every view: every clan nearest its own colour and of its state's kind (it exits 1
  otherwise). Set a bar for the tinted pixels per clan at k 18 and at phone k 6 (if the cells draw there) that the
  views show is enough to read the colour, hold every clan above it, and make `--clans` exit 1 under it, as #443's
  `TINT_BAR`. Report each kind's tinted pixels and share per view.
- **Time.** `--time medium9-clans-edge` on both trees: the branch's median beside main's. Over twice main's, find
  why (an uncached canvas per clan, a leaning colour per alpha) and fix it.

## Evidence

- `relay/specs/444/views.md`: per view, what main shows and what the branch shows; the `--diff`, `--clans` and
  `--time` tables; the bar you set and why; `play/clans.mjs`'s result; every change to the easel.
- `relay/specs/444/shots/`: main's and the branch's PNGs of `clans-lineup-close`, `clans-lineup-play`,
  `clans-lineup-edge`, `medium9-clans-play`, `medium9-clans-close`, `small2-clans-close` and `medium8-clans-play`,
  named `<view>-main.png` and `<view>-branch.png`, cropped round the clans where the full canvas is mostly empty, as
  443's were (`relay/specs/443/proto/crops.mjs`).
- `atlas.md` updated with the final hashes and recipes.

Run `node www/play/all.mjs` and `engine/check.sh` whole once before you finish.
