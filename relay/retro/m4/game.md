# M4 retro: the game

Verdict: every M4 piece is in and the rival plays, and the game is still decided at the pick.
With every civ on the AI no game of seven ends before its limit and no capital falls. A civ the
planner leads beats its idle self in 21 of 21 games, by 4.9 points of Standing, so orders now
pay. What a player meets on a small map is still an end-turn button: on the owner's map (small
61288) an idle civ is second from the pick to the limit and nothing reaches it.

Sources: `game/sweep.txt` (`sansfiat sweep --judge` on main's engine, 12 Nov, 63 s on a desktop);
`game/idle-61288.txt` and `game/scout-61288.txt` (the owner's map in the text view); `owner.md`;
`play.md`; the ledgers in `docs/design/`.

## 1. Each piece's question, on main

| Piece | Question | On main |
|---|---|---|
| S, the game proves itself | Health and decisions: a known map, placed units, checked | The stage is in core and 13 page scripts still read a seeded pick (`architecture.md` §3). The stage's tests run in `cargo test` only: none can be watched |
| E, a game that ends | Doubt: games end | 7 of 7 end at the turn limit, 0 before it, 0 capitals fallen. Led by the planner: 0 of 21 before the limit. Led and idle: 1 of 21, by the led civ's fall |
| T, scouts that work | Decisions: an errand finishes | AI scouts are fed: 3 hungry scout-turns of 2,864 inside the ring, 24 of 1,278 outside. A led scout sent 18 steps on small 61288 stands on one tile from +10 to +200 with 5 tiles left to search, never hungry (`game/scout-61288.txt`) |
| D, the day | Decisions: the ring says where an army eats | Army-turns inside the ring 3,476 (266 hungry), outside 1,243 (363 hungry): 8% hungry inside, 29% outside. The ring reads true |
| F, fields | Standing: farming civs gain | 14 of 21 civs farm; they gain 5.2 points of Standing against the no-fields control and the others lose 0.8. The top farmer's store is at a pack on half its turns on 4 of 7 maps |
| W, towns | Motion: clans gather and trade | 13 towns at +200 on 4 of 7 maps: 9 on large 8, 2 on medium 9, 1 each on small 2 and small 6, none on small 3, small 9 or medium 8. Town clans trade 1.2 times what farmers outside do; trade over all clans is unchanged (0.0603 against 0.0614) |
| R, a rival that plays | Doubt: a led game is in doubt | The led civ's place changes in 16 of 21 games (idle 15). It ends ahead of its idle self in 21 of 21, by 4.9 points, and wins 6 (idle 2). People fallen 34 over seven maps, tiles taken 14, on two maps only (small 2 and small 3); pillage is 0.07 orders per 100 turns |

Standing's top share at +200 is 16.3% and aligned 35.3%. Lead changes: 8 on 5 maps.

What this says: F and R answered their questions. E's end rules work and nothing triggers them:
a capital needs a guard, an army or a person of militia gone to fall, and the AI's guard holds
35.19 of its 35.24 claimed tiles with 8.3 people. W's towns form where clans are many, which
small maps are not.

## 2. The owner's notes, traced

| Note | What main does | Goes to |
|---|---|---|
| Remove the militia and the guard | The guard holds 99.9% of every AI claim with 7.6 people a civ. Disbanding it turns 39 held tiles to 2 with no word why (`play.md`). No capital falls in the sweep | Piece L |
| Scouts go back to the capital for food; foraging doesn't work | A scout out of its line's reach leaves its errand for rich ground and waits there (#218, #223: under a quarter of a fed scout's work). On small 61288 it waits 190 turns | Piece C |
| Warriors kill scouts; a warrior's painted tiles trap them | A scout neither fights nor is fought, and a zone doesn't stop it (`war.md`, Decided) | Piece C |
| The scout's test viewable and runnable on the page, and all such | The stage's tests are Rust tests. `Sim.stage` exists in the wasm page for scripts, with no screen | Piece V |
| A click centres the camera on the tile | The owner's note from the page; this session read no page code for it | Piece P |
| Play the page, name the worst, propose fixes | `players.md` | Piece P |
| Towns: one town on small 61288 | One town forms, at (-12, 2) on turn 258, in Gold's country; Red and Blue get none. Small maps form 0 or 1 | Piece N |
| The scout issued its own order at the pick | Not seen in the text view: a new game's scout reads "no route". The planner gives scouts no orders (`architecture.md`). Not reproduced | Nothing, unless seen again |
| Starving tests | §4 | The ledger, and piece Y |

## 3. The ledger

M4 ended with 149 `## Decided` lines over eleven design files. The proposal PR strikes 104 and
keeps 45. Kept: what shapes the game (what Standing counts, how a game ends, what land is, who
fights, what a clan is, what a farmer and a town are, who pays a walk). Struck, by kind:

| Kind | Example |
|---|---|
| How the page looks and words | "The feed starts open only on windows 1400 px wide or more" |
| The AI's thresholds | "It leaves its own land only with four fifths of its pack" |
| A number or a tuning | "A load of 2 food is about nine days' eating" |
| How a rule is built | "Every order to a unit names its civ and its unit, and goes through one door" |
| Ended or replaced by a later line | "Until the AI learns to claim, it claims every land tile within a step" |

`docs/design/README.md` now says what the ledger is. One line is added, to `vision.md`: how many
starve is a consequence of growth, not a score.

## 4. Numbers nobody asked for

The owner removed the starving bands in #156. The same habit is still in the sweep: its fields
and towns groups print 22 verdicts, `pass` or `MISS`, against bounds a designer wrote
("13 towns at +200 over the seven (≥ 10), on 4 of 7 maps (≥ 5) MISS"). Six read MISS on main and
the pieces merged anyway, rightly, so the bounds decide nothing and still read as verdicts. Two
builds loosened `scout_errand` to pass (0 lost to 5, then 4). Ten of 39 core test files and 16
of 19 page scripts read a seeded retro pick (`architecture.md`), and four of the five second sessions in M4 were
those moving under a float.

What helps: a piece's question names one number and its control, the sweep prints numbers with
no verdict, and a rule's test is on a built map on the stage. Piece Y takes the verdict words
out of the sweep; piece V moves the page scripts to the stage.

## For other angles

- **players**: the scout's wait, the town count and the idle game above are a player's whole
  small-map game.
- **retro and proposal**: nothing ends a game because nothing can lose land. The guard is the
  reason, and it is the owner's first game note.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| A claim is held without a guard; the guard and militia go | §1 E and R, §2 | high: land can be lost, capitals can fall | a piece: `claim.rs`, the planner, the page | take, piece L |
| A scout eats at its capital and walks back out | §2; #215, #217, #218, #223 | high: an errand finishes | small | take, piece C |
| An army kills a scout on its tile, and a zone stops one | §2 | medium | small | take, piece C |
| The stage's tests listed and played on the page | §2 | high: the owner sees a rule work | a piece | take, piece V |
| The sweep prints numbers, no pass or MISS | §4 | medium | small | take, piece Y |
| Towns on small maps | §1 W | medium | the owner's call on what a town needs | take, piece N |
