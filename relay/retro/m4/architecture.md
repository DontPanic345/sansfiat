# M4 retro: architecture

The engine on `main` at `7417e2d` (Fixes for M4, #301), against `docs/architecture.md` and the M3 retro's angle
(`relay/retro/m3/architecture.md`). What I measured:

- line counts of every engine file at M3's end (`ec1e47b`) and at each M4 merge: S `99a6904` (PR 159), E `b377337`
  (PR 178), T `270d0d3` (PR 204), D `659d4d7` (PR 226), F `929b84a` (PR 245), W `06ee26b` (PR 268), R `b345517`
  (PR 294), X `7417e2d` (PR 301). The script is `architecture/lines.sh` and the table `architecture/lines.txt`;
- `git show --stat` of the eight merges, and the briefs in `relay/specs/151` to `162`;
- `sansfiat bench --size large 8 --after 50` built from F, W and main, run in turn twice in one session on the
  Relay runner (`architecture/bench.txt`);
- a scratch profiler outside the repo that times each `TURN` step past the pick (`architecture/prof-main.rs`), and an
  instrumented copy of main's `units/ai.rs` and `units/line.rs` that times the planner's parts and counts its path
  searches (`architecture/instrument.diff`). The output is `architecture/steps.txt`;
- `sansfiat digest` from R's merge and from main on four retro maps, watched and led, for 250 turns, and on the check's
  save (`play/small-2.json`).

Everything else is read from the code, and says so where it is inferred.

**Verdict.** M4 grew the engine by 4,060 lines of core (9,720 to 13,780) and 124 tests (307 to 431), and the shape
held. Every piece but R grew the modules its brief named: S put the harness in `stage.rs` and `maptext.rs`, T split
the search out of `scout.rs`, D moved raising into `units/muster.rs` and the ring counts into `judge_ring.rs`, and W
moved homes and dividing out of `clans.rs` before it grew. The actor took the labour split (`actors::split`, which
the fields job and the guard both use) and the guard's meal (`actors::meal`). Three things slipped. **The planner
costs 6.5 ms a turn on large 8**, more than the festival search, so main runs 22.9 ms a turn: 0.52 of the turn before
#156, past `architecture.md`'s bound of 0.45. Most of it is 16 whole-map path searches a turn that a cache or a
reachability test removes. **`data.rs` is 1,024 lines**, and F's brief kept it whole because CLAUDE.md says every
number lives there, so two rules collide. **Tests and page scripts sit on retro picks**: 16 of 19 touch-point scripts
and 10 of 39 core test files read a seeded pick, and the ladder angle traces four of five second sessions to them.
M5 needs four things first, in order. The owner's clock call is first: a person gives 4.67 days a year (`CLAN.work`
3.5 × `DAY`), and pack animals price days per load. Second, a good with its own weight, spoil and use: today every
good is a food, spoils at one rate on the actor, and is summed as food by `market::total` in 34 places. Third, one
carrying rule on the actor that an animal changes: carrying is read in 15 files through three constants. Fourth,
`data.rs` split into a folder before M5 adds goods, herds and beer to it.

## 1. What grew where

### The modules

Core files at or past 450 lines on main (`architecture/lines.txt` has every file):

| File | M3 end | S | E | T | D | F | W | R | X (main) |
|---|---|---|---|---|---|---|---|---|---|
| `map.rs` | 1,236 | 1,238 | | 1,239 | | 1,244 | | | 1,244 |
| `data.rs` | 758 | | **797** | **817** | 887 | 928 | 955 | **1,024** | 1,024 |
| `units/ai.rs` | 140 | | 134 | | | | 153 | **620** | 620 |
| `civs.rs` | 610 | 618 | 615 | | 616 | | | | 620 |
| `clans.rs` | 542 | | | | | 621 | 615 | | 615 |
| `stage.rs` | | 500 | 523 | | | | 557 | | 557 |
| `units/scout.rs` | 630 | | | 538 | 534 | | 537 | | 538 |
| `markets.rs` | 418 | 421 | | | 428 | | 495 | | 495 |
| `judge.rs` | 492 | 556 | 623 | | 633 | 641 | 453 | 479 | 479 |
| `units/fight.rs` | 466 | | | | | | | | 466 |
| `units/mod.rs` | 781 | | | | 447 | | | | 447 |

New modules, by the piece that made them: S `stage.rs`, `maptext.rs`, `cli/compare.rs`, `cli/game_judge.rs`,
`cli/sweep_judge.rs` (the split of `cli/judge.rs`, 833 lines); E `over.rs`, `cli/sweep_text.rs`; T `units/search.rs`,
`units/forage.rs`; D `units/muster.rs`, `judge_ring.rs`; F `fields.rs`, `judge_fields.rs`, `cli/sweep_fields.rs`; W
`homes.rs`, `dividing.rs`, `reach.rs`, `towns.rs`, `judge_towns.rs`, `judge_scores.rs`, `cli/sweep_towns.rs`,
`cli/sweep_total.rs`; R `cli/sweep_led.rs`. The CLI is 4,509 lines and no file passes 494 (`story.rs`).

- **Every split the M3 retro asked for landed.** `units/mod.rs` went from 781 to 447 when D moved raising and
  disbanding to `muster.rs` (D's brief, `relay/specs/154/spec.md`). `cli/judge.rs` became `sweep_judge.rs` and
  `game_judge.rs` byte for byte in S (151 build, `steps.json` line 40). `scout.rs` lost its route search to
  `search.rs` in T (`relay/specs/153/spec.md` item 7).
- **Briefs named the file to grow, and the files to split into.** E split `sweep_text.rs` out of `sweep_judge.rs`
  before adding to it (`152/build-3.md`); D put its ring counts in `judge_ring.rs` because `judge.rs` was over 600
  (`154/spec.md` item 10); W made `judge_scores.rs` and moved 220 lines out of `judge.rs`, and put homes and dividing in
  their own files rather than growing `clans.rs` past 800.
- **R grew `units/ai.rs` from 153 to 620** in one whole session (#293), which had no brief to name a file. 620 is past
  the 600 at which a brief names a split, and the planner is the module the next milestone's herds and beer extend
  (an AI that herds, brews and feasts). Its natural cut is in its own doc: the needs (`gift`, `land`, `muster`) and the
  army's orders (`direct`, `raid`, `across`, `cut`, `stand`, `post_zone`), about 270 and 350 lines. This is inferred
  from reading it, not measured.
- **`data.rs` passed 800 in T and reached 1,024**, with every piece adding to it (+266 in M4; D's relabel in days was
  +70 of it). F's brief says "`data.rs` is over 600 lines and stays whole: every number lives there (`CLAUDE.md`)"
  (`155/build-1.md` item 1). The root CLAUDE.md says every number lives in `data.rs`; `architecture.md` says split a
  file past 800 when a piece touches it. 288 of its 1,024 lines are doc comments, and each of its rule tables stands
  alone. A `data/` folder with one file per system, re-exported as `data::*`, keeps "every number lives in data"
  and lets a session read the tables it needs. That is a move, digest-safe by construction.
- **`map.rs` (1,244) stayed untouched** but for 8 lines, as the M3 retro advised. M7 rebuilds the map, and is the
  piece to split it.
- **`engine/CLAUDE.md` is at 8,176 of its 8,192 bytes** (check.sh holds it there; at M3's end it was 20,050). M4
  added 22 modules to its layout table. The next module added to the map has 16 bytes to be named in, so M5's first
  piece rewrites the table whatever it builds.

### Per row of the engine table

| System | Target | On main | Held? |
|---|---|---|---|
| Actors | One `Actor` for every actor | `Actor` (people, home, store, territory, routes, market) on clans, civs and units. `spoil`, `eat_by_share`, `eat_by_diet`, `stride`, `walk_cost`, `divide`, `split`, `meal`. The guard now eats through `actors::meal` (`units/guard.rs:52`), which closes M3's third road | Yes |
| Labour across jobs | An actor splits its days between jobs, then each job over its tiles | `actors::split(people, labour, routes, shares, jobs)`; the clan's gathering and fields (`fields.rs:106`) and the guard (`claim.rs:131`) use it, and `divide` is `split` with one job. The camp and scout still spend down a `Ladder` (`units/forage.rs`) | Yes for clans and the guard; units on their own road |
| The day | One unit of labour, walking, carrying; the ring per civ | Every number in days (`DAY` = 4/3, a flat trail one day); walks per walker (`actors::carriers`); `line::ring` computed, drawn (`www/map/layers/ring.js`) and counted (`judge_ring.rs`). A turn is 3.5 × `DAY` = 4.67 days of a person's work | Yes; the clock call is open |
| Units | A state per kind, many per civ | `units/{mod,camp,scout,search,forage,line,ai,guard,fight,muster}.rs`; `mod.rs` 447 | Yes |
| Orders | One door | Holds; the planner gives through a `Give` sink that is `World::order` for the AI and `World::act` for `script::Auto` (`units/ai.rs:1-6`) | Yes |
| Land | A claim the guard divides | Holds; R's planner claims and widens round its clans' homes (`claim_plan`) | Yes |
| Fighting | A contest per tile | Holds; R's armies march, pillage and take capitals (`relay/specs/157/canvas.md`: 17 tiles taken over seven maps) | Yes |
| Goals | Standing; win and lose; an end | `over.rs`: `Phase::Over` with winner, `Why` and turn, by conquest, the led civ's fall and the limit | Yes |
| AI | A planner for Standing with the player's levers | `units/ai.rs`: gift, claim, guard, muster, march, raid, pillage, zone, cut, stand. No scout orders and no `Route` | Yes; 6.5 ms a turn (section 2) |
| Tiles shared | Many clans on a tile | `homes.rs` (derived, never saved); towns as festivals with `TOWN.homes` homes | Yes |
| Goods | A basket of goods with weight, spoiling and a use | `Basket = [f64; N]`, `GOODS = FOODS` (`data.rs:11`); one spoil rate for every good | No (M5) |
| Markets | Any participant's row | `market::clear(&[Basket], …)` takes rows; the civ adds its row through the levy and the gift | Half |
| Stage | A known map, placements, orders, reads | `Stage`, `StageSpec`, `map::parse`; `tests/common` built on it | Yes, but see section 3 |
| Save | Versioned | `SAVE_VERSION` 7 to 19: E 1, T 4, D 2, F 2, W 2, R 1, S and X none | Yes |
| Scale | Large under 10 ms past the pick; at most 0.45 of the turn before #156 | 0.52 (section 2) | No |

### The rules that hold

- **Digest-safe refactors.** Each split above was a move with the digests as proof (S's judge split, T's search, D's
  muster). X (#301) changed the scout's trail choice in `units/search.rs` with no bump. Its digests match R's on
  small 2, small 6, medium 8 and large 8, watched and led, for 250 turns, and on the check's save. The branch it adds
  fires on a led scout's route with a cheaper possible path, which none of those runs give. I didn't build a save that
  takes it, so whether an old save replays differently is unmeasured.
- **Shared behaviour on the actor.** Held: fields reuse `split`, `change` and `recentre`, the guard reuses `split` and
  `meal`. Carrying is the exception (section 4).
- **One `note` beside a number.** `PLAN`'s note runs to four long lines of canvas findings (`data.rs:733-737`), as
  `FIGHT`'s did in M3.

## 2. Scale

`sansfiat bench --size large 8 --after 50`, the three builds run in turn twice in one session
(`architecture/bench.txt`):

| Build | ms a turn (runs 1, 2) | Of the turn before #156 |
|---|---|---|
| F (`929b84a`), before #156 | 43.5, 44.0 | 1.00 |
| W (`06ee26b`), towns | 15.6, 15.8 | 0.36 |
| main (`7417e2d`) | 22.9, 22.4 | **0.52** |

This runner is faster than #156's (64 ms there before #156, 44 here), so the ratio is the figure, and 0.45 is broken
by R. Main's turn by step (`architecture/steps.txt`, the scratch profiler, mean of 50 turns):

| Step | W | main |
|---|---|---|
| go (festival search) | 6.3 | 6.3 |
| move (`clans::adjust`) | 6.0 | 6.1 |
| **ai** | **0.1** | **6.5** |
| units | 0.7 | 1.3 |
| eat, harvest | 2.1 | 2.1 |
| the rest | 0.6 | 0.7 |
| total | 15.8 | 22.9 |

Inside the planner (instrumented copy, three civs): `muster` 2.9 ms and `direct` 3.4 ms a turn; `path_between`, a
whole-map search at about 0.34 ms each, is called 16 times a turn for 5.6 ms of the 6.5. Two places do it:

- `muster` asks whether any rival's capital is within `PLAN.march` turns (`units/ai.rs:285`). It calls
  `path_between(capital, rival capital)` for every rival every turn, outside the `Paths` cache that `post` already
  keeps. Capitals don't move and `walkable` changes only when a trail is built, so the answer is the same nearly every
  turn.
- `raid` and `across` filter candidate tiles by `path_between(...).is_some()` (`units/ai.rs:550`, `568`): a
  reachability test answered with a full path. A per-turn connected-component map (one flood fill) or the turn's
  `Paths` answers it.

Caching both is a digest-safe change. If it removes what the counts say (most of 5.6 ms), main returns to about 17 ms,
0.39 of the old turn. That is an estimate from the counts, not a measurement. `move` doubled with towns (2.7 ms before
#156 in `relay/specs/156/scale.md`, 6.0 now) and is the next step to read, once the planner's searches are gone.

## 3. The stage and the pick

S built the stage so that a rule is proved on a known map, not on whatever a seed's pick happens to be. M4's tests and
scripts still lean on picks:

| Where | On a seeded or retro pick | Staged or built |
|---|---|---|
| `www/play/*.mjs` touch points | 16 of 19 (`army`, `art`, `bar`, `camp`, `civ`, `feed`, `fields`, `game`, `layout`, `pick`, `ring`, `save`, `scout`, `select`, `start`, `views`) | 3 (`buttons`, `end`, `towns`) |
| `core/tests/*.rs` files | 10 of 39 read `Stage::retro` (`cases`, `judge`, `ring`, `scout`, `scout_errand`, `scout_feed`, `scout_orders`, `stage`, `walk`, `wear`) | the rest build their worlds |

The ladder angle (`ladder.md` §7) counts what they cost: four of the five second sessions, and one owner stop in F
when `scout_errand` turned out to be a snapshot of one float path. A rule's bound on the retro maps is a fair test of a
piece (the CLAUDE.md "retro maps show the piece does"). A page script that checks a button draws something is not, and
it needs no pick. The sizing the ladder asked for:

- **The page scripts**: 16 scripts move from `?size=small&seed=N` to `Sim.stage` with a `StageSpec`, as `buttons.mjs`
  and `towns.mjs` already do. Each needs a spec with the unit or tile it touches (an army for `army.mjs`, fields for
  `fields.mjs`). `pick.mjs`, `start.mjs` and `game.mjs` test the seeded start itself and stay. So 13 scripts, about one
  Ford build with Sonnet: the pattern exists and the scripts are 50 to 200 lines each.
- **The core tests**: the `scout_*` files assert counts over the seven retro maps (`scout_errand.rs:173`: at most 4
  scouts lost, all on medium 9). Those are a piece's bounds and stay on the retro maps, but a build may not loosen them
  (the ladder's idea). `ring.rs` and `walk.rs` prove a rule and move to built maps.

## 4. What M5 needs first

M5 is herds and beer (`roadmap.md`): a store that walks and breeds, pastoral clans with wide thin land, pack animals
that cut a route's cost, horsemen, and beer, a good that isn't eaten. Read against the code:

**1. The clock call.** `labour.md` says a turn is a year of a fixed count of days; the count is `CLAN.work` = 3.5 ×
`DAY` = 4.67 days a person gives a year (`data.rs:524`). D labelled every number in days without rescaling, as the
owner chose (`m4.md`, answer 1). A pack animal is priced in loads a day and days a tile, so every herd and carrying
number is written against whichever clock holds. Choosing after M5's numbers exist means relabelling them twice. The
roadmap says the clock is decided before M5, and it is still open: it is the retro's first question.

**2. A good that is not a food.** Today:

- `GOODS = FOODS` (`data.rs:11-12`) and `Basket = [f64; N]` (`market.rs:5`), used in 11 core files (the tile's `cap`,
  `stock`, `unworn`, `wear` and `take` are baskets too);
- `Actor::spoil` keeps `1 − CLAN.spoilage` of every good alike (`actors.rs:49`), and units and the civ use it;
- `eat_by_share` and `eat_by_diet` eat from every good (`actors.rs:54-74`);
- `market::total`, "the amount of food in a basket", is called 34 times in core, and in each it means food;
- `market::clear` values a basket at Σ √amount over every good, so a beer row would be valued as food.

The seam is a `Good` table in data (weight, spoil, eaten or not, its use) beside `FOOD`, with a `foods` mask that
`eat_*`, `total` and the clearing's worth read, and `spoil` reading each good's rate. With beer's row at zero
everywhere it is digest-safe: a clay with the digests as its proof, before any herd exists. Growing `N` from 5 changes
the save's shape, so the clay bumps `SAVE_VERSION` even if no number moves.

**3. One carrying rule on the actor.** What a person carries is read in 15 files through three constants:
`CARRY.load` (a clan's festival walkers, `actors::carriers`), `UNIT.pack` × `UNIT.food` (an army's pack, `pack_cap`),
and `UNIT.carry` (a line's loads, `line.rs`, the ring). `walk_cost` is the one shared piece. A pack animal changes all
three, so it needs one `carry(actor)` on `Actor`, which the clans, the line and the pack read. That is a refactor,
digest-safe, and it is the "carrier on the shared actor" the roadmap names. With it, an animal is a change to one
function. Without it, it is three rules kept in step by hand.

**4. A herd is an actor whose store breeds.** The `Actor` has the store and the walking home a herd needs; breeding
exists only as a tile rule (`FOOD[GAME].breeds`, regrowth in `land::regrow`). A herd is a new state on the actor (the
rule that holds: "a new actor is a new state on it, not new machinery"): animals in the store that breed by the tile
rule's logistic and graze the territory. Pastoral clans' wide, thin land is `claim_land` and `divide` with a grazing
job, through `split`, which exists.

**5. `data.rs` into a folder**, before M5 adds a goods table, herd and beer rules and their notes to 1,024 lines.
**6. The planner's searches cached** (section 2), before the planner learns to herd, which adds more searches a civ a
turn. **7. `units/ai.rs` split** at its needs and its army orders, if an M5 piece adds to it.

Of these, 2, 3, 5 and 6 are digest-safe clay that one Ford or Easel piece can carry together. 1 is the owner's. 4 is
the first game piece on top.

## For other angles

- **ladder**: the four second sessions from pinned picks are 13 page scripts that can move to the stage (§3), about
  one Sonnet Ford build. The core bound tests stay on the retro maps.
- **relay**: whole steps have no brief, and R's grew `ai.rs` to 620 with nobody to name a split. `whole.md` could carry
  the file rules of `architecture.md` (a file over 600 names its split) so a whole session sees them.
- **game**: the planner gives no scout orders and no `Route`, so the AI's scouts run on their own errand only. The
  check's "no pillage order ever listed" is the view listing orders `check_order` accepts (`cli/src/view.rs:376-383`):
  pillage shows only when an army stands on a rival's tile.
- **retro and proposal**: M5's first piece is clay: goods with a use, one carrying rule, `data/` as a folder, the
  planner's paths cached, and `engine/CLAUDE.md`'s layout rewritten to fit. The clock call comes before it.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| Cache the planner's capital-to-capital paths per turn and answer reachability by a flood fill, back under 0.45 | §2: `ai` 6.5 ms of 22.9, 16 path searches a turn | high | small, digest-safe | take |
| A `Good` table: weight, spoil, eaten, use; `spoil`, `eat_*`, `total` and the clearing read it | §4.2 | high | medium, one bump | take, M5 clay |
| One `carry` on `Actor` for the festival walk, the pack and the line | §4.3: 15 files, three constants | high | medium, digest-safe | take, M5 clay |
| `data.rs` becomes `data/` with a file per system, re-exported as `data::*` | §1: 1,024 lines, F's brief kept it whole | medium | small, a move | take |
| Say in CLAUDE.md that "every number in `data`" means the folder, so the 800-line rule and it agree | §1 | medium | small | take |
| Move 13 page scripts from seeded picks to `Sim.stage` | §3; ladder §7 | high | one Ford build | take |
| Split `units/ai.rs` into needs and army orders when a piece adds to it | §1: 620 lines | medium | small | take, with the piece that grows it |
| Rewrite `engine/CLAUDE.md`'s layout to leave room (one line per folder, not per file) | §1: 8,176 of 8,192 bytes | medium | small | take |
| `whole.md` carries `architecture.md`'s file rules | §1: R's `ai.rs` | low | small | try |
| Read `clans::adjust` (6 ms, doubled with towns) once the planner is cheap | §2 | low | small | later |
| Split `map.rs` (1,244) | §1 | low | medium | later (M7, which rebuilds it) |
| The camp and scout spend through `split` instead of the `Ladder` | §1, Labour row | low | medium | drop until a unit takes a second job |
