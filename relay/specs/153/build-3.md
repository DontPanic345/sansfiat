# #153 build-3 (easel): the scout's feeding

A scout forages only what a tile regrows, leaves its errand for rich ground when it runs low,
waits on rich ground when idle, and says why it went hungry. Read `spec.md` items 14–22 and
`probe.md` (why scouts die today: they strip their ground and leave it fallow, past a line that
can't feed them). Builds 1 and 2 are in: flat trails (`flat.md`), and the search split into
`units/search.rs`, with the scout's `Move` and `dest`, `line_from` and `past_reach`
(`search.md`). The code: `core/src/units/scout.rs` (`scout_turn`, `forage_round`, `bring`,
`near`, `line_price`), `core/src/units/mod.rs` (`strip`, `strip_at`, `supply`, `room`, `step`,
`UnitFlat`), `core/src/land.rs` (`regrow`, `wear`), `core/src/clans.rs` (`harvest`, where
`take` is reset and filled), `core/src/world.rs` (`TURN`: `harvest` and `wear` run before
`units`), `core/src/data.rs` (`ScoutRules`, `FOOD`, `UNIT`), `core/tests/scout.rs`,
`core/tests/scout_errand.rs`, `core/tests/cases.rs` (`errand`, for building an errand on the
stage), `web/src/lib.rs`.

## What to build

1. **`land::yield_of(t: &Tile, f: usize) -> f64`**: the sustainable yield `wear` computes
   (`regen × cap`, or `(regen / 4 + restock) × cap` for a food that breeds). `wear` calls it.
   **`land::spare(t: &Tile) -> Basket`**: per food, `min(stock, max(0, yield_of − take))`, and
   nothing at all on a fallow tile.
2. **A scout's take** (spec item 14). A new `graze_at(w, id, tile, labour, room, rate)` beside
   `strip_at` in `units/mod.rs` takes food at `rate` a unit of labour, at most the tile's
   `spare` (each food in proportion to its share of the spare), adds what it took to the
   tile's `take`, and **never sets `fallow`**. Every scout take uses it: `forage_round`, and
   the one-shot Forage and Pillage in `scout_turn`. Armies keep `strip`/`strip_at`.
   `forage_round` reads `total(&spare(t))` where it read `total(&t.stock)`, and the fallow
   filter goes (a fallow tile spares nothing).
3. **Rich ground** (spec item 15): `fn rich_ground(w, id) -> Option<usize>`. The tiles within
   `SCOUT.wander` steps that the scout can walk to (`path_between`). A tile's round is
   `near(w, t, SCOUT.reach)`, walkable or the tile itself, as `forage_round` reads it. The
   first choice is the nearest tile, by walk cost, whose round spares at least
   `people × UNIT.food`, ties to the lower tile. Failing that, the tile whose round spares the
   most. If nothing in range spares anything, `None`.
4. **The wander** (spec item 16). `ScoutState` gains `wander: Option<usize>`, saved in
   `UnitFlat` with `#[serde(default)]`. At the start of the scout's feeding: if its pack holds
   less than `SCOUT.low × people × UNIT.food`, or `hungry > 0`, and it isn't already
   wandering, it sets `wander` to `rich_ground` and notes "leaves its route to forage at X, N
   steps away" ("leaves for X…" with a `dest` and no route). While `wander` is set, the scout
   marches there first, then forages round it (`forage_round` from its tile) and auto-supplies
   as now. It neither searches nor walks to its `dest`. When its pack holds
   `SCOUT.fed × people × UNIT.food`, `wander` clears and it notes "fed: back to its route" (or
   "back to X"). The errand resumes next turn. Arriving at rich ground whose round no longer
   spares its eating, it picks again. If `rich_ground` is `None`, it stays and the line feeds
   what it can.
5. **The idle scout** (spec item 17). With no route and no `dest`, it sets `wander` to
   `rich_ground` (once, noting "waits on rich ground at X") and stays there. It never clears
   `wander` for being fed, since there is no errand to go back to. It picks again only when
   its tile's round spares less than its eating. A route or a `Move` clears `wander` (in
   `order_route` and `order_move`).
6. **The hunger reason** (spec item 18). Track what this turn's foraging and line brought.
   Each turn the scout eats less than its need, note "goes hungry at X: foraging brought
   {f:.1} and its line {l:.1} of the {eats:.0} it eats; its line costs {cost:.1} and carries
   {carry:.1} a turn from here", or "…; no line reaches it". Keep the turn's numbers on the
   scout as derived state (`#[serde(skip)]` on `ScoutState`, outside `UnitFlat`'s saved
   fields, rebuilt as the turn runs), so `Sim.view()`'s scout row can give
   `hunger: {forage, line, eats, cost, carry}` (or `null` when it ate its fill). The row also
   gains `wander`. The `ScoutHungry` event stays on the first hungry turn only.
7. **`data.rs`**: `ScoutRules` gains `wander: usize` (4), `low: f64` (1.0) and `fed: f64`
   (3.0), each with a note. `reach` and `people` stay. These are starting numbers: build-5
   tunes them.
8. **`scout_errand` on the stage** (spec item 20). Rewrite `core/tests/scout_errand.rs` on
   `Stage::retro`, building errands as `cases.rs`'s `errand` does. On each of the seven
   retro maps there are four errands, each its own stage, with no refill: `Route` to the
   nearest rival capital, `Route` to `Lead`'s target, `Move` to `Lead`'s target, and no order
   (idle). Each runs 50 turns. Print a table (map, errand, hungry turns, people lost, turns
   wandering, alive) and assert every scout is alive at 50 turns, naming the ones that
   aren't. If the starting numbers don't keep all 28 alive, keep the assertion, mark the test
   `#[ignore = "build-5 tunes the feeding"]`, and say so in its doc. Either way, put the table
   in `relay/specs/153/feeding.md`.
9. **The other tests** (spec item 21), on stages:
   - `an_idle_scout_at_the_capital_neither_forages_nor_starves` becomes
     `an_idle_scout_waits_on_rich_ground`: on a plain stage with food on a tile two steps from
     the capital and none at it (`Stage::food`), the idle scout goes there, forages, keeps its
     people over 50 turns with the store empty, and no tile it took from is fallow;
   - a scout foraging a tile takes no more than its `spare` and leaves it unfallowed, while an
     army stripping the same tile leaves it fallow;
   - a routed scout whose pack is emptied leaves its route for rich ground, refills to
     `SCOUT.fed` turns, and resumes: its route is unchanged and it searches again;
   - the hunger note names the turn's forage and line.
10. **Save version**: `wander`, the graze and the idle rule move `stats::digest`.
    `SAVE_VERSION` bumps and the pins move.

## Proof and record

`engine/check.sh` green. `cargo test --release -p engine --test scout_errand -- --nocapture`
(with `--ignored` if you had to mark it): its table in `feeding.md`, beside build-2's table
from `search.md`. Then `sansfiat sweep --judge --json` on the branch before and after this
build, and `judge --compare`: put the groups that moved in `feeding.md`. The AI's idle scouts
now wander, so a small move is expected; say what it is.

## What later steps rely on

The feeding rule runs with its numbers in `SCOUT` (`wander`, `low`, `fed`, `reach`, `people`).
`scout_errand` is the canvas's proof, ignored or not. `Sim.view()` gives a scout's `wander`
and `hunger`. Build-4 draws them, and build-5 tunes the numbers.

## Done

`engine/check.sh` green; `feeding.md` written; `engine/CLAUDE.md` unchanged unless a line is
now wrong.
