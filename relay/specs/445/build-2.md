# #445 build-2 (easel): the places on the map, the views and the checks

Read `relay/specs/445/spec.md` first. Its "How the places are drawn", "The views" and the lineup under it are your
rules, word for word. Build-1 has cut `place.town`, `place.settlement`, `place.capital` and the two masks at 176×128,
anchor (88, 118), and exported `PLACE` from `placeholders.js`; its summary and `relay/specs/445/atlas.md` say what it
left. Build-3, the canvas, tunes the table and the cells by eye. Your job is rules that are exact, numbers that run, and
checks that hold the rules whatever numbers build-3 picks. Don't change Rust: nothing under `engine/core`, `engine/cli`
or `engine/web`. Leave `www/assets/` alone: it is build-1's and build-3's.

Read these before you edit: `www/map/layers/{clans.js,capitals.js,settle.js,towns.js,centres.js,units.js,icon.js}`,
`www/atlas.js` (`scaled`, `shaded`), `www/map/view.js` (`LAYERS`, `drawNow`, `stats`, `clanProbe`), `www/legend.js`,
`www/shots.mjs`, `www/play-stage.mjs`, `www/play/{clans.mjs,towns.mjs,settle.mjs,views.mjs,all.mjs}`, and
`relay/specs/445/proto/lineup.mjs` (the lineup, run on main). `clans.js` is the pattern for nearly all of it: a table at
the top, the flat test, pre-scaled cells, the multiply, `stats`, and `probe`.

## 1. `www/map/layers/places.js`

- `PLACE`, at the top, each number with a note: `w` `{ town: 1.6, settlement: 1.8, capital: 2.0 }` (world units wide),
  `feet` 0.3, `from` 28 (css px of terrain cell). Name it so it doesn't clash with `placeholders.js`'s `PLACE` (the cell
  size), or import that one under another name.
- `export const placesFlat = (view) => …`, as `clansFlat`: the terrain is flat, or `cellHeight(view) / view.dpr <
  PLACE.from`.
- **Above it:** per the spec, the places on screen: each living civ's capital, each settlement not on a capital, each town
  not on either, in that order. Per place: its cell (`atlas.scaled`) anchored at `(px, py + feet)` at its role's width; for
  a capital or settlement the mask (`atlas.shaded`) in its civ's colour over it. At the flat zoom it draws nothing.
- `view.stats.places`: each place, `{ tile, role, civ, x, y, w, h }` in canvas px, `civ` null for a town; at the flat zoom
  each capital and settlement on screen as `role: 'mark'`. Reset each frame as `stats.clans` is.
- `export function probe(model, view)`, as `clans.js`'s: per entry above the flat zoom, the place alone (cell and mask) on
  two transparent canvases, once in its colour and once in `#000000`; its rectangle, `opaque`, `tinted`, `mean`, and
  `nearest` among the living civs' colours by `chroma` (or `null` for a town). `MapView.placeProbe()` calls it.
- In `LAYERS`, `places` goes just before `clans`. Add it to the `civs` group in `GROUPS` so View → Civs turns it off
  with the capitals.

## 2. The flat path, the markers and the legend

- `capitals.js` draws as on main where `placesFlat(view)`, and nothing above it. `settle.js` draws its signposts as on
  main at every zoom, and its settlements' houses as on main where `placesFlat(view)`, nothing above it. `towns.js` and
  `centres.js` are unchanged.
- `legend.js`: the capital row and the settlement row show the new marks, and a town row is added ("town: a festival
  several clans live on"). A swatch may be the atlas cell (as #444's clan rows show theirs) or the plain shape it is
  today; keep `play/views.mjs`'s legend checks passing.

## 3. The views, in `www/shots-views.mjs`

- `shots.mjs` is 563 lines. Move its views table (and the lineups' specs it copies) into `www/shots-views.mjs`, which
  `shots.mjs` imports; both files then copy onto a main worktree together. Say so in `shots.mjs`'s head.
- The spec's 10 place views, each with `places: true`. `stage: 'places'` stages `PLACES`; on a staged view, `turns` ends
  that many turns after staging (the lineup's settlement is founded on its first turn); `states: true` then sets
  `PLACE_STATES`. Copy `PLACES` from `proto/lineup.mjs` as it is, and write `PLACE_STATES` as the spec says.
- `--places DIR`: for each view with `places: true`, shoot it and print, per place in `placeProbe()`, its tile, role, civ
  (or `none`), whether it reads as a town, the clans on its tile, `opaque`, `tinted`, `mean` and `nearest`; then per view
  the count of each role. Exit 1 if a capital's or settlement's `nearest` is not its own civ, or a place's role is not its
  tile's. On main there is no `placeProbe`: print `-` and exit 0.
- Shoot every new view on the branch once to prove the cameras, and record each `digest` and its counts in
  `relay/specs/445/easel.md`. The real views' digests must be the spec's (`proto/scenes.txt`); the lineup's
  `514f01f255b6d28f`.

## 4. `www/play/places.mjs` (in CI)

Keep `PLACES` and `PLACE_STATES` in `play-stage.mjs` too (`shots-views.mjs` copies them, as `CLANS` is copied, so the
file still copies alone onto main). Stage the lineup, end one turn, set the states, camera between tiles 47 and 140 at
k 35, laptop. Check, each with a `need(…)` line, everything the spec's "For the review" lists for `play/places.mjs`: each
place's role against the rule on the view (not a list), the colours and the town's zero tint, every clan on a place's tile
drawn after the place (its `stats.clans` entry is there; the clans layer follows `places` in `LAYERS`), the click on 140
reaching the settlement and its clans (`page.select` and `page.click` show how; `play/settle.mjs` cycles a settlement's
tile), and the marks at the flat zoom (stage again on a phone page at k 5, as `play/clans.mjs` does). Write `places.png`
to `--out`. `all.mjs` runs every script in `www/play/`, so add it to nothing.

## 5. The callers

- `play/settle.mjs` checks the settlement is drawn where the signpost was; `play/towns.mjs` checks the town mark and its
  clans. Both must still pass. If one fails only because it reads main's house pixels above the flat zoom, change it to
  read `stats.places` (the same claim, measured the way the piece measures it) and say so in your summary.

## 6. Checks

`node www/play/all.mjs` passes. Run `engine/check.sh` whole once before you finish. `engine/CLAUDE.md` names
`play/places.mjs`, `--places` (with `stage: 'places'`) and `shots-views.mjs` in as few bytes as it can, within
`check.sh`'s 8,192: build-1 left it under 8,100; fold the line into the `--units`/`--clans` line if that fits.

Build-3 relies on: `PLACE` in `places.js` as the one table to tune; `scaled` and `shaded` doing the drawing; the views and
`--places` running on the branch and on a main worktree with the branch's `shots.mjs` and `shots-views.mjs`.
