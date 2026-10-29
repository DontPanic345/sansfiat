# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
cargo test   # [profile.dev] opt-level = 3: the engine is too slow unoptimised for whole pre-games
cargo fmt --check && cargo clippy --all-targets -- -D warnings
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large | --radius N]   # the map as ASCII, a legend and counts
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--every 25] [--save game.json] [--json | --events-json] [--no-events]  # stops at the pre-game's end (before the pick), then reports each civ; prints "turn  text" per event
cargo run -p sansfiat -- sweep [seed ...] [--radius N | --size small|medium|large|all] [--turns N] [--every 50]   # seeds default 1..=10, size small; seeds run in parallel; each seed's line, its turn/clans/people every N turns, and per size whether people levelled off over the last 100 turns
cargo run -p sansfiat -- sweep --after 200 --maps retro [--no-army] [--no-gifts] [--lever G]   # the camps and joining: run to the pick with all on the AI, then 200 turns on; one line per civ (its camp, pack, hunger, starvation, line, cut off, foraging, its clans' people) and the totals: civs that kept their first camp, and the clans' people. --no-army is the control (no camp, no AI raising, no levy). Without --maps retro: the seeds and --size as above
cargo run -p sansfiat -- replay game.json [--json | --events-json] [--no-events]
cargo run -p sansfiat -- bench [seed ...] [--turns N] [--radius N]
cargo run -p sansfiat -- clear baskets.json     # or - for stdin
cargo run -p sansfiat -- digest [--seed N] [--radius N] [--turns N] [--lead]   # "turn digest" lines, to compare builds; --lead picks the first civ and gives orders (see lead_orders in cli/src/main.rs, mirrored in cross-build.mjs)
./build-web.sh && python3 -m http.server -d www  # http://localhost:8000
./build-web.sh && ./cross-build.sh              # native and wasm (under Node) give the same digests (small and medium maps; the line `2 16 400 lead` compares them through orders)
./build-web.sh && node www/check.mjs            # the data the page relies on, from the wasm build
./check.sh                                       # every check CI runs, in order
node www/play.mjs "?size=small&seed=2&run=1" --until "document.body.dataset.phase === 'choose'" --shot pick.png   # the page in headless Chrome; import { open } from it to click tiles and buttons
node www/play-camp.mjs [out-dir]   # leads a civ, moves its camp out on its land, ends turns, screenshots the line (default relay/specs/29/screens)
```

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, its `phase`, `Action`, the `TURN` steps, `act`/`run`/`save`/`replay`, and `Event`s. |
| `core/src/data.rs` | Every rule's numbers (`CLAN`, `TRAIL`, `MARKET`, `CIV`, `UNIT` …) and `Config`; tune here first. |
| `core/src/map.rs` | Map generation: terrain, water, closed country, rivers and floods, foods, hidden trail costs, `ascii`. |
| `core/src/land.rs` | Neighbours, move costs (`step_cost`), `cost_map` and `path`, and the `regrow`, `wear` (depletion) and `hold` (trails, tame grain) steps. |
| `core/src/actors.rs` | Actors: `Actor`, `walk_cost`, `Walks` inside a territory, `divide` and the add/drop rule `change`. |
| `core/src/clans.rs` | Clans on the actor: seeding, `reap` and `value`, the `move`, `harvest` and `eat` steps, dividing. |
| `core/src/market.rs` | `clear`: barter exchange ratios by tâtonnement. |
| `core/src/markets.rs` | Markets: reach, the `go` step (one festival route by worth), trading, forming and fading. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, the `end` step and `top_up`. |
| `core/src/civs.rs` | Civs: `found` (the civ's own clan, the capital on its home), the `levy` (called by `trade`), the `civs` step (store upkeep), territory and the pick's `cards`. |
| `core/src/units.rs` | The camp (one per civ, an actor of warriors): raise, disband, the pack, the `line` to the capital and `cut_off`, forage and pillage, the `ai` and `units` steps, the per-civ `Counters`, and the `check_*` that `validate` calls. |
| `core/src/stats.rs` | `summary`, the evidence reports, and `digest` to compare builds. |
| `core/src/hex.rs`, `core/src/rng.rs` | Axial pointy-top hexes; mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | The wasm-bindgen `Sim` the page drives: `layout()` once per map, `view()` each turn, `act`/`check` for orders. |
| `www/` | The page, plain ES modules: `main.js` (controls and `body.dataset` for scripts), `select.js` (the selection panel), `map-view.js` (the canvas, `screenOf(i)`), `feed.js`; `check.mjs` checks its data under Node, `play.mjs` drives it in headless Chrome and `play-camp.mjs` plays a camp through it. The selected camp's line home is drawn in `map-view.js` (`setLine`), coloured in `main.js` (`lineColour`). `www/pkg` is built, not committed. |

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
