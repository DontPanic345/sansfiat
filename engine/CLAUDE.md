# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
./check.sh                                   # every check CI runs, in order
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large]   # the map as ASCII
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--save game.json] [--why ID]   # to the pick unless --turns; --why prints a subject's reasons
cargo run -p sansfiat -- sweep [seed ...] [--size small|medium|large|all] [--turns N]   # the pre-game, seeds in parallel
cargo run -p sansfiat -- sweep --judge [--bands FILE] [--json]   # the retro maps played (army twice, --no-army once) and scored by the judge; reads judge.toml, exits 1 on a fail
cargo run -p sansfiat -- sweep --after 200 --maps retro [--no-army] [--no-gifts] [--no-zone] [--lever G]   # the retro maps past the pick, one line per civ and the totals; the --no-* flags are controls
cargo run -p sansfiat -- replay game.json | bench | digest [--lead] [--state]   # replay a save; time turns; digests to compare builds (--state: the world without its log)
node www/play.mjs "?size=small&seed=2&run=1" --shot pick.png   # the page in headless Chrome; import { open } from it to click
node www/play-<area>.mjs [out-dir]           # leads a civ through one area and screenshots it; one per area in www/
```

`cargo test` runs optimised (`[profile.dev] opt-level = 3`): whole pre-games are too slow otherwise.

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `phase`, `Action`, the `TURN` steps, `act`/`run`/`save`/`replay`, reasons (`World::note`), alerts (`flag_alerts`), `SAVE_VERSION`. |
| `core/src/events.rs` | `Event` and `EventKind`, their wording (`text`), `Event::new` and its setters, and `World::emit`. |
| `core/src/data.rs` | Every rule's numbers and `Config`; tune here first. |
| `core/src/map.rs` | Map generation: terrain, water, closed country, rivers, foods, hidden trail costs. |
| `core/src/land.rs` | Neighbours, move costs, `path`, `steps` (the tile distance), and the `regrow`, `wear` and `tend` steps. |
| `core/src/actors.rs` | `Actor` and what every actor does once: `spoil`, `eat_by_share`, `eat_by_diet`, `stride` (walking a path on labour), `walk_cost`, `Walks`, `divide` and the add/drop rule `change`. |
| `core/src/clans.rs` | Clans on the actor: seeding, `reap`, `value`, and the `move`, `harvest` and `eat` steps. |
| `core/src/market.rs` | `Basket` and its helpers (`total`, `take`, `add`, `sum`); `clear`: barter exchange ratios by tâtonnement. |
| `core/src/markets.rs` | Festivals (markets in code): reach, the `go` step, trading, forming and fading, people moving. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, `top_up`. |
| `core/src/civs.rs` | Civs, each an actor (its store is the civ's): `found`, the `levy`, `keep`, the `civs` step, territory and the pick's `cards`. |
| `core/src/leaning.rs` | Unaligned clans leaning toward and joining a civ: `give` and the `lean` step. |
| `core/src/units/mod.rs` | `Unit` and `UnitState` (its flat JSON), `Counters`, the queries (`camp`, `scout`, `pack_cap`, `leader`), raising and disbanding, `strip` and `supply`, and the `units` step. |
| `core/src/units/line.rs` | What a camp may walk, `path_between`, `route`, the line to the capital (`line`, `cut_off`, `cut_at`) and `turns`. |
| `core/src/units/camp.rs` | `CampState`, the camp's turn (march, `hold`, forage, supply, eat) and its zone (`zone_ok`). |
| `core/src/units/scout.rs` | `ScoutState`, the scout's route search (`route_plan`, trails, reveals) and `scout_turn`. |
| `core/src/script.rs` | `Lead`, the scripted game (`digest --lead`, `Sim::lead_orders`): the engine's second player. |
| `core/src/orders.rs` | `Order` (the wire form of a unit's order), `World::order` and `check_order`: every order's checks and its applying, the player's and the AI's. |
| `core/src/units/ai.rs` | The AI's `post` and `ai`, and its zone (`ai_zone`). |
| `core/src/stats.rs` | `summary`, `standing` (each civ's share of the people), the evidence reports, and `digest` and `state_digest` to compare builds. |
| `core/src/judge.rs` | `Watch`: a game scored turn by turn from its pick (Standing, the leader and lead changes, the spread and swings, what changes hands), and its `Scores` in the five groups. Reads the world, changes nothing. |
| `core/src/hex.rs` | Axial pointy-top hexes. |
| `core/src/rng.rs` | Mulberry32, matching the JS prototypes bit for bit. |
| `core/src/lib.rs` | The crate root: one `pub mod` line per module. |
| `cli/src/main.rs` | The `sansfiat` binary: `USAGE`, the size names, `main` and the dispatch to one module per command. |
| `cli/src/run.rs` | `run` and its `Output` (the report, the JSON or the events), with the events and reasons it prints. |
| `cli/src/map.rs` | `map`: the map as text, with a legend and counts. |
| `cli/src/digest.rs` | `digest`: a turn and its digest per line; `--lead` plays the scripted game, `--state` hashes the world without its log. |
| `cli/src/sweep.rs` | `sweep`: the pre-game over seeds in parallel, its series lines, and the `Run` and `Point` they keep. |
| `cli/src/retro.rs` | `sweep --after`: the retro maps past the pick, each civ's camp and clans, and the controls. `to_pick` and `kept_camp` are shared. |
| `cli/src/judge.rs` | `sweep --judge`: the retro maps judged, the totals, the bands of `judge.toml` checked, the text and `--json` report. |
| `engine/judge.toml` | The judge's bands: `[group.metric]` tables with `fail_min`, `fail_max`, `warn_min`, `warn_max` and a `note`. A piece that moves one says so in its PR. |
| `cli/src/replay.rs` | `replay`: a save's world, printed as `run` prints it. |
| `cli/src/bench.rs` | `bench`: the time per turn for each seed. |
| `cli/src/clear.rs` | `clear`: the barter clearing of a basket file (`basket_of`, `clear_file`). |
| `cli/src/report.rs` | The printers the commands share: the summary line, the centres, the report and the civs' text. |
| `web/src/lib.rs` | The wasm-bindgen `Sim` the page drives: `layout()`, `view()`, `act`/`check`, the order form. |
| `www/index.html` | The page's markup and styles. |
| `www/main.js` | The page's controls and wiring, with `body.dataset` for the scripts. |
| `www/select.js` | The selection panel: one function from a selection to `{title, html, actions}`; `unitOrder` and `civOrder` send the order form. |
| `www/map-view.js` | The map canvas: pan, zoom, picking and the drawing. |
| `www/feed.js` | The event feed: newest first, a filter by kind. |
| `www/check.mjs` | The page's data checks under Node, sending the wire forms through `Sim`; `check.sh` runs it. |
| `www/play.mjs` | Drives the page in headless Chrome: clicks by tile and label, waits on state, screenshots; the play scripts import it. |
| `www/play-camp.mjs` | A camp's line home: moves it out on its own land and screenshots the walk. |
| `www/play-hold.mjs` | A rival camp's line cut: a thicker guard marches onto the nearest rival camp's tile. |
| `www/play-lean.mjs` | A civ's gift: raised above the default, and the clan that leans to it. |
| `www/play-read.mjs` | The reasons and alerts: a clan's and the camp's panels, with Hidden country on. |
| `www/play-scout.mjs` | A scout's route: the scout selected, and its start and end picked by clicking. |
| `www/pkg/` | The wasm build (`build-web.sh`), not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player or an AI does to a unit is an `Order` (`orders.rs`, through `World::order`; the player's go through `Action::Order` and are logged). Anything else the player does is a variant of `Action`, checked in `validate` and applied in `act`. `World::run` takes the player's pick itself (watching), so tests and the CLI that run past the end keep working; a front end that calls `act(EndTurn)` itself must send `ChooseCiv` when `phase` is `choose` (as `cross-build.mjs` does).
4. A module of its own under `core/src/`, tests under `core/tests/`.
5. Shared actor behaviour (spoil, eat, walk) goes in `actors.rs`; a new actor is a new state on it, not new machinery.

## Invariants

- Reasons and alerts are derived: `World.notes` and the alert candidates are `#[serde(skip)]`, so a
  digest and a save don't see them, and a replay rebuilds them. A rule writes a reason only where it
  acts on the subject (a note is one `w.note` call beside the decision), never a per-turn status.
- A change that moves `stats::digest` for a seed bumps `SAVE_VERSION` in `core/src/world.rs` and the
  pinned digests in `cli/tests/save.rs` (`the_save_version_pins_the_rules`) together: a save from the
  old rules would otherwise replay into a different world without a word.
- The world changes only through `act` (and the AI's orders, which a replay re-derives). A `Save` (config and action log) replays to an identical world;
  `replaying_a_save_rebuilds_the_world` holds it to that.
- All randomness goes through `rng::rand` on `World.rng`.
- Floating-point `exp`, `ln`, `powf` and the like go through `libm` (clippy's `disallowed_methods` enforces it), so a save
  replays identically native and in the browser.
- The native and wasm builds give the same `stats::digest` for the same seed after every turn; `./cross-build.sh` checks it in CI.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
