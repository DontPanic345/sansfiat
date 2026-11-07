# #153 build-2 (easel): the search and the orders

The scout's route search orders mountains by their best case and says why it leaves one; the
scout gets a `Move` order; the line's reach can be read from any tile. Read `spec.md` items
7–13 and `relay/specs/151/cases.md` (the cases this flips). Build-1 gave flat land its trail
cost (`flat.md` says what moved). The code: `core/src/units/scout.rs` (all of it),
`core/src/units/mod.rs` (`UnitFlat`, `ScoutState`'s fields there, `step`),
`core/src/units/line.rs` (`line`, `line_with`, `route`), `core/src/orders.rs` (`check_order`,
`check_move`, `order_move`, `order_route`), `core/tests/cases.rs` (`best_case`, `through`,
`check_search_order`), `core/tests/scout.rs`, `web/src/lib.rs` (`view`'s unit rows, `route`),
`cli/src/view.rs`.

## What to build

1. **Split `units/scout.rs`** (630 lines). Move the route search, byte for byte, to
   `core/src/units/search.rs`: `revealed`, `mountain_may_bridge`, `tile_cost`, `may_trail`,
   `step_as`, `needs_trail`, `trail_left`, `Plan`, `route_plan`, `walk_costs`, `beside`,
   `approach`, `ordered`, `search` and `reveal`. Put `march`, `scout_event`, `clear_route` and
   `blocked` wherever both files can reach them (`pub(super)` in `search.rs` is fine). `units/mod.rs`
   re-exports `route_plan` and `Plan` from the new module. Check `stats::digest` is unchanged
   for small 2 at +50 before you change anything (`sansfiat digest`), and commit the move on
   its own.
2. **Best-case order** in `ordered` (spec item 8). Mountains stay the first tier, keyed by
   best case: `through(m)` plus the walk. The walk is 0 when `steps(home, m) <= 1`, else the
   least cost, priced by `step_as(w, civ, i, j, false)`, from the scout's tile to any
   neighbour of `m`. One `path_by`-style cost map from `home` with that pricing gives every
   walk; it must equal `best_case` in `cases.rs` (which calls `route_plan(home, n).known` per
   neighbour) to `EPS`. Ties go to the lower tile. The forest and hills tier keeps its order
   (nearest by `beside`, then the biggest saving). Remove `#[should_panic]` from
   `a_scout_searches_a_range_by_best_case` and make its doc say what it checks. If build-1
   moved this case to other maps, use its maps. The owner's invariant
   (`settled_scouts_leave_nothing_unrevealed_that_could_beat_them`) and the forest control
   must stay green. Leave `a_settled_route_has_no_cheaper_trail` as build-1 left it.
3. **`search_view(w, unit) -> Option<SearchView>`** in `search.rs` (spec item 9):
   `SearchView { next: Vec<usize>, left: Vec<(usize, f64)>, known: Option<f64> }`. `left` is
   the unrevealed mountains of every range touching the possible or known path (on it, or
   beside one of its tiles, and every unrevealed mountain joined to one of those), whose cost
   through is not below the known cost by more than `EPS`, sorted by cost then tile, ten at
   most. `None` without a route. `Sim.view()`'s scout row gains
   `search: {next, left: [{tile, through}], known}` (or `null`), documented in `view`'s doc.
4. **The scout's `Move`** (spec item 10). `ScoutState` gains `dest: Option<usize>`, read from
   and written to `UnitFlat.dest` (a camp's `dest` field, already there). `check_order` takes
   `Move` for a camp or a scout. `check_move`'s errors name the unit's kind ("a scout can only
   go over land"). `order_move` on a scout sets `dest` (`None` for its own tile), clears its
   route (`heard`, `blocked`, `built` too) and notes "going to X, N steps away". `order_route`
   clears a scout's `dest`. In `scout_turn`, the walk home without a route goes. A scout with
   a `dest` and no route marches toward it on the labour left after it has fed itself, banking
   a part-paid step as `search` does, and notes "at X, foraging" on arriving. A scout with
   neither stays where it is (build-3 sends it to rich ground). `line::route` returns a
   scout's march to its `dest` as it does a camp's, so `Sim.route(id)` and the page's march
   path work for both.
5. **`line_from(w, unit, tile) -> Option<(f64, f64)>`** in `line.rs` (spec item 11): `line`'s
   cost and carry as if the unit stood on `tile`. Factor `line_with` so both share one body;
   don't copy it. Add `pub fn past_reach(w, unit, tile) -> bool`: no line, or a carry below
   `people × UNIT.food`. `order_move` and `order_route` (for the route's end) add the note
   "past its line's reach: its line there would carry {carry:.1} of the {eats:.0} it eats", or
   "past its line's reach: no line reaches X". `Sim.lineFrom(unit, tile)` returns
   `{cost, carry, eats}` or `null`.
6. **The CLI view** (`cli/src/view.rs`): a scout's line reads "going to X" when it has a `dest`.
   Keep the file under 80 lines.
7. **Tests** in `core/tests/scout.rs` (if it passes 400 lines, put the new ones in
   `core/tests/scout_orders.rs`), each on a stage:
   - a scout moved four steps out on a plain stage walks there over turns, stays for five
     more, and has no route;
   - `Route` after `Move` clears `dest`, and `Move` after `Route` clears the route;
   - `Move` to a sea tile is refused with the scout's message;
   - on a staged range (`Stage::hidden`, as `sealed_valley` builds one), `search_view`'s `left`
     names a mountain whose cost through is above the known route's, with that cost, and
     `next` holds the mountains it will try, in best-case order;
   - `past_reach` is false at the capital and true eight steps out on a plain stage, and the
     `Move` there notes it;
   - an order replay (`a_route_order_replays_to_the_same_world`'s pattern) with a `Move`.
8. **Save version**: the scout's `dest` is new state on the saved unit and the walk home is
   gone, so `stats::digest` moves. `SAVE_VERSION` bumps and the pins move.

## Proof and record

`engine/check.sh` green. `cargo test -p engine --test cases -- --nocapture`: the search-order
case passes without `should_panic`. Run `cargo test --release -p engine --test scout_errand --
--ignored --nocapture` before and after, and put both tables in `relay/specs/153/search.md`
with a line on what moved (the idle scout no longer walks home). Put in `search.md` the
`search_view` of small 9's errand to the rival at its first turn: `next` and `left`, with
costs.

## What later steps rely on

`units/search.rs` holds the search, and `scout.rs` the state, the turn and the feeding (now
about 250 lines). `ScoutState.dest` and `Order::Move` work for a scout. `line_from` and
`past_reach` exist. `Sim.view()` gives a scout's `search` and `dest`, and `Sim.lineFrom` and
`Sim.route` work for a scout. Build-3 adds the feeding in `scout.rs`, and build-4 draws all of
it.

## Done

`engine/check.sh` green; `search.md` written; `engine/CLAUDE.md`'s `units/` line names
`search.rs` (the route search) beside `scout.rs` (the scout's turn and feeding).
