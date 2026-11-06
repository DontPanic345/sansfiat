# #151 build-3 (ford): the cases, red on main

The owner's scout case, a river-bridge case found in spec-review, and Disband all's engine
control, in `core/tests/cases.rs`. Read `spec.md` items 10–12, the "Measured before cutting"
section (spec-review's scout probe), build-1 and build-2's summaries,
`core/src/stage.rs`, `core/tests/scout_errand.rs` (the errand's setup you reuse),
`core/src/units/scout.rs` (`route_plan`, `Plan`, `search`) and `core/src/orders.rs`'s
`Disband` and `units::disband_kind` in `units/mod.rs`.

## The scout

The owner saw the scout check one mountain of a range and leave two that might have carried a
trail (`relay/retro/m3/owner.md` §1.3). The M3 retro traced it: every skip was right by
`scouting.md`, but the search sweeps a range nearest-first (`ordered` in `units/scout.rs`
sorts mountains by `reach`), slowly and silently. T's decided fix is the order: "the search
orders a range's mountains by their best case (the cost through them plus the walk there),
not by nearness". So the red case is the order, and the owner's settle invariant is a green
guard. Spec-review measured all three below on main with a throwaway probe (spec.md,
"Measured before cutting").

Setup for every errand: `Stage::retro(size, seed)`, lead the first civ, route its scout from
its capital to the target (the nearest rival capital, or `Lead`'s target: the first forest or
hills tile 7 steps from the capital, as `scout_errand.rs` finds them), and **each turn refill
both the civ's store and the scout's pack** to its `pack_cap` (with the store alone, small 3's
scout starves on turn 16). "Settle" is `built` true, or the route cleared with the scout
alive, at the end of the turn. Cap the loop at 120 turns; a scout that has not settled fails
with its own message. A helper `errand(size, seed, rival) -> Errand` in `cases.rs` holds this.

1. **`settled_scouts_leave_nothing_unrevealed_that_could_beat_them`**, a plain `#[test]`, green
   on main: on all 14 errands (seven maps, both targets), at settle, walk
   `route(civ, cap, to).possible`'s path; no tile on it that is unrevealed and has no trail may
   have a possible step cheaper than its known one. You need the per-step costs: the
   `candidates` list is exactly the tiles where they differ while the known path is dearer, so
   a non-empty `candidates` at settle is the failure, and the message names the map, the turn
   and the tiles. Keep it under 20 s in `cargo test` (the large map's pre-game comes from the
   stage's cache once).
2. **`a_scout_searches_a_range_by_best_case`**, `#[should_panic]`, red on main: on small 9 and
   medium 8 to the rival. At each turn's start, take the open mountains: unrevealed, and their
   cost through (possible cost capital → m plus m → target, from `route`) below the known
   route's cost by more than `EPS`. A mountain's best case is that cost plus the scout's walk
   to it: the least known cost from the scout's tile to a passable tile beside it (`route`'s
   `known`), 0 when it stands beside it. End the turn; if a mountain was revealed (at most one
   a turn: a mountain takes the whole turn's labour, begun beside it), it must have been open
   and its best case within `EPS` of the lowest best case of the open set. The message: "<map>
   turn <t>: the scout revealed mountain <m> (best case <b>) while <m2> (best case <b2>) was
   open". Spec-review's probe, with exactly this definition, found 12 of 29 reveals out of
   order on small 9 (the first at turn 2: mountain 304 at 28 while one at 26 was open) and 4
   of 11 on medium 8 (the first at turn 2: 242 at 28 against 27). A third errand, small 9 to the forest (two reveals, in
   order), runs the same check as a plain `#[test]`: the control. The test's doc says: "Red on
   main: T orders the search by best case, then removes the `should_panic`." Write the best case
   in one function the doc names, so T reads the rule it must meet. The probe's version, which
   priced every open mountain every turn, took 27 s for the three errands in the test profile:
   keep the turn's start (a clone of the world, or the scout's tile and the revealed set) and
   price the open set only on a turn that revealed a mountain.
3. **`a_settled_route_has_no_cheaper_trail`**, `#[should_panic]`, red on main: on small 2 to the
   rival and small 3 to the forest, at settle `possible.cost >= known.cost − EPS`. Spec-review
   found it fails on both (small 2: settles turn 2, known 15 against possible 14; small 3:
   turn 51, 19 against 18), and that every differing step is a *revealed* river tile a trail
   would bridge (hills 144 on small 2, hills 436 on small 3): the scout builds trails only on
   its known path. The message names the map, the turn, both costs and the tile where the
   possible path's step is cheaper than the known one. Its doc says: "Red on main, and not T's:
   the scout builds trails only on its known path, so a bridge on a revealed river tile that
   would cut the route is never built."

Each red test's `expected` is a stable fragment of its message (the map and the phrase), not
a number.

## Disband all

On a stage with a camp of 4.6 people, a scout of 1.5 and a guard of 2.5 (set the units'
people through `world_mut` and keep their stack, pack and `people` consistent; `Unit` has
`take_people` and `take_kind` to read): for each, `order(civ, Order::Disband { unit, n:
None, fighter: None })` sends every person home at once: the unit is gone, the clan's people
rose by exactly the unit's, and the world's people are conserved. It passes on main; keep it
a plain `#[test]`. It is the control for the page arm (build-4), which is the red one: the bug
is the page's order form. Write no Rust test that sends the page's count (`n: Some(_)`) and
expects the unit gone: that would pin an engine change T doesn't make.

## `relay/specs/151/cases.md`

For each case: the command that runs it, what it says on main (the panic message, copied
exactly), and what T must do (for the river bridge: nothing; it waits for a fix piece). If a case is not red on main for the reason `spec.md` states
(main may have moved since the probe), do not weaken the invariant: record the run and finish
`pending` with that note.

## Done

`cargo test -p engine --test cases` is green (the red cases by `should_panic`), `clippy`,
`fmt`. No digest moves; no rule is changed. The three scout messages (two red, one control) and the Disband control are in `cases.md`.
