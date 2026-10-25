# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
cargo test   # [profile.dev] opt-level = 3: the engine is too slow unoptimised for whole pre-games
cargo fmt --check && cargo clippy --all-targets -- -D warnings
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large | --radius N]   # the map as ASCII, a legend and counts
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--every 25] [--save game.json] [--json | --events-json] [--no-events]  # stops at the pre-game's end; prints "turn  text" per event
cargo run -p sansfiat -- sweep [seed ...] [--radius N | --size small|medium|large|all] [--turns N]   # seeds default 1..=10, size small; seeds run in parallel
cargo run -p sansfiat -- replay game.json [--json | --events-json] [--no-events]
cargo run -p sansfiat -- bench [seed ...] [--turns N] [--radius N]
cargo run -p sansfiat -- clear baskets.json     # or - for stdin
cargo run -p sansfiat -- digest [--seed N] [--radius N] [--turns N]   # "turn digest" lines, to compare builds
./build-web.sh && python3 -m http.server -d www  # http://localhost:8000
./build-web.sh && ./cross-build.sh              # native and wasm (under Node) give the same digests (small and medium maps)
```

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `Action`, `TURN`, `act`, `save`, `replay`; `Event` and `EventKind` (`World.events`, pushed by the steps where things happen, not saved and not in the digest, rebuilt by replay; `Event::text` is the one wording). |
| `core/src/data.rs` | Terrain, foods (the goods), `CLAN`, `TRAIL`, `TENDING`, `MARKET` and `PREGAME` rules, `Config`. Tune here first. |
| `core/src/map.rs` | The map: noise terrain in a hex, sea at the rim, inlets, inland seas and lakes, ridges, rain-shadow deserts, coast, patchy fertility, closed country (`close_country`: valleys behind a pass, sealed valleys, peninsulas on a neck, tagged by `Zone`) with `open_sites` and its re-rolls, rivers down a distance-to-water field (`carve_rivers`; lakes fed and drained), floodplains and deltas (`flood`); `finish_caps` turns terrain, river and fertility into food; hidden trail costs from their own rng stream; `ascii` draws it. |
| `core/src/land.rs` | Neighbours, `passable`, move costs (`step_cost(from, to)`: entering a river off its course costs `RIVER.cross`, a trail bridges it), bounded Dijkstra (`cost_map`), the `regrow` step, and the `hold` step: held counters, trails, taming and hearths. |
| `core/src/round.rs` | A clan's round: leg and loop costs (`Legs` caches them per home), cheapest insertion, free edge, middle, territory, what it can harvest. |
| `core/src/clans.rs` | Clans: seeding (never in a sealed valley), a round's `value`, the `move` (`adjust`: moving on, one round change, home follows the middle, the limit), `harvest` and `eat` steps; dividing and daughters after eating. |
| `core/src/market.rs` | `clear(baskets, start_prices, params)`: barter exchange ratios by tâtonnement, no numeraire. A library `markets` calls. |
| `core/src/markets.rs` | Markets: `reach`, the `go` (worth the walk?), `trade` (clearing) and `markets` (fade, then form one) steps. |
| `core/src/pregame.rs` | The pre-game's end: centres (age, traders, a hearth in reach, far apart), land cover, and the `end` step. At the turn limit with fewer than three centres it tops up (`top_up`): existing markets, then the most tended land, then any grain land, all in one region of passable land (`End.topped_up` lists the ids added). |
| `core/src/stats.rs` | `summary` (counts off the world), `centre_report` (fertility near each centre vs all land), `distances`: the evidence the CLI and the web page show; `digest` (FNV-1a of the world JSON) to compare builds. |
| `core/src/hex.rs`, `core/src/rng.rs` | Axial pointy-top hexes; mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | wasm-bindgen `Sim` (`new`, `load`, `act`, `state`, `summary`, `events(from)`, `save`, `digest`) and `terrain`, `goods`, `rules`, `sizes`; `www/main.js` draws the map (fertility view, territories, rounds, trails, hearths, markets, centres) and a Run to end button. `www/pkg` is built, not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player does is a variant of `Action`, checked in `validate` and applied in `act`.
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
