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
node www/assets/draw-atlas.mjs [--check]     # atlas.png and atlas.json from placeholders.js and sources.js (--check: no Chrome)
node www/shots.mjs --out DIR                 # the map's fixed views, as canvases; --diff A B compares two folders
node www/shots.mjs --cull DIR                # each mountain view with the ridge layer's culling on and off: identical, or exit 1
```

`cargo test` runs optimised: whole pre-games are too slow otherwise.

## Layout

| Folder | What it holds |
| --- | --- |
| `core/src/` | The world and its systems, in the order a turn runs them: `world.rs` first (`World`, the `TURN` steps, `act`, `run`, `save`, `SAVE_VERSION`), then `land.rs` (regrow, wear, tend), `clans.rs` (move, harvest, eat; `homes.rs`, `fields.rs`, `dividing.rs` beneath), `markets.rs` and `market.rs` (festivals and barter), `settle.rs`, `settling.rs` (a clan's settled state: a reading), `leaning.rs`, `civs.rs`, `units/` (the AI's turn, then the armies), `claim.rs`, and `pregame.rs` and `over.rs` (the end). `actors.rs` is what every actor does (`spoil`, `eat`, `walk_cost`, `carry`: what a carrier brings); `map.rs` makes the ground; `orders.rs` takes orders; `stats.rs` makes the digests; `judge*.rs` score a game and never change it; `stage*.rs` and `maptext.rs` build a staged world. |
| `core/src/data/` | Every rule's number and `Config`, one file a system (`goods.rs`, `map.rs`, `clans.rs`, `civs.rs`, `units.rs`, `game.rs`), each in days (`DAY`), food, turns, people or a share; tune here first. `mod.rs` re-exports them all. |
| `core/src/units/` | Armies and scouts: `ai.rs` (the planner, which the `ai` step runs), `camp.rs` (an army's turn and zone), `line.rs` (paths, cuts, `ring`), `forage.rs` (what a unit eats), `muster.rs`, `search.rs` (the route search), `paths.rs` (the turn's cached paths and reach), `scout.rs`, `fight.rs`. |
| `core/stage/` | Stage tests as data, one JSON file each (a `StageSpec`, its orders and holds), listed in `core/src/stage_test.rs`'s `LIST`; `Run` plays one a turn at a time. |
| `core/tests/` | Tests, one file a system (`clans.rs`, `fields.rs`, `goods.rs`, …), with the helpers they share in `common/`. |
| `cli/src/` | The `sansfiat` binary, one module per command: `main.rs` (USAGE), `sweep*.rs` (the sweep and its judges), `game_judge.rs` (a played file's judge), `view.rs` (the text view), `story.rs` (stories of war). |
| `web/src/` | The wasm `Sim` the page drives: `lib.rs` (`layout`, `view`, `act`, the orders; `Sim.stage` builds one, `Sim.stage_test` a test) and `stage_test.rs`. |
| `www/` | The page: `main.js` (the router), `app.js`, `model.js` (the view model), `panel.js` and `panels.js` (a selection's shape and buttons), `save.js`, `check.mjs` (the page's data checks under Node, through `Sim`), `play.mjs` (headless Chrome; `play/live.mjs` holds one page across shell calls), `shots.mjs` (the fixed map views). `assets/` is the atlas: `placeholders.js` (every cell's drawing and `layout()`), `sources.js` (the regions and the cells cut from them), `cut.mjs` (hex, fill and sprite regions; ops `mirror`, `ramp`; `rotate`; a derived fringe cell from another; a `flat` rectangle), `png.mjs`, `draw-atlas.mjs`. |
| `www/map/` | `view.js` (`MapView`), and `layers/`, a file per layer. `ridges.js` makes the mountain pieces (a peak per tile, a ridge per shared edge; pure), which `layers/ridges.js` draws. |
| `www/screens/` | One file a screen: `pick.js`, `play.js`, `orders.js`, `start.js`, `end.js`, `tests.js` (the Tests screen). |
| `www/play/` | One script per touch point of `docs/design/page.md`, each exiting non-zero when its touch point is missing; each stages its world through `../play-stage.mjs`; `all.mjs` runs them. |

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
- A rule's proof is a test in `core/stage/`, and a button's is `www/play/buttons.mjs`.
- All randomness goes through `rng::rand`; `exp`, `ln`, `powf` go through `libm`, so native and wasm
  give the same digest every turn (`./cross-build.sh`).
- `clear` conserves every good exactly.
- `wasm-bindgen` is pinned to the installed `wasm-bindgen-cli`.
- This file stays under 8 KB; `check.sh` holds it there.
