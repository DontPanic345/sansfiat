# #339 Tests you can watch

**Question:** health. How many page scripts and core test files read a seeded pick, so that a
float change that moves the pick breaks them though their point isn't the pick? The control is
main at 63b941e, counted in `count.md`: **page scripts 19 of 22, core test files 10 of 45.**
(The issue's 16 of 19 and 10 of 39 are M4's count in `relay/retro/m4/architecture.md` §3; main
has grown since, and `count.md` recounts it the same way.) The piece's target: **5 of 23 page
scripts** (`tests.mjs` is the 23rd, and stages) (game, layout, pick, save, start: their point is a seeded game) and **8 of 45 core
files**. The review recounts on the branch with the same rule (`count.md`, top) and puts both
rows beside the control in the PR.

Beside the count, two health lines, reported not targeted:

- `node www/play/all.mjs`: every script passes, and its total seconds against main's
  (`allmjs-main.txt`, measured in this step). Staged scripts skip the pre-game, so it should fall.
- `engine/check.sh` passes, and `stats::digest` doesn't move: no rule changes, so any
  `SAVE_VERSION` bump is a fault.

## What the piece delivers

1. **A stage test as data.** One JSON file per test under `engine/core/stage/`, read by core
   (`include_str!`) so the native tests and the wasm page read the same bytes. A test is: a
   `name`, a one-sentence `claim`, the stage (every `StageSpec` field: the map as text, marks,
   hidden costs, food, civs, lead, armies, clans, signposts, plus the new `scouts` and
   `stores`), `orders` at turns, `turns`, and `holds`: what must hold.
2. **One runner in core** (`core/src/stage_test.rs`): it builds the stage, then turn by turn
   applies the orders due and the stores kept, ends the turn, and checks each hold. It can be
   stepped one turn at a time, so the page drives the same code `cargo test` does.
3. **`cargo test` runs the list**: `core/tests/stage_list.rs` runs every listed test and names
   each failing hold with its turn; a second test fails if a file in `core/stage/` isn't listed.
4. **The list starts with the scout's tests from C**, the four in `core/tests/scout_feed.rs`:
   the errand (one trip to the store, done by turn 10) first, then a rival army kills a scout, a
   scout in a rival's zone stays until it lifts, and a scout walks round a zone. They move to
   data and `scout_feed.rs` goes.
5. **The wasm stage door**: `tests()` gives the list (name, claim, turns, the kept stores and the
   holds as words, and `json`, the file's own text, which the page hands back to build it);
   `Sim.stage_test(json)` builds a test's world with its run attached; `test_step()` plays one
   turn and returns the holds' states; `test_state()` reads them. The verdict is core's, never
   the page's.
6. **The Tests screen**: from Start (a Tests button) or `?tests` (`?tests=NAME` opens one): the
   list of names and claims; choosing one shows its map at fit and its holds; Play plays it a
   turn at a time at a watchable pace, each hold marked pending, held or failed as it settles;
   the result says passed, or which hold failed at which turn.
7. **The page scripts move to the stage**: the 14 in `count.md` marked staged, through
   `game.stage(spec)` as `towns.mjs` and `settle.mjs` do. Each keeps every claim it checks
   now. `camp.mjs`'s hold, skipped since the start had no rival army, runs on a staged rival
   camp at its post. One check may keep a seed: `bar.mjs`'s pre-game counts need a pre-game,
   which a stage hasn't; if so bar counts as reading a seed (6 of 23) and `recount.md` says why.
8. **Two core files off the retro pick**: `judge.rs`'s three economy tests and eight of
   `scout.rs`'s nine move to built stages. The ninth, `a_route_order_replays_to_the_same_world`,
   can't: `World::replay` rebuilds from `World::new(config)` and a staged world's tiles are set
   outside the log, so a stage never replays. Its point is a real game's save, so it moves to
   `core/tests/stage.rs` beside the retro round trips, a file that already reads the pick.

## Shape: Henry Ford

As the issue says: the stage exists (`Stage`, `StageSpec`, `Sim.stage`, and three page scripts
already on it), and every move after the first is the same work. What would be clay, the
test's data form, is settled here to the field (below), so build-1 has an exact done and the
rest build on it.

- **build-1, core:** the format, the runner, the four scout tests as data, `stage_list.rs`, the
  wasm door, `judge.rs` and `scout.rs` off the retro pick.
- **build-2, the Tests screen** and `www/play/tests.mjs`.
- **build-3, page scripts of units and the civ:** army, camp (with the hold), civ, fields,
  goods, ring, scout.
- **build-4, page scripts of the map and the bar:** art, bar, buttons (its opening), camera,
  feed, select, views; then the recount in `count.md`'s form, written to `recount.md`.

## The format, settled

```json
{
  "name": "scout-errand",
  "claim": "A scout with five turns in its pack walks to its store once and builds its route's trails by turn 10.",
  "map": ["          ~ ~ ~ …", "…"],
  "marks": ["…"],
  "hidden": [{ "at": { "mark": "a" }, "trailCost": 2.0 }],
  "civs": [{ "mark": "A" }, { "mark": "B" }],
  "people": 14,
  "lead": 0,
  "scouts": [{ "civ": 0, "at": { "mark": "S" }, "pack": 10 }],
  "stores": [{ "civ": 0, "food": 50, "everyTurn": true }],
  "orders": [{ "turn": 0, "civ": 0, "order": { "type": "route", "unit": { "scout": 0 }, "from": { "mark": "A" }, "to": { "mark": "T" } } }],
  "turns": 40,
  "holds": [
    { "every": { "not": { "scoutHungry": { "civ": 0 } } } },
    { "count": { "scoutHoming": { "civ": 0 } }, "until": { "scoutBuilt": { "civ": 0 } }, "eq": 1 },
    { "by": 40, "fact": { "scoutDoing": { "civ": 0, "doing": "back" } } },
    { "by": 10, "fact": { "scoutBuilt": { "civ": 0 } } },
    { "end": { "trail": { "mark": "a" } } }
  ]
}
```

- **The stage** is `StageSpec`, flattened into the test (`#[serde(flatten)]`), so a test's stage
  is what `Sim.stage` takes. `map` and `marks` take a string or an array of rows (joined with
  newlines), so a map reads as a map in the file; `StageSpec` gains this for every caller.
- **`scouts`**: moves civ `civ`'s scout (every civ has one at founding) to `at`, with `pack`
  food of grain in its pack and, if given, `dest`, a tile it walks toward (as `scout_feed.rs`'s
  `rival_scout` sets it).
- **`stores`**: civ `civ`'s store set to `food` grain after the stage is built, and before every
  turn too when `everyTurn` is true. `everyTurn` is the stage's hand, as the Rust tests'
  `stock` is: the Tests screen shows it beside the claim ("the store is kept at 50").
- **`orders`**: before the turn after `turn` turns have ended (turn 0 is before the first),
  civ `civ` gives `order`, an `Order` as JSON in which any `{"mark": c}` or `{"tile": i}` is a
  tile and any `{"scout": k}` or `{"camp": k}` is the id of civ `k`'s scout or camp, resolved by
  walking the JSON before it is read as an `Order`. Only a one-key object is resolved, so
  `{"type":"signpost","tile":{"mark":"a"}}` reads as tile `a` and a bare number stays a number. The led civ's go through `act` as the player's do; the
  others through `World::order`. A refused order fails the test with the engine's reason.
- **Every `civ` in a test** (in `scouts`, `stores`, `orders`, facts and refs) is an index into
  `civs`, as `armies` and `clans` already are; the runner keeps the ids and the marks, which
  `Stage::into_world` drops.
- **Numbers are written as serde prints them**: a hidden trail cost of one day is
  `1.3333333333333333` (`DAY`), so the data world is the Rust test's world exactly.
- **`holds`**, each one of:
  - `{"every": FACT}`: holds after every turn;
  - `{"by": N, "fact": FACT}`: holds after some turn up to N;
  - `{"at": N, "fact": FACT}`: holds after turn N;
  - `{"end": FACT}`: holds after the last turn;
  - `{"count": FACT, "until": FACT?, "eq": N}`: the turns FACT becomes true, from false the turn
    before, until `until` first holds, number exactly N.
- **Facts**, each a one-key object, read from the world by core: `not`; `scoutAt {civ, at}`;
  `scoutAlive {civ}`; `scoutHungry {civ}` (its `hungry` above 0); `scoutHoming {civ}`;
  `scoutDoing {civ, doing}` (`units::scout::Doing`'s variant name in lower camel case: `back`, `trapped`, `toStore`, …);
  `scoutBuilt {civ}`; `shut {civ, at}` (tile `at` is shut to civ `civ`'s units by a rival
  camp: `units::shut(w, civ)` holds it); `scoutShut {civ}` (`shut` at its scout's tile);
  `trail {mark|tile}`; `events {kind, eq}` (the world holds exactly `eq`
  events of `kind`). A fact on a unit that is gone is false. build-1 adds no fact the four
  tests don't need.
- **Each hold has words**, written by core (`hold.words(&stage)`): "every turn: the scout of
  Red is not hungry", "by turn 10: the scout of Red has built its trails". The page shows them;
  it doesn't compose them.
- **A hold's state** after each turn is `pending`, `held` or `failed {turn, why}`. `every`
  fails at the first turn it doesn't hold; `by` fails after turn N; `at`, `end` and `count`
  settle at their turn. The test **passes** when every hold is held after the last turn, and
  fails at the first failed hold; the runner plays on to the end either way, so the page shows
  every hold settled.

## Checked by running (spec-review)

- The four scout tests' world rebuilds from the data form exactly: `Stage::flat(BIG)`, civs,
  lead, `plain()` and the errand's paint give the same `stats::state_digest` (`f81d47a26acde6cb`
  for the errand, `973a5aaf19d0d654` bare) as `Stage::from_spec` of the map text with `'f'` and
  `'^'` glyphs and `hidden` at `DAY`, with no `plain` step. So `from_spec` needs no plain step,
  and the led civ's revealed set is empty either way.
- `count.md`'s control recounts: 20 scripts in `www/play/` name a seed, `live.mjs` is the driver,
  19 counted; 45 `.rs` files in `core/tests/`, 10 call `Stage::retro`.
- `www/play/all.mjs` runs every `.mjs` in `www/play/` but `all.mjs` and `live.mjs`, so a shared
  helper there would be run and counted as a script: it lives at `www/play-stage.mjs`, beside
  `play.mjs`.

## Decisions I made

- The list is the tests written as data, starting with C's four scout tests. Other stage tests
  stay Rust until a piece moves them: a test that draws from rng seeds or calls a step directly
  (the pass's clan half, `settle::draw` over 40 seeds) has no form as orders and turns.
- The facts are a closed vocabulary in core, not expressions: a test reads only what a fact
  names, and a new test adds the facts it needs beside the others.
- A store kept each turn (`everyTurn`) is allowed and shown on the screen, because the scout's
  tests rely on it as written and dropping it changes what they claim.
- The Tests screen is a fifth screen beside Start, pick, Play and The end. Design says
  (`page.md`, Screens) four screens; the issue decides a Tests screen, reached from Start.
- The Tests screen has its own `MapView`, not the play screen's: `screens/play.js` is 759
  lines, and a test is watched, not played, so it has no panel, bar or orders.
- A page script reads a seeded pick only where its point is a seeded game: game, layout, pick,
  save (a stage is not a save) and start. The rest stage the world they check.
- `camp.mjs`'s hold runs on a staged rival camp at its post, not skipped: the stage gives it
  the army the start never did.
- Core: `judge.rs`'s economy tests and `scout.rs`'s rule tests move to built stages; the files
  whose point is the seven maps (`cases`, `scout_errand`, `scout_routes`, `scout_orders`'s
  search over every map, `ring`'s bands, `walk`, `wear`, `stage`'s cache) stay, as the issue
  leaves the retro maps' bound tests out of scope. `scout.rs`'s replay test joins `stage.rs`'s
  round trips, since a staged world doesn't replay.

## Unsettled

Nothing the score depends on. Left to the builds, each with an exact done: the screen's layout
and pace (build-2; the done is the script's checks, not the look), and each moved script's
spec (builds 3 and 4; the done is that each claim the script checks now it still checks).

## Ideas taken

None of the milestone's open ideas is about tests or the stage.

## For the review

- Recount with `count.md`'s rule on the branch, beside the control (19 of 22, 10 of 45) and
  the target (5 of 23, 8 of 45), in the PR. Say plainly if a script that should have moved
  didn't.
- Run `node engine/www/play/all.mjs`; give its total seconds beside `allmjs-main.txt`'s.
- Open the page at `?tests` with `node engine/www/play.mjs` (or `play/live.mjs`), pick
  `scout-errand`, press Play, and screenshot the result: it says passed, the holds all held.
  Then stage a copy with `"by": 2` on the trails hold through `game` (as `tests.mjs` does) and
  see it fail at turn 2 with its words.
- `cargo test -p engine --test stage_list` passes; break one hold in a file and see it fail
  naming the test, the hold and the turn (don't commit that).
- For each moved page script, read its old and new checks side by side for one or two scripts
  and confirm none was dropped or weakened (a `need` that became an `if`).
- `scout.rs` reads no `Stage::retro`; its replay test runs in `stage.rs`.
- `stats::digest` and `SAVE_VERSION` unchanged.
