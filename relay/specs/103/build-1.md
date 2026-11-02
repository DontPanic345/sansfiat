# #103 build-1 (easel): the map side: Standing in the view, the view model, layers, the atlas

You build what everything after this step stands on. That is:
- each civ's Standing in `Sim.view()`;
- the view model every layer and panel reads;
- the map split into one layer per mechanic;
- one atlas of placeholder art the map draws from;
- the driver's new powers, with `play/all.mjs` in `check.sh`.

The page must look and behave as it does now when you finish. The screens, the bar and the
panel are build-2's and build-3's. Read `spec.md` first: items 1–6, 21–23, and the touch-point
table.

## Before you change anything

Shoot the current page with `play.mjs` (small seed 2) at laptop size, at the pick and five turns
after leading Red, with the camp selected. You will compare against these by eye at the end.
Read `www/map-view.js` whole, `www/main.js`'s `refresh` and `routes`, and `www/select.js`'s uses of
`v`.

## What to build, and where

1. **The engine** (spec item 21). In `web/src/lib.rs`, `view()` gives each civ `standing`,
   `standing_clans` and `standing_land` from `stats::standing_parts`, and `rules()` gives
   `save_version` (`world::SAVE_VERSION`). `www/check.mjs` checks them: each in [0, 1], their sum
   at most 1, and parts that add up. `stats::digest` must not move. Run
   `cargo run --release -p sansfiat -- digest` for small seeds 1–3 before and after, and put both in
   your summary.

   In `cli/src/digest.rs`, `digest --game FILE` replays a save (`World::replay`) and prints one
   line, `<turn> <digest>`, as the other forms do. Today no command prints a save's digest, and the
   canvas and the review match a game saved on the page against it. Add a test in
   `cli/tests/` that saves a played world and checks the line, and the usage line in `main.rs`.

2. **The view model** (`www/model.js`, spec item 3). `build(sim, layout, rules, prev)` returns
   `{ v, ... }` with:
   - `clanCiv` (Map);
   - `stacks` (tile → units, sorted by id);
   - `holders(tile)`;
   - `claims` (civ → claimed, held, paper, thinnest: today's `claimCounts`);
   - `standing` (civ → {total, clans, land, rank});
   - `leader`;
   - `walkers` (festival → clan ids);
   - `mine` (the player's units);
   - `history`, the Standing per turn since the pick, carried over from `prev`.

   `main.js`'s `refresh` builds it once per change and hands it to the map and the panel. Move
   the derivations that `map-view.js` (`buildCivs`, `clanAt`, `tintOf`) and `select.js`
   (`claimCounts`, `holders`) make into it. Layers and panels read the model, not `sim`.

3. **The layers** (spec item 4).
   - `www/map-view.js` becomes `www/map/view.js`, holding `MapView` with pan, zoom, picking, the
     offscreen terrain layer, and the layer list.
   - Each layer is a file in `www/map/layers/`, exporting `{ name, static, draw(ctx, model, view) }`.
     Static layers draw into the offscreen terrain layer.
   - The order is spec item 4's. `drawCivs` splits into land, held, lines, leanings, clans, units
     and routes. `drawMarks` splits into festivals and trails, and the debug views each become a
     layer.
   - `view.setLayer(name, on)` replaces `setOptions`, and `main.js`'s View menu calls it for now.
     Build-2 moves the toggles into View and Debug.
   - The map's state for the selection (`setFocus`, `setLine`, `setPlan`, `setRoutes`,
     `setPainting`, `setHighlight`) stays on `MapView`, where the layers read it.
   - Nothing changes in what is drawn.

4. **The atlas** (spec item 5).
   - `www/assets/atlas.json` names every cell with its `x, y, w, h` and a `note`, using spec
     item 5's names and sizes, laid out in rows by kind, 1024 px wide.
   - `www/assets/placeholders.js` exports `draw(name, ctx, w, h)`, which draws each cell from
     today's look:
     - terrain: the flat colours, the mountain's ▲, the desert's dots as an overlay, marsh's lines;
     - units: today's shield;
     - icons: the capital's house, the festival's circle, the centre's ring, the hearth's square,
       the pass's dot.
   - `www/assets/draw-atlas.mjs` opens a blank page with `play.mjs`'s Chrome, draws every cell
     into one canvas, and writes `www/assets/atlas.png`. Commit the PNG.
   - `www/atlas.js` loads the JSON and the image and gives `cell(name)`, and `tinted(name,
     color)` for a `.mask` cell tinted into an offscreen canvas, cached per colour.
   - The terrain layer draws each tile's cell, picking variant `i % 3`. Units and icons draw
     their cells.
   - The terrain layer redraws on pan and zoom. Measure `stats.terrainMs` on a large map (seed 2)
     at fit, at laptop size, as the median of ten redraws, before and after. It must stay under
     16 ms, one frame. Spec-review measured main at 8.5 ms there, and a bare `drawImage` per tile
     of 3,367 tiles alone at 10–12 ms (`spec-review.md`), so expect to need a cache of the cells
     pre-scaled per zoom step, or flat fills where a cell is only a few pixels across. Give both
     numbers in your summary.
   - `www/play.mjs`'s server already serves `.png` and `.json`.

5. **The driver** (spec item 22).
   - Add to `play.mjs`: `SIZES`, `open(query, {size})`, `page.reload()` (same profile, so
     `localStorage` stays), `page.key(name)`, `page.file(selector, path)` via
     `DOM.setFileInputFiles`, `page.drag(from, to)`, `page.tip(target)`, and `lead(page, {seed,
     civ})`.
   - `www/play/all.mjs` runs every other `.mjs` in `www/play/` with a temp `--out`, and prints a
     table of script, pass and seconds. It exits non-zero if any fail. Without Chrome it fails
     under `CI` and skips with a warning otherwise.
   - Add `node www/play/all.mjs` to `check.sh` after `check.mjs`. In `.github/workflows/ci.yml`,
     the syntax-check loop covers `engine/www/**/*.js` and `**/*.mjs` (a `find`).

6. **Two scripts.**
   - **`www/play/art.mjs`**:
     - every cell in `atlas.json` lies inside `atlas.png` and is not blank (read through the page:
       `game.atlas`);
     - the map drew its terrain from the atlas (`game.map.stats.atlas` true, or what you expose);
     - shoots the map at the pick.
   - **`www/play/views.mjs`**:
     - pan, zoom and fit move the view (`game.map` k, cx, cy);
     - each layer toggles by name and the map redraws;
     - H toggles hidden country;
     - a hover shows a tooltip naming the terrain.

     Build-2 adds the View and Debug menus to this script.

7. The old `www/play-*.mjs` scripts keep passing, except `play-hold`, which fails on main
   (spec item 24) and build-3 rebuilds. Run each once at the end, and fix what you moved under
   them.

8. `engine/CLAUDE.md`: the `www` rows for the files you add, move and remove. `play/all.mjs`
   goes in the command block.

## Done

- `./check.sh` passes, including `play/all.mjs` with `art` and `views`.
- The digests of small seeds 1–3 are unchanged.
- Your after-shots match your before-shots by eye, except where the placeholders differ in
  detail. Put both in `relay/specs/103/screens/build-1-*.png`, at half scale.
- `terrainMs` before and after, on a large map at fit: under 16 ms.
- `sansfiat digest --game FILE` prints the same digest as the page's `Sim.digest()` for a game
  saved on the page.

Build-2 and build-3 rely on:
- `model.js`'s fields;
- `MapView`'s layer API;
- `atlas.js`;
- the driver's helpers (`lead`, `reload`, `key`, `file`, `drag`, `tip`, `SIZES`);
- `play/all.mjs` picking up every new script.
