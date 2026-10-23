# engine

Barter-market slice of Sans Fiat in Rust. Clans harvest goods from the land around them and trade
at the nearest market in reach; prices are exchange ratios found by tâtonnement each turn, with no
numeraire. One engine, two front ends: a CLI and a wasm web page.

```
cargo test
cargo run -p sansfiat -- run --seed 3 --turns 50 [--every 10] [--json]
cargo run -p sansfiat -- clear baskets.json     # or - for stdin
./build-web.ps1; python -m http.server -d www   # http://localhost:8000
```

## Layout

| Path | What |
| --- | --- |
| `core/src/data.rs` | Goods, terrain yields, `SEASON`, `Config`. Tune here first. |
| `core/src/market.rs` | `clear(baskets, start_prices, params)`, a port of `poc/src/market.js` without `wants`. |
| `core/src/world.rs` | `World::new(config)` (Voronoi map, coast, markets spread apart, clans) and `step()`: season → harvest → trade → stats. |
| `core/src/rng.rs` | mulberry32, matching the JS prototypes bit for bit. |
| `cli/` | The `sansfiat` binary. |
| `web/` | wasm-bindgen `Sim` class; `www/main.js` draws it. `www/pkg` is built, not committed. |

## Invariants

- All randomness goes through `rng::rand` on `World.rng`, so a config replays exactly.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
