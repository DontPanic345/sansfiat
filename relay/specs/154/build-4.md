# #154 build-4 (canvas): the per-person walk

A walk is paid by every person who walks it. A clan of twenty carrying its surplus to the
festival pays more than a clan of two, and festivals and territories don't collapse. Then the
ring's proof on the finished rules. Read `spec.md` (item 7, the measures table, What I
settled on the weighting, Unsettled, decisions 4–7) and `probe.md`, where the spec step tried
the walk on main.

What the easels give you: `actors::split` with jobs, each job's walks one per tile (build-1);
`actors::Ladder`, through which the camp's and the scout's turns spend their labour (build-1);
every number in days and the path searches on `grid` (build-2), with `core/tests/day.rs`
guarding reach; `line::ring`, the ring layer and the judge's `supply` group with hungry rates
and territory tiles (build-3). You may change an easel where the walk needs it; record it as a
decision.

The code: `core/src/actors.rs`, `core/src/clans.rs` (`walks`, `reap`, `harvest`'s `c.walk`),
`core/src/markets.rs` (`worths`, `go`), `core/src/civs.rs` (the civ clan's route to the
capital), `core/src/claim.rs` (the guard's division), `core/src/units/camp.rs` (`hold`, the
zone's guard walk), `core/src/units/scout.rs` (`march`, `round_plan`, `forage_round`),
`core/src/units/search.rs` (`search`: its marches), `core/src/data.rs`,
`core/tests/scout_errand.rs`, `cli/tests/save.rs`.

## The rule

- **Inside a territory**, a tile is walked by the job's people working it: the job's people
  (people × its share) over its tiles. Its yearly walk is the path's cost × that many × the
  job's inside weight. `split` takes the people a tile so every actor that divides (clans, the
  guard over the claim, a camp's zone) pays the same way.
- **On a route**, the walkers are the carriers of what the actor brings: brought ÷ `CARRY.load`,
  at least one. A clan's festival route reads what it brought last turn (its basket, capped at
  `MARKET.carry × people` as `trade` caps it), in `worths` (so the clan weighs the cost) and in
  `go`; the civ clan's route to the capital likewise. Add `CarryRules` / `CARRY` to `data.rs`
  with `load` (food a person carries on a route) and a note.
  `MARKET.reach_walk` stays a limit on one walker's walk (`worths`, `civs.rs`'s capital switch,
  the clans in reach of a forming market all read it from the path): the carriers multiply
  what a route costs, not whether a festival is in reach. The probe ran it this way.
- **A unit walks together.** The camp's march already pays one warrior's labour a step, each.
  The scout's march, and the walks of its foraging round, are paid by each of its people:
  a step costs `step_cost × people` from its `Ladder`, so a scout walks at one person's pace. Its
  reveals and trail work are not walks and stay pooled.
- The camp's line needs no change: what it carries is already its people's days over its cost.
- `SAVE_VERSION` bumps and the pins move.

## What to tune

`CARRY.load` and the inside weights (`CLAN.inside`, `GUARD.inside`, `UNIT.inside`), and, if the
scouts need it, `SCOUT.wander`, `low`, `fed` and `rich`. Starting point from `probe.md`: the
territory walk at half its naive weight (`CLAN.inside` 0.125) and a load of 2 food. A load of 3
or more makes every clan send one walker (no clan brings 3), so it changes nothing; 1 collapses
the festivals. Guard and zone weights start where they are and move only if the claim or the
zone collapses (`held_200`, `guard_200`).

## What proves it

Run `sansfiat sweep --judge --json --floor` on the branch, on the base (`git worktree add
/tmp/base origin/main`) and on two last-bit nulls of the base (`CLAN.work` × (1 ± 1e-12), each a
worktree), once each for the controls; then one sweep per setting you try.
`judge --compare` base against each. A setting passes when:

1. Pre-games end by centres on 7 of 7; or on 6 if the same setting with `CLAN.work` × (1 + 1e-12)
   ends 7 (chaos alone lost one in `probe.md`'s unit2).
2. Markets at the pick, and the trading share at the pick and at +200, are no lower than the
   lowest of base and the nulls by more than the nulls' span (the largest of base and the nulls
   less the smallest): the sweep's floor is 0 at the pick, and the nulls move these by 0.015 and
   8 markets (`probe.md`).
3. Mean territory tiles a clan (the `supply` group), clans joined, people changed and land
   changed stay above three quarters of the lowest of base and the nulls; AI orders a hundred
   turns above half the base's.
4. The ring holds: `cargo test --release -p engine --test ring -- --ignored --nocapture` (build-3's
   staged armies on the retro maps) passes, and scouts inside the ring are hungry under 0.05 of
   their turns (the `supply` group). The camp's line doesn't change, so a failure here is the
   store or the territory, not the ring: say which.
5. `scout_errand`: all 28 alive at 50 turns.
6. **A walk costs more for a clan that carries more**: a table, on the seven maps at +100, of
   each clan's festival walk in days against what it brought, in bins of brought a person, rising
   with what a clan brings; and a stage test (`core/tests/actors.rs` or `clans.rs`) where a clan
   of twenty bringing its fill pays more days to the same festival than a clan of two.

Of the settings that pass, take the one nearest the probe's start. If none passes 1–5 with the
festival walk still rising (6), send an `ask` with the nearest settings and what each failed,
and keep the nearest.

`core/tests/day.rs` changes only where the per-person walk changes a reach (the scout's march,
a territory); list each in `walk.md`.

## Record

`relay/specs/154/walk.md`: the settings tried and each one's bounds; the chosen one's compare
against base and the nulls (the totals of standing, doubt, decisions, motion, land, and the
`supply` group); the walk table; the ring table on the finished rules beside build-3's
`ring.md`; `scout_errand`'s table. The note beside each tuned number in `data.rs` names `walk.md`.

## Done

`check.sh` green; `walk.md` written; the bounds met or the `ask` sent.
