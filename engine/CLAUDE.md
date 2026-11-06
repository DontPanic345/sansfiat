# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions, with a CLI and a wasm page as its two front ends. Build what `docs/design/` says.

```
./check.sh                                   # every check CI runs, in order
cargo run -p sansfiat -- map | run | sweep | play | story | judge | replay | bench | digest   # USAGE in cli/src/main.rs
cargo run -p sansfiat -- sweep --judge [--json] [--solo]   # the retro maps scored beside their controls: a report
cargo run -p sansfiat -- play --game g.json [--seed N --size NAME [--civ K]]   # a line at a time; g.lines.jsonl records it
cargo run -p sansfiat -- judge g.json [--json]   # a played game beside the same map on the AI
node www/play.mjs "?size=small&seed=2&run=1" --shot pick.png   # the page in headless Chrome
node www/play/all.mjs                        # every touch-point script; check.sh runs it
```

`cargo test` runs optimised: whole pre-games are too slow otherwise.

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `phase`, `Action`, the `TURN` steps, `act`/`run`/`save`/`replay`, reasons, alerts, `SAVE_VERSION`. |
| `core/src/events.rs` | `Event`, `EventKind` and their wording. |
| `core/src/data.rs` | Every rule's number and `Config`; tune here first. |
| `core/src/map.rs` | Map generation: terrain, water, closed country, rivers, foods, hidden trail costs. |
| `core/src/land.rs` | Neighbours, move costs, `path`, `steps`, and the `regrow`, `wear` and `tend` steps. |
| `core/src/actors.rs` | What every actor does: spoil, eat, `stride`, `walk_cost`, traffic, `divide`, `change`. |
| `core/src/clans.rs` | Clans: seeding, `reap`, `value`, and the `move`, `harvest` and `eat` steps. |
| `core/src/market.rs`, `markets.rs` | Barter clearing; festivals: reach, the `go` step, trading, forming, fading. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, `top_up`. |
| `core/src/claim.rs` | A civ's claim, its guard's division, `held`, `tile_civ` and the `claim` step. |
| `core/src/civs.rs` | Civs: `found`, the levy, `keep`, the `civs` step and the pick's `cards`. |
| `core/src/leaning.rs` | Leanings: fading, land, gifts, joining and leaving. |
| `core/src/units/` | `line.rs` (paths, the line home, cuts), `camp.rs` (an army's turn and zone), `guard.rs`, `scout.rs` (the route search), `ai.rs` (the AI's orders), `fight.rs`. |
| `core/src/orders.rs` | `Order`, `World::order` and `check_order`: the player's and the AI's. |
| `core/src/script.rs` | `Lead`, the scripted game: the engine's second player. |
| `core/src/stage.rs`, `maptext.rs` | `Stage`: a world on a known map (the `map` text, or a retro pick, cached), layers, placements, orders and reads; `StageSpec` is its data form. `maptext.rs` reads the map's text (`map::parse`). A staged world is not a save. |
| `core/src/stats.rs` | `summary`, `standing`, the evidence reports, `digest` and `state_digest`. |
| `core/src/judge.rs` | `Watch`: a game scored from its pick in the score groups. Reads the world only. |
| `core/src/hex.rs`, `rng.rs` | Axial hexes; Mulberry32, matching the JS prototypes bit for bit. |
| `cli/src/` | The `sansfiat` binary, one module per command; `judge.rs` (the sweep's judge and the file's), `view.rs` (the text view, under 80 lines), `story.rs` (the scripted stories of war). |
| `web/src/lib.rs` | The wasm `Sim` the page drives: `layout`, `view`, `act`/`check`, the order form. |
| `www/` | The page: `main.js` (the router), `app.js`, `screens/`, `bar.js`, `panel.js` and `panels.js` (a selection's shape and its buttons), `model.js` (the view model), `map/` (`MapView` and one file per layer), `feed.js`, `save.js`, `sound.js`, `atlas.js` and `assets/`. |
| `www/check.mjs` | The page's data checks under Node, through `Sim`. |
| `www/play.mjs` | Drives the page in headless Chrome: clicks by tile and label, waits, screenshots, `panel()`, `stepper()`. |
| `www/play/` | One script per touch point of `docs/design/page.md` (start, pick, bar, save, end, art, views, camp, scout, civ, select, feed, army, layout), each exiting non-zero when its touch point is missing; `all.mjs` runs them. |

## Adding a mechanic

1. Its state is a field on `World`, built in `World::new`; its per-turn work is `Step`s in `TURN`.
2. What the player or an AI does to a unit is an `Order`; anything else the player does is an
   `Action`, checked in `validate` and applied in `act`. `World::run` takes the pick itself; a
   front end that calls `act(EndTurn)` must send `ChooseCiv` when `phase` is `choose`.
3. A module of its own under `core/src/`, tests under `core/tests/`. Shared actor behaviour goes in
   `actors.rs`; a unit kind is a `UnitKind` and a `UnitState` variant. A kind with no state of its
   own (the guard) takes no order but raising and disbanding, and its turn is one arm in
   `units::step` (`units/mod.rs`).

## Invariants

- Reasons and alerts are derived (`#[serde(skip)]`): a rule writes one `w.note` where it acts.
- A change that moves `stats::digest` bumps `SAVE_VERSION` and the pins in `cli/tests/save.rs`.
- The world changes only through `act` and the AI's orders; a save replays to an identical world.
- All randomness goes through `rng::rand`; `exp`, `ln`, `powf` go through `libm`, so native and wasm
  give the same digest every turn (`./cross-build.sh`).
- `clear` conserves every good exactly.
- `wasm-bindgen` is pinned to the installed `wasm-bindgen-cli`.
- This file stays under 8 KB; `check.sh` holds it there.
