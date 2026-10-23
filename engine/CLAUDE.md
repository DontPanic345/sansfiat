# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
cargo test
cargo fmt --check && cargo clippy --all-targets -- -D warnings
cargo run -p sansfiat -- run --seed 3 --turns 50 [--save game.json] [--json]
cargo run -p sansfiat -- replay game.json [--json]
cargo run -p sansfiat -- bench [seed ...] [--turns N] [--radius N]
cargo run -p sansfiat -- clear baskets.json     # or - for stdin
./build-web.sh && python3 -m http.server -d www  # http://localhost:8000
```

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `Action`, `TURN`, `act`, `save`, `replay`. |
| `core/src/data.rs` | Terrain, foods (the goods), `CLAN`, `TRAIL`, `TENDING`, `MARKET` and `PREGAME` rules, `Config`. Tune here first. |
| `core/src/map.rs` | Placeholder map: Voronoi terrain in a hex, sea at the rim, food scaled by patchy fertility. A map piece replaces it. |
| `core/src/land.rs` | Neighbours, move costs, bounded Dijkstra (`cost_map`), the `regrow` step, and the `hold` step: held counters, trails, taming and hearths. |
| `core/src/round.rs` | A clan's round: leg and loop costs (`Legs` caches them per home), cheapest insertion, free edge, middle, territory, what it can harvest. |
| `core/src/clans.rs` | Clans: seeding, a round's `value`, the `move` (`adjust`: moving on, one round change, home follows the middle, the limit), `harvest` and `eat` steps; dividing and daughters after eating. |
| `core/src/market.rs` | `clear(baskets, start_prices, params)`: barter exchange ratios by tâtonnement, no numeraire. A library `markets` calls. |
| `core/src/markets.rs` | Markets: `reach`, the `go` (worth the walk?), `trade` (clearing) and `markets` (fade, then form one) steps. |
| `core/src/pregame.rs` | The pre-game's end: centres (age, traders, a hearth in reach, far apart), land cover, and the `end` step. |
| `core/src/hex.rs`, `core/src/rng.rs` | Axial pointy-top hexes; mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | wasm-bindgen `Sim` (`new`, `load`, `act`, `state`, `save`); `www/main.js` draws it. `www/pkg` is built, not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player does is a variant of `Action`, checked in `validate` and applied in `act`.
4. A module of its own under `core/src/`, tests under `core/tests/`.

## Invariants

- The world changes only through `act`. A `Save` (config and action log) replays to an identical world;
  `replaying_a_save_rebuilds_the_world` holds it to that.
- All randomness goes through `rng::rand` on `World.rng`.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
