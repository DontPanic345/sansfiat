# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
cargo test   # [profile.dev] opt-level = 3: the engine is too slow unoptimised for whole pre-games
cargo fmt --check && cargo clippy --all-targets -- -D warnings
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large | --radius N]   # the map as ASCII, a legend and counts
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--every 25] [--save game.json] [--json | --events-json] [--no-events]  # stops at the pre-game's end (before the pick), then reports each civ; prints "turn  text" per event
cargo run -p sansfiat -- sweep [seed ...] [--radius N | --size small|medium|large|all] [--turns N]   # seeds default 1..=10, size small; seeds run in parallel
cargo run -p sansfiat -- replay game.json [--json | --events-json] [--no-events]
cargo run -p sansfiat -- bench [seed ...] [--turns N] [--radius N]
cargo run -p sansfiat -- clear baskets.json     # or - for stdin
cargo run -p sansfiat -- digest [--seed N] [--radius N] [--turns N] [--lead]   # "turn digest" lines, to compare builds; --lead picks the first civ and gives orders (see lead_orders in cli/src/main.rs, mirrored in cross-build.mjs)
./build-web.sh && python3 -m http.server -d www  # http://localhost:8000
./build-web.sh && ./cross-build.sh              # native and wasm (under Node) give the same digests (small and medium maps; the line `2 16 400 lead` compares them through orders)
./build-web.sh && node www/check.mjs            # the data the page relies on, from the wasm build
```

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World` (with `phase`: `Pregame`, `Choose` once founding has made the civs, `Main` after `ChooseCiv`; `player`; `civs`), `Action` (`EndTurn`, refused in `Choose`; `ChooseCiv { civ }`, only in `Choose`; `MakeUnit`, `MoveUnit { unit, to }`, `DisbandUnit { unit }`, all in `Main` for the player's civ, checked by `units::check_*`), `TURN` (… `eat`, `civs`, `units`, `hold` …), `act`, `run` (watches at the pick), `save`, `replay`; `Event` and `EventKind` (`World.events`, pushed by the steps where things happen, not saved and not in the digest, rebuilt by replay; `Event::text` is the one wording). |
| `core/src/data.rs` | Terrain, foods (the goods), `CLAN`, `TRAIL`, `TENDING`, `MARKET`, `PREGAME`, `CIV` (`CIVS` names and colours) and `UNIT` rules, `Config`. Tune here first. |
| `core/src/map.rs` | The map: noise terrain in a hex, sea at the rim, inlets, inland seas and lakes, ridges, rain-shadow deserts, coast, patchy fertility, closed country (`close_country`: valleys behind a pass, sealed valleys, peninsulas on a neck, tagged by `Zone`) with `open_sites` and its re-rolls, rivers down a distance-to-water field (`carve_rivers`; lakes fed and drained), floodplains and deltas (`flood`); `finish_caps` turns terrain, river and fertility into food; hidden trail costs from their own rng stream; `ascii` draws it. |
| `core/src/land.rs` | Neighbours, `passable`, move costs (`step_cost(from, to)`: entering a river off its course costs `RIVER.cross`, a trail bridges it), bounded Dijkstra (`cost_map`), the cheapest route (`path(from, to, allowed)`: the tiles after `from` and the cost, same tie-breaks as `cost_map`), the `regrow` step, and the `hold` step: held counters, trails, taming and hearths. |
| `core/src/round.rs` | A clan's round: leg and loop costs (`Legs` caches them per home), cheapest insertion, free edge, middle, territory, what it can harvest. |
| `core/src/clans.rs` | Clans: seeding (never in a sealed valley), a round's `value`, the `move` (`adjust`: moving on, one round change, home follows the middle, the limit), `harvest` and `eat` steps; dividing and daughters after eating. |
| `core/src/market.rs` | `clear(baskets, start_prices, params)`: barter exchange ratios by tâtonnement, no numeraire. A library `markets` calls. |
| `core/src/markets.rs` | Markets: `reach`, the `go` (worth the walk?), `trade` (clearing) and `markets` (fade, then form one) steps. |
| `core/src/pregame.rs` | The pre-game's end: centres (age, traders, a hearth in reach, far apart), land cover, and the `end` step. At the turn limit with fewer than three centres it tops up (`top_up`): existing markets, then the most tended land, then any grain land, all in one region of passable land (`End.topped_up` lists the ids added). |
| `core/src/civs.rs` | Civs: `found` (called by `check_end` once: a civ and capital on each centre's market, in `End.centres` order named `CIVS`; clans with a round tile in the market's reach join the cheapest, a civ with none takes the nearest unaligned clan; each civ's starting warrior, phase `Choose`), the `civs` step (`upkeep`, after `eat`: spoil, levy from fed civ clans, feed units, cap at `CIV.store_cap`), `territory`/`tile_civ`/`land`/`work_tiles` (the capital's land within `CIV.territory`, else the civ of the clan holding the tile), and `cards` for the pick. `Clan.civ` is inherited by daughters. A civ's market never fades. |
| `core/src/units.rs` | Units: `Unit` (civ, `tile`, `dest`, `banked` labour, `hungry` turns), the one kind `Warrior` (moves, eats, disbands; holds no land, harvests nothing, blocks no one, any number to a tile). `make` (a person from the near civ clan with the most people, 2+, `UNIT.min_clan`; at the capital; costs no food) and `make_first` (the starting warrior `civs::found` makes), `disband` (person to the civ clan with the nearest home, else lost), `feed` (called by the `civs` step between the levy and the cap: `UNIT.food` from the store in proportion to the goods; hungry `UNIT.hungry_turns` in a row disbands with `UnitStarved`), and the `units` step `march` (each unit with a `dest`, in id order, recomputes its cheapest route by `step_cost` over land and spends `banked + UNIT.labour` on steps; what is left is banked towards a step dearer than a turn's labour). `route(unit)` and `turns(path, from, banked)` serve the page: `turns` and `march` share one stepping function (`walk`). Hidden `trail_cost`/`span` are never read. `check_make/move/disband` are what `validate` calls. |
| `core/src/stats.rs` | `summary` (counts off the world), `centre_report` (fertility near each centre vs all land), `distances`: the evidence the CLI and the web page show; `digest` (FNV-1a of the world JSON) to compare builds. |
| `core/src/hex.rs`, `core/src/rng.rs` | Axial pointy-top hexes; mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | wasm-bindgen `Sim` (`new`, `load`, `act`, `check(action)` (`""` or why not, without applying), `phase`, `pick` (the cards), `route(unit)` and `path(from, to)` (`{path, cost, turns}` or `null`), `tile_civ(i)` (-1 for none), `ended`, `layout`, `view`, `tile`, `events(from)`, `state`, `summary`, `save`, `digest`) and `terrain`, `goods`, `rules` (also `unit_labour`, `unit_food`, `hungry_turns`, `store_cap`, `territory`), `sizes`. `layout()` is the static map as columns (once per map) and `view()` is what changes each turn (`phase`, `player`, clans, markets, trails, tame, claims, `civs` and `units` keyed by id, `clan_civ` `[clan, civ]`, summary, `end`; each civ has `people`, `clans`, `territory`, `work`); `state()` is the whole world, for save and debug. |
| `www/` | The full-screen page, plain ES modules, no bundler. `index.html` (top bar, map, event feed, legend, tooltip), `main.js` (controls, play loop that stops at the pick, the pick card, the civ group of the top bar, orders, URL parameters `?size=&seed=&play=1&speed=slow\|normal\|fast\|max&run=1`; `body.dataset.turn/phase (pregame\|choose\|main)/centres/civ/units/selected (unit:40, capital:<civ id>, tile:812)` for headless scripts), `select.js` (the selection panel: `describe(ctx, selection)` gives `{title, html, actions: [{label, action}]}` and main.js greys each button by `Sim.check`; add an action by adding an entry; `candidates` is what a left-click cycles through: the player's units, capitals, other units, the tile), `map-view.js` (canvas: pan and zoom, picking (`onPick` on left release, `onCommand` on right release, no context menu), `screenOf(i)` client coordinates of a tile, a cached terrain layer with rivers, floodplains, closed-country outlines and marks, the per-turn overlay), `feed.js` (events by cursor into `Sim.events`, filter, 300-line cap). `check.mjs` checks under Node the data the page relies on (not in CI yet: the engine job needs a `node www/check.mjs` step after `./cross-build.sh`). `window.game` is read-only (`view`, `layout`, `save`, `map`, `feed`): every order goes through a button or a right-click, so headless scripts click on `map.screenOf(i)`. `www/pkg` is built, not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player does is a variant of `Action`, checked in `validate` and applied in `act`. `World::run` takes the player's pick itself (watching), so tests and the CLI that run past the end keep working; a front end that calls `act(EndTurn)` itself must send `ChooseCiv` when `phase` is `choose` (as `cross-build.mjs` does).
4. A module of its own under `core/src/`, tests under `core/tests/`.

## Invariants

- The world changes only through `act`. A `Save` (config and action log) replays to an identical world;
  `replaying_a_save_rebuilds_the_world` holds it to that.
- All randomness goes through `rng::rand` on `World.rng`.
- Floating-point `exp`, `ln`, `powf` and the like go through `libm` (clippy's `disallowed_methods` enforces it), so a save
  replays identically native and in the browser.
- The native and wasm builds give the same `stats::digest` for the same seed after every turn; `./cross-build.sh` checks it in CI.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
