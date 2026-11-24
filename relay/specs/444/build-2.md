# #444 build-2 (easel): the clans on the map, the views and the checks

Read `relay/specs/444/spec.md` first. Its "How the clans are drawn", "The views" and the lineup under it are your
rules, word for word. Build-1 has cut `clan.tent`, `clan.hearth`, `clan.walk` and their masks at 112×96, anchor
(56, 90), and exported `CLAN` from `placeholders.js`; its summary and `relay/specs/444/atlas.md` say what it left.
Build-3, the canvas, tunes the table and the cells by eye. Your job is rules that are exact, numbers that run, and
checks that hold the rules whatever numbers build-3 picks. Don't change Rust: nothing under `engine/core`,
`engine/cli` or `engine/web`. Leave `www/assets/` alone: it is build-1's and build-3's.

Read these before you edit: `www/map/layers/{clans.js,leanings.js,units.js,woods.js,terrain.js}`, `www/atlas.js`
(`scaled`, `shaded`, `outline`), `www/map/view.js` (`LAYERS`, `drawNow`, `stats`, `unitProbe`), `www/model.js`
(`clanCiv`, `tintOf`), `www/legend.js`, `www/shots.mjs`, `www/play-stage.mjs`, `www/play/{units.mjs,towns.mjs,goods.mjs,all.mjs}`,
and `relay/specs/444/proto/lineup.mjs` (the lineup and its states, run on main). `units.js` is the pattern for nearly
all of it: a table of numbers at the top, pre-scaled cells, the multiply, the outline, `stats`, and `probe`.

## 1. `www/map/layers/clans.js`

- `CLAN`, at the top, each number with a `note`: `per` 0.42, `max` 1.5, `minPx` 10 (css px), `feet` 0.4, `ring` 0.42,
  `shared` 0.8, `starving` 3.5 (css px, times `view.dpr` for device px), and `from`, the flat test (start: the
  terrain's own, `isFlat(cellHeight(view), view)`). Name the table so it doesn't clash with `placeholders.js`'s `CLAN`
  (the cell size), or import that one under another name.
- **The flat zoom:** main's `draw` and `drawClan`, unchanged, so the flat views keep main's pixels. Each clan drawn
  goes into `stats.clans` as a `dot`.
- **Above it**, per the spec: every on-screen leaning disc first (export the disc from `leanings.js` as a function
  and call it), then per home, clans in id order placed as the spec says, drawn back to front by y. Per clan: its
  kind (`since ≤ 1` walk, else `settled` hearth, else tent), its drawn height, its device height
  `round(h·k·dpr)`; the red outline if starving (`atlas.outline`, `#ff2a2a`); the cell (`atlas.scaled`); the mask
  (`atlas.shaded` in the civ's colour from `model.clanCiv`, or `UNALIGNED`). The cell's anchor on the clan's point,
  the pre-scaled canvases mapped back to world size.
- `view.stats.clans`: each clan drawn, `{ clan, civ, kind, x, y, w, h, starving }`, its rectangle in canvas px, `civ`
  null for none. Reset it each frame as `stats.units` is.
- `export function probe(model, view)`, as `units.js`'s: for each entry of `stats.clans`, draw that clan alone (cell and
  mask, or its circle at the flat zoom; no disc, no outline) on two transparent canvases, once in its colour and once
  with every colour `#000000`, and return its rectangle, `opaque`, `tinted`, `mean` and `nearest`: among the living
  civs' colours and `UNALIGNED`, the one nearest `mean` by chromaticity (reuse `units.js`'s `chroma`; export it or move
  it where both import it), as the civ's id or `'none'`. `MapView.clanProbe()` calls it with the view's model.

## 2. `www/map/layers/leanings.js` and `www/legend.js`

- `leanings.js` draws as on main at the flat zoom and nothing above it. It exports the disc (one clan's, main's
  radius, alpha and edge) for `clans.js`. `LAYERS` keeps its order.
- `legend.js`'s clan rows name the new marks: a tent (a clan that gathers), a hearth (a settled clan), walkers (a
  clan that came to its home this turn), the red edge (starving), the cloth in the civ's colour (pale: no civ), and
  the disc under an unaligned clan (the civ it leans to). The tamed-grain hearth row stays. A swatch may be the atlas
  cell (the atlas image as a CSS background, cropped and scaled to the cell) or the plain shape it is today; keep
  `play/views.mjs`'s legend checks passing.

## 3. The views, in `www/shots.mjs`

- The spec's 12 clan views, each with `clans: true`, and the option `states: true` for the lineup. `stage: 'clans'`
  stages `CLANS`; copy `CLANS` and `CLAN_STATES` into `shots.mjs` (it imports `grid` and `marks` from
  `./play-stage.mjs`, which main has), so the file still copies alone onto a main worktree. `CLAN_STATES` is the
  in-page code that sets the states on `game.map.model.v.clans` and redraws, as `proto/lineup.mjs` does.
- `--clans DIR`: for each view with `clans: true`, shoot it and print, per clan in `clanProbe()`, its id, kind, civ
  (or `none`), terrain, `starving`, `since`, `settled`, `opaque`, `tinted`, the tinted share, `mean` and `nearest`;
  then per view the count of each kind. Exit 1 if any clan's `nearest` is not its own (its civ, or `none`), or its
  kind is not its state's. On main there is no `clanProbe`: print `-` and exit 0.
- Shoot every new view on the branch once to prove the cameras, and record each `digest` and its counts in
  `relay/specs/444/easel.md`. The real views' digests must be the spec's (`proto/clans.txt`): medium 9 +16
  `68a2c856313bf378`, small 2 +16 `0a19bbf2eb1dce5c`, medium 8 +16 `f2bcde40e4686b25`, small 9 +8 `a742b8a68f9a8081`;
  the lineup `4157b18e28b9bd5c`.
- `shots.mjs` is 435 lines. If it passes 600, move the views table into `www/shots-views.mjs`.

## 4. `www/play/clans.mjs` (in CI)

Keep `CLANS` and `CLAN_STATES` in `play-stage.mjs` too (the comment at its top says why the copy in `shots.mjs`).
Stage the lineup, set the states, camera between tiles 93 and 170 at k 35, laptop. Check, each with a `need(…)` line,
everything the spec's "For the review" lists for `play/clans.mjs`: the kinds, the colours, the starving clan's red
edge, the shared home's four places inside its hex, the click cycling the shared home's four clans (`page.select`
and `page.click` show how), and the dots at the flat zoom. On a laptop the lineup's map holds `k` at 7.47 or more (its `minK`), so the flat
check stages the lineup again on a phone page (`open('', { size: 'phone' })`) at k 5, where `stats.atlas` is false
(`terrain.js` sets it so when the terrain is flat); at phone k 6 it is still true. Run on main by the spec step. Write `clans.png` to `--out`. `all.mjs` runs every script in
`www/play/`, so add it to nothing.

## 5. The callers

- `play/towns.mjs` counts pixels within 24 of each town clan's flat colour, with at least 8 needed. A multiplied cloth
  is shaded, so it may fall short. If it does, change that check to read `clanProbe()` (each town clan drawn, nearest
  its own colour): the same claim, measured the way the piece measures it. Say so in your summary.
- `play/goods.mjs` turns `clans` off: a clan's leaning disc is drawn by `clans.js` above the flat zoom, so it goes too.
  Check that `goods.mjs` still passes.

## 6. Checks

`node www/play/all.mjs` passes. Run `engine/check.sh` whole once before you finish. `engine/CLAUDE.md` names
`play/clans.mjs` and `--clans` (with `stage: 'clans'` and `states`) in one line, within its 8 KB: tighten other
lines of the commands block if it doesn't fit (`check.sh` fails over 8,192 bytes).

Build-3 relies on: `CLAN` in `clans.js` as the one table to tune; `scaled`, `shaded` and `outline` doing the drawing;
the views and `--clans` running on the branch and on a main worktree with the branch's `shots.mjs`.
