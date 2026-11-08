# #155 Fields: spec

## What it delivers

After founding, a clan splits its days between two jobs: gathering over its territory, and fields on the
river-side and floodplain ground of its tiles. The split moves a notch a turn toward the one that feeds it
better. Nobody orders a clan to farm. A field yields far more grain a day than gathering, and it grows with the
tile's fertility and its tame. It takes no stock and wears nothing. A farmer's home follows its fields. A
farming clan reaches its harvest target from less land, so the land carries more people. Its bigger basket pays
a bigger levy into the store.

1. **Field ground** (`fields::ground(w, i)`): the field days a tile takes a turn. A floodplain takes
   `FIELD.flood` days. A river tile takes `FIELD.river` × min(river ÷ `MAP.big_river`, 1), and a floodplain on a
   river takes both. Every other tile takes none. Any land terrain counts: the bank is farmed whatever grows
   back from it, and the tile's fertility scales the yield. Ground is worked out from the map's rivers and
   floodplains, not rolled with the world's rng, so no map changes.
2. **The yield** (`fields::yield_a_day(w, i)`): grain a field day, `FIELD.grain × fertility × (domestic(tame) − 1)`.
   Wild grain (tame 0) yields nothing in a field, and a hearth of fertility 1 yields `1.5 × FIELD.grain`. The
   yield is net of seed.
3. **The split** (`Clan.fields`, a share of the days left after routes, 0 to 1). `clans::reap` calls
   `actors::split` with two jobs: gathering at `1 − fields` over the territory, and fields at `fields` over the
   territory's tiles with ground, each tile paying its walk as now. A field tile works at most its ground's
   days. Days past that fill the job's other field tiles up to their ground, and what no tile takes stays
   unworked. Field grain comes from no stock, and `land::wear` never sees it. `CLAN.surplus_target` caps
   gathering and fields together, scaled by one factor as now.
4. **The notch** (the `move` step, after the territory changes). With `Config.fields` on and civs founded, a
   clan weighs `value` at its share ± `FIELD.notch` on this turn's stock, as `change` weighs a tile. It moves
   one notch toward the better, if that beats the current share by more than `FIELD.margin`. A starving clan
   with fields drops a notch instead. When the share rises from 0, a `ClanFarming` event goes to the feed:
   "Clan 140 takes up fields at (3, −2)". When it falls back to 0, the clan gets a note.
5. **The farmer's home.** A clan whose share is at least `FIELD.farmer` is a farmer. Its home steps one tile a
   turn toward its field tile with the most field days, ties to the lower tile, as `recentre` steps toward the
   middle. The territory then trims to the radius. The civ's own clan stays on the capital.
6. **Tame under fields.** A tile worked as a field this turn tames `FIELD.tame` more steps a turn, past the
   hearth too.
7. **`Config.fields`**: on by default and in old saves. Off is the control, in which no clan farms.
   `SAVE_VERSION` bumps.
8. **The judge** (`core/src/judge_fields.rs`, a `fields` group): field tiles, field days and farming people at
   +100 and +200, and farmers. For each civ: its farming share at +200, its store turns at a pack, and its
   Standing at +200 against the no-fields control's. Civs never at a pack. Home moves and moves-on per 100
   clan-turns, for settled farmers (farmers ten turns in a row) and for gatherers. Starving clan-turns for
   farmers, for gatherers, and in all against the control. `sweep --judge` runs the no-fields control as a
   sixth run a map.
9. **The page.** The civ panel shows how its people live: "Living: gathering 82 · farming 14 · in the army 6". The
   clan panel shows a farmer's fields: tiles, days and grain, and the doing line "Farms a fifth of its days".
   A fields layer draws furrows on every tile with fields, on by default. The tile tooltip gives the tile's
   field ground and its fields. The feed shows `ClanFarming`. The text view prints a clan's fields.
10. **The design edits** (build-2): `clans.md` Taming and Out of scope, `actors.md` Later actors (the text is
    in build-2's brief).

## Shape: Easel and Canvas

The piece's `Shape:` line suggests Clay, Kiln, Glaze, with the field job and its notch as the clay. The spec
step built that clay as a sketch on main (`probe.md`, `probe.diff`), about a hundred lines, and ran it on the
retro maps. Its shape held at the first try. Farming civs gained Standing, fields-off matched main byte for
byte, and the turn got faster. Built properly, it scores 4 (`steps.json`): a rule set with its numbers given,
proved by tests and by a sweep against the probe. That isn't the hard part. The hard part is what only the
maps judge: the yield, the notch, the margin, the ground and the farmer's line, tuned together until the store
holds a pack, settled farmers stop moving, and nobody starves who shouldn't. The probe meets some of these
bounds and misses others. So the rules are easels, all settled, and the tuning is the canvas on top:

- **build-1** (easel): the field job. Field ground, the yield, the split's two jobs, the notch, the starving
  fallback, the farmer's home, tame under fields, `Config.fields`, `ClanFarming`, and tests on a stage.
  Starting numbers are the probe's y8.
- **build-2** (easel): the judge's `fields` group, the no-fields control in the sweep, the feed's kind and
  sound, and the design edits.
- **build-3** (easel): the page. The civ's living row, the clan's fields, the fields layer, the tile tooltip,
  the text view, and `www/play/fields.mjs`.
- **build-4** (canvas): the numbers, tuned on the retro maps against the no-fields control, until every bound
  below holds. It may change the easels' rules where a bound needs it, and records each change.

## The question and how it is measured

**The question:** Standing and motion. On the retro maps, a civ whose clans farm gains Standing over one whose
clans don't. Its store holds an army's packs on most maps. Farming clans stop moving.

**The control.** The no-fields control is the same map with `Config.fields` off. Fields open at the pick, so the
pre-game is untouched and the control differs from the run only from the pick, as the floor's null does.
`sweep --judge --floor`'s floor holds, and no last-bit nulls are needed: the probe's fields-off sweep equals
main's byte for byte (`probe.md`). A move counts when it is past the floor.

A civ **farms** when its clans put at least a tenth of their people's days to fields at +200 (`farming share`,
Σ people × fields ÷ Σ people over its clans). A **farmer** is a clan whose share is at least `FIELD.farmer`. A
**settled farmer** has been a farmer for ten turns in a row. A **gatherer** has share 0.

| Score | Measure | Control | Probe (y8) |
|---|---|---|---|
| Standing | **Farming civs gain**: per civ, Standing at +200 less the no-fields control's. Farming civs gain at least +0.01 on average, three quarters of them gain, and their mean beats the mean of the civs that don't farm by at least 0.01. Aligned Standing at +200 rises past the floor. | No-fields control, per civ | +0.036, 11 of 12 gain; the others −0.002; aligned 0.253 → 0.312 |
| Standing, health | **Fields pay**: at a hearth of fertility 1, a field day yields at least 1.5 times a gathering day's 0.75 food (`1 ÷ CLAN.harvest_labour`): `farming.md`'s "far more per day". | The rule | 1.2 food a day, 1.6 times |
| Health | **The store holds a pack** (`units::pack_cap(UNIT.first)`, 15 food): on at least 5 of 7 maps, the civ that farms most holds one on at least half its turns, and at most 2 of 21 civs never hold one. | No-fields control: 4 of 21 never | 5 of 7; 2 of 21 never |
| Motion | **Settled farmers stop moving**: home moves per 100 clan-turns for settled farmers, summed over the seven maps, are under half the gatherers' rate. Settled farmers move on (`ClanMoved`) no more than gatherers do. | Gatherers in the same runs | Not measured: the probe counted farmers from their first notch, and they moved 0.13% against 0.24% |
| Motion | People, land and clans changed hands no fewer than in the control, less the floor. | No-fields control | Joined 125 → 172, people 629 → 915, land 1022 → 1105 |
| Health | **Nobody starves who shouldn't**: starving clan-turns as a share of clan-turns, over the seven maps, at most 1.25 times the control's, and on no map over 1.6 times. Farmers starve at a lower rate than gatherers. | No-fields control | 1.29 times; large 8 1.47 |
| Health | Pre-games by centres 7 of 7. Panics 0. Digest mismatches 0. With fields off, `sweep --judge --json` equals main's in every map's scores. AI orders a hundred turns no fewer than half the control's. | Main; the control | 7, 0, 0; equal; 7.4 against 6.9 |
| Health | `sansfiat bench --size large 8 --after 50` within 1.25 times main's ms a turn. | Main | 90.8 against 158.8 |
| Doubt | Lead changes and the spread, reported against the control. Not bound. | No-fields control | Lead changes 9 → 4, spread at +200 0.050 → 0.095 |
| Health | `engine/check.sh` green at every step. `www/play/fields.mjs` passes. | | |

**Before the piece** (`probe.md`). The probe's rules at yield 0.8 meet the Standing bound and the store bound,
and miss the starving bound overall (1.29 against 1.25). Its home rule makes new farmers walk to their fields,
so farmers counted from their first notch move more than gatherers on 4 of 7 maps. Without the home rule they
move less on 6 of 7. The proposal's "14 of 21 never hold a pack" is the M3 retro's count. Today 4 of 21 never
do. On the two civs that still never do at y8, the guard eats the levy.

## What I settled

- **Fields open at the pick.** Before founding, no clan farms (decision 1).
- **Field ground per terrain and river** (the designer settles): a floodplain 20 days (`15 × DAY`), a river tile
  up to 10 days (`7.5 × DAY`) in proportion to its size up to `MAP.big_river`, on any land terrain, and nothing
  elsewhere. Lakes and the coast take none: irrigation widens the ground later. A clan of eight gives 37 days
  a turn, so one floodplain can take most of a small clan's days. That is the scale `labour.md` asks for: a
  tile is far bigger than anyone works.
- **The yield curve against tame** (the designer settles): linear in the tame grain past wild,
  `FIELD.grain × fertility × (domestic(tame) − 1)`. Fields are worth nothing until the grain has changed under
  the clan's hands, and pay more as it keeps taming. Start: `FIELD.grain` 0.8 food a day.
- **The split's notch and margin** (the designer settles): 0.1 of the days a turn, moving only if the better
  share beats the current by more than 0.01 food a person in `value`, as `CLAN.land_margin` (0.02) does for a
  tile.
- **How tame moves under fields** (the designer settles): a tile with fields tames one more step a turn
  (`FIELD.tame` 1), past the hearth too. Grain that is sown and cut tames faster than grain that is only cut.
- **The farmer's line**: `FIELD.farmer` 0.1. Every clan with fields counts as a farmer to start, as in the probe.
- **The page's look**: furrows, three short parallel strokes in the tile's lower half, darker with more field
  days. Drawn over the terrain and under the territories, on by default under View → Fields.
- **Where the code goes**: `core/src/fields.rs` for the field job. `clans.rs` is 542 lines and calls into it.
  `core/src/judge_fields.rs` for the judge, since `judge.rs` is over 600 lines.

## Unsettled, and which step holds it

All in build-4, the canvas:

- **`FIELD.grain`, `notch`, `margin`, `flood`, `river`, `farmer` and `tame`**, tuned together. What proves them
  is every bound in the table, read from `sweep --judge --json --floor` against the no-fields control. The
  starving bound is the one the probe misses. The store bound passes at y8 with nothing to spare.
- **Whether the home rule needs a line.** If settled farmers still move, the canvas may raise `FIELD.farmer` so
  that only a clan farming a real share follows its fields. It may also make the home follow the field days'
  middle rather than the best tile. Either is recorded as a decision.
- **If no setting meets the store bound** because the guard eats the levy, the canvas reports the store's flow
  for each failing civ (levy in, gifts, posting, the guard's meals, spoilage, the cap) and sends an `ask`. The
  levy, the store, the gifts and the guard work as now (the piece's Decided).

## What the review runs

- `engine/check.sh`.
- `sansfiat sweep --judge --json --floor` on the branch and on main (a worktree), then
  `sansfiat judge --compare main.json branch.json`. The fields group's per-civ table and every bound above go in
  the PR: Standing against the control, the store, the moves, the starving.
- With fields off (`Config::default()`'s `fields` false, locally): `sweep --judge --json` equals main's in every
  map's scores.
- `cargo test --release -p engine --test fields -- --nocapture`.
- `sansfiat bench --size large 8 --after 50` on the branch and on main.
- `node www/play/fields.mjs`: the staged farmer's fields row, the civ's living row and the fields layer, with a
  look at its screenshot.
- A played game (`sansfiat play --game g.json`, small seed 9, the civ with river ground): play 60 turns past the
  pick, and read the feed's "takes up fields", the clan panel's fields and the civ's living line.

No new button or order: nobody orders a clan to farm.

## Decisions I made that the piece left open

1. **Fields open at the pick.** The pre-game is `clans.md`'s proof, whose Out of scope keeps farmers out. The
   hearths tamed by the pick are what farming starts on ("Tamed grain is what makes a fertile area a centre that
   can feed a civ"). The pre-game and its end stay byte for byte, so the no-fields control differs only from the
   pick and the sweep's floor holds. Every clan may farm after the pick, unaligned clans included.
2. **Design says the map rolls field ground.** It is worked out from the rivers and floodplains the map already
   rolls, not given a roll of its own. Another rng call would move every map and every retro pick. The variety
   is in the rivers' sizes and the tiles' fertility.
3. **Field ground is in days**, the days a tile's fields take a turn (`labour.md`: "A field is sized by the days
   it takes"). Days past a tile's ground go to the job's other field tiles, and past all of them stay unworked.
4. **A field yields nothing on wild grain**: the yield is in the tame past wild, so fields follow taming.
5. **A field draws on no stock.** The tile's wild grain stays to gather, and its wear is the gathering's alone
   ("a field doesn't wear").
6. **The harvest target caps fields and gathering together.** A farmer that reaches its target with fewer days
   leaves the rest unworked. Its gain is less land for the same food, so the land carries more people, and
   more of its basket goes to the levy.
7. **The notch weighs this turn's stock, as `change` does**, and runs in the `move` step after the territory
   changes, so a clan settles its land and its split in one step.
8. **A farmer is a clan with at least `FIELD.farmer` of its days on fields.** Its home follows its field tile
   with the most days. The judge and the page use the same line.
9. **The shape is Easel and Canvas**, not Clay, Kiln, Glaze. The probe settled the clay's shape on the maps, and
   what is left is tuning that only the maps judge.
10. **Design says 14 of 21 civs never hold a pack today.** That was the M3 retro's count. Today it is 4 of 21,
    so the store bound is read against today's control: the civ that farms most holds a pack on at least half
    its turns on 5 of 7 maps, and at most 2 of 21 civs never hold one.
11. **"Farming clans stop moving" is read for settled farmers**, ten turns in a row a farmer. A new farmer's walk
    to its fields is the home rule working, not moving on.
12. **"In the army" on the civ panel counts every unit's people**: camps, the guard and scouts. They are the
    people the civ has taken from its clans.
