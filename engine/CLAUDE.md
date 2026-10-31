# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
./check.sh                                   # every check CI runs, in order
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large]   # the map as ASCII
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--save game.json] [--why ID]   # to the pick unless --turns; --why prints a subject's reasons
cargo run -p sansfiat -- sweep [seed ...] [--size small|medium|large|all] [--turns N]   # the pre-game, seeds in parallel
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
| `core/src/actors.rs` | `Actor`, `walk_cost`, `Walks`, `divide` and the add/drop rule `change`. |
| `core/src/clans.rs` | Clans on the actor: seeding, `reap`, `value`, and the `move`, `harvest` and `eat` steps. |
| `core/src/market.rs` | `Basket` and its helpers (`total`, `take`, `add`, `sum`); `clear`: barter exchange ratios by tâtonnement. |
| `core/src/markets.rs` | Festivals (markets in code): reach, the `go` step, trading, forming and fading, people moving. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, `top_up`. |
| `core/src/civs.rs` | Civs: `found`, the `levy`, `keep`, the `civs` step, territory and the pick's `cards`. |
| `core/src/leaning.rs` | Unaligned clans leaning toward and joining a civ: `give` and the `lean` step. |
| `core/src/units/mod.rs` | `Unit` and `UnitState` (its flat JSON), `Counters`, the queries (`camp`, `scout`, `pack_cap`, `leader`), raising and disbanding, `strip` and `supply`, and the `units` step. |
| `core/src/units/line.rs` | What a camp may walk, `path_between`, `route`, the line to the capital (`line`, `cut_off`, `cut_at`) and `turns`. |
| `core/src/units/camp.rs` | `CampState`, the camp's turn (march, `hold`, forage, supply, eat), its orders (`check_*`, `order_move`, `set_zone`, `with_camp`) and its zone. |
| `core/src/units/scout.rs` | `ScoutState`, the scout's route search (`route_plan`, trails, reveals), `scout_turn` and the route orders. |
| `core/src/units/ai.rs` | The AI's `post` and `ai`, and its zone (`ai_zone`). |
| `core/src/stats.rs` | `summary`, the evidence reports, and `digest` and `state_digest` to compare builds. |
| `core/src/hex.rs`, `rng.rs` | Axial pointy-top hexes; mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | The wasm-bindgen `Sim` the page drives: `layout()`, `view()`, `act`/`check`. |
| `www/` | The page, plain ES modules: `main.js` (controls, `body.dataset` for scripts), `select.js` (the selection panel), `map-view.js` (the canvas), `feed.js` (events and alerts); `check.mjs` and the play scripts. `www/pkg` is built, not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player does is a variant of `Action`, checked in `validate` and applied in `act`. `World::run` takes the player's pick itself (watching), so tests and the CLI that run past the end keep working; a front end that calls `act(EndTurn)` itself must send `ChooseCiv` when `phase` is `choose` (as `cross-build.mjs` does).
4. A module of its own under `core/src/`, tests under `core/tests/`.

## Invariants

- Reasons and alerts are derived: `World.notes` and the alert candidates are `#[serde(skip)]`, so a
  digest and a save don't see them, and a replay rebuilds them. A rule writes a reason only where it
  acts on the subject (a note is one `w.note` call beside the decision), never a per-turn status.
- A change that moves `stats::digest` for a seed bumps `SAVE_VERSION` in `core/src/world.rs` and the
  pinned digests in `cli/tests/save.rs` (`the_save_version_pins_the_rules`) together: a save from the
  old rules would otherwise replay into a different world without a word.
- The world changes only through `act`. A `Save` (config and action log) replays to an identical world;
  `replaying_a_save_rebuilds_the_world` holds it to that.
- All randomness goes through `rng::rand` on `World.rng`.
- Floating-point `exp`, `ln`, `powf` and the like go through `libm` (clippy's `disallowed_methods` enforces it), so a save
  replays identically native and in the browser.
- The native and wasm builds give the same `stats::digest` for the same seed after every turn; `./cross-build.sh` checks it in CI.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
