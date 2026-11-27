# M5 retro

M5 merged 21 pieces where [m5.md](m5.md) proposed eight: its own L, J, S, R, C, G and X; P, V
and Y, which the re-cut had moved to M6; K #426 and Q #427, which you opened partway through;
the eight map-art pieces #439 to #446; and the fixes piece #350. N #341 closed into Q as
`proof/341`, and H #342 stopped after its clay, with PR #418 open. Five angles read the
milestone. Their findings and evidence are in `relay/retro/m5/`: `play.md` (the play check),
`ladder.md`, `architecture.md`, `relay.md`, `owner.md`, `game.md` and `players.md`. No notes
from you came in the 72 hours, so this retro and the proposal go without them (`owner.md`).
What M6 does about it goes in `m6.md` and `m6-relay.md`.

M5 cost $229 at list, at least 1,684 5-hour points and 135 weekly points. The loop closes in
the AI's games: capitals fall, a game ends by conquest, and a civ the planner leads beats its
idle self by more than in M4. A player's game is still open. Two rules from the view's own
advice win it: keep four warriors home, and send the claim the Reach line names every turn.

## Game

### What M5 answered

Measured on main with every civ on the AI (`game.md` §1, `game/sweep.txt`), against M4's main:

- **Capitals fall (L).** 6 capitals fall on 5 maps (M4: 0), 1 game of 7 ends by conquest, and
  89 claimed tiles change hands (M4: 14). Lead changes rise from 8 to 14, but the leader's
  margin at +200 grows from 0.098 to 0.261.
- **Orders pay more.** A led civ ends 8.0 points of Standing above its idle self (M4: 4.9). It
  is ahead in 15 of 21 games, and the 6 it loses are the led civ falling early.
- **Scouts finish (C).** On your map, small 61288, a led scout builds its route to Gold's
  capital by +21 with fighting on, after one trip home for food (`game/play/scout-61288.*`).
  In M4 it never finished.
- **Settlements pay, once their store has a use (S then R).** Civs with a settlement score
  19.2% against 16.5% with no signposts. At S's merge the sign was the other way.
- **Salt and obsidian move (G), but seldom reach a civ.** 40.7% of festivals trade one or the
  other. Only 2 civ stores hold either, and 7 armies over seven maps carry obsidian.
- **Towns spread (K, N).** 16 towns on 6 of 7 maps (M4: 13 on 4), but only 3 of 15 capitals
  are towns.
- **Not answered:** R's hungry army-turns outside the capital's ring (32%; M4 29%), because
  settlements form too few and too far from the fronts. X's armies fed at a foreign market:
  21 army-turns in 1,400, proved only on a built map. J joins at 0.0 turns, but nobody played
  it.

### The claim is the game

`game/reach.py` plays a civ in the text view by two rules: raise a warrior at the capital while
fewer than four hold it, and send the claim the view lists as "the claim in reach" every turn.
It never gives a gift, sends a scout or marches an army (`game.md` §2):

| Map, civ | Two rules | Same civ on the AI | Land (AI) |
|---|---|---|---|
| small 2, Red | **89.5%** | 33.8% | 91 tiles (53) |
| small 6, Red | **51.7%** | 26.7% | 125 (60) |
| medium 8, Red | **51.9%** | 16.3% | 219 (61) |

No lead changes, and on medium 8 no rival takes a tile in 250 turns. The planner holds itself
to `PLAN.hold` tiles per person in its armies and widens by at most `PLAN.widen` tiles a turn.
A player has neither limit. A claimed tile costs nothing until a rival army ends its turn on
it, and the AI never comes for a wide, empty claim. P's build filed this on 18 Nov ("A claim
with no size limit wins a dozen clans on the first turn", #351), and it is still open. M4's
verdict has turned over: the game is no longer decided at the pick but by the claim.

### The opening is a raise race

A civ starts with no army, and the AI's first army can reach a near capital in two to four
turns. On small 3, Gold falls at +2 in the AI's own game. Of 21 idle led civs, 7 fall by +6.
On small 6, one raise on the pick turn still falls at +4, and a raise every turn holds. The
pick's card doesn't warn of any of this (`players.md` §1, `game/play/s3*`, `r6.*`, `t6.*`).

### What a player meets

The check played small 2 for 110 turns and ended at 36.3%, against 32.7% on the AI and 27.1%
idle (`play.md`). Six of M5's eight play steps lost to the AI on their own civ; the two that won
claimed what the Reach line named (`players.md` §2). Once the claims run out, there is nothing
to do. The gift reaches no clan on small 2 (four times on #351, and the check's first finding).
The store sits at 50 with no use shown. No pillage order is ever offered. A short scout route
reads "0 tiles to search" for 110 turns. Nobody played the page for this retro, so P's play is
M5's last on the page.

### What the scores can't see

- **The judge has no player.** Every M5 judge compares the AI, idle and the piece's own run.
  None plays a civ the way the view advises, so none saw the claim. The two-rule player is a
  control the sweep should run (`game.md` §4).
- **The play step plays small 2 every time.** It plays like a new player, which is its use.
  But it never plays a map where the piece's rule bites, such as a settlement near a front for
  R (`players.md` §5).
- **Crowding killed 4,582 people** on seven maps (2,482 on large 8). It isn't starvation, and
  no score counts it (`game.md` §4).

### The ledger

The design files carry 79 `## Decided` lines, 43 of them from M5. `game.md` §3 lists 15 to
strike: numbers, how the view words a thing, the AI's habits, and three lines that later lines
ended (the levy at the capital's festival, raising only at the capital, and settling as a
reading no rule reads). It also lists one clause to strike and 4 lines to trim to their rule.
C's "a mountain tile a scout has bridged is walked by every actor" is in no ledger yet. X's
and K's lines went in when you merged over a `merge: false` review, so they stand as your
numbers. The proposal PR makes these changes; the prose is yours.

### Questions for you

`m6.md` opens with these, answered as below. Change one and see what moves.

1. **What bounds a claim?** My recommendation: one rule for every civ, player or AI. A claim
   grows by a bounded number of tiles a turn, and the Reach line names what fits. The other way
   is a cost to hold each tile. The bound is one number in `data/` and keeps the claim free, as
   L made it. The AI's `PLAN.widen` becomes that number (`game.md` §2).
2. **Should the opening be survivable?** My recommendation: yes. A civ founds with a few
   warriors from its founding clan, enough that small 3's Gold holds past +10 on the AI. The
   pick's card and the first view say which rival army can reach the capital, and in how many
   turns. A third of the cards now lose by +6, and one loses whatever you do (`players.md` §1).
3. **Does M6 hold to a list?** M5 opened 23 pieces against eight, and for 58% of its hours
   nothing was ready, waiting on you (`relay.md` §1). My recommendation: M6 is the claim and the
   opening, the clay, herds rebuilt from H's spec, and the page played, in that order, with beer,
   jobs and trade after them. Knacks move to M7, which needs them for the camel anyway.
4. **Where does a rule's proof live?** `engine/CLAUDE.md` says "a rule's proof is a test in
   `core/stage/`", but only V's four scout tests are there. K, Q and X proved their rules in
   Rust that the Tests screen can't show (`architecture.md` §3). My recommendation: a rule a
   player sees is proved on the stage. One ford moves the claim, the settlement's store and the
   line to the nearest store there, and the rest stay in Rust.
5. **Is crowding the right size?** 4,582 deaths against a starving share of 0.033. My
   recommendation: report it in the judge beside starving, and leave the rule until a map shows
   it's wrong.
6. **Relay on Codex (#503).** It changes who builds M6 and is open. My recommendation: decide
   it before M6 starts, because it changes `m6-relay.md`'s pacing and costs.

## Process

### 1. You set M5's pace, not the crew

M5 ran 292 hours from first session to last merge (`relay.md` §1):

| | Share | M4 share |
|---|---|---|
| Sessions | 15% | 33% |
| 5-hour stops | 21% | 9% |
| Weekly ramp | 6% | 52% |
| Nothing ready, waiting on you | 58% | 5% |

The 58% includes the stops you chose, such as the play after C. Two of the waits were crew
faults: L's merge refused for 49 hours because squash merges were off, and L's whole blocked for
9.7 hours. You changed the crew by hand six times mid-milestone (three in M4). One of those,
92939e3, dropped the weekly check, and the week went from 70% to 97% in 22 hours. The handover
on a crew change made the hand changes safe. Every restart came two minutes after its reset or
lift, so the M4 fix for the weekly lift works. No work was dropped after 13 Nov, nothing
compacted, and CI was never red on main.

### 2. Fable wholes spent the window

The five whole Fable pieces cost $11.41 each, the same list dollars as a stepped game piece
($11.42). But they took 113 5-hour points against 70, and 9.4 weekly points against 5.3. In M5,
Fable took 11.0 5-hour points per list dollar, against Opus 5.9 (M4: every model 5.0 to 5.5).
A window bought about 0.9 Fable pieces or 1.4 stepped game pieces. Your change of 19 Nov,
stepped on Opus by default, is what the figures say (`ladder.md` §3). Quality couldn't be
compared, because the pieces did different work. All five Fable pieces merged on their first
review; K's Opus whole failed twice.

### 3. The ladder predicted; Haiku still polls

Points rank with cost at 0.70 (M4: 0.48), and the size line ranks with actual lines at 0.94.
Only G build-1 ran off the ladder: a glaze at 6 points, written by a whole step that didn't
score it. Two Haiku builds re-armed a Monitor 89 and 124 times, 28% of Haiku's spend, despite
`build.md`. The fixes spec ran on Opus for $5.62 where the crew writes Sonnet. `ladder.md` §9
drafts the prompt edits: take the Monitor tool from Haiku, have the whole step score its
builds, run fixes specs on Sonnet, and fix `spec.md`'s opus-low example.

### 4. The loop repeats what it can't move

29 ideas and 8 fixes, on one ideas issue instead of 85 issues. But the play step is told only
the open lines, so a ticked idea that changed nothing a player sees comes back under new words:
the gift, four times (`relay.md` §3). And no play step found the claim, because each plays a
few legal orders on small 2 (`players.md` §2).

### 5. The code

Y did all of the M4 retro's clay in one piece (`data/` as a folder, `Actor::carry`, the goods
table, the planner's paths cached), and every new system got its own module. Four things
slipped (`architecture.md`):

- **Scale is still broken.** Large 8 runs at 0.53 of the turn before #156 (bound 0.45). The
  paths cache saved 5.4 ms, but G had already added 2.7 ms by reaping twice per candidate
  territory. A one-line guard takes main to 0.47 with digests unchanged (`architecture/guard.diff`).
- **A settlement's store is not an `Actor`.** 29 calls in 9 files branch on capital or
  settlement, and the clearing posts only the capital's store.
- **Stage tests stopped at V's four** (question 4).
- **`engine/CLAUDE.md` is full** (8,162 of 8,192 bytes), mostly atlas cell sizes from the art
  pieces.

H's PR #418 conflicts with main in 20 files. Its spec still holds, but the code is cheaper to
rebuild than to rebase.

### 6. Crew fixes

From `relay.md` §6, for `m6-relay.md`: a review isn't done without its `merge` field (K's
reviews wrote it as prose); the milestone is built when every labelled piece closes (`m5` was
tagged before #445 and #446); the play step sees the ticked ideas too; a merge falls back to a
merge commit when squash is refused; one dispatch per batch of label events (91 cancelled runs
in 2.5 minutes); and a killed step keeps its `done` result if its commit passed. From
`players.md` §5: the play step plays a map chosen for the piece and runs the two-rule player
beside it.

## Ideas, ranked

| # | Idea | From | Call |
|---|---|---|---|
| 1 | A claim grows by a bounded number of tiles a turn, for every civ | `game.md` §2; #351 | take: question 1 |
| 2 | A civ founds with warriors; the card says which rival army reaches it and when | `game.md` §2; `players.md` §1 | take: question 2 |
| 3 | The judge plays the two-rule Reach player beside the AI and idle | `game.md` §4 | take |
| 4 | Clay: jobs as a named table, one store (a settlement holds an `Actor`), the `move` guard, carry by the actor, CLAUDE.md trimmed | `architecture.md` §4 | take, first |
| 5 | Herds rebuilt on main from H's spec; PR #418 closed | `architecture.md` §4.1 | take |
| 6 | The clearing's rows built one way, so a trader sits in two markets | `architecture.md` §4.3 | take, with trade |
| 7 | The ledger: strike 15 lines and a clause, trim 4, add C's mountain line | `game.md` §3 | take, in the proposal PR |
| 8 | A gift that reaches nobody says where it could | `play.md`; `players.md` §3 | try |
| 9 | Crew fixes 1 to 7 above; Monitor off for Haiku; the whole step scores its builds | `relay.md` §6; `ladder.md` §9 | take, `m6-relay.md` |
| 10 | M5's player-facing rule proofs moved into `core/stage/` | `architecture.md` §3 | take: question 4 |
| 11 | The play step plays a map chosen for the piece | `players.md` §5 | take |
| 12 | Hand crew changes go through a PR | `relay.md` §6 | ask you |

### Not taking

- A whole step on Fable: same dollars as a stepped piece, 1.6 times the window (`ladder.md` §3).
- Pillage offered one step away: there's nothing to pillage until armies go abroad
  (`players.md`).
- Splitting `map.rs` or `ai.rs` now: M7 rebuilds the map, and nothing has grown `ai.rs` since R.
- Rebasing H's PR #418: rebuild it instead.
