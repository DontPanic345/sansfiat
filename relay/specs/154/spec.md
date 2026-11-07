# #154 The day: spec

## What it delivers

Every number reads in days of one person's labour and turns of one person's eating; an actor
splits its days between jobs before each job divides over its tiles; a walk is paid by every
person who walks it; and the supply ring is drawn.

1. **The split** (`actors::split`). An actor pays its routes once, gives what is left to each
   job by its share, and each job divides over its own tiles as `divide` does. With one job it
   is `divide` exactly, and `clans::reap` calls it. `stats::digest` is unchanged for every seed.
2. **The ladder** (`actors::Ladder`). The camp's and the scout's turns spend their labour down
   one type: the same rungs (one-shot forage or pillage, one-shot resupply, auto-forage,
   auto-supply, and the scout's march and search between), the same subtractions in the same
   order. Digest unchanged.
3. **The guard's meal** goes through the actor: `actors` gains the one rule for eating from a
   store and then from kin's stores, and `guard::turn` calls it. Digest unchanged.
4. **`units/mod.rs` splits.** Raising, disbanding, surrender and giving up move into
   `units/muster.rs`; `strip` and `supply` already sit in `units/forage.rs`, beside `line.rs`,
   and `supply` reads its line from `line.rs` as now. `mod.rs` keeps the types, queries and the
   `units` step. Digest unchanged.
5. **The labels.** Every number in `data.rs` says its unit in its doc: days a person a turn,
   days a tile (a step), days a unit of food (harvest), food a person a turn, food a day (forage),
   turns, people, hex steps or a share. `view.rs`, the panels and the page's tooltips print
   labour as days ("4.7 days").
6. **The day as the step's unit.** A trail on flat land costs one day. Every terrain cost,
   hidden trail cost, river crossing, trail's work, labour a person gives, labour a unit of
   food, and every limit or worth written in labour scales by 4/3; food per labour scales by
   3/4; shares, turns, food and hex steps stay. The `MAP` hidden-cost rolls become `f64`. The
   path searches compare costs on a grid of a twelfth of a day, so paths of equal cost stay
   equal although a third isn't exact in binary. No reach changes: a stage test shows every
   reach the same before and after.
7. **The per-person walk.** A walk's days are its path's cost times the people walking it.
   Inside a territory the people walking to a tile are the job's people working it; on a route
   they are the people carrying what the actor carries there, a load each, and at least one. A
   clan of twenty carrying its surplus to the festival pays more than a clan of two. The
   scout's march and foraging round are paid by each of its people, as the camp's march already
   is. `SAVE_VERSION` bumps.
8. **The ring** (`units::line::ring(w, civ)`): for every tile a line from it may reach the
   capital, the share of an army's days the line takes to feed it there:
   `UNIT.food × line cost / (UNIT.labour × UNIT.carry)`, which no army's size changes. Inside
   the ring the share is under one. `Sim.ring(civ)`; a `www/map/layers/ring.js` layer drawn when
   an army or the civ is selected; the tile tooltip gives the share; the judge counts it.
9. **The design edits.** `supply.md`, The pack: the pack is the army's train, and its size
   stays in turns.

## Shape: Easel and Canvas

The piece's `Shape:` line suggests Clay and Kiln, with the split and the ladder as the clay.
Scored, that clay comes to 3 points (`steps.json`): it is a refactor proved byte for byte by
the digest pins, not the hard part. The hard part is the per-person walk. A naive one collapses
festivals and territories on the retro maps (`probe.md`), and how it is weighted can only be
judged on the maps, together with the ring proof it changes (it halves a two-person scout's
march). So the split, the ladder, the labels, the step's unit and the ring are easels, all
settled, and the walk is the canvas on top of them:

- **build-1** (easel): the split, the ladder, the guard's meal, `units/muster.rs`. Digest
  unchanged.
- **build-2** (easel): the labels, then the day as the step's unit, on the twelfth-of-a-day
  grid. Digest moves, reach doesn't.
- **build-3** (easel): the ring in core, wasm, the page and the judge, and `supply.md`'s line.
- **build-4** (canvas): the per-person walk and its weighting, tuned on the retro maps, and the
  ring's proof on the finished rules.

## The question and how it is measured

**The question:** motion and health. No digest moves under the easel of build-1. The ring is
where armies and scouts live, and past it is where they starve, on the retro maps. A walk costs
more for a clan that carries more, and festivals and territories don't collapse.

**The control.** `sweep --judge --floor`'s floor comes from one null that differs at the pick,
so it holds nothing that moves the pre-game. Any change to a float does: main with `CLAN.work`
nudged by one part in 10¹² flags 20 of 63 metrics against main, and two such nudges span the
AI's orders a hundred turns from 6.4 to 8.8 and clans joined from 106 to 141 (`probe.md`). So a rule that
touches the pre-game is read against **the last-bit nulls**: two worktrees of the branch's base
with `CLAN.work` × (1 ± 10⁻¹²), each swept with `--json --floor`. A move counts when it is
past the spread of main and both nulls by more than the sweep's floor. A fixed seed's change
is chaos, not a score.

| Score | Measure | Control |
|---|---|---|
| Health | **No digest moves under the split, the ladder, the guard's meal and the muster split**: after build-1, `cli/tests/save.rs`'s pins and `SAVE_VERSION` are unchanged, and `sansfiat digest --seed N --size small --turns 600 --lead` for seeds 1, 2 and 9 matches main's. | Main's pins. |
| Health | **No reach changes under the day**: `core/tests/day.rs` (build-2) records, on staged maps (`Stage::plain`, a hills and forest stage, a river crossing, a trailed path), the camp's `turns` to every tile within 6 steps, `line_from`'s carry over the people's eating, the clan's territory after 20 turns, a scout's march steps, a festival's reach and `MARKET.reach`'s tiles, run before the scale with the numbers pinned; after it they match: counts exactly, labour over `DAY` within 10⁻⁹. On the retro maps `judge --compare` against main is reported beside the two nulls, not bound. | The pinned numbers from before the scale; the nulls. |
| Health, motion | **The ring is where armies live, on the retro maps**: the AI raises no army before R (`units/ai.rs`), so `sweep --judge`'s runs have no army-turns to count (`probe-ring.md`). The proof stages them: `core/tests/ring.rs`'s report test (`#[ignore]`, run with `--ignored --nocapture`), on each of the seven retro picks, leads each civ in turn and stages a 3-person army (`Stage::army`, auto-supply and auto-forage on) on one tile in each band of the ring's share (under 0.5, 0.5–0.75, 0.75–0.9, 0.9–1, 1–1.1, 1.1–1.25, 1.25–1.5, 1.5–2, 2 and up) within 8 steps of the capital, and runs 20 turns. A turn counts as starving when the army is hungry, or after it is gone with its civ's store holding; a turn whose civ's store ends under the army's turn of food is set apart as dry, since an empty store feeds no line (on main four civs' stores run dry under a 3-person army, and their armies die inside the ring). Under share 1 the starving rate is under 0.05; at 1.25 and up it is at least four times that and at least 0.2. Main: 0.000 under 1, 0.24–0.38 from 1.25 (hungry turns alone; the gone add more). Measured after build-3 and again after build-4; the table goes in `ring.md`. | Inside the ring against outside it, on the same maps and civs. |
| Health | **Scouts inside the ring**: the judge's `supply` group counts scout-turns and army-turns inside and outside the ring with the hungry turns in each, a report (army-turns are zero under the AI; a played game has them). Scouts inside the ring hungry under 0.05 of their turns (main: 3 of 12,355). Scouts forage past the line by design, so outside is reported, not bound. | Main. |
| Health, motion | **The ring on a staged walk-out**: `core/tests/ring.rs` (build-3) stands a 3-person army on each tile of a straight plains stage from the capital, no food on the tiles, auto-supply on, and runs 40 turns: on every tile with share under one it is never hungry; on the first tile past it is hungry before turn 40 (at share 1.14 its line brings seven eighths of what it eats, and a full pack spoiling a tenth a turn runs out near turn 29). `ring` agrees with `line_from` on every tile. | The rule's own edge. |
| Motion | **A walk costs more for a clan that carries more**: after build-4 the canvas tables, on the retro maps at +100, each clan's festival walk in days against what it brought, in bins of brought a person; the days rise with what a clan brings, and a clan of twenty bringing its fill pays more than one of two (also a stage test). | The same clans' walks at one walker (main). |
| Health, motion | **Festivals and territories don't collapse** (build-4): against main and the two nulls, with the per-person walk on: pre-games by centres 7 of 7 (or 6 when a last-bit nudge of the same setting ends 7, since the day's scale alone lost one to chaos); markets at the pick and trading share at the pick and +200 no lower than the lowest of main and the nulls by more than the nulls' span (the largest of main and the nulls less the smallest); mean territory tiles a clan (build-3 adds the count) and clans joined, people changed and land changed above three quarters of the lowest of main and the nulls; AI orders a hundred turns above half of main's. | Main and the two nulls. |
| Health | `engine/check.sh` green at every step; `scout_errand` keeps all 28 scouts alive after build-4. | |

**Before the piece** (`probe.md`, run by the spec step on main; `probe-ring.md`, run by the spec-review on main: the ring as the formula draws it is where staged armies live on the retro maps, and inside it they starve only when the civ's store runs dry): a 4/3 scale of the costs and the
labour, with or without the twelfth-of-a-day grid, moves the retro maps about as much as the
nulls do (clans joined 99 and 105 against the nulls' 106 and 141 and main's 131), and in one of the two
runs a pre-game ends by its backstop. A per-person walk with every route walker paying (a load
of 1 food) and every worker walking (no weight) collapses: clans joined fall to 78, people
changed to 378, held land to 21, a pre-game ends by its backstop. Halving the territory's
walkers and a load of 2 hold the pre-game, markets and trading.

## What I settled

- **The day's scale list** (build-2's brief): which numbers scale by 4/3, which by 3/4 and which
  stay, with the reason. `UNIT.carry` stays: it is food carried a day per day of line, and both
  days scale.
- **The grid**: path searches compare costs on a twelfth of a day. Every step cost after the
  scale is a multiple of a third of a day, so the grid keeps main's ties exactly.
- **The ring's formula** has no army size in it, so one map serves every army and the scout,
  and the page draws one ring a civ.
- **What the ring's layer shows at a glance** (the designer settles): the tiles inside the ring
  tinted in the civ's colour, deeper where the line takes less of an army's days, and a solid
  line along the ring's edge; nothing past it. A glance gives where the edge is and how much
  room there is to fight inside it. The tile tooltip says "Ring: its line takes 40% of an
  army's days here", or "Past the ring: its line can't feed an army here".
- **How the per-person walk is weighted** (the designer settles): by who walks, not by a
  discount. Inside a territory, each tile is walked by the job's people on it (the job's people
  over its tiles), at the job's inside weight; on a route, by the carriers of what the actor
  brings (brought ÷ a load, at least one); every unit walks together, so a camp's or a scout's
  step is paid by each person. Two numbers carry the weighting and the canvas tunes them:
  `CARRY.load` (food one person carries on a route) and the inside weights (`CLAN.inside`,
  `GUARD.inside`, `UNIT.inside`). The probe's starting point: the load at 2 food (before the
  day's scale; it is food, so the scale leaves it), and `CLAN.inside` halved. A person eats one food a turn,
  a turn of `CLAN.work` days (4.67 after the scale), so a load of 2 food is about nine days'
  eating: `labour.md`'s "a full load is some days' food". A load the canvas settles on outside
  1 to 3 food is read against that line and recorded.
- **The pack is the train**, its size in turns (`supply.md`).

## Unsettled, and which step holds it

All in build-4, the canvas:

- **`CARRY.load` and the inside weights.** What proves it: the festival and territory bounds
  above against main and the two nulls, the walk table rising with what a clan brings, the
  ring's hungry-turn bound, and `scout_errand`'s 28 scouts alive. The probe's halved territory
  walk and a load of 2 pass the pre-game and markets bounds; clans joined stayed at 116 and the
  AI's orders fell to 3.4 (chaos, or the walk: the canvas tells which against the nulls).
- **Whether a two-person scout at one person's pace keeps its errands.** If `scout_errand`
  loses scouts, the canvas retunes `SCOUT.wander`, `low`, `fed` and `rich` as #153 did, or sends
  an `ask` if no setting keeps all 28 alive.

## What the review runs

- `engine/check.sh`.
- `sansfiat digest --seed N --size small --turns 600 --lead` for seeds 1, 2 and 9 on the branch at
  build-1's commit and on main: identical.
- `cargo test --release -p engine --test day --test ring -- --nocapture`.
- `cargo test --release -p engine --test scout_errand -- --nocapture`: 28 alive.
- `cargo test --release -p engine --test ring -- --ignored --nocapture`: the retro maps' ring table, under 1 against 1.25 and up.
- `sansfiat sweep --judge --json --floor` on the branch, on main (a worktree), and on the two
  last-bit nulls; `judge --compare` main against each. The ring table, the walk table and the
  festival and territory bounds go in the PR. The `supply` group's scout-turns are a report.
- `node www/play/ring.mjs` (build-3): selecting the camp and then the civ draws the ring, and
  selecting a clan clears it; a look at its screenshot.
- A played game (`sansfiat play --game g.json`, small seed 2): raise an army, march it out past
  the ring's edge and back, and read the panel and the notes in days.

## Decisions I made that the piece left open

1. **Design says the step's unit leaves `sweep --judge --compare` with no move past the floor.**
   It can't: a third of a day isn't exact in binary, the pre-game is chaotic, and the floor
   holds only the game after the pick. A nudge of 10⁻¹² to one number flags 20 metrics
   (`probe.md`). The proof is a stage test that every reach is the same, and the retro maps are
   read against two last-bit nulls.
2. **The path searches compare costs on a twelfth of a day**, so paths that cost the same on
   main still cost the same.
3. **The shape is Easel and Canvas**, not Clay and Kiln: the split is a refactor proved by the
   digest, and the hard part is the walk's weighting, which only the maps judge.
4. **A walk is weighted by who walks**: a tile's workers, a route's carriers, a unit's people.
   No discount on the walk itself.
5. **A route's walkers are what the actor brought last turn over a load, and at least one**:
   the walk is decided before the harvest, so it reads the last one. A civ clan's route to the
   capital pays the same.
6. **The scout walks at one person's pace** (`labour.md`: nothing moves faster than a person):
   both its people pay each step, as a camp's warriors do. Its search labour (reveals and
   trails) is not walking and stays pooled.
7. **The camp's line needs no change for the per-person walk**: what it carries is already its
   people's labour over the line's cost.
8. **The ring is one map a civ**: the share has no army size in it, and the layer draws it for
   the selected army's civ.
9. **The ring reads held ground** as the line does (`civs::holder`): a rival's held tile is past
   the ring.
10. **The judge's ring counts go in `core/src/judge_ring.rs`**, since `judge.rs` is over 600
    lines, and the sweep's report gains a `supply` group.
11. **`strip` and `supply` stay in `units/forage.rs`**: they already sit beside `line.rs`, and
    the split is raising and disbanding into `muster.rs`.
12. **Design says the ring is where armies live on the retro maps; the AI raises no army there** (none before R), so the
    judge counts army-turns only for a played game, and the retro maps' proof stages a 3-person army in each band of
    the ring's share for each civ (spec-review, `probe-ring.md`). Turns whose civ's store has run dry are set apart:
    the ring is the line's reach, and a line from an empty store carries nothing.
13. **`MARKET.reach_walk` limits one walker's walk**, not the carriers': a festival is in reach
    by its path, and the carriers multiply what going costs (spec-review; `probe.md` ran it so).
