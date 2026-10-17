# Sans Fiat

Proof-of-concept hex 4X with no money: goods are physical and worth what people will trade for them.
A simulated pre-game (hex-economy) grows clans and markets; the best markets become civs; the player
leads one. Plain ES modules, no build step, no dependencies. Spec: `docs/DESIGN.md` (items marked
**proposed** are in scope unless under "Out (for later)"). Original design notes: `docs/notes/`.

```
npm start          # http://localhost:8000  (?seed=N&players=N)
npm test           # node --test (all of test/*.test.js)
npm run sim -- --seed 3 --players 3 [--turns 500] [--main 20] [--every 25] [--json]
npm run bench      # time the headless pre-game for seeds 1-3 (or: npm run bench -- 7 8)
```

`sim` runs the pre-game, founds the civs with every civ on the AI (`chooseCiv {civ: null}`), then
`--main` turns of the main game. In the browser console: `game.state`, `game.act({...})`,
`game.history`, `game.replay()`.

## Layout

| File | What |
| --- | --- |
| `src/data.js` | Core tuning: goods, terrain, `TRAILS`, `MAP`, `CLAN`, `MARKET`, `TENDING`, `PREGAME`, `CITY`, `NOTES`, `CIVS`. |
| `src/core.js` | Shared helpers for every module (below). Imports nothing from rules, clans or modules. |
| `src/clans.js` | The pre-game clan rules (forage, trade, settle, occupy, markets) and founding (`foundCivs`). Runs for every clan in the main game too. |
| `src/market.js` | `clear(baskets)`: exchange ratios where supply meets demand over `GOODS`, then rationing. |
| `src/rules.js` | `PHASES`, the turn pipeline, the merged action table, `dispatch`. Wires in the modules. |
| `src/game.js` | `newGame(config)`, `replay(config, actions)`, `runPregame(s, n)`, `runPregameCounted`. |
| `src/mapgen.js` | Height noise, inlets, inland seas, ridges with flanks and passes, deserts, rivers, coast, fertility, hidden trail costs. |
| `src/hex.js`, `src/rng.js` | Axial pointy-top hexes; mulberry32 `rand(state)`, `weighted`. |
| `src/render.js`, `src/main.js`, `src/views.js` | Canvas, controls and side panel, debug map views. The only DOM code. |
| `src/modules/index.js` | Rules registry: `MODULES = [civ, scout, army, supply]`. |
| `src/modules/ui.js` | UI registry: each module's `ui.js`, sorted by `z`. |
| `src/modules/<name>/{rules,data,ui}.js` | One module. |
| `test/<name>.test.js` | Tests per module; `rules.test.js` (clans, markets) and `core.test.js` (core, registry). |

## Module registry contract

Each module is `src/modules/<name>/`:

- **`rules.js`** exports
  - `name` — the module name.
  - `init(s)` — called from `newGame` after the map and seed clans exist. Create `s.mods.<name>`.
  - `onStart(s)` — called once by `chooseCiv` when the main game begins, in registry order
    (civ, scout, army, supply). `s.civs` exists; the human civ has `isHuman: true`.
  - `actions` — `type -> { validate(s, a) -> error|null, apply(s, a), candidates(s, civId) -> [action] }`.
    Every module action carries `civ` (the acting civ id). Core rejects it unless `s.phase === 'main'`
    and `s.civs[a.civ]` exists, before your `validate` runs. Type names are global: a clash throws at load.
    `apply` mutates the clone `dispatch` hands it; apply immediately unless your design queues orders
    for the `orders` phase (store queues under `s.mods.<name>` or the civ).
  - `turnSteps` — `[{ phase, name, run(s, ctx), order?: 'before'|'after', when?: 'main'|'always' }]`.
    `order` places the step before or after the core step of that phase (default after);
    `when` defaults to `'main'` (pre-game steps are core's). Unknown phases throw at load.
  - `hooks` (optional) — seams into the clan rules: `daughterSite(s, clan, {options, pop})` returns a
    tile key from `options` (where a splitting clan's daughter goes), `null` (she stays), or
    `undefined` (let the clan choose); first module with an answer wins. `yieldFactor(s, tile)`
    scales regrowth (good and bad years); factors multiply.
  - `ai(s, civId) -> [action]` (optional) — core applies each valid one in the `orders` phase for every
    non-human civ. Must be deterministic in state (use `rand(s)`), so replays reproduce it.
- **`data.js`** — the module's tuning. Import only from `src/data.js` here.
- **`ui.js`** exports (all optional) `z` (layer order; scout's fog is 90, drawn last),
  `drawLayer(ctx, s, view)`, `panel(s, selection, ui) -> HTML`, `onClick(s, hex, ui) -> consumed?`.
  `view` = `{size, at(hex) -> {x, y}, hexPath(ctx, center, size), selected, mode, viewer, local}`.
  `ui` = `{act(action) -> ok, local, selection}`. `local` is the module's UI-only scratch state
  (selected unit, brush) kept by main.js, never in game state. In panel HTML, any element with
  `data-action='{"type":"...","civ":N,...}'` dispatches that JSON when clicked.

Imports: a module imports `../../core.js`, `../../clans.js`, `../../data.js`, `../../hex.js`,
`../../rng.js`, `../../market.js` and its own files. Never `../../rules.js` (cycle), and never another
module's files: modules talk only through the documented state below.

## Phases

`PHASES` in `src/rules.js`, from DESIGN.md's turn order. One `endTurn` runs every phase in order; in
each, module `before` steps, then core, then module `after` steps (modules in registry order).

| Phase | Core step (pre-game and main) | Expected module work |
| --- | --- | --- |
| `regrow` | tiles regrow (skips `fallow` tiles, applies `yieldFactor`) | civ: good/bad years |
| `forage` | clans move, adjust land, decide to trade, harvest | civ: city workers, materials |
| `trade` | each market clears clan baskets plus `m.posts` | civ: post store `before`, collect `m.received` `after` |
| `settle` | clans eat, grow, starve, divide, split | civ: city labour, gifts |
| `occupy` | held grain tames, abandoned grain goes feral | |
| `markets` | gatherings form and fade; markets form | civ: leanings, joining |
| `orders` | (main) AI civs' actions from each module's `ai` | scout routes, captains, porters, projects |
| `armies` | — | army: spread, skirmish, capture; supply: eat, resupply |
| `discoveries` | — | civ: discoveries, victory; scout: fog |

After the phases core records stats, refills every unit's `labour` to `labourMax`, sweeps notes of
entities that are gone, ends the pre-game when `pregameOver`, and increments `s.turn`.

## State shape

Plain JSON. Every entity id comes from `s.nextId` (`newId(s)`), so ids are unique across kinds.

```
s.config   {seed, players, width, height}
s.rng      RNG state; only rand(s) touches it
s.turn, s.phase   'pregame' -> 'choose' -> 'main'
s.events   this dispatch's log lines (event(s, text))
s.notes    {entityId: [{turn, text}]}  the "why" log, last NOTES.perSubject per entity (note(s, id, text))
s.mods     {civ, scout, army, supply}  each module's own global state
s.stats    {pop, clans, volume} per turn
s.map      {width, height, tiles: {"q,r": tile}}
s.clans    {id: clan}   s.markets {id: market}   s.civs {id: civ}   s.cities {id: city}   s.units {id: unit}
```

- **tile**: `q, r, terrain, fertility, river, coast, shallows, flow?, cap, stock, wildGrain?, tame?,
  occ, occTotal`; `claim` (clan id holding it as land, or null); `trailCost` (hidden: cost once trailed,
  null = never; forest/hills/mountain only); `span` (mountains: bridge span needed); `trail` (built,
  shared by everyone; scout); `fort` (null | `{owner, ...}`; army); `cache` (null | `{owner, store}`;
  supply); `fallow` (turns left yielding nothing; supply sets, core counts down); `held` (civ id of
  troops standing on it, or null; army keeps it current).
- **clan**: hex-economy fields (`pop, land, store, harvest, basket, fed, starving, fedRatio, hungry,
  route, dest, market, trip, since, trail`...) plus `owner` (civ id or null = unaligned) and
  `leaning` (`{civId: number}`, civ module). Daughters inherit both.
- **market**: `owner` (civ id for a founding market), `temporary` (pre-game gathering), `prices`,
  `lastTrade`, `history`, `traders`, `posts` (`{traderId: basket}`, written before `trade`),
  `received` (`{traderId: basket}` after it).
- **civ** (created by `foundCivs` at the end of the pre-game): `id, name, color, isHuman, marketId,
  cityIds, store` (pooled goods by name; use `takeGoods`/`putGoods`), `seen` (`{"q,r": true}` fog;
  scout), `revealed` (`{"q,r": true}` hidden trail costs known; scout), `founded`.
- **city** (civ module creates them): `id, owner, name, q, r, founded` plus civ-module fields
  (projects, buildings). Capture goes through `transferCity`.
- **unit**: core fields `id, owner, type, q, r, labour, labourMax` (`spawnUnit`, `removeUnit`).
  Module fields live under the module's name: `u.scout`, `u.army`, `u.supply`. Agreed cross-module
  fields: `u.supply.food` / `u.supply.foodMax` (supply owns; army may subtract food eaten in a fight),
  `u.army.troops` and `u.army.zone` (army owns; supply reads them to cost a captain's line).

## Core helpers (`src/core.js`)

`newId, event, note, notesFor, sweepNotes, total, addGoods, scaleGoods, takeGoods, putGoods,
tileAt, passable, stepCost, costMap, pathCost, clanAt, marketAt, cityAt, unitsAt, holder, totalPop,
civOf, territoryOf, transferCity, civClans, civCities, civUnits, humanCiv, spawnUnit, removeUnit`.

- `stepCost(s, h)`: labour to enter h, trail-aware (a built trail drops it to `trailCost` and opens
  mountains); Infinity if impassable. Ignores what a civ knows; scout owns believed costs.
- `pathCost(s, from, to, {cost, maxCost})` -> `{cost, path}` or null; `costMap(s, from, maxCost, {cost})`
  -> Map of key -> cost. Pass your own `cost(s, h)` to block enemy land, use known costs, and so on.
- `territoryOf(s, h)`: `tile.held`, else fort owner, else the nearest city within `CITY.territoryRadius`.
- `civOf(s, x)`: the civ of a clan, unit, city, market, or any entity id.

From `src/clans.js` modules may use the exported helpers: `spawnClan, siteOptions, pickSite,
marketReach, reaches, nearestMarket, range, forageTiles, planHarvest, nutrition, catchment,
foundMarket, hearths, roamCost, footprint`.

## Invariants

- State is plain JSON. `dispatch(state, action) -> {state, error?}` is pure; `apply` mutates only the clone.
- All randomness goes through `rand(state)`, so a seed plus the action list replays exactly (tested
  through founding into the main game). AI runs inside `endTurn` and is deterministic, so it is not
  recorded.
- Only `render.js`, `main.js` and module `ui.js` hooks (through what they are handed) touch the DOM.
  The UI never mutates state; it dispatches actions.
- `runPregame` clones once and runs turns in place; a test pins it equal to dispatching `endTurn`.
- Every rule that acts on an entity should `note(s, id, reason)` so the panel can show why.

## Performance

Headless pre-game to founding (`npm run bench`, 3 civs, WSL2): seed 1 about 0.74 s (148 turns),
seed 2 about 0.33 s (154), seed 3 about 0.57 s (173). Dispatching `endTurn` per turn (the UI's
animated fast-forward) takes roughly twice as long because it clones the state every turn.

## Rules for parallel agents

Four agents build `civ`, `scout`, `army` and `supply` at once, each in its own worktree.

- **You own** `src/modules/<name>/**` and `test/<name>.test.js`. Put all your logic, tuning, UI and
  tests there.
- **Core files** — `src/rules.js`, `src/game.js`, `src/core.js`, `src/clans.js`, `src/data.js`,
  `src/market.js`, `src/mapgen.js`, `src/render.js`, `src/main.js`, `src/views.js`, `index.html`,
  `src/modules/index.js`, `src/modules/ui.js`, `CLAUDE.md` — edit only when the contract truly cannot
  do what you need, in the smallest possible change, in its own commit whose message starts `core:`.
  Prefer adding a hook or helper over changing behaviour, and document it here in the same commit.
- Do not import another module's files, and do not write another module's namespaced fields except
  the agreed cross-module fields above. If you need something from another module that isn't in the
  state shape, read it defensively (it may be missing while that module is a stub).
- Keep `npm test` green and the app loading with no console errors. Keep pre-game behaviour unchanged
  (core tests pin it): your turn steps default to `when: 'main'`.
- Ownership by feature: civ — founding follow-up (cities, civ clans), expansion, materials and pottery,
  projects, civ trading, good/bad years, leanings, joining, food gifts, discoveries, victory.
  scout — scout unit, routes, revealing, trails/bridges, fog. army — captains, zones, spread, detachments,
  skirmish, entrenchment, retreat, forts, raising troops, city capture. supply — unit food, supply lines,
  resupply, forage/pillage, porters, caches, auto-supply/auto-forage.

## Decisions taken where DESIGN.md is open

- One pooled `civ.store` per civ, not a store per city, for the PoC.
- Hills move cost stays 2 (the table says 2-3).
- Founding-market selection is hex-economy's (`foundCivs` in clans.js): most tamed grain, then longest
  held, spaced `TENDING.hearthSpacing` apart. The civ module turns those records into cities and civ
  clans in `onStart`; nothing is converted before the player chooses.
- Fog (`civ.seen`) and hidden-cost knowledge (`civ.revealed`) are separate maps.
- Good and bad years scale regrowth through the `yieldFactor` hook.
