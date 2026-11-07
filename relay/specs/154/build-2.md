# #154 build-2 (easel): the labels and the day

Two commits. First every number says its unit, with no arithmetic change (digest unchanged).
Then the day becomes the step's unit: a trail on flat land costs one day, everything priced in
labour scales with it, and no reach changes. Read `spec.md` items 5 and 6, decisions 1 and 2,
and `probe.md`, where the spec step ran this scale on main.

The code: `core/src/data.rs`, `core/src/map.rs` (`roll_trails`, `MAP.mountain_cost`, `weighted`),
`core/src/land.rs` (`step_cost`, `flat_cost`, `cost_map_by`, `path_by`, `Entry`),
`core/src/actors.rs` (`stride`, `Walks::new`), `core/src/markets.rs` (`worths`: "every step costs
at least 1"), `core/src/stage.rs`, `core/src/maptext.rs`, `cli/src/view.rs`, `www/panels.js`,
`www/screens/play.js` (the tooltip), `web/src/lib.rs` (`rules`), the tests under `core/tests/`
and `cli/tests/save.rs`.

## What to build

1. **The labels** (commit 1, digest unchanged). Every number's doc in `data.rs` names its unit,
   in these words: *days a person a turn* (labour a person gives), *days a step* (a move or trail
   cost), *days* (a trail's work, a reveal, a worth, a margin or a limit of a walk), *days a
   food* (harvest labour), *food a day* (forage, pillage), *food a person a turn* (eating),
   *food* (a load, a store, a gift), *turns*, *people*, *hex steps*, or *a share*. `UNIT.carry`
   reads *food a day of line per day it costs*. `view.rs`, the panels and the tooltips print a
   labour or a cost as days: "line 5.3 days, carries 2.0 food a turn", "known route 12 days".
   Add one line to `data.rs`'s top: a day is one person's labour for one day; a turn is a year
   of `CLAN.work` days, the days a person gives (`labour.md`).
2. **The pins before the scale** (commit 1). Write `core/tests/day.rs`: on staged maps
   (`Stage::plain`; a stage of hills and forest with hidden trail costs set; a river with a
   crossing; a plains path worn to a trail), record and assert: the camp's `line::turns` to
   every tile within 6 steps of the capital; `line_from`'s carry over a 3-person army's eating at
   each; the clan territory after 20 turns from a fixed start; a 2-person scout's march steps on
   a long path over one turn; the festivals in a clan's `worths`; and `MARKET.reach`'s tiles round
   a market. Assert counts and tile lists exactly, and labour or cost divided by `DAY` (1.0 in
   this commit) to 10⁻⁹. It passes on the unscaled rules.
3. **The day** (commit 2). Add `pub const DAY: f64 = 4.0 / 3.0;` to `data.rs` with a note: the
   days a bare plains step takes, so a trail on flat land (`TRAIL.flat` of it) takes one day.
   - **× `DAY`**: every `TERRAIN.move_cost`; `RIVER.cross`; the `MAP` hidden costs
     (`forest_cost`, `hills_cost`, `mountain_cost`, now `f64`, so `roll_trails` and the mountain
     line take them as they are); `TRAIL.work`; `MARKET.reach`, `reach_walk`, `gather_bonus`,
     `desire`, `company`, `switch_margin`, `stay_margin`; `CLAN.work`, `CLAN.harvest_labour`;
     `CIV.pull`; `SCOUT.reveal`; `CLAIM.floor`; `GUARD.guarding`; `UNIT.labour`,
     `UNIT.guarding`, `UNIT.guard`. Write each as `3.5 * DAY`, so the old number shows.
   - **× 1/`DAY`**: `UNIT.forage`, `UNIT.pillage` (food a day).
   - **Unchanged**: weights (`CLAN.inside`, `GUARD.inside`, `UNIT.inside`), `TRAIL.flat`,
     `UNIT.carry` (days over days), food, turns, people, hex steps and shares. Check every other
     literal compared with a cost or a labour in the code (`grep` for `cost`, `labour`, `left`)
     and scale it if it is one; list what you found in `day.md`.
   - **The grid.** A third isn't exact in binary, so sums that tie on main stop tying. In
     `land.rs` add `fn grid(c: f64) -> i64` (`(c * 12.0 / DAY).round()`: a twelfth of a day, on
     which every step after the scale lands exactly) and compare costs through it in
     `cost_map_by` and `path_by` (the heap's `Entry` order, the stale check, the relax and the
     `max` limit) and in `Walks::new` (the scan and the relax); `stride` takes a step whose cost
     is within 10⁻⁹ of what is left. The spec step tried this (`probe.md`, unit2).
   - The stages (`Stage::plain`, `maptext::parse`) take their costs from `TERRAIN` and
     `flat_cost`, so they scale with it; a test that sets a cost by hand scales it by `DAY`.
   - `day.rs`'s assertions pass unchanged (its divisions by `DAY` now divide by 4/3). An
     assertion of a rule elsewhere may change only by `DAY`; list every changed assertion in
     `day.md`.
   - `SAVE_VERSION` bumps and the pins move: the digest moves with every float.

## Proof and record

- `engine/check.sh` green after each commit; `day.rs` unchanged between them.
- `sansfiat sweep --judge --json --floor` on the branch, on the base, and on two last-bit nulls
  (worktrees of the base with `CLAN.work` × (1 + 1e-12) and × (1 − 1e-12)); `judge --compare`
  base against each. Table the three changes per metric in `day.md` beside `probe.md`'s unit2
  column. A report: the scale's moves should look like the nulls'. A metric whose change is past
  both nulls' and the floor by more than either null's change is listed with what you checked.
- `node www/play/all.mjs` passes with the days in the panels; one screenshot of the camp's panel
  in `relay/specs/154/` named in `day.md`.

## What later steps rely on

`DAY` and every number in days; `grid` in the path searches; `day.rs` as the reach guard
(build-4 keeps it green or changes only what the per-person walk changes, and says so).

## Done

`check.sh` green; `day.md` written; `engine/CLAUDE.md`'s `data.rs` line says numbers are in days.
