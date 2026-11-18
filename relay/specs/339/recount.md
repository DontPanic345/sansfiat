# #339 the recount: who reads a seeded pick, on this branch (build-4)

The rule is `count.md`'s: a page script reads a seeded pick when it opens `?size=…&seed=…` or calls `lead(page, { seed })`;
a core test file reads one when it calls `Stage::retro`. Counted on the branch at the end of build-4.

## Page scripts: 6 of 23 read a seeded pick (count.md: 19 of 22 on main)

| Script | Opens | Its point is a seeded game? | On the branch |
|---|---|---|---|
| `bar.mjs` | `?size=small&seed=2` for the pre-game's counts only; the rest on a stage | no, but its pre-game check needs a pre-game, which a stage is not | seeded pre-game (one check) |
| `game.mjs` | `lead` small 9 | **yes**: a whole game through the page | seeded (stays) |
| `layout.mjs` | small 1, large 1 | **yes**: real maps at real sizes | seeded (stays) |
| `pick.mjs` | small 2 | **yes**: the pick's cards | seeded (stays) |
| `save.mjs` | small 2 | **yes**: a save of a played game | seeded (stays) |
| `start.mjs` | the start screen to small 2 | **yes**: the start screen | seeded (stays) |

Without `bar`'s pre-game check the count is **5 of 23**, the target. `bar` stays at 6 because a stage has no pre-game.
The other 17 open `''` and stage their world: army, art, buttons, camera, camp, civ, end, feed, fields, goods, ring, scout,
select, settle, tests, towns, views.

## Core test files: 8 of 45 read `Stage::retro` (count.md: 10 of 45 on main)

| File | Its point is the retro map? | On the branch |
|---|---|---|
| `cases.rs` | yes: the owner's cases on the seven maps | retro (stays) |
| `ring.rs` | one rule test plus the bands report on the seven maps | retro (stays: the report is a bound) |
| `scout_errand.rs` | yes: bounds on the seven maps | retro (stays) |
| `scout_orders.rs` | partly: `search_view_on` runs every retro map | retro (stays) |
| `scout_routes.rs` | yes: a route to every rival capital | retro (stays) |
| `stage.rs` | yes: the retro cache, the round trips, and the replay test | retro (stays) |
| `walk.rs` | yes: a report on the seven maps | retro (stays) |
| `wear.rs` | yes: a retro errand's candidates | retro (stays) |

`judge.rs` and `scout.rs` left the list in build-1 (built stages), `scout_feed.rs` was deleted (its tests are data in
`core/stage/`), and `stage_list.rs` does not read a retro map. 45 files on the branch, 8 read one: the target.

## Timing: `node www/play/all.mjs`, one run each

| | Scripts | Total seconds |
|---|---|---|
| `allmjs-main.txt` (main, 63b941e) | 22 | 174.7 |
| this branch (build-4) | 23 | 191.3 (wall 191.2 s) |

The branch adds `tests.mjs` (22.6 s, build-2). The staged scripts got faster: army 11.1 → 7.7, camp 16.3 → 9.8,
ring 8.9 → 1.7, scout 9.6 → 3.1, select 5.9 → 2.5, feed 6.9 → 4.5, art 2.6 → 1.5, camera 3.7 → 2.7, views 2.8 → 2.0.
`game.mjs` took 41.0 s in this run and 10.4 s when run alone (main: 10.4); it is unchanged by the piece, so the 41 s is
load on the runner, not the branch. `buttons.mjs` 22.8 s (main 22.9), `layout.mjs` 31.6 s (main 31.4).
