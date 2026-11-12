# M4 retro

M4's eight pieces merged: #151 (S), #152 (E), #153 (T), #154 (D), #155 (F), #156 (W), #157 (R)
and the fixes piece #162. The crew tagged `m4` and opened this retro itself. Seven angles read
the milestone, and their findings and evidence are in `relay/retro/m4/`: `play.md` (the play
check), `ladder.md`, `architecture.md`, `relay.md`, `owner.md` (your notes, and where each
stands), `game.md` and `players.md`. Relay ran the first four; the rest, this retro and the
proposal were written in a local session, on your call. What M5 does about it is in
[m5.md](m5.md) and [m5-relay.md](m5-relay.md).

Relay ran clean: 69 sessions, eight merges by the crew on the first try, no crash, timeout or
compaction. M4 cost $149 at list, 785 5-hour points and 68 weekly points, 39% over the
proposal's money. What went wrong is, a third time, the game: every piece answered its
question, the rival plays, and a game is still decided at the pick.

## Game

### Built, and decided at the pick

Measured on main with every civ on the AI, and each civ led by the planner (`game.md` §1):

- **Orders pay now.** A civ the planner leads ends ahead of its idle self in 21 of 21 games, by
  4.9 points of Standing, and wins 6 where idle wins 2. In M3 an idle civ did as well as the AI.
- **Fields pay.** 14 of 21 civs farm and gain 5.2 points of Standing against the no-fields
  control.
- **Nothing ends.** 7 of 7 games run to the turn limit and no capital falls. Led by the
  planner, 0 of 21 end early.
- **Nobody loses land.** The AI's guard holds 35.19 of its 35.24 claimed tiles. People fall on
  two maps of seven; pillage is 0.07 orders per hundred turns.
- **Towns form where clans are many.** 13 towns on 4 of 7 maps, 9 of them on the large map. A
  small map gets one or none.
- **A player ends turns.** The check's player gave 11 orders in 103 turns and went 95 turns
  with none. Idle on your map (small 61288) a civ is second from the pick to the limit, with no
  lead change (`players.md` §1).

The end rules work and nothing sets them off. A capital falls only with no guard, no army and
no militia on it, and a claim is lost only where its guard is thin, and the AI keeps both
thick. So your first game note, removing the guard and the militia, is also what the numbers
point at: it is what would let land and capitals change hands.

### Your notes

Each is a piece, a line of a piece, a Relay change or an answer here (`owner.md` has the table;
`game.md` §2 traces the game ones):

| Note | Finding | Goes to |
|---|---|---|
| Remove the militia and the guard | The guard is what makes a claim held, so the piece has to say what holds land instead: question 1 | Piece L |
| Scouts go back to the capital for food | Reproduced on your map: a scout sent 18 steps stands on one tile from +10 to +200 with five tiles left to search, foraging, never hungry (`relay/retro/m4/game/scout-61288.txt`) | Piece C |
| Warriors kill scouts, and painted tiles trap them | Today a scout neither fights nor is fought, and a zone doesn't stop it | Piece C |
| The scout's test, and all such, viewable and runnable on the page | The stage's tests run only in `cargo test` | Piece V |
| A click centres the camera on the tile | Yours from the page | Piece P |
| Play the page, name the worst, propose fixes | This retro played the text view only. The page gets its own played pass | Piece P |
| Towns: one town on small 61288 | As built: one forms, in Gold's country, on turn 258. Small maps form one or none | Piece N |
| The scout gave its own order at the pick | Not reproduced; the planner gives scouts no orders | Nothing, unless you see it again |
| The starving tests | Recorded: `vision.md`'s ledger and `process.md`, Scores, line 5 | Done; and piece Y |
| The Relay notes | Built in this PR | `m5-relay.md` |

### What else isn't helping

You asked what else is going on like the starving bands. Three things (`game.md` §4):

1. **The sweep still gives verdicts.** Its fields and towns groups print 22 `pass` or `MISS`
   words against bounds designers wrote. Six read MISS on main and the pieces merged anyway.
   The bounds decide nothing and still read as the result.
2. **Builds loosen a test to pass.** `scout_errand` went from 0 scouts lost to 5, then 4.
3. **Tests sit on seeded picks.** Ten of 39 core test files and 16 of 19 page scripts read a
   retro pick, which moves with any float the pre-game reads. Four of M4's five second sessions
   were these.

The fix is the same each time: a piece's question names one number and its control; the sweep
reports and gives no verdict; a rule's test is a built map on the stage, which you can watch.

### The ledger

M4 ended with 149 `## Decided` lines. This PR strikes 104 and keeps 45: what Standing counts,
how a game ends, what land is, who fights, what a clan, a farmer and a town are, who pays a
walk. Struck: how the page looks and words, the AI's thresholds, numbers, how a rule is built,
and lines a later line replaced. `docs/design/README.md` says what the ledger now is, and the
review's prompt holds it to that. The prose is yours.

### Questions for you

These change the pieces in `m5.md`; its opening states the answers it assumes.

1. **What holds land without a guard?** My recommendation: a claimed tile joined to the capital
   is the civ's land until a rival army ends its turn on it. Armies are then the only thing
   that holds or takes, a claim costs nothing to keep and is lost by being walked on, and a
   capital falls when a rival army stands on it with no army of its own there.
2. **Is M5 your notes or herds and beer?** My recommendation: both halves, cut smaller. Your
   notes are five small pieces and come first, since they are what makes a small map a game.
   Then the clay the code needs, herds, and pack animals. Beer moves to the head of M6.
3. **The clock, open since M3.** My recommendation: close it as built. The turn stays a year,
   distance isn't rescaled, and an animal is priced in the days it saves a walk.
4. **What should a small map's towns be?** A town needs three clans on one tile with a
   festival. My recommendation: leave the rule, and let a civ's capital count its festival, so
   the place a player looks at is the first town.
5. **A looser `fix` rule for readouts?** Six findings were fixes and 85 asks, because the
   design rarely names a readout. My recommendation: no new rule. From M5 a designer takes the
   obvious ideas into its own piece, which does the same job without a class to argue over.

## Process

### 1. The ramp set the pace, not the crew

M4 took 104.7 hours from first session to last merge: sessions 33%, the weekly ramp 52%,
5-hour waits 9%, one blocked step 5%. You were 5% of the clock, against 58% in M3. The crew
sat 2.5, 15.9 and 14.5 hours past a weekly lift because nothing restarted it. A weekly stop now
starts a wait, as a 5-hour stop does (`relay.md` §1).

### 2. Opus low lost both blind picks

The twin gave the first controlled comparison of models in four milestones, for 6% of M4.
Sonnet high and Opus medium each found a cause where Opus low tuned until a bound passed. The
ladder drops Opus low: a canvas runs Sonnet high, and Opus medium from 11 points. The twin and
the pick come out, on your call: M5's experiment is the whole piece on Fable (`ladder.md` §3).

### 3. A whole piece on Fable spends a window in ninety minutes

R ran whole on Fable medium: $28.20 and 142 5-hour points, about 1.3 times its stepped plan.
Its first session went from 1% to 102% of the window in 89 minutes and died there. From M5 a
whole step starts only on a fresh window, is told its usage as it starts and every fifteen
minutes, may hand builds, a play step and a review on to cheaper sessions, and a session the
limit ends costs no attempt (`ladder.md` §6).

### 4. Ideas piled up unread

85 `relay:proposed` issues are open and none was promoted. The play steps filed the same four
findings piece after piece under new titles. From M5 an idea is a line on the milestone's one
ideas issue, a session is shown the open lines, a designer takes the obvious ones, and the
proposal ends every one: `m5.md`, Ideas, does that for the 91 open now (`relay.md` §3).

### 5. Tests ran long, and ran twice

`engine/check.sh` took 12m24s on CI. One test ran the whole seven-map sweep to check JSON keys,
3m44s of it; it is now a report, and `check.sh` takes 9m22s and fails on any test file over a
minute. Four Haiku sessions re-armed a Monitor on it 41 to 155 times, $12.53 of Haiku's spend.
"No time for another step in this job" is the job's 250-minute limit on starting a step, not a
session that ran out: four in M4, each with the next run queued. Sessions keep the one-hour
cache, since a five-minute one rewrites the context on every wait longer than that.

### 6. The code

The shape held: 4,060 lines of core and 124 tests added, and every piece but R grew the modules
its brief named. Three things slipped: the planner's path searches take main past the scale
bound (0.52 of the turn before #156), `data.rs` is 1,024 lines, and tests sit on seeded picks.
M5's clay piece takes all three with the goods and carrying seams herds need
(`architecture.md` §4).

## Ideas, ranked

| # | Idea | From | Call |
|---|---|---|---|
| 1 | Land held without a guard; the militia goes | your note; `game.md` §1 | take, L |
| 2 | A scout eats at its capital; an army kills it; a zone stops it | your notes; `game.md` §2 | take, C |
| 3 | The stage's tests listed and played on the page | your note | take, V |
| 4 | The page played, its worst three fixed; the camera; the gift's readout | your notes; `players.md` §2 | take, P |
| 5 | The capital's festival counts for a town | your note; `game.md` §1 | take, N |
| 6 | Clay: a goods table, one carrying rule, `data/`, the planner's paths cached, no verdicts in the sweep | `architecture.md` §4; `game.md` §4 | take, Y |
| 7 | Herds; pack animals and horsemen | the roadmap | take, H and K |
| 8 | The whole step that hands on, usage in the session, one log and one ideas issue a milestone, draft PRs | your notes; `relay.md` §6 | built |
| 9 | No Opus low; magnum on Opus medium | `ladder.md` §9 | built |
| 10 | The play step plays the page | `players.md` §4 | try, in P |

### Not taking

- The twin for another milestone (`ladder.md`): your call, and the whole step is the experiment.
- A looser `fix` rule for readouts (`relay.md`): question 5.
- Splitting `map.rs` now: M7 rebuilds it.
- Beer in M5: question 2.
