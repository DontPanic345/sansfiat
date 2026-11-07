# #153 build-2: the search and the orders

What build-2 moved, what it could not move, and the numbers behind both.

## What moved

- **The split.** `units/scout.rs` gave its route search to `units/search.rs`, byte for byte, as its own
  commit (`a285682`). `stats::digest` for small 2 at 50 turns is `ede78089d1e3106b` before and after.
- **The best-case key.** `ordered` orders the mountains of the candidates' ranges by best case: the cost
  through the mountain (`through_maps`: one forward and one backward cost map, priced as `route_plan`
  prices a possible path) plus the walk, priced by `step_as(known)` from the scout's tile to a neighbour.
  It is `cases.rs`'s `best_case`. `cost_map_by` (in `land.rs`) is the path-priced cost map the walks use.
- **`search_view`** (`units/search.rs`): `next` is the order the search will try, `left` the unrevealed
  mountains of the touching ranges whose cost through is not below the known route (cheapest first, ten at
  most), `known` the known route's cost. `Sim.view()`'s scout row gains `search`.
- **`Move` for a scout.** `ScoutState.dest` (saved as `UnitFlat.dest`). A scout with a `dest` and no route
  marches to it on the labour its feeding left, and notes "at X, foraging" on arriving (and then has no
  `dest`). `Route` clears `dest`; `Move` clears the route. The walk home to the capital is gone: an idle
  scout stays where it is.
- **The reach.** `line_from(w, unit, tile)` is `line_by`'s body from any tile (`line_at`); `past_reach`
  is no line or a carry under `people × UNIT.food`. `Move` and `Route` to a scout note "past its line's
  reach: ..." when the far end is past reach. `Sim.lineFrom(unit, tile)` returns `{cost, carry, eats}`.
- **The CLI.** A scout's line reads "going to X" with a `dest`.
- **The save.** `SAVE_VERSION` 9 → 10. The scout's `dest` is new state.

## What did not move: the owner's case stays red

`a_scout_searches_a_range_by_best_case` is still `#[should_panic]`. The brief asked for the attribute to
go, and it could not, for a reason the brief did not foresee.

The case checks that every mountain the scout reveals is, at its turn's start, the least open best case,
where "open" is every unrevealed mountain whose cost through beats the known route. On small 9 to the
rival, turn 22, the scout reveals 131 (best case 32.0) while 178 (best case 28.0) is open. 178 is in no
candidate's range, so the search, which orders only the candidates' ranges, never considers it.

Widening the set to every open mountain fixes the case and breaks the owner's invariant:

| set | key | small 9 to the rival | case |
|---|---|---|---|
| candidates' ranges (kept) | best case, priced walk | settles, invariant green | red at 131 (32.0) with 178 (28.0) open |
| every open mountain | best case, walkable walk | settles at turn 149 (budget 120) | red at medium 8 turn 14 (930 at 41.5, 973 at 39.5) |
| every open mountain | best case, priced walk | never settles: the scout ping-pongs between 257, 371 and 339 for 400 turns | red |

The second row is the cost: every open mountain must be revealed, one mountain a turn, before the
candidates on the possible path (which have higher best cases) are reached. The third row is a
second problem: the priced walk to a mountain can cross a revealed mountain with no trail yet, which
the scout's walk cannot enter, so its key and its approach disagree and the target flips as it moves.

The fix needs the owner's call: either a larger settle budget (the invariant is `TURNS = 120` in
`cases.rs`), or the scout builds the pass on the way to a mountain whose best case goes through one
(a rule the spec does not give). The case keeps its `should_panic`, with its expected text matching
the failure it now gives, and its doc says why.

`a_settled_route_has_no_cheaper_trail` moved: its settled turn is 25, not 52, under the new key. Its
expected text is updated to that turn; the red it names (18.8 known against 17.8 possible) is unchanged.

## Scout errand table (`scout_errand`, 50 turns, 2 people, fed)

Before (`c53e861`) and after (this commit). Only two rows moved.

| map | to | before: hungry turns | starved | alive | after: hungry turns | starved | alive |
|---|---|---|---|---|---|---|---|
| small 2 | nearest rival capital | 46 | 2 | no | 46 | 2 | no |
| small 2 | Lead's target | 44 | 2 | no | 44 | 2 | no |
| small 3 | nearest rival capital | 37 | 2 | no | 37 | 2 | no |
| small 3 | Lead's target | 22 | 2 | no | **26** | 2 | no |
| small 6 | nearest rival capital | 15 | 2 | no | 15 | 2 | no |
| small 6 | Lead's target | 7 | 1 | yes | 7 | 1 | yes |
| small 9 | nearest rival capital | 45 | 2 | no | 45 | 2 | no |
| small 9 | Lead's target | 41 | 2 | no | 41 | 2 | no |
| medium 8 | nearest rival capital | 28 | 2 | no | 28 | 2 | no |
| medium 8 | Lead's target | 31 | 2 | no | 31 | 2 | no |
| medium 9 | nearest rival capital | 36 | 2 | no | 36 | 2 | no |
| medium 9 | Lead's target | 0 | 0 | yes | 0 | 0 | yes |
| large 8 | nearest rival capital | 41 | 2 | no | **42** | 2 | no |
| large 8 | Lead's target | 44 | 2 | no | 44 | 2 | no |

What moved: the idle scout no longer walks home to the capital, and the search's key changed. Two rows
move by a turn or four; the rest are the same. The scouts still die on these errands: the feeding is
build-3's, and build-5 tunes it.

## `search_view` on small 9's errand to the rival, its first turn

Route set from the capital to the nearest rival capital. Known cost **55.75**. `left` is empty: no
unrevealed mountain touching the paths has a cost through at or above the known route. `next` holds 15
mountains (and no forest or hills yet), in best-case order:

| tile | through | best case |
|---|---|---|
| 338 | 23.00 | 33.25 |
| 370 | 25.00 | 35.25 |
| 403 | 27.75 | 38.75 |
| 435 | 29.75 | 44.75 |
| 466 | 31.75 | 48.75 |
| 496 | 34.75 | 53.75 |
| 495 | 34.25 | 56.25 |
| 397 | 29.25 | 77.00 |
| 430 | 31.25 | 77.00 |
| 365 | 27.25 | 78.00 |
| 462 | 32.25 | 78.00 |
| 334 | 25.00 | 79.75 |
| 304 | 25.00 | 80.50 |
| 305 | 24.00 | 81.25 |
| 306 | 23.00 | 81.25 |

Tile 338 is the one the scout tries first, and it is the one with the least best case (33.25): its
walk is 10.25 and its through 23.00. The list is printed by `search_view_on_small_9_to_the_rival_orders_its_open_mountains_by_best_case`.

## The staged left

On a staged plains map the detour test (`search_view_names_the_mountain_a_range_leaves_and_what_it_costs_through`)
has `left` name the mountain beside the detour with its cost through (`7.00` against a known `5`).

## Tests

`core/tests/scout_orders.rs` holds the new ones: four steps out and stay, Route and Move clearing each
other, the sea refused with "a scout can only go over land", `search_view` on a staged detour and on
small 9's errand, `past_reach` at the capital and eight steps out with the note, and a Move's replay.
`orders.rs`'s wrong-kind test now expects a scout's Move at the sea to say "a scout can only go over
land". `save.rs`'s versions read 10.

## Digest

The digest pins in `cli/tests/save.rs` are unchanged: none of the pinned games sends a scout on a route,
so the walk-home removal and the new key do not move them. The version bump moves no digest.
