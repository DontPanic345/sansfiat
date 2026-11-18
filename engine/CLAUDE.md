# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions, with a CLI and a wasm page as its two front ends. Build what `docs/design/` says.

```
./check.sh                                   # every check CI runs, in order
cargo run -p sansfiat -- map | run | sweep | play | story | judge | replay | bench | digest   # USAGE in cli/src/main.rs
cargo run -p sansfiat -- sweep --judge [--json] [--floor] [--solo]   # the retro maps beside their controls, and the led games: a report
cargo run -p sansfiat -- play --game g.json [--seed N --size NAME [--civ K]]   # a line at a time; g.lines.jsonl records it
cargo run -p sansfiat -- judge g.json [--json]   # a played game beside the same map on the AI
cargo run -p sansfiat -- judge --compare a.json b.json [--all]   # two sweep reports: each metric's change against its noise floor
cargo run -p sansfiat -- bench --size large 8 --after 50   # ms a turn past the pick
node www/play.mjs "?size=small&seed=2&run=1" --shot pick.png   # the page in headless Chrome
node www/play/all.mjs                        # every touch-point script; check.sh runs it
```

`cargo test` runs optimised: whole pre-games are too slow otherwise.

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `phase`, `Action`, the `TURN` steps, `act`/`run`/`save`/`replay`, reasons, alerts, `SAVE_VERSION`. |
| `core/src/over.rs` | The end: `Phase::Over` (winner, `Why`, turn), the `over` step, `limit` and `ends_at`. A game ends as a phase; the world runs on after it. |
| `core/src/events.rs` | `Event`, `EventKind` and their wording. |
| `core/src/data.rs` | Every rule's number and `Config`, each in days (`DAY`), food, turns, people or a share; tune here first. |
| `core/src/map.rs` | Map generation: terrain, water, closed country, rivers, foods, hidden trail costs and the flat trail cost. |
| `core/src/land.rs` | Neighbours, move costs, `path`, `steps`, and the `regrow`, `wear` and `tend` steps. |
| `core/src/actors.rs` | What every actor does: spoil, eat, `meal` (from kin's stores), `stride`, `walk_cost`, `carriers`, traffic, `divide` and `split` (over jobs; a tile pays its walk for each person walking it), `change`. |
| `core/src/clans.rs` | Clans: seeding, `reap` (its split between gathering and fields; a clan gathers only what it claims), `value`, and the `move`, `harvest` and `eat` steps. |
| `core/src/homes.rs` | Where clans live: `homes` (derived, never saved), `claim_land` (never on a held tile), `gathers`, `step` (a farmer's home step, with towns). |
| `core/src/dividing.rs` | A big clan's `divide_land` and a clan's `split` (a daughter to a site it can settle). |
| `core/src/fields.rs` | The field job: `ground`, `yield_a_day`, `open`, `sow`, the split's `notch`, a farmer's `home_toward`, `plots`. |
| `core/src/market.rs`, `markets.rs` | Barter clearing; festivals: reach, the `go` step, trading, forming, fading. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, `top_up`. |
| `core/src/claim.rs` | A civ's claim, which is its land: `check`, `held`, `tile_civ`, `change`, and the `claim` step; `outlook.rs`, the reach and capital lines. |
| `core/src/civs.rs` | Civs: `found`, the levy, `keep`, the `civs` step and the pick's `cards`. |
| `core/src/leaning.rs` | Leanings: fading, land, gifts, joining and leaving. |
| `core/src/settle.rs`, `judge_settle.rs` | Signposts and settlements: `draw`, `found`, `take`, `roots`; the `settle` group. |
| `core/src/units/` | `line.rs` (paths, cuts, `ring`), `camp.rs` (an army's turn and zone), `forage.rs` (what a unit eats; `Ladder`), `muster.rs`, `search.rs` (the route search), `scout.rs`, `ai.rs` (the planner), `fight.rs`. |
| `core/src/orders.rs` | `Order`, `World::order` and `check_order`: the player's and the AI's. |
| `core/src/script.rs` | `Lead`, the scripted game; `Auto`, the planner as a led player. |
| `core/src/stage.rs`, `stage_spec.rs`, `maptext.rs` | `Stage`: a world on a known map, layers, placements, orders and reads; `stage_spec.rs` is `StageSpec`, its data form. A staged world is not a save. |
| `core/src/stage_test.rs`, `core/stage/` | A stage test as data (`StageSpec`, orders, holds), one JSON file each, listed in `LIST`; `Run` plays it a turn at a time. |
| `core/src/stats.rs` | `summary`, `standing`, the evidence reports, `digest` and `state_digest`. |
| `core/src/judge.rs`, `judge_scores.rs` | `Watch`: a game scored from its pick, the end group included; `judge_scores.rs` holds the score structs. Reads the world only. |
| `core/src/judge_ring.rs` | The `supply` group: each civ's ring, the armies and scouts inside it, hungry turns, clan territory. |
| `core/src/judge_fields.rs` | The `fields` group: field tiles, farmers, store turns at a pack, moves and starving. |
| `core/src/towns.rs`, `judge_towns.rs` | Towns: `towns` (a festival with `TOWN.homes` homes on its tile) and `check` (`Market.town`, `TownFormed`); the `towns` group. |
| `core/src/sources.rs`, `judge_goods.rs` | Goods: placing salt and obsidian sources (`place`, `of`, `steps_from`); the `goods` group. |
| `core/src/hex.rs`, `rng.rs` | Axial hexes; Mulberry32, matching the JS prototypes bit for bit. |
| `cli/src/` | The `sansfiat` binary, one module per command; `sweep_judge.rs` (the sweep's judge), `sweep_fields.rs` (fields), `sweep_towns.rs` (towns), `sweep_goods.rs` (goods against the no-sources control), `sweep_led.rs` (led games beside idle), `sweep_total.rs`, `sweep_text.rs` and `game_judge.rs` (the file's judge), `view.rs` (the text view), `story.rs` (stories of war). |
| `web/src/lib.rs`, `stage_test.rs` | The wasm `Sim` the page drives: `layout`, `view`, `act`, the orders; `Sim.stage` builds one, `Sim.stage_test` a test. |
| `www/` | The page: `main.js` (the router), `app.js`, `screens/`, `bar.js`, `panel.js`, `panels.js` (a selection's shape and its buttons) and `panels-land.js` (festival, tile, settlement shapes), `model.js` (the view model), `map/` (`MapView`, a file per layer: `towns.js`, `sources.js`), `feed.js`, `save.js`, `sound.js`, `atlas.js` and `assets/`. |
| `www/check.mjs` | The page's data checks under Node, through `Sim`. |
| `www/check-stage.mjs`, `check-ctx.mjs` | The panel arm of `check.mjs`, over a staged `Sim`: the panel shapes as data, and the panel context both share. |
| `www/play.mjs` | Drives the page in headless Chrome (clicks, waits, screenshots); `play/live.mjs` holds one page across shell calls. |
| `www/play/` | One script per touch point of `docs/design/page.md`, each exiting non-zero when its touch point is missing; `buttons.mjs` presses every button on a staged game; `all.mjs` runs them. |

## Adding a mechanic

1. Its state is a field on `World`, built in `World::new`; its per-turn work is `Step`s in `TURN`.
2. What the player or an AI does to a unit is an `Order`; anything else the player does is an
   `Action`, checked in `validate` and applied in `act`. `World::run` takes the pick itself; a
   front end that calls `act(EndTurn)` must send `ChooseCiv` when `phase` is `choose`.
3. A module of its own under `core/src/`, tests under `core/tests/`. Shared actor behaviour goes in
   `actors.rs`; a unit kind is a `UnitKind` and a `UnitState` variant.

## Invariants

- Reasons and alerts are derived (`#[serde(skip)]`): a rule writes one `w.note` where it acts.
- A change that moves `stats::digest` bumps `SAVE_VERSION` and the pins in `cli/tests/save.rs`.
- The world changes only through `act` and the planner's orders; a save replays to an identical world.
- The end is a phase: only the `over` step sets `Phase::Over`, and the world runs on in it (`playing()`);
  `validate` refuses the player's orders there.
- A rule's proof is a test in `core/stage/` (under 8 KB each), and a button's is `www/play/buttons.mjs`.
- All randomness goes through `rng::rand`; `exp`, `ln`, `powf` go through `libm`, so native and wasm
  give the same digest every turn (`./cross-build.sh`).
- `clear` conserves every good exactly.
- `wasm-bindgen` is pinned to the installed `wasm-bindgen-cli`.
- This file stays under 8 KB; `check.sh` holds it there.
