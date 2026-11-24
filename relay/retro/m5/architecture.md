# M5 retro: architecture

The engine on `main` at `4da574f` (#504, the last art piece) against `docs/architecture.md` and the M4 retro's angle
(`relay/retro/m4/architecture.md`). What I measured:

- line counts of every engine file at M4's end (`7417e2d`) and at each M5 merge: L `b0aa706`, J `d66cf37`, S `ee70756`,
  R `4fea294`, C `da69498`, G `148528b`, P `63b941e`, V `4ade4df`, Y `8d1b4ed`, Q `0fd7835`, K `d28d6e5`, X `5da90d1`,
  the fixes `1c3f69b` and main. The script is `architecture/lines.sh` and the table `architecture/lines.txt`;
- `SAVE_VERSION`, core lines and `#[test]` counts at each merge, and `engine/CLAUDE.md`'s bytes at each merge;
- `sansfiat bench --size large 8 --after 50` built from F (`929b84a`, before #156), M4's end, Y and main, run in turn
  twice in one session on the Relay runner (`architecture/bench.txt`);
- a scratch profiler that times each `TURN` step past the pick (`architecture/prof.rs`), built against M4's end, L,
  S, C, G, Y and main (`architecture/steps.txt`);
- a one-line guard on main (`architecture/guard.diff`), benched, and its digests against main's on four retro maps,
  watched and led, 300 turns (`architecture/guard.txt`, `architecture/digests.sh`);
- a trial merge of H's open branch (`relay/342`, PR #418) into main with `git merge-tree`.

Everything else is read from the code, and says so where it is inferred.

**Verdict.** M5 grew core by 3,628 lines (13,780 to 17,408) and the tests by 78 (431 to 509), and the shape held
where the M4 retro asked for clay. Y (#395) did all four of its digest-safe items in one piece: `data/` as a folder of
eight files (none over 273 lines), `Actor::carry(Carrier)` for the walk, the line and the pack, the `GOOD` table that
every reader reads, and the planner's paths cached. V moved all but six of the 33 page scripts onto the stage, and the ladder
finds no second session from a stale pin (M4: four). Every new system got its own module: `settle.rs`, `settling.rs`,
`sources.rs`, `exchange.rs`, `lots.rs`, `outlook.rs`, `units/paths.rs`, each with a judge file and a sweep file beside it.
Four things slipped. **The scale bound is still broken**: main is 0.53 of the turn before #156, as at M4's end. The
paths cache took the planner from 8.4 to 3.0 ms, but G had already put 2.7 ms on `move` with a second reap in every
territory's valuation; a one-line guard takes main to 0.47 with the digests unchanged. **A settlement's store is not
an `Actor`**: capital and settlement are two branches in 29 calls across 9 files, and the clearing posts only the
capital's. **Stage tests as data stopped at V's four** (all scouts): K, Q and X proved their rules in Rust tests the
Tests screen can't show. **`engine/CLAUDE.md` is full again** (8,162 of 8,192 bytes): Y cut it to 5,418, and the art
pieces added about 2,300 bytes, most of it atlas cell sizes. M6 needs four things first: jobs as a table of named jobs, which H's
stale clay started (`jobs.rs`) and which herds, beer and trade each become a row of; one store, so a herd, a
settlement and a capital are one kind of thing a line, a levy and a clearing read; the clearing's rows built one way,
so a trader can sit in two markets; and the `move` guard before a herd adds a fourth job to every trial reap.

## 1. What grew where

### The modules

Engine files at or past 450 lines on main (`architecture/lines.txt` has every merge):

| File | M4 end | L | S | R | C | G | Y | Q | K | X | main |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `core/src/map.rs` | 1,244 | | | | | 1,248 | | | | | 1,248 |
| `core/src/clans.rs` | 615 | 615 | 632 | | | 653 | | 661 | **714** | **735** | 735 |
| `core/src/civs.rs` | 620 | 600 | 628 | | | 632 | 639 | 643 | 644 | 662 | 662 |
| `core/src/markets.rs` | 495 | | 500 | | | 531 | | | 587 | **635** | 635 |
| `core/src/units/ai.rs` | 620 | 547 | 598 | **649** | | 634 | 631 | | | | 631 |
| `core/src/stage_test.rs` | | | | | | | 584 (V) | | | | 584 |
| `core/src/actors.rs` | 408 | | | | | 486 | 503 | | | 553 | 553 |
| `core/src/judge.rs` | 479 | 464 | 491 | | | 496 | | 501 | | 506 | 506 |
| `core/src/units/camp.rs` | 413 | | | 411 | | 414 | | | | 433 | 479 |
| `core/src/units/mod.rs` | 447 | 426 | | 430 | | 434 | 435 | | | 455 | 457 |
| `core/src/units/scout.rs` | 538 | | | 539 | **451** | | | | | | 451 |
| `cli/src/view.rs` | 416 | 398 | 470 | 498 | 506 | 546 | 565 | 575 | | 580 | 573 |
| `web/src/lib.rs` | 499 | 493 | 510 | 517 | | 523 | 543 | 544 | | 551 | 551 |
| `www/screens/play.js` | 755 | 752 | 753 | | 756 | 757 | 759 | | | 779 | 779 |
| `www/map/view.js` | 607 | 603 | | | | 605 | | | 535 (art) | 543 | 583 |

New modules, by the piece that made them: L deleted `units/guard.rs` (core −413 net); S `settle.rs`, `judge_settle.rs`,
`cli/sweep_settle.rs`; G `sources.rs`, `judge_goods.rs`, `cli/sweep_goods.rs`; P `outlook.rs`; V `stage_spec.rs`,
`stage_test.rs`, `core/stage/`, `web/src/stage_test.rs`, the Tests screen; Y `data/` (eight files), `units/paths.rs`;
Q `settling.rs`, `judge_settling.rs`, `cli/sweep_settling.rs`; X `exchange.rs`, `lots.rs`, `judge_trade.rs`,
`cli/sweep_trade.rs`, `cli/view_goods.rs`, `www/panels-goods.js`; the fixes `cli/view_land.rs`. The page grew from
8,705 lines of JS to 15,619, most of it the art pieces' (`map/` pieces, `assets/cut.mjs`, `shots*.mjs`); no page file passes 800.

- **The M4 retro's clay landed, in one piece.** Y's brief named each split (`relay/specs/340/spec.md`): `data.rs`
  (1,137 lines by then) became `data/` with no file over 400, `Paths` went to `units/paths.rs` because `ai.rs` was 634,
  and the 21 digests stayed identical (`340/build-2-evidence.md`, `build-3-evidence.md`).
- **The whole sessions kept the file rule.** `whole.md` now carries it (line 41: split a file taken past about 800).
  K (whole, Opus) took `clans.rs` from 661 to 714 and `markets.rs` from 531 to 587, under 800.
- **X's brief was not kept.** It says "`markets.rs` must not pass 600 lines" (`356/build-1.md` line 68), and X left it
  at 635, merged. The planned `units/trade.rs` went with the rework (`356/rework.md`), so `ai.rs` stayed 631 and
  unsplit; nothing in M5 added to it after R.
- **Four core files sit between 600 and 800 that M6 will touch**: `clans.rs` 735, `civs.rs` 662, `markets.rs` 635,
  `ai.rs` 631. H's clay added 61 net lines to `clans.rs` (PR #418), which would take it past 800: the herds piece
  splits it.
- **`engine/CLAUDE.md` is at 8,162 of 8,192 bytes.** Y rewrote it to 5,418. Q took it to 5,469; the five art pieces
  #448 to #474 took it to 7,760, and the rest to 8,162. The `www/` row is 1,206 bytes, most of it each atlas cell's
  size and anchor (`tree.big.<v>` 80×100, anchor 40,95 …). That belongs beside the atlas (`www/assets/`), not in the
  map every session reads. The data folder's row lists six files of eight (`land.rs` and `mod.rs` aside).

### Per row of the engine table

| System | Target | On main | Held? |
|---|---|---|---|
| Actors | One `Actor` for every actor | Clans, civs and units each hold an `Actor`; `carry(Carrier)` reads `CARRY` per kind (Y). A settlement is not one: `Settlement` has its own `store`, `lots` and `market` (`settle.rs:31`), and `settle::store`, `store_mut`, `store_market`, `store_name` and `exchange::store_lots` branch capital-or-settlement, 29 calls in 9 files; `civs::levy` branches on it twice (`civs.rs:505-513`) | Half: section 4.2 |
| Labour across jobs | An actor splits its days between jobs | `fields::jobs` splits three jobs (gather, fields, dig) through `actors::split`, as a fixed tuple of three; the source rate is a branch on `SALT` in code (`fields.rs:128`). The camp and scout still spend a `Ladder` | Half: section 4.1 |
| The day | One unit of labour, walking, carrying | The clock closed as built (m5.md, answer 5); K made a clan's carrying its days' (#429) | Yes |
| Units | A state per kind | `UnitKind` is `Camp` and `Scout`; the guard is gone (L). A unit saves as one flat object every kind writes (`units/mod.rs:137`), so a new kind extends it | Yes |
| Orders | One door | Holds; `Signpost` and `Unpost` added, and the planner gives them | Yes |
| Land | A claim held unguarded, lost to an army | L's claim; a settlement roots it (`settle::roots`) | Yes |
| Fighting | A contest per tile | Capitals fall (the check: Blue to Gold at 297) | Yes |
| AI | A planner for Standing | Gift, land, signposts, muster, march, raid, pillage, cut, stand; 2.4 ms a turn. Pay and stalls are rules, not orders (`exchange.rs`) | Yes |
| Goods | A basket with weight, spoil and use | `GOOD` (7 goods: five foods, salt, obsidian) with `eaten`, weight and spoil; sources; `Lots` say where a ware came from. No crafts | Yes |
| Markets | A clearing of any participant's row | One clearing per market (`markets::clear_at`) of clans, units at a stall and the capital's civ, each row built in its own branch and tagged by `exchange::Owner`. A settlement's store posts nothing; a clan is in one market (`Actor.market`) | Mostly: section 4.3 |
| Knowledge | Knacks per civ | None | M6 |
| Routes | A route through rests | A line ends at the nearest store (R) or at a stall in a friendly market (X) | M7 |
| Stage | A known map, placements, orders, reads | `Stage`, `StageSpec`, tests as JSON in `core/stage/`, watched on the Tests screen (V). Four JSON tests, all scouts | Half: section 3 |
| Save | Versioned | `SAVE_VERSION` 19 to 32: L, J, S, R, G, K, Q 1 each, C 2, X 4; P, V, Y and the fixes none | Yes |
| Scale | At most 0.45 of the turn before #156 | 0.53 | No: section 2 |

### The rules that hold

- **Digest-safe refactors.** Y's three builds each left the 21 digests of `relay/specs/340/digests.sh` identical. X's
  exchange off equals main's at `0fd7835` on all 21 (`356/rework.md`).
- **A new actor is a new state on the actor.** Broken once: the settlement (above). H's clay keeps it for the herd,
  which is a good slot in the clan's store (`ANIMALS`), not new machinery.
- **Every number in `data/`.** Held but for `SALT`'s rate chosen in `fields::jobs` by a branch, which a `GOOD` or
  `SOURCE` column would carry.

## 2. Scale

`sansfiat bench --size large 8 --after 50`, the four builds in turn, twice (`architecture/bench.txt`):

| Build | ms a turn (runs 1, 2) | Of the turn before #156 |
|---|---|---|
| F `929b84a`, before #156 | 55.96, 56.05 | 1.00 |
| M4 end `7417e2d` | 29.57, 29.54 | 0.53 |
| Y `8d1b4ed`, paths cached | 27.47, 27.30 | 0.49 |
| main `4da574f` | 29.81, 29.37 | **0.53** |
| main with the guard | 26.41, 26.04 | 0.47 |

This runner is slower than M4's (F 56 ms against 44), so the ratio is the figure; M4's end reads 0.53 here and 0.52
there. By step (`architecture/steps.txt`, ms a turn, mean of 50 turns):

| Step | M4 end | L | S | C | G | Y | main |
|---|---|---|---|---|---|---|---|
| move (`clans::adjust`) | 8.17 | 8.00 | 8.53 | 8.64 | **11.28** | 11.52 | 11.92 |
| go (festival search) | 7.84 | 7.74 | 7.91 | | 8.01 | 7.92 | 8.19 |
| ai | **8.38** | 7.99 | 8.14 | | 7.63 | **2.97** | 2.40 |
| eat | 1.48 | | | | | 1.76 | 2.77 |
| total | 29.90 | 28.86 | 30.40 | 29.94 | 32.23 | 27.66 | 29.80 |

- **The cache did what the M4 retro said.** The planner fell from 8.4 to 3.0 ms (M4 estimated most of 5.6 ms of
  path searches would go). It didn't bring the bound back because `move` had grown 2.7 ms first.
- **G's growth is code, not a bigger world.** At +50, C's world has 741 clans holding 1,849 tiles and G's 722 holding
  1,847, and `move` went from 8.64 to 11.28. G made `clans::worth`, which values every candidate territory in
  `best_add` and `best_drop`, reap twice: once for food and once more (`reap_within` at budget 0) for what its sources
  would give (`clans.rs:342-346`), whether or not the territory holds a source.
- **The guard.** Skipping the second reap when no tile of the territory has a source (`architecture/guard.diff`, one
  line) gives the same answer: `dug` is 0 there. Its digests equal main's on small 2, small 6, medium 8 and large 8,
  watched and led, at 300 turns (`architecture/guard.txt`). `move` falls to 8.84 ms and the turn to 26.2 ms, 0.47.
  I didn't commit it: a retro step doesn't change the engine.
- **What's left.** `move` and `go` are 20 of 26 ms with the guard. Every job added to `fields::jobs` costs every trial
  reap in `move`, and a herd is one (H's clay adds `herd` to `jobs`). Without a cheaper valuation, herds will push
  `move` back up; that's inferred from G's cost, not measured on H's branch. `eat` grew 1.0 ms between Y and main
  (K, Q, X), unread.

## 3. The stage and the Tests screen

V built tests as data: a `StageTest` in `core/stage/*.json`, listed in `stage_test.rs`'s `LIST`, run by core and
watched on the page's Tests screen (`page.md`: "each stage test is listed by name and claim … so a rule's proof can be
seen"). `engine/CLAUDE.md`'s invariants say "A rule's proof is a test in `core/stage/`".

| Where | On main |
|---|---|
| `core/stage/*.json` | 4: `scout-errand`, `scout-killed`, `scout-trapped`, `scout-walks-round`, all from V's move of C's tests. None added since |
| `core/tests/*.rs` using `Stage` | 21 of 48 files; Q's `settling.rs`, K's carry tests in `markets.rs` and `towns.rs`, X's `exchange.rs` and `lots.rs` prove their rules here |
| `www/play/*.mjs` on a seeded pick | 6 of 33: `layout`, `save`, `live`, `pick`, `start`, `bar` (the first five test the start itself or hold a page) |

So the invariant reads as a rule and isn't kept: three pieces after V proved rules in Rust that no one can watch. The
roadmap puts "every staged test listed and watched on the page" in M6. Whether M6 moves the M5 rules' proofs (claim,
settlement, store, line to the nearest store, exchange, carry) into `core/stage/` or the invariant narrows to "a rule a
player sees" is the owner's question: it's a cost per piece, not a refactor. A `StageTest` reads orders and holds;
whether its `holds` can say what `exchange.rs`'s tests assert (a stall's food, a pay's wares) is unread.

## 4. What M6 needs first

M6 (`roadmap.md`): the page's worst faults; every staged test watched; herds as a store that walks and breeds,
pastoral clans with wide thin land, pack animals, horsemen; beer, a good that isn't eaten; jobs and trade, a clan's
days in named jobs with trading as a route between two markets; and knacks. Read against the code:

**1. Jobs as a table, and H's clay.** H (#342) stopped after its clay; PR #418 is open on a base of Y (`8d1b4ed`).
It moved `fields::jobs` into `jobs.rs` returning a `Jobs` struct and added the herd as a fourth job and an `ANIMALS`
slot in the basket, with herds off equal to main in all 19,953 judge numbers (`relay/specs/342/clay.md`). A trial
merge into main conflicts in 20 files, nine hunks in `clans.rs` alone, because K, Q and X rewrote the same code since.
Its probe and spec are the design work and still read; its code is cheaper rebuilt than rebased. The seam M6 needs is
the one under it: `jobs` as a table of named jobs (gather, fields, dig, herd; then brew and trade), each a share, its
tiles and its product, that `actors::split` divides, instead of a tuple each new job widens by hand. With the herd
row at share 0 it is digest-safe, and it carries the `SALT` branch into data.

**2. One store.** A herd is "a store that walks"; a pack animal carries a line from a store; a cache in M7 is a store
left behind. Today a store is either `Civ.actor.store` or `Settlement.store`, chosen by `capital == tile` in each
reader (§1). A settlement holding an `Actor` (people 0, as the civ's does) and one lookup from a tile to its store's
`Actor` makes the levy, the line, the pay and the clearing read one thing. Digest-safe, but `Settlement`'s save shape
changes, so one bump.

**3. The clearing's rows built one way.** `clear_at` builds clans' rows, units' rows and the capital's row in three
branches and tags them with `exchange::Owner` (`markets.rs:322`). A trader is in two markets, and a settlement's
store doesn't post at all. A `Row { owner, brought, kept, worth }` that each kind makes, and `by_market` listing a clan
in every market its routes reach, makes trade a fourth kind of row rather than a fourth branch. `markets.rs` is 635:
the clearing goes to its own file when this lands.

**4. The `move` guard, then a cheaper valuation.** The guard (§2) first, digest-safe. Then before the herd job lands,
read whether `worth` can value a territory without a full reap per candidate: herds make it a fifth evaluation of
`split` per trial.

**5. Carry by the actor.** `Actor::carry(Carrier)` is per kind. A pack animal changes what one actor carries by the
animals it holds, so `carry` reads the actor. Small, digest-safe, with the pack-animal piece.

**6. Knacks** are a field on `Civ` and their own `knacks.rs` and `data/knacks.rs` (`civs.rs` is 662). Contact spreads
where people trade: `judge_trade.rs` already reads who trades with whom from the clearing, so the clearing's rows (3)
are where contact is counted.

**7. `engine/CLAUDE.md`'s atlas detail moved** beside the atlas, so M6's first new module has room.

Of these, 1 to 5 and 7 are clay one piece can carry before herds, as Y did for M5; 1 and 2 bump the save once
between them. 3 is the trade piece's first build if the proposal keeps trade in M6.

## For other angles

- **ladder**: Y was four refactors in one stepped piece and held its estimates' shape; M6's clay (§4) is the same
  kind. X's brief said `markets.rs` stays under 600 and the build passed it unchecked: a review line, not a ladder one.
- **relay**: H's open PR #418 is 20 conflicting files behind main. Whether a stopped piece's clay merges, closes or
  is rebuilt is a crew rule the M5 runs didn't have.
- **game**: the check found gifts reach no clan at the capital's market. Settlement markets post no civ row
  (`clear_at`), so a civ can give only where its capital is (inferred from the code; the check played one civ).
- **retro and proposal**: M6's first piece is clay: named jobs, one store, rows built one way, the `move` guard, carry
  by actor, the CLAUDE.md map. Then herds rebuilt on it from H's spec. The owner's question: does every rule's proof go
  into `core/stage/` to be watched?

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| Skip `worth`'s source reap when the territory has no source | §2: `move` +2.7 ms from G; guard to 0.47, digests equal | high | one line, digest-safe | take |
| Jobs as a table of named jobs that `split` divides; the herd row at 0 | §4.1; H's `jobs.rs` | high | medium, digest-safe | take, M6 clay |
| A settlement holds an `Actor`; one tile-to-store lookup | §4.2: 29 calls, 9 files | high | medium, one bump | take, M6 clay |
| One row builder for the clearing; a clan listed in every market it reaches | §4.3 | high | medium | take, with trade |
| Rebuild herds on main from H's spec, and close PR #418 | §4.1: 20 conflicting files | high | a piece | take |
| `Actor::carry` reads the actor, for pack animals | §4.5 | medium | small | take, with pack animals |
| Move the atlas cell sizes out of `engine/CLAUDE.md` | §1: 8,162 of 8,192 bytes | medium | small | take |
| M5's rule proofs into `core/stage/`, or narrow the invariant | §3: 4 JSON tests | medium | a build, or a line | ask the owner |
| Split `clans.rs` (735) when herds touch it | §1 | medium | small | take, with herds |
| Read `eat` (+1.0 ms since Y) and `worth`'s full reap per candidate | §2 | low | small | later |
| Split `map.rs` (1,248) | §1 | low | medium | later (M7) |
| Split `ai.rs` (631) | §1: nothing added since R | low | small | drop until a piece grows it |
