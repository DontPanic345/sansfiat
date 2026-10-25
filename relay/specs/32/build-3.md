# Build 3: the page

Piece #32, Founding. Builds 1 and 2 put civs, the pick, the store and units in the engine.
You give the player all of it through the page, with every order a button or a right-click.
Work in `engine/web/src/lib.rs` and `engine/www/`. Read `engine/CLAUDE.md`, builds 1 and 2's
results, and the page as #31 left it (`www/main.js`, `map-view.js`, `feed.js`,
`index.html`) first. `relay/specs/32/spec.md` has the plan. Plain JS, ES modules, no
framework, no build step beyond `./build-web.sh`.

## The design (from `docs/design/founding.md` and the piece)

> The player picks a civ to lead, or watches all three on the AI. The pick shows each civ's
> clans, people, hearths, terrain and distance to the nearest rival.
>
> Select a unit, right-click a tile, and it paths there over the cheapest route it knows;
> disband is a button in the selection panel.

The piece decides:

- The top bar adds the player's civ, its people, its store and its units; the tooltip adds
  the tile's civ.
- A selection panel: what's selected, and the actions it can take as buttons.
- Every order is given through the page: no debug tools. Debug views stay.

## The wasm API (`web/src/lib.rs`)

Add, each with a doc comment saying its JSON shape:

- `phase()` (build-1 may have added it): `"pregame" | "choose" | "main"`.
- `check(action) -> String`: `validate` without applying: `""` if it would go through, else
  why not. The page greys buttons with it and shows the reason.
- `pick() -> String`: `civs::cards` as JSON.
- `view()` adds `phase`, `player`, `civs` (each with `id, name, color, market, capital,
  store, lost`, plus `people` and `clans` counted from its clans, and `land`: its capital
  territory's tiles) and `units` (as stored).
- `route(unit) -> String`: `{path: [tiles], cost, turns}` for a unit with a `dest`, else
  `null`; `turns` is how many `units` steps it takes at `UNIT.labour` a turn with its banked
  labour (count it by walking the path, as `march` does).
- `path(from, to) -> String`: the same for a hover preview: `{path, cost, turns}` or `null`.
- `tile_civ(i) -> i32`: `civs::tile_civ`, `-1` for none.
- `rules()` adds `unit_labour`, `unit_food`, `hungry_turns`, `store_cap`, `territory`.

## The page

### Phases and the play loop

- Play, Run to end and End turn stop at the pick: the loop stops when the phase becomes
  `choose` (not at `v.end` any more). In `main`, Play and End turn run turns; Run to end is
  hidden or disabled (it runs the pre-game).
- `body.dataset.phase` is `pregame | choose | main`; add `body.dataset.civ` (the player's civ
  id, or empty), `body.dataset.units` (the player's unit count) and `body.dataset.selected`
  (`unit:40`, `capital:12`, `tile:812` or empty) for headless scripts.

### The pick

When the phase is `choose`, a card over the map (not a browser dialog) titled "Choose your
civ": one row per civ from `Sim.pick()`, with a colour swatch and the name, its clans, people,
hearths, top terrains with shares, and the steps to the nearest rival (and "no land route to a
rival" when `rival_by_land` is false). Each row has **Lead Red** and **Show** (centres the map
on the capital and highlights its land); one **Watch all three** button. Picking acts
`chooseCiv`, closes the card, centres the map on the player's capital and selects it. The
legend folds shut when the card appears (it stays toggleable), so it covers neither.

### The top bar

In `main`, a civ group: the player's swatch and name; **People** (its clans' people, rounded,
with `+n in units` when it has units); **Store** (`31 / 50`, with a `title` listing each good
and last turn's loss to the cap); **Units** (count). Watching, the group reads "Watching" and
shows nothing else. The pre-game stats stay.

### The selection panel

The existing detail panel becomes the selection panel: what's selected and, under it, its
actions as buttons. Keep it one function from a selection to `{title, html, actions:
[{label, action}]}` (in its own module, say `www/select.js`) so later pieces (the scout's
"start here" and "route to here") add actions by adding entries. Each button runs
`sim.check(action)`: if refused it's disabled, with the reason beside it in dim text.
Clicking an enabled button acts, refreshes and shows any error in `#error`.

- **A unit**: kind, civ, tile, "hungry for n turns" when hungry, its destination with the
  route's cost and turns, or "Right-click a tile to move it there" (the player's units only).
  Actions: **Disband**, and **Stop** when it has a destination (`moveUnit` to its own tile).
  Another civ's unit shows the same facts and no actions.
- **A capital**: civ, name ("Red's capital"), its land and work tiles, the store by good, its
  units. The player's capital has **Make warrior**.
- **A tile**: as now (terrain, foods, clan, market), plus its civ.
- Several of the player's units on a tile: the panel lists them ("Units here") and picking one
  selects it.

Left-click picks, in order: the player's units on the tile (the lowest id), any civ's
capital, any other units, then the tile. Escape clears the selection.

### Right-click

With one of the player's units selected, a right-click (or a press with button 2 that
doesn't drag) on a tile acts `moveUnit {unit, to}`: the route shows at once; a refusal shows
in `#error`. The canvas never opens the browser's context menu. Right-click doesn't select.
`map-view.js` today calls `onPick` for any button's release: route button 2 to a new
`onCommand(i)` instead. While one of the player's units is selected, the tooltip adds "Move
here: cost 7, 2 turns" from `Sim.path`, or "No route".

### Drawing

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

### Feed and tooltip

- `feed.js` `KINDS` adds `civFounded`, `unitMade`, `unitHungry`, `unitDisbanded`,
  `unitStarved`, with icons, colours and a "Civs" and "Units" filter. Events about the
  player's civ are emphasised. Clicking a unit event selects the unit if it still exists,
  else its tile; clicking `civFounded` selects the capital.
- The tooltip adds the tile's civ: "Red's land" (capital territory), "Red, clan 40's round",
  or nothing; and "Red's capital" on a capital tile; and units on the tile.

### No debug orders

`window.game` keeps read-only access (`view`, `layout`, `save`, `map`, `feed`) and loses
`act`. Add `map.screenOf(i) -> {x, y}` (client coordinates of a tile's centre) so a headless
script can find where to click and right-click; scripts order only by clicking.

## `www/check.mjs`

After the pre-game ends: `phase()` is `"choose"`; `pick()` has three cards with every field;
`endTurn` throws and `check('{"type":"endTurn"}')` names the pick; `chooseCiv` the first civ;
`view()` has `civs`, `units` (three warriors), `player`; `check` of `makeUnit` is `""` or a
reason; make one if allowed; `moveUnit` it towards a rival capital (or any reachable land 5+
steps away) and `route()` has a path; run 10 turns, the unit has moved; `disbandUnit` it; the
new event kinds appear with text; `tile_civ` of a capital is its civ. Keep it fast.

## Done when

- `./build-web.sh`, `node www/check.mjs`, `./cross-build.sh`, and the Rust checks (`cargo
  test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`) pass.
- You've driven the page in headless Chrome (as #31 did: `/usr/bin/google-chrome --headless=new
  --remote-debugging-port=…`, driven by a small Node script over the DevTools protocol, with
  right-clicks as `Input.dispatchMouseEvent` with `button: "right"`; keep the script outside
  the repo or under `/tmp`) through clicks only: Run to end on small seed
  2 (`?size=small&seed=2`), the pick card, Lead, the top bar, the capital selected with Make
  warrior, a warrior selected from the feed, a right-click move with its route drawn, End
  turn a few times, Disband. Read your screenshots, fix what reads badly, and say in the
  result what you saw. Don't commit screenshots.
- `engine/CLAUDE.md`'s `web/` and `www/` rows say what's new (the API, the modules, the
  dataset keys, `screenOf`, no `act`).
