# Build 4: the page's look

Piece #32, Founding. Builds 1 and 2 put civs, the pick, the store and units in the engine;
build-3 gave the page its logic (wasm API, phases, the pick card, top bar, selection panel,
right-click orders, `check.mjs`). You make it readable on the map: drawing, the feed's new
kinds, the tooltip and the legend. Work in `engine/www/` (and `web/src/lib.rs` only if a field
you need is missing from `view()`). Read `engine/CLAUDE.md`, builds 1 to 3's results, and the
page as they left it (`www/main.js`, `select.js`, `map-view.js`, `feed.js`, `index.html`).
`relay/specs/32/spec.md` has the plan. Plain JS, ES modules, no framework.

`window.game` has no `act` (build-3); keep it so.

## Drawing

Drawn after clans, each with a dark casing, as #31's review did for centres so they read on
a busy map:

- Each civ's capital: a city mark in its colour on the capital tile; its land (the capital
  territory) outlined in its colour; its work tiles faintly marked.
- Civ clans: their clan circle gets a ring in the civ's colour.
- Units: a small shield or pennant in the civ's colour per tile, with a count when there are
  several; hungry units marked (red edge); the selected unit highlighted.
- The selected unit's route: a line along the path to its destination, with a flag at the
  end. The player's other routes faint.
- A "Civs" toggle in the View menu (on) for land outlines and clan rings.
- The legend gains capital, civ land, civ clan, warrior, hungry, route.

## Feed and tooltip

- `feed.js` `KINDS` adds `civFounded`, `unitMade`, `unitHungry`, `unitDisbanded`,
  `unitStarved`, with icons, colours and a "Civs" and "Units" filter. Events about the
  player's civ are emphasised. Clicking a unit event selects the unit if it still exists,
  else its tile; clicking `civFounded` selects the capital.
- The tooltip adds the tile's civ: "Red's land" (capital territory), "Red, clan 40's round",
  or nothing; and "Red's capital" on a capital tile; and units on the tile.


The tooltip also adds, while one of the player's units is selected, "Move here: cost 7, 2
turns" from `Sim.path(from, to)`, or "No route". Reuse build-3's selection and `onCommand`; don't
change how orders are given.

Draw order and legibility (from #31's review): capitals, civ outlines, units and routes come
after clans, centres and markets, each with a dark casing, so Gold reads apart from hearth
yellow and centre rings on a busy map, and the legend folds shut while the pick card is open.
Per-turn drawing must not grow with the clan count beyond what the clan circles already cost;
build the civ land outline path once per `view()`, not per frame.

## Done when

- `./build-web.sh`, `node www/check.mjs`, `./cross-build.sh` and the Rust checks (`cargo test`,
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`) pass.
- You've played it in headless Chrome as build-3 did (a small Node script under `/tmp` over the
  DevTools protocol, `Input.dispatchMouseEvent` for clicks and right-clicks, clicks only): Run
  to end on small seed 2, the pick card with Show, Lead, the capital and its land drawn, **Make
  warrior**, a warrior selected from the feed, a right-click move with its route drawn and the
  tooltip's preview, End turn until it arrives, a hungry unit if you can make one (an
  empty store; if you can't, say so), **Disband** and the feed line, and **Watch all three**.
  Take screenshots at each point, Read them, and fix what reads badly: can you find the capital,
  the units, the route and the civ land at the default zoom and zoomed out? Do the Gold marks
  read apart from hearths and centres? Say in the result what you saw. Don't commit screenshots.
- `engine/CLAUDE.md`'s `www/` row says what's new (the Civs toggle, the feed kinds, the legend).
