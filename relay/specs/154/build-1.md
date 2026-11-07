# #154 build-1 (easel): the split, the ladder, the guard's meal, the muster

Four refactors, each proved byte for byte: `stats::digest`, `SAVE_VERSION` and the pins in
`cli/tests/save.rs` don't change. Read `spec.md` items 1–4. Later steps build every M4 rule on
these shapes, so name them as here.

The code: `core/src/actors.rs` (`divide`, `Actor`), `core/src/clans.rs` (`reap`),
`core/src/claim.rs` (`actors::divide` for the guard), `core/src/units/camp.rs` (`camp_turn`),
`core/src/units/scout.rs` (`scout_turn`, `march`, `forage_round`, `bring`),
`core/src/units/search.rs` (`search`, which spends `left`), `core/src/units/forage.rs`
(`strip`, `graze_at`, `supply`), `core/src/units/guard.rs` (`turn`), `core/src/units/mod.rs`,
`cli/tests/save.rs`.

## What to build

1. **`actors::split(labour, routes, shares, jobs) -> Vec<Vec<f64>>`**. `routes` are paid once
   from `labour`; what is left goes to each job by its share (`shares`, summing to one); each
   job divides its part evenly over its tiles, each tile's share paying its walk first, as
   `divide` does. `jobs` is each job's walks (yearly, one per tile). Returns each job's work per
   tile. With one job of share 1 it is `divide` exactly, the same floating-point operations in
   the same order: `divide` becomes `split` with one job (keep `divide` as a thin wrapper or
   replace its callers; `reap` and `claim.rs` call `split`). Unit tests in
   `core/tests/actors.rs`: one job equals `divide` bit for bit on several inputs; two jobs at
   shares 0.25/0.75 give each job its part less its walks; routes are paid once, not per job; a
   job with no tiles gets nothing and its share is not handed on (say so in the doc).
2. **`actors::Ladder`**: the labour an actor has left this turn, spent down rung by rung. It
   holds `left` and offers the spends the camp and scout share: a forage or pillage on a tile
   (`strip` for the camp, `graze_at` for the scout: the rung takes the function's result, so the
   two keep their own take rules), a resupply along the line (`supply`), and a plain `spend(used)`
   for the march and the search. `camp_turn` and `scout_turn` spend their `left` through one
   `Ladder`, the same subtractions in the same order as now; `march`, `forage_round`, `bring`
   and `search` take `&mut Ladder` (or its `left`) instead of `&mut f64`. Put `Ladder` in
   `actors.rs` if it knows nothing of units; if it must call `strip` and `supply`, put it in
   `units/forage.rs` and say why in the doc. A camp's hold and march already return its left;
   keep that.
3. **The guard's meal through the actor.** Add to `actors.rs` the one rule for eating a need
   from a store and then from kin's stores, biggest first (ties to the lower id), returning what
   was got: what `guard::turn` does now. `guard::turn` calls it with the civ's store and its
   clans' stores. Same order, same `take`, same result.
4. **`units/muster.rs`**: move raising (`raise`, `raise_scout`, `raise_scout_at_founding`,
   `make_scout`, `make_camp`, `join`, `take_people`, `givers`), disbanding (`disband`,
   `disband_kind`, `home_clan`), `give_up`, `surrender` and `nearest_clan` out of
   `units/mod.rs`, re-exported from `units` so no caller changes. `mod.rs` keeps the types, the
   JSON, the queries and the `units` step, and drops under 450 lines. Fix `mod.rs`'s module doc
   and `engine/CLAUDE.md`'s `units/` line (add `muster.rs`, `forage.rs`).

## Proof and record

- `engine/check.sh` green with `cli/tests/save.rs` untouched and `SAVE_VERSION` 12.
- `sansfiat digest --seed N --size small --turns 600 --lead` for seeds 1, 2 and 9 (past the
  pick, the scripted game giving orders, so camps, scouts and guards all run) on the branch and
  on the branch's base (`git worktree add /tmp/base origin/main`): identical. Paste the three
  lines in your summary.

## What later steps rely on

`split` with jobs (build-4 puts per-person walks into `jobs`); `Ladder` in every unit turn
(build-4 charges the scout's walk through it); the kin meal in `actors`; `muster.rs`.

## Done

`check.sh` green, the pins untouched, the digests equal.
