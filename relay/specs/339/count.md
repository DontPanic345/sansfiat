# #339 the control: who reads a seeded pick, on main (63b941e)

A page script reads a seeded pick when it opens `?size=…&seed=…` or calls `lead(page, { seed })`, even if it
stages a game after. A core test file reads one when it calls `Stage::retro`. `all.mjs` and `live.mjs` are
drivers, not touch points, and are not counted.

## Page scripts: 19 of 22 read a seeded pick

| Script | Opens | Its point is a seeded game? | After the piece |
|---|---|---|---|
| `army.mjs` | `lead` small 6 | no: the army's march, ring and skirmish | staged (build-3) |
| `art.mjs` | small 2 | no: atlas cells drawn | staged (build-4) |
| `bar.mjs` | small 2 | no: the top bar, laptop and phone | staged (build-4) |
| `buttons.mjs` | small 2, then stages | no: it only needs a page | opens `''` (build-4) |
| `camera.mjs` | small 1 | no: a click centres | staged (build-4) |
| `camp.mjs` | small 3, small 4 (hold, skipped) | no: the camp's zone, paint, and the hold | staged, the hold run (build-3) |
| `civ.mjs` | small 6 | no: the civ's panel, packs, gift | staged (build-3) |
| `feed.mjs` | small 2 | no: feed items and clicks | staged (build-4) |
| `fields.mjs` | `lead` small 1 | no: a farming clan drawn | staged (build-3) |
| `game.mjs` | `lead` small 9 | **yes**: a whole game through the page | stays |
| `goods.mjs` | small 2 | no: source marks and the civ's goods | staged (build-3) |
| `layout.mjs` | small 1, large 1 | **yes**: real maps at real sizes, the large map at fit | stays |
| `pick.mjs` | small 2 | **yes**: the pick's cards | stays |
| `ring.mjs` | small 2 | no: the ring drawn | staged (build-3) |
| `save.mjs` | small 2 | **yes**: a save of a played game (a stage is not a save) | stays |
| `scout.mjs` | small 2, small 12 | no: the scout's orders, lines and hunger | staged (build-3) |
| `select.mjs` | small 2 | no: clicking cycles a tile's things | staged (build-4) |
| `start.mjs` | the start screen to small 2 | **yes**: the start screen | stays |
| `views.mjs` | small 2 | no: View and Debug layers | staged (build-4) |
| `end.mjs`, `settle.mjs`, `towns.mjs` | `''`, then stage | staged already | — |

Target: **5 of 22** (23 with `tests.mjs`, which stages): game, layout, pick, save, start.

## Core test files: 10 of 45 read `Stage::retro`

| File | Its point is the retro map? | After the piece |
|---|---|---|
| `cases.rs` | yes: the owner's cases on the seven maps | stays |
| `judge.rs` | no: three economy-group tests that need any civs | built stage (build-1) |
| `ring.rs` | one rule test plus the bands report on the seven maps | stays (the report is a bound) |
| `scout.rs` | no: the real map is painted into each case | built stage (build-1) |
| `scout_errand.rs` | yes: bounds on the seven maps | stays (out of scope) |
| `scout_orders.rs` | partly: `search_view_on` runs every retro map | stays |
| `scout_routes.rs` | yes: a route to every rival capital | stays (out of scope) |
| `stage.rs` | yes: the retro cache and round trips | stays |
| `walk.rs` | yes: a report on the seven maps | stays |
| `wear.rs` | yes: a retro errand's candidates | stays |

Target: **8 of 45** (`scout_feed.rs` goes, its tests now data; `stage_list.rs` comes).
