# #151 build-3 (ford): the two cases, red on main

Two cases in `core/tests/cases.rs`. Read `spec.md` items 10–12, the "Measured before cutting"
section (the scout's repro on small 2 and small 3), build-1 and build-2's summaries,
`core/src/stage.rs`, `core/tests/scout_errand.rs` (the errand's setup you reuse),
`core/src/units/scout.rs` (`route_plan`, `Plan`, `search`) and `core/src/orders.rs`'s
`Disband` and `units::disband_kind` in `units/mod.rs`.

## The scout

For each of small 2 (to the nearest rival capital) and small 3 (to the forest or hills tile 7
steps out, `Lead`'s target): `Stage::retro("small", seed)`, lead the first civ, route its
scout from its capital to the target, and each turn refill the civ's store and the scout's
pack (as `scout_errand.rs` does). When the scout settles (`built` true, or its route cleared
with the scout alive), assert the invariant of `spec.md` item 11:
every tile of `route(civ, from, to).possible`'s path that is not revealed has a possible cost
equal to its known cost, and `possible.cost >= known.cost − EPS`. The assert message names the
map, the turn, both costs and the tile (if any), so it is the evidence. Cap the loop at 120
turns; a scout that has not settled by then fails with its own message.

Each is `#[should_panic(expected = "…")]` with a stable fragment of the message (the map and
"could beat"), so CI holds the failure and T flips it. Their doc comment says: "Red on main:
T fixes the search, then removes the `should_panic`."

A control passes: small 6's errand to the rival (it settles with known equal to possible).

You may add a built-map case (a hand-painted range of three mountains with hidden costs on
`Stage::parse`, the best behind a worse one, so the nearest-first order matters). Try at most
three layouts; keep one only if it is red on main for the same reason, and otherwise leave it
out and say so.

## Disband all

On a stage with a camp of 4.6 people, a scout of 1.5 and a guard of 2.5 (set the units'
people through `world_mut` and keep their stack, pack and `people` consistent; `Unit` has
`take_people` and `take_kind` to read): for each, `order(civ, Order::Disband { unit, n:
None, fighter: None })` sends every person home at once: the unit is gone, the clan's people
rose by exactly the unit's, and the world's people are conserved. The assertion is expected to
pass on main; keep it a plain `#[test]`. A second test sends the order the page's `all` option
makes today, `Disband { n: Some(floor(people)) }`, and asserts the unit is gone: this is red
on main (0.6 stays) and is the same `should_panic` kind. It is the Rust shadow of the page arm
in build-4.

## `relay/specs/151/cases.md`

For each case: the command that runs it, what it says on main (the panic message, copied
exactly), and what T must do. If a case is not red on main for the reason `spec.md` states
(main may have moved since the probe), do not weaken the invariant: record the run and finish
`pending` with that note.

## Done

`cargo test -p engine --test cases` is green (the red cases by `should_panic`), `clippy`,
`fmt`. No digest moves; no rule is changed. The scout's red messages are in `cases.md`.
