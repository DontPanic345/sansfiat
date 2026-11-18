# #340 build-3 (ford): the planner's paths cached, no verdicts in the sweep, architecture.md

Read `relay/specs/340/spec.md` (items 4 to 6). Nothing a player meets changes: the proof is
`relay/specs/340/digests.sh` (from `engine/`, after `cargo build --release`), byte for byte
`digests-main.txt`, and the bench below.

## 1. The planner's paths, in `core/src/units/ai.rs`

The planner calls `path_between` outside its `Paths` cache many times a turn: `muster`'s `near`
test (each store to each rival capital, every turn), `raid` (`path_between(from, t).is_some()`
over candidate tiles) and `across` (the path's tiles); and `front`/`post_by` get a fresh `Paths`
each call. `fight.rs`'s and `orders.rs`'s are not the planner's: leave them. Spec-review timed them
(`relay/specs/340/spec-review.md`): `near` is two-thirds of the planner's path time and never
repeats within a turn, so it needs a bound, not a cache.

- **`near`, bounded.** Before the `path_between` for a store and a rival capital, run one
  `land::cost_map` from each store, bounded at `PLAN.march × UNIT.labour` (with a hair of slack:
  spec-review used `× 1.001 + 1e-6`), over `walkable` tiles, and skip the capitals not in it.
  `turns` walks at most `UNIT.labour` of step cost a turn, so a capital past the bound is past
  the march; the ones inside still get `path_between` and `turns`, so the answer is the same.
  `relay/specs/340/near-bound.diff` is spec-review's prototype (all 21 digests the same, 13.7%
  off the turn): take it, with a comment saying why the bound holds.
- `ai(w)` makes one `Paths` for the turn and passes it through `plan` to every civ's `muster` and `direct`
  (and through them `raid`, `across`, `post_by`, `front`): a path found once is found once a
  turn, for every civ. `front`, `post_by`, `direct` and `across` read it through `path_cached`.
  `plan` is public: a caller outside the AI step (`script::Auto`, the page) keeps its own `Paths`
  for its call, or `plan` keeps its signature and a `plan_with(w, civ, paths, give)` holds the body.
- Reach is a flood fill (`raid` never searches on large 8, so this doesn't show in the bench; it
  is for the maps and turns where it does): a `Reach` cache beside `Paths` (or in one struct with it), keyed by
  `from`, holding the tiles reachable from `from` over the steps `path_between` takes: from `i`
  into neighbour `j` when `step_cost(w, i, j).is_some()` and `walkable(w, j)`. `raid`'s
  `path_between(from, t).is_some()` becomes `t == from || reach(from).contains(t)`. A flat
  `Vec<bool>` over the tiles is cheapest.
- Nothing inside the planner's step changes trails, terrain or rivers, so the cache holds all
  turn (spec, "Settled here"). If the digests move, that is not true somewhere: find what, and
  narrow the cache to it.
- `ai.rs` is 634 lines and this step adds to it. Put `Paths`, `path_cached` and the new reach in
  `core/src/units/paths.rs` (with `pub(super)` items), so `ai.rs` shrinks.

**The bench**, the piece's number: build release on the branch and on main (`git worktree add
/tmp/main main`, `cargo build --release` there), then run five times each, alternating, in one
session: `target/release/sansfiat bench --size large 8 --after 50`. Write the ten lines and both
medians to `relay/specs/340/bench.md`. Done: the branch's median is below main's by more than
the spread of main's five runs in your session (spec-review's: main 41.1 to 41.5 ms, the bounded
`near` 35.7; the runner changes the numbers nearly 2×, so only your session's main counts). If it isn't, find
what is left in the planner's step (time it as `relay/retro/m4/architecture/instrument.diff`
did, not committed) and say so in `bench.md`.

## 2. No verdicts in the sweep, in `cli/src/`

`sweep_fields.rs`, `sweep_towns.rs` and `sweep_led.rs` print `pass` or `MISS` against bounds
written for past specs (`mark`, `Bound`'s `ok`, the floor test in `sweep_led.rs:210`). Print the
number beside its control and drop the verdict: no `pass`, `MISS`, `ok` or "≥ half" left in the
text, and the JSON keeps every number (drop only the verdict fields; `judge --compare` reads
numbers, check it still builds and runs on two reports). Module docs that say "each marked pass
or miss" say what they print now. `grep -rn '"pass"\|MISS' cli/src` finds nothing but `map.rs`'s
`pass` tile glyph. `cli/tests/` that assert a verdict assert the number instead.

## 3. `docs/architecture.md`

- Goods row, On main: the table holds (weight, spoil, eaten, use), every reader reads it.
- Actors row: one `carry` (`Actor::carry`, by carrier) that the festival walk, the pack and the
  line read.
- Scale row: the bench's two medians from `bench.md`, same session, and "the planner's paths and
  reach are cached a turn".
- The judge row: "a report with no verdicts; a piece's question names its number and control".
- Rules that hold: "every number lives in `core/src/data/`".

Done: digests match, the bench is in `bench.md`, the greps are empty, `engine/check.sh` passes.
Commit as `#340 build-3: planner paths cached, no verdicts`.
