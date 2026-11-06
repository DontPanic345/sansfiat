# #151 The game proves itself: spec

## What it delivers

Anything in the game can be placed on a known map, given orders and checked with the real
engine; and a change can be compared against main with the drift measured, so a review knows
a move from noise.

1. **A `Stage`** (`core/src/stage.rs`, public like `script.rs`): a world built from the `map`
   command's own text, or a retro map at the pick; layers for marks, hidden costs and food;
   clans, civs and armies placed; orders through `World::order` or `act`; `turn`, `until`;
   reads. A staged world is not a save (its placements bypass the log), so it never writes
   one: it asserts outcomes. `StageSpec` is its data form, so the CLI, the tests and the wasm
   page build the same stage from one JSON.
2. **`map::parse`**, the inverse of `map::ascii` (`core/src/maptext.rs`, re-exported from
   `map`).
3. **The fixtures on it.** `tests/common` and `core/tests/scout.rs`'s painters are rewritten on
   the stage, with every existing assertion untouched.
4. **The owner's two bugs, red on main**: the scout that searches a range nearest-first, not
   by best case (with the owner's settle invariant as a green guard), and Disband all that
   sends one person. Each is a test that expects its failure, so CI holds the failure and T
   flips it (decisions 3 and 6). A third red case, the scout that never builds the river
   bridge that would cut its settled route, is spec-review's find and not T's.
5. **`Sim.stage`** in wasm, a `check.mjs` arm that builds the page's panel shapes from a staged
   game as data, and the Disband all page arm, since that bug is in the page's order form.
6. **The judge**: `judge --compare A.json B.json` with a noise floor per metric; the idle led
   civ as `judge FILE`'s second control; an economy group in `sweep --judge`; the split of
   `cli/src/judge.rs`; `bench --after N`.
7. **`www/play/buttons.mjs`**: every button of every panel pressed once on a staged game, and
   the world checked. `www/play/all.mjs` runs every script in the folder, so `check.sh` runs
   it.

No rule changes. `stats::digest` is unchanged for every seed; the pins in `cli/tests/save.rs`
and `./cross-build.sh` hold.

## The question and how it is measured

**The question:** health and decisions (`docs/process.md`, Scores). Can a session now prove
a rule on the maps and the page, and can a review tell a move from noise?

| Score | Measure | Control |
|---|---|---|
| Health | The owner's two bugs are red on main for the stated reason: the scout's search order (`should_panic`) and the page's Disband all (`expectBug`), each carrying its failing message, and `relay/specs/151/cases.md` records what each says. The river-bridge case is red too, recorded as not T's. `check.sh` is green; every digest is unchanged. | The same checks where the bug can't act: the Rust Disband with no count passes on fractional units; the search-order check passes on small 9 to the forest; the owner's settle invariant passes on all 14 errands. |
| Health | `sweep --judge --json` on the branch against main's: `judge --compare main.json branch.json` flags no existing metric (the new economy group is listed as only in B). | main against itself: nothing differs and nothing is flagged. |
| Health | The noise floor is sane: `sweep --judge --json --floor` on main gives a floor per metric; a compare of main against a perturbed run flags fewer metrics than it lists as changed. | Floors are 0 only where a metric is exactly stable; the build says which. |
| Decisions | `buttons.mjs` presses every button label `describe` yields on the staged game, each checked against the world; it fails on a label with no case. | The review changes one order in `panels.js` (for example Forage sends `pillage`) and the script goes red. |
| Decisions | `judge FILE` on a played game reads three: the game, the AI, and the idle led civ. | The idle control gives the same Standing every run (deterministic), and the led civ in it has no orders in its log. |

## Measured before cutting

Run on main at the start of this step:

- `sansfiat sweep --judge --json` takes 32 s (2 min of CPU over seven maps in parallel).
  The pre-game to the pick of all seven retro maps takes 5 s together, so a per-process cache
  is worth having: the large map's alone is about 3 s.
- `World`, `Tile` and `Civ` are `Clone`. `stats::digest` hashes the world's JSON, so a
  `#[serde(skip)]` field moves no digest.
- **The scout, measured again in spec-review** (a throwaway probe on main, the scout's store
  and pack refilled each turn: the pack matters, since with the store alone small 3's scout
  starves on turn 16). The owner's note (`relay/retro/m3/owner.md` §1.3) is that the scout
  checked one mountain and ignored two that might have carried a trail; the M3 retro traced it
  (`docs/proposals/m3-retro.md`, note 3): every skip was right, but the search goes
  nearest-first along a range, slowly and silently. T's fix is the order: a range's mountains
  by their best case (the cost through them plus the walk there). So:
  - **Nothing unrevealed beats a settled route, on main.** On small 2 to the rival (settles
    turn 2, known 15, possible 14) and small 3 to the forest (settles turn 51, known 19,
    possible 18), every tile where the possible path's step differs from the known one is
    *revealed*: hills 144 (river 5) on small 2 and hills 436 (river 1) on small 3. The gap is
    the river crossing a trail on a revealed tile would bridge, which `route_plan`'s possible
    path drops and the known path pays, and the scout builds trails only on the known path. It
    is a real bug, but not the owner's and not T's: build-3 keeps it as its own red case and
    the spec-review files it as a fix. The other errands settle with known equal to possible.
  - **The search is out of best-case order on main.** Counting each turn a mountain is
    revealed against the unrevealed mountains whose cost through them beats the known route,
    with the walk as the scout's known cost to beside each (the definition build-3's case
    uses): small 9 to the rival reveals 29 mountains over 110 turns, 12 of them with a better
    best case left open; medium 9 to the rival 11 of 34;
    medium 8 to the rival 4 of 11; small 9 to the forest 0 of 2. That is the red case for the
    owner's note.
- **Disband all.** `www/panels.js`'s `disband()` makes its `all` option a stepper option, so
  the order carries a count `n`; the stepper opens at 1 (`www/panel.js`, `stepping.n = 1`),
  and its cap is `max(1, floor(people))`. Pressing Disband… → all → Go disbands one person
  (the owner's note §1.2); even at the cap an army of 4.6 keeps 0.6. The engine's
  `Order::Disband { n: None }` sends all: the Rust case checks that and passes on main; the
  page arm is the red one.

## Shape: Henry Ford

Taken as the piece suggests. Everything is settled in its rules; the proof is that the
existing suite and every digest hold, that the two cases are red for the stated reason, and
that the judge and the page scripts report. The steps are cut by area, in an order each one
needs:

| Step | Area | Why here |
|---|---|---|
| build-1 | `map::parse`, `Stage`, `StageSpec`, the retro cache, the stage's own tests | Everything else uses it |
| build-2 | `tests/common` and `scout.rs`'s painters on the stage | Needs the stage; leaves the cases a home |
| build-3 | The two cases in core (`tests/cases.rs`), `cases.md` | Needs the rewritten fixtures |
| build-4 | `Sim.stage`, `check.mjs`'s arm, `window.game.stage`, the Disband all page arm | Needs the spec form and the case |
| build-5 | The judge's split, the idle control, the economy group | A refactor first, byte for byte, then the additions |
| build-6 | `judge --compare`, the floor, `bench --after` | Needs the split's modules; independent of the stage |
| build-7 | `www/play/buttons.mjs`, `engine/CLAUDE.md`, `docs/architecture.md` | Needs `game.stage` |

The largest ford is build-1; it runs a rung below its score.

## Settled: the stage (build-1)

1. **Text.** `map::parse(text) -> Result<Parsed, String>`, with `Parsed { radius, tiles }`
   (tiles as `World::new`'s tile layout, bare). The text is `ascii`'s: one row per `r` from
   `−radius` to `radius`, each row indented as `ascii` does, one glyph a tile, a space
   between glyphs. A row count short of `2·radius + 1`, a row of the
   wrong width or an unknown glyph is an error naming the row and column. Blank lines at the
   foot and the `map` command's legend and count lines (anything after the last full row) are
   ignored: the text that `sansfiat map` prints parses as printed. So the radius comes from
   the first non-blank row (its glyph count less one), not from the row count, and the parser
   reads `2·radius + 1` rows; leading blank lines are skipped and an indent common to every
   row (a raw string in a test) is removed, so the rows' own indents are `ascii`'s
   (spec-review: the legend and count lines follow the rows, so a row count can't give it).
2. **Glyphs to tiles.** `~` sea; `=` sea with `lake`; `,` coast; `.` plains; `f` forest; `h`
   hills; `^` mountain; `:` desert; `%` marsh; `r` plains with `river` 1; `z` plains with
   `flood`; `V`, `S`, `N` plains with `country` Valley, Sealed, Neck; `P` plains with `pass`.
   A glyph that hides a terrain (`r`, `z`, `V`, `S`, `N`, `P`) parses to plains, since the text
   does not say. **The round trip is at the text:** `ascii(parse(t)) == t`, and on each of the
   seven retro maps at the pick `ascii(parse(ascii(w.tiles)).tiles) == ascii(w.tiles)`. Hidden
   costs, food, flow and fertility are not in the text; layers carry them.
3. **The bare base.** A parsed world is built as `tests/common`'s `flat_radius` builds one:
   `World::new(Config { seed: 1, radius, army: true, gifts: true, zone: true, ..})`, no
   clans, no markets, `rng` 4151932303 and `next_id` 4, every tile's food, wear, claim, trail,
   river, flow, lake, flood, country, pass, hidden cost and span cleared and then set from the
   glyph. So every id and draw the existing tests rely on holds.
4. **Layers.** A layer is a grid in the same shape as the map, parsed by one shared grid
   parser (the map's own parser is that grid parser plus the glyph table). `marks(grid)`:
   every non-`.` character names its tile for `mark(c)`. `hidden(grid, table)`: each non-`.`
   character looks up `(Option<f64> trail_cost, Option<u8> span)` in `table` and sets the
   tile's hidden fields. `food(tile, good, amount)` sets `cap`, `unworn` and `stock` of that
   food, so a staged army has something to forage or pillage. `ascii()` draws the map with
   marks over it (`ascii_marked`).
5. **Placing.** `clan(tile, people) -> id` is `common::put`'s single tile clan; `civs(&tiles,
   people) -> Vec<u32>` opens a market on each tile, puts a clan of `people` there and founds
   the civs as `common::three()` does (founding's camp, scout and guard take 8 of 14);
   `lead(civ)` sends `ChooseCiv`, clears the events and fills the scouts' packs as
   `common::led()` does; `army(civ, tile, people) -> id` is `put_camp` (it moves the civ's
   camp onto `tile`) and sets its people, its stack and its pack to match.
6. **Driving.** `order(civ, Order)` is `World::order` (any civ); `act(Action)` is the
   player's door. `turn(n)` ends `n` turns; `until(max, pred) -> Option<u32>` ends turns
   until `pred(&World)` holds and returns how many it took, `None` at `max`. A refused order
   returns its `Err` text and is never unwrapped by the stage.
7. **Reading.** `revealed(civ, tile)`, `trail(tile)`, `route(civ, from, to) -> units::Plan`
   (the known and possible costs and paths), `people(unit)`, `store(civ)` (the food total),
   `holder(tile)` (`civs::tile_civ`), `events(kind)`, and `world()` for the rest.
8. **Retro.** `Stage::retro(size, seed)` clones the world at the pick (`Phase::Choose`) from
   a process-wide cache keyed by `(radius, seed)`: a `Mutex<HashMap<_, Arc<OnceLock<World>>>>`,
   so two tests asking for the same map run its pre-game once. The retro list
   (`RETRO: [(&str, &[u32]); 3]`) moves to `stage.rs`; `cli/src/retro.rs` reads it from there.
9. **The spec form.** `StageSpec` (serde, camelCase) is `{ map | retro: {size, seed}, marks?,
   hidden?, food?, civs?, people?, lead?, armies? }`, a tile named by `{mark: 'A'}` or
   `{tile: 12}`; `Stage::from_spec(&StageSpec)` builds it. A spec with `retro` runs the
   world's own pre-game when built (the wasm build has no cache); a spec with `map` is bare.
   The shape beyond that is the build's, with the same fields for the same things in the
   stage's builders.

## Settled: the cases (build-3)

10. **Where.** `core/tests/cases.rs`.
11. **Scout.** Each errand is staged by `Stage::retro`, led by the first civ, routed from its
    capital to the nearest rival capital or to `Lead`'s forest-or-hills tile 7 steps out, with
    the civ's store and the scout's pack refilled each turn. "Settle" is the scout's `built`
    true, or its route cleared with the scout alive, at the end of the turn it happens. Three
    tests:
    - **The owner's invariant, green on main** (`settled_scouts_leave_nothing_unrevealed_that_could_beat_them`):
      on all 14 errands of the seven retro maps, at settle no tile on `route(civ, from,
      to).possible`'s path that is unrevealed and untrailed has a possible step cheaper than
      its known one. It guards T.
    - **The search order, red on main** (`a_scout_searches_a_range_by_best_case`): on small 9
      and medium 8 to the rival, each turn the scout reveals a mountain, that mountain's best
      case is within `EPS` of the lowest best case among the mountains open at the turn's
      start. A mountain is open when it is unrevealed and its cost through it (possible cost
      capital → it, plus it → target) beats the known route; its best case is that cost plus
      the scout's walk to it (the scout's known cost to the cheapest tile beside it, 0 when it
      stands beside it). The message names the map, the turn, the mountain and its best case,
      and the better one left open with its own. This is the decided order in T's own words,
      so T makes it pass; small 9 to the forest (2 reveals, in order) is its control.
    - **The river bridge, red on main** (`a_settled_route_has_no_cheaper_trail`): on small 2
      to the rival and small 3 to the forest, at settle `possible.cost >= known.cost − EPS`.
      It fails because a trail on a revealed river tile (hills 144, hills 436) would cut the
      route by one and the scout builds trails only on its known path. Not T's: it stays red
      until a fix piece takes it.
    Each red test is `#[should_panic(expected = "…")]` with a stable fragment of its own
    message.
12. **Disband all.** Through the stage, on a camp, a scout and a guard each with fractional
    people (a camp of 4.6, a scout of 1.5, a guard of 2.5; their civ's clan gets them back):
    `Order::Disband { n: None, .. }` sends every person home at once, the unit is gone, and
    the clan's people rise by exactly the unit's. This passes on main: it is the control for
    the page arm (item 14), which is the red one. No Rust test sends the page's count: the bug
    is the page's form, and a Rust test that `n: Some(4)` removes 4.6 people would pin an
    engine change T doesn't make.

## Settled: the page (build-4)

13. **`Sim.stage(spec_json) -> Sim`** is a constructor like `Sim.load`; it builds
    `Stage::from_spec` and keeps the world. `window.game.stage(spec)` in `main.js` shows the
    play screen on a `Sim.stage(spec)` at the phase the stage is in (a led stage is at
    `main`), so `play.mjs` and the scripts can eval it.
14. **The page arm.** In `www/check-stage.mjs` (run by `check.mjs`, which is near 600
    lines), a staged game with an army of 4.6 people: take the
    camp panel's `Disband…` button, its `all` option, and make the order the page sends when
    the player presses Go as the stepper opens (the count 1, `www/panel.js`), apply it with
    `Sim.act`, and assert the army is gone and the clan is up by 4.6. Today one person goes
    home. The same at the option's cap (`n: 4`, 0.6 stays) is a second assert in the same
    case. It is wrapped in `expectBug(name, message, fn)`: a helper that fails if `fn` does not
    throw `message`, so T, which makes `all` a button that sends no count, finds the button
    its new way, keeps the asserts and deletes the wrapper. `check.mjs` also builds the camp,
    scout, guard, civ, clan and tile shapes from the staged game and checks every button has
    a label and an action, an arm, a paint or a stepper.

## Settled: the judge and the bench (build-5, build-6)

15. **The split.** `cli/src/judge.rs` becomes `cli/src/sweep_judge.rs` (the sweep's judge:
    `judge`, `judge_map`, the totals, `print_text`) and `cli/src/game_judge.rs` (the file's:
    `judge_game`, `GameJudged`, `print_game`, `rank_of`). Output byte for byte as before.
16. **The idle control.** `judge FILE` also plays the same map to the same pick with the led
    civ chosen at the pick and given no order (`EndTurn` only), the AI on every other civ.
    The AI skips the civ the player leads (`units/ai.rs`), so the idle civ is the led civ doing
    nothing, which is a different control from the all-AI one. The JSON and the text carry its
    Standing and rank at the end beside the game's and the AI control's.
17. **The economy group** in `engine::judge::Scores` (`economy`), read each turn by `Watch`
    over the civs that are alive, past the pick:
    - `store_mean`, `store_max`: the food total of a civ's store, mean over civs and turns, and
      the largest seen.
    - `turns_at_pack`: the civ-turns on which a store held at least a pack's worth,
      `units::pack_cap(UNIT.start)`.
    - `levy_per_turn`: the mean over civ-turns of the food the civ's market levy brought.
      This needs a number the rules don't keep: `Civ.levied`, an unsaved (out of `CivFlat`) per-turn
      counter set in `civs::levy` (and zeroed where the turn's levy begins), so no digest moves.
    - `largest_army`: the most people in one camp at any turn.
    - `guard_mean`: the guard's people, mean over civ-turns.
    The sweep's `Total` carries each of them (means over maps, the max of the maxes).
18. **The compare.** `sansfiat judge --compare A.json B.json [--all]`: flattens each report to
    `path -> number` (arrays of objects keyed by their `id`, or `size seed` for maps, else by
    index), lists the metrics side by side in group order (standing, doubt, decisions, motion,
    land, economy, armies, fight, health), the change (B − A), and a floor and a flag. By
    default the totals; `--all` adds the per-map metrics. A metric present in one report only
    is listed as "only in A/B", never flagged. A change is flagged when `|B − A| > floor`; with
    no floor in either report, nothing is flagged and the output says so.
19. **The floor.** `sweep --judge --json --floor [--nulls N]` adds `floor` to the report: the
    sweep once more per null perturbation, each with one harmless difference at the pick (one
    extra draw taken from the world's `rng` after the pick, so every later roll differs and no
    rule does), and per metric `max |null − base|` over the nulls. `N` is 1 by default, as the
    owner said ("the sweep once more"). The compare's floor for a metric is the largest of A's
    and B's.
20. **`bench --after N`.** The pre-game is run untimed (its time is printed on its own), then
    `N` turns past the pick with every civ on the AI are timed, and the line gives ms a turn.
    `--turns` keeps its meaning without it. `docs/architecture.md`'s Scale row names it.

## Settled: the buttons (build-7)

21. **The script.** `www/play/buttons.mjs` stages one game with `game.stage(spec)` (two civs
    on a small hand-built map with food, a camp placed beside food and beside a rival tile, a
    scout, the guard, the led civ), then for every selection kind (the led civ, a rival civ, a
    clan, a camp, a scout, a guard, a market, a tile) builds the panel and presses every
    button the led civ's own panels show, one at a time on a fresh stage, and checks the world.
    The inventory is `describe`'s: a label with no case in the script's table fails the script
    ("button X has no case"), so a new button forces a case. Cases, by label, with their
    check:

    | Panel | Button | Press | Check |
    |---|---|---|---|
    | Army | Move to… | arm, click tile | the camp's `dest` is the tile |
    | Army | Stop | on a marching army | `dest` is its own tile or none |
    | Army | Paint (P), Unpaint (U) | arm, click a tile | its zone gains the tile, then loses it |
    | Army | Forage | press | the order is accepted and its pack or its tile's stock moved after a turn |
    | Army | Pillage | press, army on a rival tile | the tile's food is pillaged (event `campPillaged`) |
    | Army, scout | Resupply | press | the pack rose, the store fell by as much |
    | Army, scout | Auto-forage, Auto-supply | press | the switch flips and its label says so |
    | Army | Undeploy | on a deployed army | its zone is empty |
    | Army | Raise… (warriors, archers) | each option, count 1, Go | the army's people rose by 1 in that kind |
    | Army, scout, guard | Disband… (each kind, all) | each option, Go | people home as the option says; `all` leaves nothing |
    | Scout | Move to…, Route from… | arm, click | the route's start and end are set |
    | Scout | Clear route | on a routed scout | no route |
    | Civ | Raise… (warriors, archers, a guard, a scout) | each option, 1, Go | the unit exists |
    | Civ | Gift −1, +1; Keep −1, +1, default | press | the civ's gift or keep moved by one, or is the default |
    | Civ | Claim (P), Unclaim (U) | arm, click | the civ's claim gains or loses the tile |

    Disband all is the case that is red today (item 14); in `buttons.mjs` it is pressed as a
    player presses it (the option chosen, Go at the count the stepper opens at, no count set),
    wrapped like the page arm, so T flips it. The staged game's exact numbers and the DOM driving
    (`page.button`, `page.click`, the stepper's `stepper()`) are the build's.

## Unsettled, and which step holds it

Nothing in the rules. Two measurements are the build's, each in its step, with its proof:

- build-6: whether `--nulls 1` gives usable floors. It records main's floors, and a compare
  of main against a perturbed run, in `relay/specs/151/floor.md`. If most metrics' floors are
  0 (a single null is too lucky), it says so and sets the default to 3, as a decision.
- build-3: if the search-order case is not red on small 9 or medium 8 to the rival, or the
  river-bridge case on small 2 or small 3, as spec-review's probe says (main may move), it
  records the run in `cases.md` and finishes
  `pending` rather than weakening the check.

## What the review runs

- `engine/check.sh`.
- `cargo test -p engine --test cases` and `node www/check.mjs`: the search order and the page's
  Disband all are red for the stated message, the river bridge too (read `cases.md`).
- `sansfiat sweep --judge --json > A.json` on main (a worktree) and on the branch, then
  `sansfiat judge --compare A.json B.json`: no existing metric flagged, the economy group "only
  in B". Then `sweep --judge --json --floor` on the branch and the compare again.
- `sansfiat bench --size large 8 --after 50`: a ms a turn is printed.
- `node www/play/buttons.mjs`, then change one order in `panels.js` and see it fail.
- `sansfiat judge` on a played game (`sansfiat story`'s save or a `play --game` file) shows
  the idle control.

## Decisions I made that the piece left open

1. **Round trip at the text, not the tile.** A glyph hides some terrain (`r`, `V`, …), so
   `parse(ascii(t))` is plains there; the guarantee is that the text comes back the same.
2. **A food layer.** Staged armies forage and pillage, and the base has no food, so `food(tile,
   good, amount)` is part of the stage.
3. **The cases that are red on main are `should_panic` / `expectBug`**, not `#[ignore]`: CI
   holds the failure, and its message is the evidence. T deletes the attribute or the wrapper.
4. **`StageSpec`** is the data form so the page, the CLI and the tests build one stage the
   same way. The wasm retro stage runs the pre-game; the cache is native only.
5. **The retro list moves to core** (`stage::RETRO`) because the stage's `retro` needs it and
   the CLI already reads it.
6. **The scout's red case is the search order, in T's decided words** (spec-review): the
   owner's settle invariant holds on main on every retro errand, so it is a green guard, and
   what is wrong on main is the nearest-first order the owner saw. The best case is the cost
   through the mountain plus the scout's known walk to beside it. The river-bridge gap the
   spec step found is a third, separate red case and a fix idea, not T's.
7. **`Civ.levied`**, a derived counter, for "levy a turn": no rule, no digest.
8. **The floor** is `max |null − base|` over `--nulls N` (1 by default) runs, with one extra
   `rng` draw at the pick as the harmless difference.
9. **`pack's worth`** is `pack_cap(UNIT.start)`.
10. **`judge.rs` splits into `sweep_judge.rs` and `game_judge.rs`.**
11. **The idle control is the led civ with no orders**, not the all-AI run (which the file's
    judge already has).
12. **Disband all's Rust half passes on main and is the control;** only the page's form is
    red, pressed as the player presses it (Go at the count the stepper opens at, 1), and at
    its cap. No Rust test sends the page's count (spec-review).

## Spec-review

Checked against the code and the maps on main (a throwaway probe, not committed):

- **The scout's red case was the wrong bug.** At settle on small 2 and small 3, every step
  where the possible path beats the known one is a revealed river tile (hills 144, hills
  436), so "nothing unrevealed could beat the route" holds on main. The owner's bug, as the M3
  retro traced it, is the nearest-first order, and T's decided fix is the order by best case:
  the red case is now that order (12 of 29 reveals out of order on small 9, 4 of 11 on
  medium 8, 0 of 2 on the control), the owner's invariant is a green guard on all 14 errands,
  and the river bridge is a third red case, not T's (decision 6, items 4 and 11).
- **The errand needs the scout's pack refilled**, not only the store: small 3's scout starves
  on turn 16 otherwise.
- **Disband all on the page sends 1, not the cap**: the stepper opens at 1. The page arm and
  `buttons.mjs` press Go as it opens, and at the cap; the Rust shadow of the page's count is
  dropped, since it would pin an engine change T doesn't make (items 12 and 14).
- **`map::parse` takes the radius from the first row**: `sansfiat map` prints a legend and
  counts after the rows, so the row count can't give it (item 1).
- **`Civ` serialises through `CivFlat`**, so `levied` stays out of `CivFlat` rather than
  carrying `#[serde(skip)]` (build-5).
- **`check.mjs` is 542 lines**, so the staged arm goes in `www/check-stage.mjs` (build-4).
