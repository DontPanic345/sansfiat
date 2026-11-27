# M5 retro: the game

Verdict: the loop closes in the AI's games and is wide open in a player's. With every civ on
the AI, capitals fall (6 on 5 maps; M4: 0), a game ends by conquest (1 of 7) and the led civ
beats its idle self by 8.0 points of Standing (M4: 4.9). But a player who does two things the
view tells them, keep four warriors home and send the claim the Reach line names each turn,
ends at 89.5% of the people on small 2 against 33.8% for the same civ on the AI, 51.7% against
26.7% on small 6 and 51.9% against 16.3% on medium 8, and no rival takes a tile of it on medium
8. The claim has no limit for a player and a planner's limit for the AI, and a wide claim
costs nothing until an army walks onto it. Of M5's pieces, L, C, G, P and K answered their
questions; S's answer turned after R gave its store a use; R and X didn't move theirs; J was
never played.

Sources: `game/sweep.txt` (`sansfiat sweep --judge` on main at 4da574f, 27 Nov, 235 s on the
runner); `game/play/` (scripted games in the text view, each with its save, lines and
`judge`; the script is `game/reach.py`); the M5 PRs (#349 to #504) for each piece's control;
`relay/retro/m4/game.md` for M4's main; `play.md`; the ledgers in `docs/design/`.

## 1. Each piece's question, on main

M4's main is the control unless the row says otherwise; "at merge" is the piece's own PR.

| Piece | Question | At merge | On main now |
|---|---|---|---|
| L #336, land without a guard | Doubt: do games end early, do capitals fall? (M4: 0 of 7, 0 capitals, 14 tiles taken) | 0 of 7, 2 capitals, 36 tiles (#349) | **1 of 7** (small 6, conquest at +49), **6 capitals** on 5 maps, 89 claimed tiles taken. Led games: 6 of 21 end by the led civ's fall; idle 7 of 21, all by +6 |
| J #352, joining made plain | Motion: clans that change civ, and turns from a home on the land to joining (about 11) | 10 turns to 0; clans changing 29.6 to 23.9 per 100 turns (#358) | 263 of 314 joins on a civ's land after **0.0 turns**; 415 clans changed. Built by hand: no play step, no review, the page's colours checked as data |
| S #353, signposts and settlements | Standing: civs with a settlement against the same civ with no signposts | 15.7% against 18.2%: worse (#360) | **19.2% against 16.5%**, all civs 13.5% against 13.0%. 20 posted, 12 founded, 8 taken, 10 standing at +200; 7 of 21 civs on 6 of 7 maps. the sign turned after R gave the store a use (not isolated) |
| R #354, supply from any store | Doubt: hungry army-turns outside the capital's ring (M4 363 of 1,243, 29%) | 522 of 1,531, 34%: not shown (#365) | **545 of 1,703, 32%**. Inside the rings 341 of 2,069 (16%). Not answered: settlements are too few and far from fronts |
| C #337, scouts that finish | Decisions: does a led scout's errand finish on small 61288? (M4: never in 200 turns) | built at +52, fight off (#369) | **Built by +21** with fight on (`play/scout-61288.*`: Red to Gold's capital, one trip home for food). Scout-turns outside the ring 0; 9 hungry of 3,559. The check's one-tile route still reads "0 tiles to search" for 110 turns (`play.md`) |
| G #355, goods | Motion: share of festivals where salt or obsidian trades (control 0) | 25.1% (#375) | **40.7%**; salt found 3.8 steps from a source (15 at most), obsidian 3.3 (11), both off their source on 7 of 7 maps. But civ stores holding either: 2 each, armies armed with obsidian: 7 over seven maps |
| X #356, exchange | Decisions and motion: army-turns fed at a market not the civ's (control: exchange off) | 0 on the retro maps; merged by the owner over `merge: false` (#409) | **21** army-turns over seven maps and 1,400 turns (control 0), 12.8 of 82.3 food needed. Not answered on the maps; proved on a built map (`core/tests/exchange.rs`) |
| P #338, the page played | Decisions: orders per 100 turns, turns with none (control 11 in 103, 95 with none) | 101 per 100, 58 of 102 with none (#380) | The check, in the text view: 18 orders in 110 turns, 99 with none (`play.md`). The script that follows Reach: 52 to 102 per 100, and it wins by 2 to 3 times (§2). The line works; the check didn't use it |
| V #339, tests you can watch | Health: page scripts and core tests on a retro pick (16 of 19, 10 of 39) | 6 of 23, 8 of 45 (#387) | Core test files calling `Stage::retro`: **10 of 48** (two came back). Stage tests as data: still V's 4 scout tests; K, Q and X proved their rules in Rust (`architecture.md`) |
| Y #340, data and paths | Health: digests main's, large 8 under main's turn | 21 of 21 equal, 14% faster (#395) | Digests: no mismatches. Large 8 at 0.53 of the turn before #156 (bound 0.45): G's reaping ate the gain (`architecture.md`) |
| N #341, a town where you look | Motion: towns at +200 (13 on 4 maps) | Closed into Q, `proof/341` kept | Towns **16 on 6 of 7 maps** (small 3 none). Capital towns **3 of 15** |
| K #426, carrying | Motion: do farmers come to markets on their own? Towns, capital towns, trade per person, births | Towns 10 to 19, capital towns 5 to 3, the four scores worse; merged by the owner (#429) | Towns 16, capital towns 3. A town clan trades 0.230 a person a turn, outside a town 0.173, a farmer outside 0.085. Born a person a turn: settled 0.0100, gathering 0.0079 (civ clans). Crowding killed 4,582 people (2,482 on large 8) |
| Q #427, settling read | Motion: settled clans and people at +100 and +200 | 181 of 629 civ clans at +200 (#432) | Civ clans: 148 of 511 settled at +200, 31% of their people; unaligned 188 of 1,324 (20%). 0.106 changes per 100 clan-turns |
| Fixes #350 | Each fix's repro a test | 8 findings, 6 fixed, 2 already met (#489) | View text; no score moves |
| Art #439 to #446 | Frame time and legibility on the page | measured in #504 | Not a game score; not re-measured here |

The four scores beside M4's main:

| Score | M4 main | M5 main |
|---|---|---|
| Standing: top civ at +200 / aligned | 0.163 / 0.353 | 0.277 / 0.406 |
| Doubt: lead changes / maps / spread at +200 | 8 / 5 / 0.098 | 14 / 5 / 0.261 |
| Doubt: games ended early / capitals fallen | 0 of 7 / 0 | 1 of 7 / 6 |
| Decisions: AI orders per 100 turns | 60.4 | 56.7 (claim 10.6, move 30.8, raise 6.5, signpost 0.4) |
| Motion: people changed / claimed tiles taken | 1,440 / 14 | 2,382 / 89 |
| Led over idle / ahead in | +4.9 / 21 of 21 | +8.0 / 15 of 21; won 7, idle 1 |
| Health: pre-games by centres / panics / mismatches | 7 / 0 / 0 | 7 / 0 / 0 |

Spread at +200 rose from 0.098 to 0.261: the leader leads by more. That is less doubt, not
more, though lead changes rose too.

## 2. What the scripted player shows

`game/reach.py` plays a civ in the text view by two rules: raise a warrior at the capital while
fewer than four hold it, and send the claim the view lists as "the claim in reach" every turn.
Nothing else: no gift, no scout, no army abroad.

| Map, civ | Script | AI control | Idle | Orders per 100 | Land | Rival took |
|---|---|---|---|---|---|---|
| small 2, Red | **89.5%** #1 | 33.8% #1 | 28.2% #2 | 73.5 | 91 tiles (AI 53) | 2 tiles |
| small 6, Red | **51.7%** #1 | 26.7% #1 | 0.0%, fell +4 | 52.5 | 125 (60) | 0 |
| medium 8, Red | **51.9%** #1 | 16.3% #1 | 10.7% #1 | 102 | 219 (61) | 0 |
| small 3, Gold | fell +13 | fell +2 | fell +3 | 114 | 0 | — |

Files: `game/play/p-*.json`, `.lines.jsonl` and `.judge.txt`. Lead changes in the scripted games:
0, 0, 0.

Why: the planner claims no more than `PLAN.hold` tiles a person of its armies and widens
`PLAN.widen` tiles a turn (`data/units.rs`); a player's claim has no limit, and the Reach line
names the next claim that brings a clan, every turn. A claimed tile costs nothing to hold until
a rival army ends a turn on it, and in 250 turns on medium 8 no rival army did. Four warriors
at home hold the capital. The owner was asked this in M5 (#351: "A claim with no size
limit wins a dozen clans on the first turn", from #338's build); it is still open.

The opening is the other half. A civ starts with no army (`founding.md`, #152), and the AI's
first army reaches a near capital in two to four turns: on small 3 Gold's capital falls at +2
in the AI's own game; idle, 7 of 21 led civs fall by +6. On small 6 a player who raises one
warrior on the pick turn still falls at +4 (`play/r6.*`); one who raises every turn holds
(`play/t6.*`). On small 3 as Gold the store can't fill a second warrior's pack and Gold falls at
+13 whatever the script does. A third of the pick's cards lose in the first few turns unless the
first orders are raises, and small 3's Gold loses even then.

## 3. The ledger

The design files carry 79 `## Decided` lines; M5 added 43. M4's 45 kept lines are left as they
were, except where an M5 line ended them. The proposal PR strikes 15 (one of them a clause) and
trims the numbers from 4; the rest stay.

| Line | File | Why it goes |
|---|---|---|
| An army trades at its stall's market, wares in its pack, only as many as buy its pack's room less its stall (#356) | actors.md | How it's built; supply.md's #356 line holds the rule |
| A clan grows at most 15% a turn gathering, 25% settled (#426) | clans.md | A number; labour.md's child-cost line holds the rule |
| The civ panel's Living row splits people by way of life (#427) | farming.md | How the page words it |
| Settling is a reading: no rule reads it (#427) | farming.md | Ended by #426: growth reads the way of life. K's review flagged it (#429). The owner rewords or strikes |
| The levy is a share of the food civ clans bring to the capital's festival (#72) | founding.md | Ended by #353 and #354 (any store's market) and #356 (the levy takes light goods) |
| Raising names a kind and a place: a new army at the capital, or into an army (#104) | founding.md | Ended by #354: raised at any store |
| A settlement's store: "nothing draws on it yet" (#353) | founding.md | The clause is false since #354. Keep the line, strike the clause |
| The gift's readout is the clans reached, gave and left (#338) | founding.md | How the view words it |
| The founding clan is settled at founding (#427) | founding.md | How a reading is seeded |
| Salt and obsidian weigh a fifth of food; a cap counts food only (#355) | goods.md | Numbers |
| 12 salt and 5 obsidian sources, a twentieth of the days, three quarters a day (#355, #356) | goods.md | Numbers |
| A unit of salt keeps 10 food at half its spoiling (#355) | goods.md | A number; goods.md's prose holds the use |
| A warrior takes half a unit of obsidian and fights a quarter stronger (#355) | goods.md | Numbers; the prose holds the use |
| A starving clan gives none of its days to a source (#355) | goods.md | A tuning fix, not a choice |
| The AI's army goes home to its store, posts nearest a rival, raises there (#354) | war.md | The AI's behaviour |

Trimmed to the rule, numbers to `data/`: crowding kills (#426, keep the rule, drop 8, 0.5% and
2%); the levy takes all the light goods its clans bring (#356, drop four turns and eighty
warriors); obsidian is a clan's tools (#356, drop the tenths and the fifth); the first load
rides with the walker and a child costs days (#426, drop 2, four and a day and a half).

Two lines the pieces decided aren't in a ledger: C's "a mountain tile a scout has bridged is
walked by every actor" (m5.md, C) is in no file, and m5.md §6 said C's review would replace
`scouting.md`'s mountain line. X's and K's ledger lines went in on the owner's merge over
`merge: false`, so they are the owner's numbers (`owner.md`).

## 4. What the scores can't see

- **The player's game.** The sweep scores AI games. The scripted player is a fourth control the
  sweep doesn't have: a led civ that follows the view's own advice. Every M5 piece's judge
  missed §2 because none ran one.
- **The gift.** `setGift` is 1.0 of 56.7 AI orders. On small 2 it reaches nobody (`play.md`); on
  small 61288 as Red it gave 70 food and the civ took 19 clans by +91 (`play/scout-61288.*`,
  the view's "since founding" line). Whether it does anything depends on whether outsiders
  trade at the capital's market, which the map decides.
- **Crowding.** 4,582 people died of crowding on seven maps, beside a starving share of 0.033.
  It isn't starving and isn't scored (`clans.md`, #426); nobody has said whether that is the
  size it should be.

## For other angles

- **players:** §2 is what a player meets: the opening is a raise race and the middle is
  claiming. The check's player claimed 11 times and ended at 36.3%; the script claimed 142 times
  and ended at 89.5% on the same map and civ.
- **retro and proposal:** the claim's limit is M6's first rule question; without one the led
  game is decided by the claim, not the pick (M4's verdict turned over). R and X need
  settlements near fronts and armies abroad, which the maps don't make. Q's reading has no rule
  on it yet, and N's capital town waits on it.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| A claim grows by a bounded number of tiles a turn, or costs something to hold, for the player as for the AI | §2; #351 (#338 build-2) | high: the led game is in doubt again | one rule, the view's Reach line, the planner's numbers | take |
| A new civ starts with a warrior on its capital, or rivals' armies can't march before +N | §2, the opening; #152 | high: a third of the cards don't lose by +6 | one rule | ask the owner |
| The sweep plays the scripted Reach player beside the AI and idle controls | §2, §4 | high: every piece's judge sees the player's game | small: `reach.py`'s two rules in the judge | take |
| The ledger strikes 15 lines and trims 4 | §3 | medium | the proposal PR | take |
| C's bridged-mountain line goes into `scouting.md`'s ledger | §3 | low | one line | take |
