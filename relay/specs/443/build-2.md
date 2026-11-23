# #443 build-2 (easel): the figures on the map, the views and the checks

Read `relay/specs/443/spec.md` first. Its "How the figures are drawn" and "The views" are your rules, word for word.
Build-1 has cut `unit.camp`, `unit.scout` and their masks at 96×96, anchor (48, 92); its summary and
`relay/specs/443/atlas.md` say what it left. Build-3, the canvas, tunes `FIGURE` and the cells by eye. Your job is
rules that are exact, numbers that run, and checks that hold the rules whatever numbers build-3 picks. Don't change
Rust: nothing under `engine/core`, `engine/cli` or `engine/web`. Leave `www/assets/` alone: it is build-1's and
build-3's.

Read these before you edit: `www/map/layers/{units.js,icon.js,capitals.js,woods.js}`, `www/atlas.js`,
`www/map/view.js` (`drawNow`, `stats`, `setSelection`), `www/map/input.js` (selection by tile), `www/shots.mjs`,
`www/play.mjs` (`lead`, `raiseArmy`), `www/play-stage.mjs`, `www/play/{army.mjs,select.mjs,woods.mjs,all.mjs}`, and
`relay/specs/443/proto/{scenes.mjs,lineup.mjs}` (each view's setup and the lineup, run on main).

## 1. `www/atlas.js`

- `shaded(name, colour, h)`: the cell pre-scaled to `h` px tall (as `scaled` does), multiplied by `colour` with
  the cell's alpha kept (draw the scaled cell, `multiply` a fill of the colour, then `destination-in` the scaled
  cell). Cached by name, colour and height, cleared as `scaled`'s cache is.
- `outline(name, colour, h, width)`: the cell's alpha pre-scaled to `h` px, dilated by `width` device px (the
  scaled alpha drawn at every offset within the width, or any method that gives the same set of pixels) and filled
  in `colour`, on a canvas `2·width` larger each way so its anchor is the cell's plus `width`. Cached likewise.
- `tinted` stays as it is, for `icon.capital.mask`.

## 2. `www/map/layers/units.js`

- `FIGURE`, at the top, each number with a `note`: `s` 0.6, `minPx` 6, `h` 2.4, `feet` 0.9, `gap` 1.7, `selected`
  2.4 and `hungry` 3.5 (css px, times `view.dpr` for device px), and the numbers' place (`labelX` 0.95, `labelY`
  -0.3, as main's, in `s`).
- Per tile, units in id order. Unit `n`'s slot and feet are the spec's. Its drawn height is `FIGURE.h·s` world
  units; its device height `round(FIGURE.h·s·view.k·view.dpr)`. Draw with the cell's anchor on the feet, the
  pre-scaled canvases mapped back to world size.
- Order on a tile, as the spec: hungry outline (red `#ff2a2a`), selected outline (white), figure (`scaled`), mask
  (`shaded` in `v.civs[u.civ].color`), for each unit; then each unit's number and its archers' bow, at its figure's
  upper right and upper left. A fallen civ's units draw nothing, as now.
- `view.stats.labels` stays as it is. `view.stats.units` lists each figure drawn: `{ unit, civ, kind, x, y, w, h }`,
  its rectangle in canvas px, with `selected` and `hungry`.
- `export function probe(model, view)`: draws the units layer alone on two transparent canvases the size of the
  map's, once as drawn and once with every civ's colour `#000000`, and returns for each unit in `stats.units` its
  rectangle, `opaque` (pixels with alpha in the first), `tinted` (pixels that differ between the two), `mean` (the
  mean RGB of the tinted pixels in the first) and `nearest` (the civ whose colour is nearest `mean` in RGB, of the
  living civs). `MapView.unitProbe()` calls it with the view's model.

## 3. The views, in `www/shots.mjs`

- The options `lead`, `raise`, `stage: 'lineup'` and `on: 'at'` (`at` or `between`), as the spec says, and its 13
  views. `raise` uses `play.mjs`'s `raiseArmy`, so the file still copies alone onto main. Copy `LINEUP` from
  `proto/lineup.mjs` into the file (it imports `grid` and `marks` from `./play-stage.mjs`, which main has).
- Before setting a camera, wait for `!game.map.anim && !game.map.frame`: raising pans to the civ.
- `--units DIR`: for each view with `units: true` (all 13), shoot it and print, per unit, its kind, civ, terrain,
  `opaque`, `tinted`, the tinted share, `mean` and `nearest`; exit 1 if any unit's `nearest` is not its own civ.
  On main there is no `unitProbe`: print `-` and exit 0.
- Shoot every new view on the branch once to prove the cameras and record each `digest` and its units in
  `relay/specs/443/easel.md`.

## 4. `www/play/units.mjs` (in CI)

Stage the lineup (import it from where `shots.mjs` keeps it, or keep one copy in `play-stage.mjs` that both import),
camera between tiles 125 and 170 at k 35, laptop. Check, each with a `need(…)` line:

- six figures in `stats.units`: three camps and three scouts, on their tiles;
- every unit's `nearest` in `unitProbe()` is its own civ, and its `tinted` is above zero;
- a click on Gold's tile (126) selects one of its two units, a second click the other: `data().selected` changes
  and `stats.units` marks the selected one;
- both numbers on Gold's tile are in `stats.labels`, and each number's anchor lies outside the other unit's figure
  rectangle.

Add it to nothing: `all.mjs` runs every script in `www/play/`.

## 5. Checks

`node www/play/all.mjs` passes, `army.mjs`, `select.mjs`, `camp.mjs` and `scout.mjs` among them. Run
`engine/check.sh` whole once before you finish. `engine/CLAUDE.md` names `play/units.mjs`, `--units` and the new
options in one line each, within its 8 KB.

Build-3 relies on: `FIGURE` as the one table to tune; `shaded` and `outline` doing the drawing; the views and
`--units` running on the branch and on a main worktree with the branch's `shots.mjs`.
