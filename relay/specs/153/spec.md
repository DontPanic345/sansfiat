# #153 Scouts that work: spec

## What it delivers

A scout does what its buttons say and lives through its errand, and a trail makes a difference
wherever it is.

1. **The flat trail.** A trail on plains or coast brings the tile to three quarters of its bare
   cost, worn by traffic as now. `scouting.md`'s "flat land gains nothing from a trail" is
   reworded.
2. **The scout's orders.** Move to… is a new scout order: it walks to a tile and stays there,
   foraging. Route from… and Route to… set the route as today's buttons do, under their right
   names. Each of the three has a visible result.
3. **The search by best case.** The search orders a range's mountains by the cost through them
   plus the walk there. The panel's Unrevealed row says why a mountain is left: it can't beat
   the known route, with the two costs.
4. **The scout's feeding.** A scout forages only what a tile regrows and leaves no tile fallow.
   A hungry scout leaves its errand for rich ground nearby, eats its fill and goes back. An
   idle scout waits on rich ground, not at the bare capital. Its hunger has a reason, and Move
   to… and Route to… warn past the line's reach.
5. **Disband all** is a button that sends no count, for every unit kind.
6. **The errand test**: `scout_errand` on the stage, every scout alive at 50 turns.

## The question and how it is measured

**The question:** decisions and health (`docs/process.md`, Scores). The scout's three buttons
each have a visible result. The scout lives through its first errand on every retro map. A
trail on flat land moves trade and Standing on the retro maps against main without stilling
the map.

| Score | Measure | Control |
|---|---|---|
| Decisions | **The three buttons each have a visible result**, pressed in `www/play/scout.mjs` on small seed 2 (a retro map) and in `buttons.mjs` on the staged game. Move to…: the scout's `dest` is the tile, its route to it is drawn and the panel reads "Going to …", and after enough turns it stands there. Route from…: a start mark is drawn on the tile and the hint names it. Route to…: the route is set from the start (or the scout's tile) and its known and possible paths are drawn. Disband all on a camp of 4.6, a scout and a guard sends every person home (`check-stage.mjs`, the four `expectBug` wrappers gone). | Main: Move to… sets a route (`scout.mjs`), Route from… has no mark, and Disband all keeps 0.6 of the camp. |
| Health | **Every scout lives through its first errand**: `core/tests/scout_errand.rs` on the stage. On each of the seven retro maps the first civ's scout gets four errands, each with no refill: routed to the nearest rival capital, routed to `Lead`'s target, moved (Move to…) to `Lead`'s target, and left idle. All 28 are alive at 50 turns. The table it prints (hungry turns, people lost) goes in `errand.md`. | Main: 11 of 14 routed errands lost (`probe.md`). |
| Health | **The AI's idle scouts live**: on `sweep --judge`'s runs no AI scout is lost by +200 (the canvas counts them; see build-5). | Main's sweep. |
| Standing, motion | **The flat trail moves trade and Standing without stilling the map**: `sweep --judge --json --floor` on main and on the branch, `judge --compare`. Trading share at the pick and at +200, and the aligned share at +200, rise past the floor. Every pre-game ends by centres. The AI's orders per 100 turns stay above half of main's, and land changing hands stays above half of main's. | Main (a worktree at the branch's base). |
| Health | `check.sh` green. The flat trail and the feeding move `stats::digest`, so `SAVE_VERSION` bumps and the pins move, once per build that moves them. `cases.rs` keeps the owner's invariant green, and `a_scout_searches_a_range_by_best_case` loses its `should_panic`. | |

**Before the piece** (`probe.md`, run by this step): a flat trail at 0.75 raises the trading
share from 0.898 to 0.929 at the pick and the aligned share from 0.209 to 0.236, against floors
of 0.000 and 0.008. Every pre-game ends by centres on the seven maps and on seeds 1–10 of every
size, and the AI's orders fall by 32%, not by half. Main loses 11 of 14 routed errands. A
one-person scout loses 12, so the person count is not the lever. The feeding rule is.

## Shape: Easel and Canvas, with the flat trail moved into the easel

The piece suggests the flat trail as the canvas. The spec step measured it instead
(`probe.md`): 0.75 meets both bounds the piece names and moves trade and Standing, so it is
settled. The flat trail also changes every retro map's pre-game. It goes first, so every later
step's tests, pins and red-case texts are written on the maps the piece ships with.

The unsettled work is the scout's feeding: how far it wanders, when it counts as fed, and
whether it needs one person or a bigger pack. Only the 28 errands on the retro maps can judge
that. It sits on top of the orders and the feeding rule, so it is the canvas. The canvas also
re-measures the flat trail on the finished branch, and may move it if a bound fails.

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | easel | The flat trail: `data.rs`, `map.rs`, `maptext.rs`, `stage.rs`, the fallout on the retro maps, `scouting.md`'s line | Every later step stands on the new maps |
| build-2 | easel | The search and the orders, in core and `Sim`: `scout.rs` splits into `units/search.rs`, best-case order, the reasons a mountain is left, the scout's `Move`, `line_from` and the reach notes | The page and the feeding read these |
| build-3 | easel | The feeding, in core: regrow-only foraging, the wander, the idle scout on rich ground, the hunger reason, `SCOUT`'s starting numbers, `scout_errand` on the stage | Needs `Move` (build-2) for its errand |
| build-4 | easel | The page: the scout's row, the start mark, the warnings, the Unrevealed row, the hunger line, Disband all for every unit, the scripts | Needs every view field from builds 2 and 3 |
| build-5 | canvas | The feeding's numbers against the 28 errands and the AI's scouts; the flat trail re-measured against main; `errand.md`, `trail.md` | Needs all of it |

## Settled: the flat trail (build-1)

1. **`TRAIL.flat: 0.75`** in `data.rs`: the share of its bare cost that a trail brings plains
   and coast to, with a `note` (M4 answer 4; `probe.md`). It is a share, not a roll: every flat
   tile gets the same, so the map draws nothing new and the rest of the map rolls as before.
2. **The map gives it.** A helper `map::flat_cost(terrain) -> Option<f64>`
   (`Some(bare × TRAIL.flat)` for plains and coast, else `None`) sets `trail_cost` in
   `roll_trails`, in `map::parse` (`maptext.rs`, which otherwise clears it) and in
   `Stage::plain`, so a parsed or plain stage walks as a generated map does. `Stage::paint`
   keeps what it is given. Marsh and desert get none: a marsh trail only bridges its river
   (`scouting.md` Decided), and desert isn't named.
3. **Nothing else in the rule changes.** `step_cost` already prices a trailed tile at the lower
   of bare and `trail_cost`, `tend` wears and clears flat trails as now, and a worn trail
   already bridges a river. The scout only wears flat trails: `TRAIL.work` gets no plains or
   coast entry, so `needs_trail` never picks one. A flat tile is never a reveal candidate,
   because its known and possible costs are equal: `tile_cost` prices an untrailed unrevealed
   flat tile at bare both ways, and a trailed one at its trail both ways. The hidden-costs
   debug layer already draws only forest, hills and mountain.
4. **A stage proves it** (`core/tests/wear.rs` or `land.rs`): on a plain stage a plains tile
   whose traffic reaches `TRAIL.on` costs 0.75 to enter, a coast tile likewise, a marsh tile
   keeps its bare cost, and a clan's walk across a worn plains run is cheaper by the quarter.
5. **The fallout.** The retro maps' pre-games change, so tests pinned to a retro map's numbers
   move. `SAVE_VERSION` bumps and the `cli/tests/save.rs` pins move. The two red cases in
   `core/tests/cases.rs` keep failing for their own reason: if one no longer fails on its maps,
   widen it to the 14 errands, pin the new first failure's text and record it. If none fails,
   record that in `flat.md`: for `a_settled_route_has_no_cheaper_trail` the test becomes a
   green guard, and for the search order the red case moves to whichever errand fails.
   `www/play/scout.mjs`'s `TARGET` and `script.rs`'s `Lead` target are found by rule (the first
   forest or hills tile seven steps out) and may move: check that both still find their route.
6. **`scouting.md`**: the sentence "Flat land gains nothing from a trail." becomes "A trail
   brings flat land down a little; walking wears one there, and a scout builds trails only on
   rough ground and mountains." (The piece decides this edit; the rest of the prose stays.)

## Settled: the search and the orders (build-2)

7. **The split.** `units/scout.rs` is 630 lines. The route search moves byte for byte to
   `units/search.rs` (`revealed`, `mountain_may_bridge`, `tile_cost`, `may_trail`, `step_as`,
   `needs_trail`, `trail_left`, `Plan`, `route_plan`, `walk_costs`, `beside`, `approach`,
   `ordered`, `search`, `reveal`, with `march`, `scout_event`, `clear_route` and `blocked`
   where both need them). `scout.rs` keeps the state, the turn and the feeding. Move first,
   with the digest unchanged, then change.
8. **Best-case order.** In `ordered`, mountains stay the first tier. Within it each mountain is
   keyed by its best case: the cost through it (`through`, as now: the possible route from the
   route's start to it plus from it to the end) plus the walk to it. The walk is 0 when the
   scout stands on the mountain or beside it, else the least known-priced cost
   (`step_as(.., false)`, the pricing `route_plan`'s `known` uses) from the scout's tile to a
   tile beside it. Ties go to the lower tile. This is `best_case` in `core/tests/cases.rs`
   exactly: one known-priced cost map from the scout's tile gives every walk. The second tier
   (forest and hills) keeps its order: nearest, then the biggest saving. A mountain the scout
   can't reach is skipped as now, by `approach`. `a_scout_searches_a_range_by_best_case` loses
   its `#[should_panic]` and passes. The invariant and the forest control stay green.
9. **Why a mountain is left.** `pub fn search_view(w, unit) -> Option<SearchView>` in
   `search.rs`, with `next: Vec<usize>` (the search's order, as `ordered` gives it) and
   `left: Vec<(usize, f64)>`: the unrevealed mountains of every range that touches the
   possible path or the known path (a mountain on either, or beside a tile of either, and every
   unrevealed mountain joined to it), whose cost through is not below the known cost by more
   than `EPS`, each with that cost. They are sorted by cost, then by tile, ten at most.
   `Sim.view()`'s scout row gains `search: {next, left: [{tile, through}], known}` when it has
   a route.
10. **The scout's `Move`.** `Order::Move { unit, to }` takes a scout as well as a camp: the same
    check (a land tile, a walk there), with the scout named in its errors. A scout's `Move`
    sets `ScoutState.dest` (written to `UnitFlat.dest`, which the camp already carries) and
    clears its route. `Route` clears `dest`. A `Move` to its own tile clears `dest`, as a
    camp's does. In `scout_turn`, a scout with a `dest` and no route walks there (`march`, the
    labour after its feeding, part-paid steps banked) and then stays. It forages round it as
    every scout does. `line::route` (the page's march path) reads a scout's `dest` as it reads
    a camp's. Notes: "going to X, N steps away", then "at X, foraging".
11. **The line's reach.** `pub fn line_from(w, unit, tile) -> Option<(f64, f64)>` in
    `line.rs`: the cost and carry of the unit's line as if it stood on `tile`, by `line`'s
    rules (rival land and held ground block it, and carry uses all its labour). A tile is
    **past the line's reach** when there is no line, or it carries less than the unit eats
    (`people × UNIT.food`). `Move` and `Route` add a note when their tile is past the reach
    (`Route`: its end): "past its line's reach: its line there would carry 0.8 of the 2 it
    eats". `Sim.lineFrom(unit, tile)` gives `{cost, carry, eats}` or `null`, for the page's
    hover.
12. **The CLI's view** (`cli/src/view.rs`): a scout's line says where it is going (`dest`)
    and, once build-3 lands, whether it is wandering. It stays under 80 lines.
13. Tests on the stage (`core/tests/scout.rs`, split into `scout_orders.rs` if it passes 400
    lines): a scout moved four steps out walks there, stays, and keeps no route; `Route`
    after `Move` clears `dest`; `Move` to a sea tile is refused; `search_view.left` on a staged
    range names a mountain whose cost through is above the known route's, with that cost;
    `line_from` past a long walk reads past the reach and the note says so. `SAVE_VERSION`
    bumps if the digest moves (`dest` on a scout is new state), and the pins with it.

## Settled: the feeding (build-3)

14. **A scout forages only what a tile regrows.** `land::yield_of(t, f)` is the sustainable
    yield `wear` uses (`regen × cap`, or `(regen / 4 + restock) × cap` for game), moved out of
    `wear` so the two can't drift. A tile's **spare** for a scout is, per food,
    `min(stock, max(0, yield − take))`. `take` is the tile's take this turn: `harvest` resets
    it and the clans' harvest fills it before `units` runs, so a scout gets only what is left
    of the tile's yield after the clans. A fallow tile spares nothing. Every scout take
    (auto-forage, the one-shot Forage and Pillage, at the scout's tile or round it) takes at
    most the spare, adds what it took to `take` (so a second scout sees it), and **never sets
    `fallow`**. Pillage keeps its rate. Armies forage as now (`strip_at` with fallow).
15. **Rich ground**: a tile within `SCOUT.wander` steps of the scout that it can walk to, whose
    round (itself and its tiles within `SCOUT.reach`, as `forage_round` reads them) spares at
    least the scout's eating for a turn (`people × UNIT.food`). The nearest by walk cost wins,
    then the lower tile. With none in range, it takes the tile in range whose round spares the
    most; with nothing spared in range, it stays where it is.
16. **A hungry scout wanders.** At the start of its feeding, a scout whose pack holds less than
    `SCOUT.low` turns of its eating, or that is hungry, leaves its errand. Its route and its
    `dest` stay set. `ScoutState.wander: Option<usize>` (`#[serde(default)]`) holds the rich
    ground it goes to. It walks there and forages round it, and auto-supply runs as now. While
    it wanders it doesn't search or walk to its `dest`. Once its pack holds `SCOUT.fed` turns
    of eating, `wander` clears and the errand resumes next turn. If its rich ground no longer
    spares its eating when it gets there, it picks again. Notes: "leaves its route to forage at
    X, N steps away", "fed: back to its route" (or "back to X" for a `dest`).
17. **An idle scout waits on rich ground.** A scout with no route and no `dest` doesn't walk
    home to the capital. It goes to rich ground (item 15) and stays there, foraging. It picks
    again only when its tile's round no longer spares its eating. Note: "waits on rich ground
    at X".
18. **The hunger has a reason.** Each turn a scout eats less than its need, its note says what
    the turn brought: "goes hungry at X: foraging brought 0.4 and its line 0.3 of the 2 it eats;
    its line costs 14 and carries 0.5 a turn from here". With no line it says "no line reaches
    it". The `ScoutHungry` event stays once, on the first hungry turn.
19. **Starting numbers**, to run and for the canvas to tune: `SCOUT.wander: 4` (steps),
    `SCOUT.low: 1.0` and `SCOUT.fed: 3.0` (turns of its eating in the pack; the pack holds
    `UNIT.pack`, 5), `SCOUT.reach` and `SCOUT.people` as now (1 and 2). Each has a `note`.
20. **`scout_errand` on the stage** (`core/tests/scout_errand.rs`, rewritten on `Stage::retro`
    as `cases.rs` does): on the seven retro maps the first civ is led. Its scout gets four
    errands, one per stage, with no refill: routed to the nearest rival capital, routed to
    `Lead`'s target, moved to `Lead`'s target (`Order::Move`), and idle. Each runs 50 turns.
    It prints the table (map, errand, hungry turns, people lost, alive, turns wandering) and
    asserts every scout is alive. If the starting numbers don't keep all 28 alive, the
    assertion stays and the test is `#[ignore = "build-5 tunes the feeding"]`, with the
    table in `feeding.md`. The canvas removes the attribute.
21. **The other scout tests.** `an_idle_scout_at_the_capital_neither_forages_nor_starves`
    becomes "an idle scout waits on rich ground and doesn't starve": on a stage with food two
    steps from the capital and none at it, the idle scout goes there, forages, keeps its
    people over 50 fed turns, and no tile it foraged is fallow. New stage tests: a scout never
    takes more than a tile's spare and never leaves it fallow, while an army on the same tile
    does; a scout whose pack runs low leaves its route for rich ground and comes back to it
    fed; the hunger note names the turn's forage and line. `SAVE_VERSION` bumps and the pins
    move.
22. **The view.** `Sim.view()`'s scout row gains `dest`, `wander` and `hunger`
    (`{forage, line, eats, cost, carry}` from the last turn it went hungry, else `null`). The
    hunger is kept on the scout as derived state (`#[serde(skip)]`, rebuilt by a replay), or
    read from its notes: the build picks, but a loaded game's panel must show it after one
    turn.

## Settled: the page (build-4)

23. **The scout's row** (`www/panels.js`'s `scoutShape`), for the player's scout:
    - **Move to…** arms `'move'`, and a map click sends `{type: 'move', unit, to}`. Hint:
      "Click the map for where the scout goes; Esc cancels".
    - **Route from…** arms `'start'`, and a click sets the start. The start is **drawn on the
      map** (a ring on the tile, in the routes layer) until a route is sent or Escape clears
      it, and the hint reads "Route from (x, y): now Route to…".
    - **Route to…** arms `'to'`, and a click sends `route` from the start or the scout's tile:
      today's "Move to…", renamed.
    - **Clear route** while it has a route. **Stop** while it has a `dest` (a `move` to its own
      tile).
    - Forage, Resupply, the two switches, **Disband…** and **Disband all**, as item 26 has
      them.
    - Right-click moves a selected scout of the player's, as it moves a camp, and the tile
      tooltip's "Move here" line shows for a scout too.
24. **What the scout is doing** (its `doing` line): "Searching A → B: known …, possible …"
    (as now); "Going to X: N turns" with its march path drawn (as a camp's); "At X,
    foraging"; "Foraging at X until fed, then back to its route" while wandering; "Waiting on
    rich ground at X" when idle. A hungry scout's line leads with "Hungry N turns: loses a
    person at 3", and a number row **Hunger** gives the reason from `hunger`: "foraging 0.4,
    line 0.3 of 2 it eats".
25. **The Unrevealed row**: its value is "N to check" (`search.next.length`), and when
    `search.left` has any, "; M left". The panel's list gains **Left unrevealed**, a row per
    mountain: "(x, y): 31.0 through it, can't beat the known 28.0". Clicking a row centres the
    map on the mountain. Its tooltip: "Mountains the search won't reveal: the cheapest route
    through each costs at least the route it knows".
26. **Disband all.** Every unit's row (camp, scout, guard) has a **Disband all** button that
    sends `{type: 'disband', unit}` with no count. **Disband…** keeps its counts: for a camp,
    one option per fighter kind it has; for a scout or a guard, an option `people`
    (cap `floor(people)`), shown only when it has two or more people. Its `all` option goes.
    `check-stage.mjs`'s `pressAll` presses Disband all, and its four `expectBug` wrappers go,
    with the messages kept.
27. **The warnings.** With Move to… or Route to… armed, the hover tooltip adds "Past the
    line's reach: its line there carries 0.8 of the 2 it eats" when `sim.lineFrom(unit, tile)`
    says so ("no line reaches it" for `null`). After the order, the scout's reasons carry the
    engine's note.
28. **The scripts.** `www/play/scout.mjs` presses each of the three on small seed 2 and checks
    the world: Move to… sets `dest` to the clicked tile, sets no route, draws the march path,
    and after enough End turns the scout stands there; Route from… draws the start mark
    (checked through the page's state or a pixel); Route to… sets the route from the start;
    Clear route clears it. It saves a screenshot of each. `buttons.mjs`'s scout cases press
    Move to…, Route from… then Route to…, Stop, Disband all for the camp, scout and guard,
    and Disband… on a scout of two.

## Unsettled, and which step holds it

All in build-5, the canvas:

- **How far a hungry scout wanders, when it counts as fed, and whether it needs one person
  or a bigger pack** (`SCOUT.wander`, `low`, `fed`, `reach`, `people`, and a `SCOUT.pack` in
  place of `UNIT.pack` for the scout if a bigger pack is the answer). What proves it:
  `scout_errand` passes with no `#[ignore]`, every one of the 28 scouts alive at 50 turns on
  the seven retro maps, and none of the AI's scouts lost by +200 on `sweep --judge`'s runs.
  Among the settings that pass, take the one that spends the fewest scout-turns wandering on
  the routed errands, so the scout spends its turns on its errand. The probe says one person
  alone does not do it.
- **The flat trail on the finished branch.** Re-run `sweep --judge --json --floor` on the
  branch and on main, then `judge --compare`. Keep 0.75 while trade and Standing rise past the
  floor, every pre-game ends by centres, and the AI's orders and land changing hands stay
  above half of main's. If a bound fails, try 0.85 and 0.65 and take the value nearest 0.75
  that holds them all. Report lead changes; they are not a bound (`probe.md`: noise at
  this count).

## What the review runs

- `engine/check.sh`.
- `cargo test --release -p engine --test scout_errand -- --nocapture`: all 28 alive, and the
  table matches `errand.md`.
- `cargo test -p engine --test cases -- --nocapture`: the search-order case green with no
  `should_panic`, the invariant and the control green.
- `sansfiat sweep --judge --json --floor` on main (a worktree) and on the branch, then
  `judge --compare`: trade and Standing past the floor, pre-games by centres 7 of 7, AI orders
  and land motion above half of main's. The totals go in the PR.
- `node www/play/scout.mjs` and `node www/play/buttons.mjs`, with a look at the scout
  screenshots: the start mark, the march path, the Unrevealed list, a hungry scout's line.
- A played game (`sansfiat play --game g.json`, small seed 2): send the scout on Move to… past
  the line's reach and read the warning, route it to the nearest rival, and play 50 turns. The
  scout lives, and its notes say when it wanders and why it went hungry.

## Decisions I made that the piece left open

1. **The flat trail is settled at 0.75 by the spec step, and moved into the easel**
   (`probe.md`). It meets both bounds the piece names (pre-games by centres, AI orders not
   halved) and moves trade and Standing past the floor. The canvas re-measures it.
2. **The flat trail is a share, not a roll**: every plains and coast tile gets three quarters
   of its bare cost. Nothing is hidden, so the scout never reveals a flat tile.
3. **The scout only wears flat trails; it builds none** (the designer settles). A scout finds
   and improves routes across hard country (`scouting.md`). A flat trail saves a quarter of a
   step, so building them would turn the scout into a road gang on every plains route. The
   clans' traffic wears them where people walk.
4. **Marsh and desert get no flat trail**: the piece names plains and coast, a marsh trail
   only bridges its river (Decided, #74 and #76), and desert is a barrier by design (#102).
5. **`map::parse` and `Stage::plain` give flat tiles their flat cost**, so a staged map walks
   as a generated one. `Stage::paint` keeps what it is given.
6. **A scout's `Move` clears its route, and `Route` clears its `dest`**: a scout has one
   errand at a time. A hungry scout keeps both while it wanders and goes back to the one it had.
7. **Clear route stays in the scout's row, and Stop joins it while it moves**: without them a
   route or a move could only be replaced, not ended.
8. **Right-click moves a selected scout** as it moves a camp: one gesture for every unit that
   moves.
9. **Route from… is visible as a ring on its tile** until the route is sent or Escape clears
   it. That is its visible result: it sets no state in the engine.
10. **Disband… keeps its counts and Disband all is its own button**. A scout or guard keeps a
    count of people, so a two-person scout can be made one.
11. **A scout's spare is what is left of the tile's sustainable yield after the clans' harvest
    this turn**: "only what a tile regrows" read against everyone who eats there, so a scout
    never pushes a tile past its yield. Its take doesn't count toward `wear`, which has run by
    then, as an army's doesn't.
12. **Hungry means a pack under `SCOUT.low` turns of eating, or hungry; fed means `SCOUT.fed`
    turns in the pack**. Pack turns are the scout's own clock, and they read the same for one
    person or two. A wandering scout doesn't search: its labour goes on getting fed.
13. **An idle scout no longer walks home to the capital.** `scouting.md` doesn't ask for it,
    and the piece asks that it wait on rich ground.
14. **"Past the line's reach" is a line that can't carry what the scout eats with all its
    labour**: the line's own measure (`supply.md`), read from the tile the order names.
15. **A mountain is "left" when it is in a range that touches the known or possible path, and
    its cost through doesn't beat the known route**. Those are the mountains a player would
    expect the scout to try.
16. **The best case's walk is priced as the known route prices it**, as the owner's test
    (`cases.rs`, `best_case`) does, so the engine and the test agree to `EPS`.
17. **The errand test gives each scout four errands**: the two routes, Move to…, and idle. The
    piece's new order and its idle rule are both errands a scout must live through.
18. **The scout keeps two people unless the canvas finds otherwise**. The probe shows one
    person loses as many errands (12 of 14 against 11).
19. **The camp's Move to… arms its own `'move'` click**, so `'to'` means only a route's end.
    One name per click, and the scout and the camp share Move to….
20. **Escape clears the route's start mark after painting and before the selection**: it is
    one more armed thing, so it takes its place in `page.md`'s Escape order (#103).
