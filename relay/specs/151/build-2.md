# #151 build-2 (ford): the fixtures on the stage

`core/tests/common/mod.rs` and `core/tests/scout.rs`'s painters become code on build-1's
`Stage`. **Every existing test passes with its assertions unchanged.** Read `spec.md` item 3,
build-1's summary and `core/src/stage.rs`, `core/tests/common/mod.rs` and
`core/tests/scout.rs` (the helpers at its top: `lead`, `end_fed`, `plain`, `open_ray`,
`capital`, `paint`).

## Do

1. In `common/mod.rs`, rewrite on the stage: `flat`, `flat_radius`, `big`, `island`, `put`,
   `open` (market), `found`, `three`, `led`, `put_camp`, `hold_land`'s helpers where the stage
   has the same thing. Keep every function's name, signature and doc so no caller changes: it
   becomes a thin layer over `Stage`. What the stage has no counterpart for (`check_territories`,
   `hearth`, `centre`, `keys_in_order`, `made`, `nbrs`, `no_scout`, `no_guard`, `total`,
   `kinds`) stays as it is. A helper that returns a `World` returns `stage.into_world()`.
2. In `scout.rs`, `lead(seed)` becomes `Stage::retro("small", seed)` led by its first civ
   (`lead` + `world()`), cloning the cached pre-game instead of running it per test. `plain`
   and `paint` become stage methods if build-1 left them out (`Stage::plain()` makes every
   land tile plain and bare and clears the led civ's revealed set; `paint(tile, terrain,
   trail_cost, span)`): add them to `stage.rs` with a doc comment and a line in `tests/stage.rs`.
   `open_ray`, `capital`, `end_fed` and the tests stay in `scout.rs`.
3. A test that replays a save (`a_route_order_replays_to_the_same_world`) must still pass:
   it paints nothing, so the cloned world's log is the real one. Keep it that way, and do
   not give the stage a `save()`.

## Rules

- Do not change an assertion, a number or a test's intent. If one fails after the swap, the
  base differs from the old fixture: fix the stage (its base must equal `flat_radius`'s), not
  the test. If one genuinely can't pass, list it in your summary with why.
- No digest moves.

## Done

`cargo test` (it takes a while: it is optimised) is green with no test edited but its setup
lines; `cargo clippy --all-targets -- -D warnings`, `cargo fmt`. Say in your summary how many
tests the rewrite touched and the time `cargo test -p engine --test scout` takes now against
before (the cache should cut it).
