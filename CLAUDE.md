# Sans Fiat

Proof-of-concept hex 4X with no money: goods are physical and worth what people will trade for them.
A simulated pre-game (hex-economy) grows clans and markets; the best markets become civs; the player
leads one. Plain ES modules, no build step, no dependencies. Spec: `docs/DESIGN.md` (vision and the
built / partial / not built status of every mechanic), with exact rules and numbers in
`docs/mechanics/economy.md` and `docs/mechanics/military.md`. Keep those in step when rules change.
Original design notes: `docs/notes/`.

```
npm start          # http://localhost:8000  (?seed=N&players=N, players 2-4, default 3)
npm test           # node --test (all of test/*.test.js)
npm run sim -- --seed 3 --players 3 [--turns 500] [--main 20] [--every 25] [--json]
npm run bench      # time the headless pre-game for seeds 1-3 (or: npm run bench -- 7 8)
```

`sim` runs the pre-game, founds the civs with every civ on the AI (`chooseCiv {civ: null}`), then
`--main` turns of the main game. In the browser console: `game.state`, `game.act({...})`,
`game.history`, `game.replay()`, and `game.ui` (below).

## Layout

| File | What |
| --- | --- |
| `src/data.js` | Core tuning: goods, terrain, `TRAILS`, `MAP`, `CLAN`, `MARKET`, `TENDING`, `PREGAME`, `FOUNDING`, `PLAYERS`, `CITY`, `NOTES`, `CIVS`. |
| `src/core.js` | Shared helpers for every module (below). Imports nothing from rules, clans or modules. |
| `src/clans.js` | The clan rules (forage, trade, settle, occupy, markets), founding sites and `foundCivs`. Runs for every clan in the main game too. |
| `src/market.js` | `clear(baskets, startPrices, {wants})` -> `{prices, after, volume, cleared}`; `cleared` false with < 2 traders or goods. |
| `src/rules.js` | `PHASES`, the turn pipeline, the merged action table, `dispatch`, AI runner. Wires in the modules. |
| `src/game.js` | `newGame(config)`, `replay(config, actions)`, `runPregame(s, n)`, `runPregameCounted`. |
| `src/mapgen.js` | Height noise, inlets, inland seas, ridges with flanks and passes, deserts, rivers, coast, fertility, hidden trail costs. |
| `src/hex.js`, `src/rng.js` | Axial pointy-top hexes; mulberry32 `rand(state)`, `weighted`. |
| `src/render.js`, `src/main.js`, `src/views.js` | Canvas (`draw`, `fogOf`), controls and side panel, debug map views (`viewValues`). The only DOM code besides module `ui.js`. |
| `src/modules/index.js` | Rules registry: `MODULES = [civ, scout, army, supply]`. |
| `src/modules/ui.js` | UI registry: each module's `ui.js`, sorted by `z` (civ 10, supply 15, army 20, scout 90). |
| `src/modules/<name>/{rules,data,ui}.js` | One module. |
| `tools/` | `serve.js`, `sim.js`, `bench.js`. |
| `test/<name>.test.js` | Per module; `rules.test.js` (clans, markets, founding), `core.test.js` (core, registry), `shell.test.js` (UI shell), `integration.test.js` (full games). |

## Module registry contract

Each module is `src/modules/<name>/`:

- **`rules.js`** exports
  - `name` — the module name.
  - `init(s)` — called from `newGame` after the map and seed clans exist. Create `s.mods.<name>`.
  - `onStart(s)` — called once by `chooseCiv` when the main game begins, in registry order
    (civ, scout, army, supply). `s.civs` exists; the human civ has `isHuman: true` (none when watching).
  - `actions` — `type -> { validate(s, a) -> error|null, apply(s, a), candidates(s, civId) -> [action] }`.
    Every module action carries `civ` (the acting civ id). Core rejects it unless `s.phase === 'main'`
    and `s.civs[a.civ]` exists, before your `validate` runs. Type names are global: a clash throws at load.
    Player actions apply immediately (between turns); `apply` mutates the clone `dispatch` hands it.
  - `turnSteps` — `[{ phase, name, run(s, ctx), order?: 'before'|'after', when?: 'main'|'always' }]`.
    `order` places the step before or after the core step of that phase (default after);
    `when` defaults to `'main'` (pre-game steps are core's). Unknown phases throw at load.
  - `hooks` (optional) — seams into the clan rules: `daughterSite(s, clan, {options, pop})` returns a
    tile key from `options` (where a splitting clan's daughter goes), `null` (she stays), or
    `undefined` (let the clan choose); first module with an answer wins. `yieldFactor(s, tile)`
    scales regrowth (good and bad years); factors multiply.
  - `ai(s, civId) -> [action]` (optional) — core applies each valid one in the `orders` phase for every
    non-human civ, in place. Must be deterministic in state (use `rand(s)`), so replays reproduce it.
- **`data.js`** — the module's tuning. Import only from `src/data.js` here.
- **`ui.js`** exports (all optional) `z`, `drawLayer(ctx, s, view)`, `panel(s, selection, ui) -> HTML`,
  `onClick(s, hex, ui) -> consumed?`. `view` = `{size, at(hex) -> {x, y}, hexPath(ctx, center, size), selected, mode,
  viewer, locals}`. `ui` = `{act(action) -> ok, local, selection}`. In panel HTML, any element with
  `data-action='{"type":"...","civ":N,...}'` dispatches that JSON when clicked.

Imports: a module's `rules.js` imports `../../core.js`, `../../clans.js`, `../../data.js`, `../../hex.js`,
`../../rng.js`, `../../market.js` and its own files. Never `../../rules.js` (cycle), and never another
module's rules: modules talk only through the documented state below. One exception, UI only:
`army/ui.js` reads supply's helpers (`supplyLine`, `foodOf`, `eatRate`, `gatherError`, actions) for the captain panel.

### UI contract (`src/main.js`)

1. A map click sets the selection (`sel`, the clicked tile) first, then offers the click to each module's
   `onClick` in z order until one returns true.
2. With a human civ, `act()` refuses any action whose `civ` is not the human's (except `chooseCiv`, `endTurn`).
   Module UIs select and command only the human's own units.
3. Escape clears the selection and empties every module's `local` in place.
4. Event lines starting `!` (after `Turn N: `) are alerts for the human civ: kept in an alerts list and highlighted.
   Use them for things that concern the human (fights, hunger, captures, lapsed choices, droughts).
5. In the main game with a human civ, panels and map views show only what it has seen: `fogOf(s)` (render.js)
   returns `{civ, seen(k), revealed(k)}` or null (pre-game, founding, watch mode); `viewValues(s, mode, fog)` filters
   debug views. Unseen tiles show terrain only.
6. Fast-forward runs the pre-game to founding, or 10 turns (`FAST_TURNS`) in the main game; all-AI watch runs to a win.
7. `window.game.ui` drives the UI without pixels: `select`/`click(hex)`, `escape`, `selection`, `locals`, `step(n)`,
   `fast`, `stop`, `running`, `alerts`, `log`, `setMode`.

## Phases

`PHASES` in `src/rules.js`. One `endTurn` runs every phase in order; in each, module `before` steps,
then core, then module `after` steps (modules in registry order). Module steps run in the main game only.

| Phase | Core step (pre-game and main) | Module steps |
| --- | --- | --- |
| `regrow` | tiles regrow (skips `fallow` tiles, applies `yieldFactor`) | civ `before`: good/bad years |
| `forage` | clans move, adjust land, decide to trade, harvest (less `clan.drafted`) | civ `before`: draft; civ: labour (projects, craft, hunt, farm, materials) |
| `trade` | each market clears clan baskets plus `m.posts` | civ `before`: post (+ clans' materials); civ: collect, trade leaning; scout: knowledge share |
| `settle` | clans eat, grow, starve, divide, split | civ: spoilage, levy, cap, site expiry, new cities |
| `occupy` | held grain tames, abandoned grain goes feral | |
| `markets` | unowned markets fade (`activity` < `gathering.minVolume`); markets form | civ: leaning decay, joining |
| `orders` | (main) AI civs' actions from each module's `ai` | scout: routes; supply: porters |
| `armies` | — | army: spread, skirmish, hold, capture; supply: eat, auto-resupply, attrition, caches |
| `discoveries` | — | civ: discoveries, victory; scout: fog |

After the phases core records stats, refills every unit's `labour` to `labourMax`, sweeps notes of
entities that are gone, ends the pre-game when `pregameOver`, and increments `s.turn`.

**Pre-game end** (`pregameOver`): `s.turn >= PREGAME.maxTurns`, or from `PREGAME.minTurns` once
`foundingSites(s)` returns one site per player: thriving markets (`FOUNDING.minTraders`, `minActivity`) with a
domesticated hearth in reach, `FOUNDING.spacing` apart, hearths `TENDING.hearthSpacing` apart, all on one landmass
(`landComponents`). `foundCivs` then uses `foundingSites(s, {fallback: true})` and sets `phase = 'choose'`.

## State shape

Plain JSON. Every entity id comes from `s.nextId` (`newId(s)`), so ids are unique across kinds.

```
s.config   {seed, players, width, height}
s.rng      RNG state; only rand(s) touches it
s.turn, s.phase   'pregame' -> 'choose' -> 'main';  s.winner  civ id once the game is won
s.events   this dispatch's log lines (event(s, text)); a leading '!' marks an alert
s.notes    {entityId: [{turn, text}]}  the "why" log, last NOTES.perSubject per entity (note(s, id, text))
s.mods     {civ, scout, army, supply}  each module's own global state (below)
s.stats    {pop, clans, volume} per turn
s.map      {width, height, tiles: {"q,r": tile}}
s.clans    {id: clan}   s.markets {id: market}   s.civs {id: civ}   s.cities {id: city}   s.units {id: unit}
```

- **tile**: `q, r, terrain, fertility, river, coast, shallows, flow?, cap, stock, wildGrain?, tame?,
  occ, occTotal`; `claim` (clan id holding it, or null); `trailCost` (hidden: cost once trailed,
  null = never; forest/hills/mountain only); `span` (mountains: bridge span needed); `trail` (built,
  shared by everyone; scout), `trailWork` (labour put into an unfinished trail); `fort` (null |
  `{owner, garrison}`; army); `cache` (null | `{owner, food}`; supply; changes owner with the tile's territory);
  `fallow` (turns left yielding nothing; supply sets, core counts down); `held` (civ id with the most troops
  standing on it, or null; army keeps it current).
- **clan**: hex-economy fields (`pop, land, store, harvest, basket, fed, starving, fedRatio, hungry,
  route, dest, market, trip, since, trail`...) plus `owner` (civ id or null = unaligned), `leaning`
  (`{civId: number}`, civ) and `drafted` (people working for the civ this turn; civ sets it before `forage`).
  Daughters inherit `owner` and `leaning`.
- **market**: `owner` (civ id for a founding market; never fades), `temporary` (pre-game gathering), `prices`,
  `activity` (smoothed volume, fades the market), `volume, lastVolume, lastTrade, traders, lastTraders`, `history`,
  `posts` (`{traderId: basket}`, written before `trade`), `received` (`{traderId: basket}` after it).
- **civ** (created by `foundCivs`): `id, name, color, isHuman, marketId, cityIds, store` (pooled goods by
  name; use `takeGoods`/`putGoods`), `seen` (`{"q,r": true}` fog; scout), `revealed` (`{"q,r": true}`
  hidden trail costs known; scout), `founded`.
- **city** (civ module creates them): `id, owner, name, q, r, founded, founding` (capital), `buildings`
  (`{granary, storehouse, wall}`), `project` (`{kind, labour}` | null), `garrison` (army, default `ARMY.garrison`).
  Capture goes through `transferCity`.
- **unit**: core fields `id, owner, type, q, r, labour, labourMax` (`spawnUnit`, `removeUnit`). Module fields
  live under the module's name. Types and their fields:
  - `scout`: `u.scout = {route: {from, to, walk?, pair?, walking?, cost?} | null, tried: [keys]}`.
  - `captain`: `u.army = {troops, zone: [keys], foughtTurn?, foughtTroops?}` (army); `u.supply = {food, foodMax, auto}`
    and `u.hungry` (supply).
  - `detachment`: `u.army = {captainId, troops, entr, cutSince?}` (army); `u.hungry` (supply).
  - `porter`: `u.supply = {food, foodMax, route: {cityId, to, target, for?} | null, returning?}` (supply).

  Cross-module reads: supply reads troops from `u.army.troops` and `foughtTroops`; army halves power and
  forbids attacks when `u.hungry` / `u.supply.hungry` (own or captain's); army multiplies defenders by
  `ARMY.wallMult` when `city.buildings.wall`; civ reads city ownership for victory.

Module globals:

- `s.mods.civ`: `regions, years` (`{region: {factor, left}}`), `siteChoice` and `citySite` (`{civId: {k, turn}}`),
  `posting` (`{civId: {good: keep}}`), `trade` (`{civId: {turn, posted, received, at}}`), `labour`
  (`{civId: {wanted, drafted, needs}}`), `gifted` (`{clanId: {civId: {turn, amount}}}`), `disc`
  (`{civId: {practice, known}}`), `startTurn, startPop` (`{civId: people}`), `lost` (`{civId: turn}`),
  `cityOwners, overflow, winKind` ('conquest' | 'prosperity'), `winTurn`.
- `s.mods.scout`: `roads` (`{civId: [pairKey]}` AI road routes done or given up).
- `s.mods.army`: `fights, captures, raised, raisedTurn, peak`.
- `s.mods.supply`: `starved, starvedBy`.

## Actions

| Module | Actions |
| --- | --- |
| core | `endTurn`, `chooseCiv {civ | null}` |
| civ | `setProject {city, project}`, `giftFood {clan, amount}`, `setDaughterSite {site: "q,r" | null}`, `setCitySite {site | null}`, `setPosting {good, keep | null}` |
| scout | `buildScout {city?}`, `assignRoute {unit, from, to, walk?, pair?}`, `cancelRoute {unit}` |
| army | `raiseArmy {city}`, `moveCaptain {unit, q, r}`, `paint`, `unpaint {unit, q, r}`, `undeploy {unit}`, `disband {unit}`, `promote {unit}`, `buildFort {unit}` |
| supply | `resupply`, `autoSupply {unit, on}`, `forage`, `pillage {unit}`, `buildPorter {city}`, `loadPorter`, `unloadPorter {unit}`, `setPorterRoute {unit, city?, to, target?, for?}`, `clearPorterRoute {unit}` |

## Core helpers (`src/core.js`)

`newId, event, note, notesFor, sweepNotes, total, addGoods, scaleGoods, takeGoods, putGoods,
tileAt, passable, stepCost, costMap, pathCost, clanAt, marketAt, cityAt, unitsAt, holder, totalPop,
civOf, territoryOf, transferCity, civClans, civCities, civUnits, humanCiv, spawnUnit, removeUnit`.

- `stepCost(s, h)`: labour to enter h, trail-aware (a built trail drops it to `trailCost` and opens
  mountains); Infinity if impassable. Ignores what a civ knows; scout owns believed costs (`plannedCost`).
- `pathCost(s, from, to, {cost, maxCost})` -> `{cost, path}` or null; `costMap(s, from, maxCost, {cost})`
  -> Map of key -> cost. Pass your own `cost(s, h)` to block enemy land, use known costs, and so on.
- `territoryOf(s, h)`: `tile.held`, else fort owner, else the nearest city within `CITY.territoryRadius`.
  Supply's `landOwner` (fort, city, else the claiming clan's civ, ignoring troops) decides forage vs pillage.
- `civOf(s, x)`: the civ of a clan, unit, city, market, or any entity id.

From `src/clans.js` modules may use: `spawnClan, siteOptions, pickSite, marketReach, reaches, nearestMarket,
range, forageTiles, planHarvest, nutrition, catchment, foundMarket, hearths, roamCost, footprint,
landComponents, foundingSites`. (`removeMarket` is internal; markets disappear only by fading or at founding.)

## Invariants

- State is plain JSON. `dispatch(state, action) -> {state, error?}` is pure; `apply` mutates only the clone.
- All randomness goes through `rand(state)`, so a seed plus the action list replays exactly (tested
  through founding into the main game). AI runs inside `endTurn` and is deterministic, so it is not recorded.
- Only `render.js`, `main.js` and module `ui.js` hooks (through what they are handed) touch the DOM.
  The UI never mutates state; it dispatches actions.
- `runPregame` clones once and runs turns in place; a test pins it equal to dispatching `endTurn`.
- Every rule that acts on an entity should `note(s, id, reason)` so the panel can show why.

## Performance

Headless pre-game to founding (`npm run bench`, 3 civs, WSL2): seed 1 about 1.03 s (172 turns),
seed 2 about 0.54 s (186), seed 3 about 0.84 s (197). Dispatching `endTurn` per turn (the UI's
animated fast-forward) takes roughly twice as long because it clones the state every turn.

## Working on the code

- **Module ownership.** civ — founding follow-up (cities, civ clans), drafted labour, materials and pottery,
  projects, store and posting, expansion and sites, good/bad years, leanings, gifts, joining, discoveries,
  victory. scout — scout unit, routes, revealing, trails/bridges, fog, knowledge spread, AI roads.
  army — captains, zones, spread, detachments, skirmish, entrenchment, retreat, forts, raising, disbanding,
  garrisons, city capture. supply — unit food, supply lines, resupply, forage/pillage, porters, caches, auto-supply.
  Put a feature's logic, tuning, UI and tests in its module and `test/<name>.test.js`.
- **Core files** (`src/rules.js`, `game.js`, `core.js`, `clans.js`, `data.js`, `market.js`, `mapgen.js`, `render.js`,
  `main.js`, `views.js`, `index.html`, `src/modules/index.js`, `src/modules/ui.js`, this file): change only when the
  contract cannot do what you need, in the smallest change, in a commit whose message starts `core:`. Prefer a hook or
  helper over changing behaviour, and document it here in the same commit.
- Do not write another module's namespaced fields except the agreed cross-module fields above; read other
  modules' state defensively.
- Keep `npm test` green and the app loading with no console errors. Keep pre-game behaviour unchanged unless that is
  the point of the change (core tests pin it); module turn steps default to `when: 'main'`.

## Decisions taken where DESIGN.md was open

Recorded in full in `docs/DESIGN.md` (look for **Decision**). The ones that shape code most: one pooled `civ.store`
per civ; labour is drafted people (`clan.drafted`); founding sites share a landmass and need a domesticated hearth;
prosperity = people now / people at founding; opaque fog with separate `seen` and `revealed`; good and bad years
through the `yieldFactor` hook; hills cost 2.
