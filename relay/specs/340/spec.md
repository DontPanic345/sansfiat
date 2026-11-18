# #340 Goods, carrying and data

**Question:** health. Is the code ready for a good that isn't eaten and an animal that carries,
with nothing a player meets moved? Two numbers, each against main:

- **Digests.** `digests.sh` prints 21 digests: the seven retro maps at turn 400, each with every
  civ on the AI, with the scripted lead (`--lead`) and as the state digest (`--state`). Main's
  are in `digests-main.txt` (at 4ade4df, measured in this step). Target: all 21 the same on the
  branch, and `SAVE_VERSION` stays 26. Any digest that moves is a fault, not a finding.
- **A turn on large 8**: `sansfiat bench --size large 8 --after 50`, ms a turn past the pick.
  The issue's 22.9 ms is main on M4's runner. The bench moves by runner, nearly 2×: the spec's
  session gave main a median of 25.2 ms (`bench-main.txt`), spec-review's 41.3 ms
  (`spec-review.md`). So the control is main **measured in the same session**: five runs of each,
  branch and main, alternating. Target: the branch's median below main's by more than main's
  spread in that session (0.45 ms in spec-review's). Expect about 14% less: spec-review's
  prototype of the bounded `near` alone gave 41.3 → 35.7 ms with every digest the same.

The review gives both rows beside main in the PR. `sweep --judge` is not the proof here: no rule
moves, so the digests already say every score is main's.

## What the piece delivers

1. **`data/` as a folder** (build-1). `core/src/data.rs` (1,137 lines) becomes
   `core/src/data/mod.rs` and a file per system, every item re-exported so `crate::data::X`
   and `engine::data::X` build unchanged. CLAUDE.md's "every number lives in `data.rs`" says
   `data/`. `engine/CLAUDE.md`'s layout is a line a folder.
2. **The goods table, finished** (build-2). The `Good` table (#355) already carries weight,
   spoil, eaten and use, and `spoil`, `eat_by_*`, `total`, `spread`, `weight` and the clans'
   worths read it. Left: `market::utility` sums √ of every good (it takes a worth now, as
   `demand` does), and `FOODS` is a hand-written list beside the table (derived from it now).
3. **One `carry`** (build-2). Three constants say what a person carries: `CARRY.load` (a
   festival walker's load, `actors::carriers`), `UNIT.pack` and `SCOUT.pack` (a pack's turns,
   `units::pack_cap`, `cap_of`) and `UNIT.carry` (a line's load, `line.rs`, `forage::supply`,
   the ring, `story.rs`). They become one rule in `actors.rs` that every caller reads:

   ```rust
   /// Who carries: a clan's people on a route, a warrior, a scout.
   pub enum Carrier { Clan, Warrior, Scout }
   /// What one person of a carrier carries. `load`: the weight a walker carries on a route, there
   /// and back once a turn. `pack`: the turns of its own eating its pack holds.
   pub struct Carry { pub load: f64, pub pack: f64 }
   impl Actor { pub fn carry(by: Carrier) -> Carry }   // reads data's CARRY
   ```

   with `data::CARRY` holding a `Carry` per carrier (clan: load 2, pack 0; warrior: load 3,
   pack 5; scout: load 3, pack 36), the numbers as they are. The readers: `carriers(brought, by)`
   (walkers), `pack_cap`/`cap_of` (`people × pack × UNIT.food`), and the line's food for labour
   over a cost (`people × UNIT.labour × load / cost`, and its inverse in `forage::supply` and
   `ring`). A pack animal is then a change to `carry`. Every product keeps its present order of
   operations, so the floats are bit for bit main's.
4. **The planner's paths cached** (build-3). `ai(w)` keeps one `Paths` for the whole turn, over
   every civ, and `front`, `post_by`, `direct` and `across` read it. Reachability
   (`path_between(..).is_some()` in `raid`) is a flood fill from `from`, over the same steps
   `path_between` takes (`step_cost(i, j).is_some()` into a `walkable` tile), cached by `from`
   for the turn. And `muster`'s `near` (is a rival capital within `PLAN.march` turns of a store?)
   runs a `cost_map` from each store bounded at `PLAN.march × UNIT.labour` first, and calls
   `path_between` and `turns` only for the capitals inside it: `turns` walks at most
   `UNIT.labour` a turn, so a capital past the bound is past the march. Spec-review measured
   (`spec-review.md`): `near` is two-thirds of the planner's path time and never repeats within
   a turn, so the turn's cache alone saves little; the bound is the win.
5. **No verdicts in the sweep** (build-3). `sweep_fields.rs`, `sweep_towns.rs` and
   `sweep_led.rs` print `pass` or `MISS` beside bounds from past specs. Each prints the number
   beside its control instead, and the JSON keeps the numbers without the verdict.
6. **`docs/architecture.md`** (build-3): the Goods row (the table holds), the Actors row (one
   `carry`), the Scale row (main's and the branch's bench from the same session) and the AI's
   cached paths.

## Shape: Henry Ford

As the issue says: every part is settled here to the function, and the digests are each
step's done. Three fords by area, in order, since builds 2 and 3 edit `data/`:

- **build-1, data:** the folder, both CLAUDE.md files.
- **build-2, actors:** the goods table's last readers, one `carry`.
- **build-3, the planner and the judge:** the cached paths and the flood fill, the bench, no
  verdicts, `architecture.md`.

Each build runs `digests.sh` from `engine/` and diffs it against `digests-main.txt`. The digests take 52 to 76 s, by runner.

## Settled here, by running

- Main's 21 digests (`digests-main.txt`) and its bench on this runner (`bench-main.txt`).
- Medium 9's `--lead` digest equals its AI digest on main: the scripted lead changes nothing
  there by turn 400. It stays a row; it must stay the same too.
- Within the planner's step nothing changes what `path_between` reads: it walks `passable`
  and `walkable` tiles at `step_cost`, which read terrain, trails and rivers, and trails change
  only in `wear` and `tend`. So a path found once is the same path all turn, for every civ.
  build-3's digests prove it.
- (spec-review) Where the planner's path time goes, and that bounding `near` keeps all 21
  digests and cuts a large 8 turn by 13.7% in one session (`spec-review.md`, the prototype in
  `near-bound.diff`). A cache that lasts the turn saves only `path_cached`'s repeats (about a
  tenth); `raid` never searches on large 8, so its flood fill is for the maps where it does.

## Decisions I made

- **Design says "one `carry` on the actor".** The actor holds no kind, and giving it one adds a
  saved field and moves the digest. So `carry` takes the carrier (`Actor::carry(Carrier)`), and
  every caller knows its own: a clan, or a unit's `kind()`.
- The festival walker's load (2) and the line's (3) stay two numbers: the same quantity, a load
  on a route there and back, for two carriers. Making them one is a number change, out of scope.
- `MARKET.carry` (0.2 a person) stays a market rule: it is what a clan brings, not what a person
  carries.
- `UNIT.food` (what a warrior eats) stays in `UNIT`: a pack is turns of eating, so `pack_cap`
  reads both.
- `data/`'s files follow the systems of `engine/CLAUDE.md`'s layout (build-1 lists them).
- (spec-review) The planner's turn gets faster by bounding `muster`'s `near`, beyond the issue's
  "paths cached": the cache alone can't beat the bench's noise, and the bound answers the same.
- The bench's control is main in the same session, not the issue's 22.9 ms, which was another
  runner.
- `sweep --judge` keeps every number a bound read and drops only the verdict, so a later piece
  can still name it.

## Unsettled

Nothing. The bench's size of win is reported, not targeted past "below main".

## Ideas taken

None. Every open idea is a rule a player meets or a view, and this piece moves neither.

## For the review

- Run `relay/specs/340/digests.sh` from `engine/` on the branch and diff it against `digests-main.txt`: all
  21 the same. `SAVE_VERSION` 26.
- Bench five times on the branch and five on main (`git worktree add /tmp/main main`), back to
  back in one session; give both medians in the PR.
- `grep -rn 'UNIT.carry\|UNIT.pack\|SCOUT.pack\|CARRY.load' engine/` finds nothing, tests included.
- `cargo run -p sansfiat --release -- sweep --judge` prints no `pass` or `MISS` (`grep -c`).
- `engine/CLAUDE.md` has one layout line a folder and stays under 8 KB; `data.rs` is gone and
  no file in `data/` is over 400 lines.
