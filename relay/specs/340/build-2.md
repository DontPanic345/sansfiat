# #340 build-2 (ford): the goods table's last readers, and one `carry`

Read `relay/specs/340/spec.md` ("What the piece delivers", items 2 and 3). build-1 made
`core/src/data/` a folder; you edit `data/goods.rs` and `data/clans.rs` and `data/units.rs`.
Nothing a player meets changes: the proof is `relay/specs/340/digests.sh` (from `engine/`, after
`cargo build --release`), byte for byte `digests-main.txt`, and `SAVE_VERSION` stays 26.

## 1. The goods table

- `market::utility(basket)` sums √ of every good. Make it `utility(basket, worth)`, Σ worth × √
  amount, as `demand` values a basket; its callers pass the worth they cleared with:
  both `cli/src/clear.rs` and `core/tests/market.rs` clear every trader at `[1.0; N]`, so they
  pass `&[1.0; N]` (not `FOOD_WORTH`, which would drop salt and obsidian from the test's
  "nobody loses").
- `FOODS` is a hand-written list. Derive it from `GOOD` in a `const` block, as `GOODS` is, with
  a const assertion that the eaten goods come first and number 5 (`FOODS` stays `[&str; 5]`, so
  the web crate and `core/tests/land.rs` build unchanged).
- Read `spoil`, `eat_by_share`, `eat_by_diet`, `total`, `spread`, `weight`, `actors::worths`
  and the market's clearing once each: each reads `GOOD`. Fix any that reads a fixed index or
  range for "food" instead (say which in your summary; `actors.rs`'s `GOOD[0].spoil` is the
  foods' rate: name it as such, e.g. a `const FOOD_SPOIL` in `goods.rs` read from `GOOD[GAME]`,
  so nothing moves).

## 2. One `carry`, in `actors.rs`

Write it as the spec writes it:

```rust
pub enum Carrier { Clan, Warrior, Scout }
pub struct Carry { pub load: f64, pub pack: f64 }
impl Actor { pub fn carry(by: Carrier) -> Carry }
```

- `data/clans.rs`'s `CarryRules` becomes `{ clan: Carry, warrior: Carry, scout: Carry }` with
  clan `{ load: 2.0, pack: 0.0 }`, warrior `{ load: 3.0, pack: 5.0 }`, scout
  `{ load: 3.0, pack: 36.0 }`. `UNIT.carry`, `UNIT.pack` and `SCOUT.pack` go, their docs and
  notes moved beside `CARRY` (`Carry` itself may live in `data/` and be re-exported by
  `actors`: your choice, one definition). Move `CarryRules` and `CARRY` to `data/units.rs` if it
  reads better beside `UNIT`; say so.
- Every reader goes through `Actor::carry`:
  - `actors::carriers(brought)` → `carriers(brought, Carrier::Clan)`
    (`(brought / load).max(1.0)`); its caller in `markets.rs`.
  - `units::pack_cap(people)` → `people * carry(Warrior).pack * UNIT.food`; `cap_of`'s scout arm
    `u.people * carry(Scout).pack * UNIT.food`.
  - The line: `line.rs` (`u.people * UNIT.labour * carry(Warrior).load / cost`, and the ring's
    `UNIT.food * cost / (UNIT.labour * load)`), `forage::supply`, `cli/src/story.rs`.
- **Keep every product's order of operations exactly as it is** (`people * pack * food`, not
  `pack * people * food`): floating point is not associative, and the digests are bit for bit.
- `grep -rn 'UNIT\.carry\|UNIT\.pack\|SCOUT\.pack\|CARRY\.load' engine/` finds nothing when you
  are done, docs and tests included: the tests that read them now are `core/tests/units.rs`
  (`pack_total`, the line and forage asserts), `core/tests/scout_routes.rs` (`SCOUT.pack`) and
  `core/tests/markets.rs` (`CARRY.load`); `Good.weight`'s doc and `line.rs`'s docs name them too.
- A test in `core/tests/` (an existing actors or units test file): `Actor::carry` gives each
  carrier's numbers, and `pack_cap(3.0)` equals `3.0 * 5.0 * UNIT.food`. Seconds, not a game.
- `engine/CLAUDE.md`'s `core/src/` row names `carry` beside `spoil`, `eat` and `walk_cost` in a
  few words.

Done: the digests match, the grep is empty, `engine/check.sh` passes. Commit as
`#340 build-2: goods table, one carry`.
