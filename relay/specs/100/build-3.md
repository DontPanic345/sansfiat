# Build 3 (ford): shared actor behaviour

Piece #100. Read `relay/specs/100/spec.md` (items 1, 7, 8). `docs/design/actors.md` is the design:
spoiling, eating from a store and walking a path are what every actor does, so they are written once,
in `core/src/actors.rs`. **No rule changes, and the digests are exact:** every function must perform its
caller's floating-point operations in the same order, or the digest moves. That is this step's whole
risk; the baseline comparison catches it.

## The work

Today the pieces are duplicated:

| Behaviour | Where it is now |
|---|---|
| Spoiling: `v * (1.0 - CLAN.spoilage)` | `clans::eat` (after eating), `civs::upkeep` (before the cap), `units/camp.rs` `camp_march` and `units/scout.rs` `scout_turn` (before marching), and nothing else |
| Eating in proportion to what the store holds | the camp's end of `camp_turn` and `scout_turn` (via `take(&mut u.store, need)` then `total(&eaten) + EPS >= need`) |
| Eating by diet (each food up to `CLAN.diet_share × need`, the excess at `CLAN.excess_value`) | `clans::eat` |
| Walking a path on a pool of labour | `units::walk` (camp, slack 0, also used by `turns`) and `units/scout.rs` `march` (slack `EPS`, marks `walked`) |

1. **`Actor::spoil(&mut self)`** in `actors.rs`; all four callers use it. The civ's store is a bare
   `Basket` today: point 4 below makes it an actor's.
2. **`Actor::eat_by_share(&mut self, need: f64) -> Basket`** (what `take(&mut store, need)` does now,
   returning what was eaten) for the camp and the scout. **`Actor::eat_by_diet(&mut self, need: f64)
   -> f64`** for the clan: the clan today eats from `c.basket` (store + harvest, set in `harvest`
   because `trade` reads it) and writes the remainder into `c.store`. Keep `basket` as it is; in `eat`,
   put the basket into the actor's store, eat, return what the clan got, then `spoil()`; the arithmetic
   `left.map(|v| v * keep * (1.0 - CLAN.spoilage))` stays two multiplications in that order. Each caller
   keeps its own hunger rule: the clan's `fed_ratio`, `fed`, `starving`, growth and starvation; the
   camp's `hungry` count, event and disband; the scout's lost person. None of that moves.
3. **`actors::stride(w, from, path, labour, slack) -> (usize, f64)`**: steps taken and labour left,
   each step taken while `cost <= labour + slack`. `turns` and the camp march call it with slack `0.0`
   and `banked + UNIT.labour` (the camp's own pool, as `walk` did), the scout with its `left` pool and
   `EPS`. The scout's `march` marks `walked` on `path[..taken]` and sets home/territory as now. Delete
   `units::walk` and the loop in `march`.
4. **The civ is an actor** (item 8): `Civ` gets `actor: Actor` (`Actor::new(capital, 0.0)`; the store
   is its store), with `Deref`/`DerefMut` to `Actor` as `Unit` has so `civ.store` still reads at its call
   sites. Its JSON keeps `store` where it was and gains no key — use a private flat mirror as build-2
   did for `Unit` (`#[serde(from, into)]`), copying the civ's current field list and order from a run.
   `civs::upkeep` calls `civ.spoil()` then applies the cap as now. The civ gains no other behaviour.
   Everything that builds a `Civ` (`civs::found`, tests) sets the actor.

## Tests

`core/tests/actors.rs`: `spoil` removes the spoilage share of every food; `eat_by_share` takes in
proportion and never more than the store holds; `eat_by_diet` on a small hand-worked case equals the old
rule (compute it by hand in the test's comment); `stride` stops at the first step it can't pay, honours
`slack`, and a path of zero steps takes none. A civ's JSON has `store` at the position it had
(`civs.rs`, or a world JSON key-order test).

## Done

`./check.sh` green; the five baseline runs identical (`digest` for the four watching, `--state` for the
led, every turn, `diff` empty) and the retro sweep `diff` empty — run all and say so. `grep -rn
'1.0 - CLAN.spoilage' core/src` shows one hit, in `actors.rs`. The engine's `bench 1 2 3` is within 10% of
`relay/specs/100/bench.md`.

## What build-4 relies on

`Actor::{spoil, eat_by_share, eat_by_diet}`, `actors::stride`, the civ an actor. `engine/CLAUDE.md`'s
`actors.rs` row names them.

The comparison with the baselines is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines.
