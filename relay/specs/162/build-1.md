# #162 build-1 (ford): the scout

Read `spec.md` items 1 and 3. Two fixes in the scout, each with a test that fails on main.

## Code
`core/src/units/search.rs` (`route_plan`, the `trail_on` match after "No reveal can be reached"), `core/src/units/scout.rs` (`choose_ground`, `wander_to`, the `leaves` closure), `core/tests/cases.rs` (`a_settled_route_has_no_cheaper_trail`, `errand`, `settle`), `core/tests/notes.rs`.

## Build
1. **Possible path trails.** With `plan.candidates` empty, `trail_on` is the possible path when `possible` is cheaper than `known` by more than `EPS` (or `known` is `None` and `possible` is `Some`); otherwise the known path as now. Nothing else in the loop changes: the route settles through the existing `heard`/`built` branches once the known cost reaches the possible. Check the `heard` branch (`known_cost <= pcost + EPS`) still only fires when settled, and that `!no_reveal` code paths read `needs` from the new `trail_on`.
2. **The test.** In `core/tests/cases.rs`, `a_settled_route_has_no_cheaper_trail`: drop `#[should_panic]` and `#[ignore]` so it asserts what it checks (at settle, `possible < known - EPS` fails). Confirm it fails on main with the attributes dropped (stash the fix, run it, restore), and passes here, for small 2 to the rival capital and small 3 to Lead's target. If the maps' picks moved so the case no longer reproduces, restage to the current map (the `errand` helper) and record in your summary which route reproduces.
3. **0 steps.** In `choose_ground`'s `leaves` closure, when `n == 0` the note reads `forages at {at}` (for both variants: with a route, `forages at {at}` too). Other texts stay. A test in `core/tests/notes.rs` (pattern: `a_led_scouts_reasons_follow_its_route`): a scout standing on rich ground, its pack low, picks that tile; the note says it forages there and never contains "0 steps away". It fails on main.
4. **Measure.** Run `sansfiat sweep --judge` on main and on your branch; write both to `relay/specs/162/sweep.md` with a line on what moved. Update pins (`cli/tests/save.rs`, others) that move with trails, naming each in your summary.

## Done
`engine/check.sh` passes; the two tests fail on main and pass here; `sweep.md` exists.
