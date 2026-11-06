# M3 retro

M3's five merged pieces: #100 (H), #101 (G, split: its held half is PR 124), #102 (L), #103 (P),
#104 (A). #105 (R, the rival that plays and the game that ends) is open, paused and unbuilt, so
M3 has no `m3` tag and you played main (ec1e47b) without it. Five sessions read the milestone
from one angle each; their findings and evidence are in `relay/retro/m3/`: `game.md`,
`players.md`, `ladder.md` (cost and briefs together), `architecture.md` and `relay.md` (crew and
prompts together). Your notes are `owner.md`. What M4 does about all of it is in [m4.md](m4.md)
and [m4-relay.md](m4-relay.md).

Relay ran clean: 38 steps in 39 sessions, no crash, timeout, compaction or blocker, four pieces
merged by the crew on the first try. M3 cost $71.40 at list, 434 5-hour points and 33 weekly
points, 7% over the proposal's money and 19% over its 5-hour points for these five pieces. What
went wrong is, again, the game: every system is built and every piece passed its question, and
with R unbuilt there is still no contest. Under that, the economy can't feed an army, so even R
would have had nothing to march.

## Game

### Built, and still no contest

Measured on main with every civ on the AI (`game.md` §1, `players.md` §1 and §5):

- **Claiming pays, but less than when it landed.** +0.012 Standing a civ (L's PR said +0.017);
  10 of 21 civs gain, 7 lose. All of the gain is clans won; people on held land add nothing,
  because a clan on held land joins within about eleven turns.
- **The lead changes hands** 10 times on 6 maps, against 5 on 3 with no claims. That is AI civs
  growing at different rates, not a contest.
- **Nobody fights.** People fallen 0, tiles taken 0, armies surrendered 0, clans left 0, no game
  ends. A's rules play out only in its scripted stories, on one map.
- **An idle led civ does as well as the AI.** A led civ given no order for 100 turns ends within
  about a point of the AI playing the same civ, on three maps (3.6% against 2.8, 13.0 against
  12.9, 2.5 against 3.5). Beating the AI means beating doing nothing.
- **The economy is the binding limit.** 14 of 21 civs never hold 15 food, one three-warrior
  army's packs, in 200 turns; 16 of 21 average under 8. The largest army on any civ is its
  founding three. The four rich civs (24–43 food) raise nothing, so there the AI stops war, not
  the store. The guard starves within ten turns of the pick in a led game.
- **Scouts still starve on their first errand**: 13 of 14, and 11 of 14 with the store kept full.
  The line carries about 1 food a turn seven steps out and foraging strips the ground round the
  scout in a few turns. The store isn't the cause; reach is.

So the order of game work is: a finish line (R's end rules, which are small), then an economy
that can feed an army (settling, and supply as a flow), then R's planner. Tuned on today's
economy, a planner would learn that raising never pays.

### Your play notes

Each traced to its cause (`game.md` §2; `architecture.md` §4 writes the first two as tests):

| # | Note | Cause | Proposal |
|---|---|---|---|
| 1 | The scout's three buttons | "Move to…" arms a destination and sends `route`: it is Route to under the wrong name. A scout has no Move order; with no route it walks home | Rename the button Route to…; add a scout `Move` with a destination it walks to and stays at, foraging; keep Route from… |
| 2 | Disband all disbands one | The page's "all" option opens the count stepper at 1 and sends `n: 1`; the engine's "all" is `n` absent and works. The guard shows no "all" word, which is why it reads right | "All" is a button that sends no count. Trivial |
| 3 | The scout ignored two mountains | The search skips a mountain only when no pass on it could beat the known route, which is `scouting.md`'s rule; in ten traced errands every skip was right. But it searches nearest-first, so it is slow along long ranges, and the panel never says why a mountain is left | Order a range's mountains by their best case; the panel's Unrevealed row says "can't beat the known route (best 21 against 16)" |
| 4 | A trail should always make a difference | Flat land has no hidden cost, so a plains trail does nothing | A plains and coast trail at 0.75, measured: trading 0.90 → 0.93, aligned share 0.200 → 0.232, held land +30%, every pre-game still ends by centres. 0.5 stills the map. Take 0.75 as the start |
| 5 | Component and integration tests | Fixtures paint tiles by hand; the stories script a civ but assert nothing; nothing places units, gives orders and checks an outcome in one builder | A `Stage` in core over the `map` command's own text, with the two cases above as its first tests (`architecture.md` §4) |

Both the page bugs came from steps no script exercised: A's build-4 shipped Disband all, and no
review pressed every new button. Both are the kind of thing a scripted outcome test catches.

### The ledger

M3 added about 60 `## Decided` lines. `game.md` §3 marks each promote, reword or strike with a
reason, by file. The pattern: promote the rules a player meets; reword every line that carries a
number so the relation stays and the number goes back to `data.rs`; strike the engineering lines
(one door for orders, the AI's logged orders) that `architecture.md` already holds, the AI's
stand-in claim when R lands, and "The game goes on" when E lands. From PR 124's `vision.md`
ledger: promote "Standing counts a civ's clans and its units' people", reword the half-point lead
margin as "a margin", strike "An army mustn't starve its own side".

### Questions for you

These change the pieces in `m4.md`; its opening states the answers it assumes.

1. **Which clock does war run on?** Your yardstick (`owner.md` §2; drafted as `design/labour.md`)
   makes a tile a day's walk and a turn a year of a hundred days. The engine today charges a tile
   28.6 days of a person's year, so the change is a rule, not a renaming: at a tile a day an
   army crosses the large map inside one turn and outruns its ring in days
   (`architecture.md` §3). The farmer's year and the soldier's day don't fit one clock as the
   engine stands. Three ways: units act in day sub-steps inside a turn (march, forage, fight,
   supply, ten steps of ten days); tiles stay larger than a day for armies; or the turn is
   shorter than a year. My recommendation: keep the year, and in M4 do the part that doesn't
   wait on the answer (every number labelled in days, walks charged per person carrying, the
   supply ring computed and drawn), and decide the clock before M5, where animals change what a
   person carries.
2. **#105 R.** Close it and re-cut it into two pieces: the end rules first (small, and every later
   judge reads them), the planner last, once there's an economy to plan on. Or run it as it
   stands, after farming.
3. **#101's held half (PR 124).** Remake it into M4: close the PR, carry over `player.md`, the
   trial's record and two ledger lines by hand, drop the bands, and rewrite `judge.md` as the
   review's rubric. Merging now means resolving ten conflicts in the code that gets dropped
   (`players.md` §2).
4. **A trail on flat land at 0.75.** Take it, with the pre-game sweep re-run?
5. **The pack.** A warrior's pack holds five turns of food: five years' eating on the day scale,
   where a man carries 30–50 days. Is the pack what each man carries, or the train behind the
   army? It decides how the ring and surrender tune. My recommendation: the train, until animals
   exist to carry it (M5), and say so in `supply.md`.
6. **How far does Relay go without you in M4?** `relay.md` §4 designs the whole loop. My
   recommendation: the play step, the fix-or-ask rule with a fixes piece per milestone, and the
   milestone-built rule now; the retro run as a try, on five angles in four sessions.

## Process

### 1. The judge reported and never checked

Every piece with a question asked it in the scores against a control and gave the old and new
numbers; two findings changed decisions (the trial's dead gift, L's lost camps). But the bands
PR 124 would have put in CI, run against each merged main, fail L on the army cost you merged on
purpose and warn on a count whose meaning L changed, and catch no regression. You were right that
CI would have caught nothing (`players.md` §2). What a judge should do instead, at review time,
is in `m4-relay.md` §3: a `judge --compare` that prints old, new and a measured noise floor for
every metric, and a written rubric the reviewer applies (better, same, worse, not shown), with
"better" needing a move past the floor on more than one map, by the mechanism the piece claims,
in a rule that fired, and a led civ beating an idle one.

### 2. Drift was found by hand

The engine is deterministic, so "drift" is clan dynamics diverging once a run differs at all.
PR 142 measured it with a civ outside the fight (about a point) and called two of three stories
drift. Nobody measured it for L, and the trial's "beat the AI by 1.1 points" is inside it. A null
perturbation (the sweep once more with one harmless change at the pick) measures it on every run.

### 3. Spec-review earned its keep

$1.98 a piece, 14% of M3. On every piece it changed something that would have failed a build or
the proof as written: G's trial couldn't have started, L's guard held nothing past the capital,
P's played proof had no command, A's surrender could never fire (`ladder.md` §5). Its blind spot
is the piece's own earlier steps: it measures main, and in L and A a step earlier in the piece
moved the number it measured. Keep it, and tell it to measure with the earlier settled steps
applied where a later step relies on the number.

### 4. The ladder, and the rung below

Points pick the model and the model sets the cost band; within a model the piece predicts cost,
not the points. Haiku outside H's refactor cost what Sonnet medium does ($1.49–3.98 against
$2.02–2.51) and used as many 5-hour points, writing two to four times the output. The rung-below
experiment finished five of five; the two Sonnet medium steps at 8 points met every target, so
8 points moves to Sonnet medium. A's build-3, a Sonnet high kiln at 8 with Open 2 and Miss 3,
cost $5.10, 83 minutes and 49 5-hour points, more than any Opus step, and its done wasn't met:
a kiln carrying unsettled work is mis-cut, and the shape rule only flags it at 11. H's Sonnet
high spec was as good as the Opus ones on a settled piece. The size line ranks right and runs
1.6× low (`ladder.md` §2, §3, §7).

### 5. The time went to waiting on you

Of M3's 57 hours from first session to last merge, sessions ran 22%, 5-hour waits 20%, and one
wait on you 58%: #101's review wrote "Verdict: merge" in the PR and left `merge: true` out of
`result.json`, so the crew said the verdict wasn't merge, and nothing restarted the crew after
your merge until you typed `/relay run`. Restart at the reset worked all four times (2.5 minutes
after the logged reset), the 80% gate held, and the expected-points table was off by half or
double and never decided a stop. After the last merge Relay sat idle 3.4 days with #105 paused,
no tag and nothing to start the retro (`relay.md` §1, §3).

### 6. Prompts

Taking the spec's decisions out of `soFar` worked at the start of a piece, then builds' decisions
refilled it to 56% of a build prompt. "Under five is usual" became a quota: 13 of 22 builds wrote
exactly five, and about a third are owner choices. `engine/CLAUDE.md` was trimmed to 5.2 KB and
grew back to 20.1 KB, mostly one row per play script. Screenshots in PR descriptions link the
branch, which is deleted on merge. Sessions wrote 68 ideas and none became an issue; eight repeat
(`relay.md` §2, §3).

### 7. The code

H's shape held through L and A: every order through one door, spoil, eat and stride on the actor,
each piece growing the module its brief named and reusing `divide` for where people stand.
`units/mod.rs` is 781 lines (A's raising and disbanding), `cli/src/judge.rs` 833, and the guard
eats by a third road. A large map runs 50 ms a turn past the pick, five times the target, and the
festival choice is 61% of it; `bench` never saw it because it stops before the pick. No two clans
share a home tile anywhere, so a town can't exist yet (`architecture.md` §1, §2, §5).

## Ideas, ranked

Across the five angles, about 90 ideas; these are the ones worth doing next. Each angle's file
has the rest with its call.

| # | Idea | From | Where |
|---|---|---|---|
| 1 | R's end rules, turn limit and end screen, ahead of its planner | game, players | M4 piece E |
| 2 | Supply as a flow: a day, walks per person, the ring computed and drawn | you, architecture, game | M4 piece D |
| 3 | Labour across jobs as a digest-safe clay; the farmer on it | architecture | M4 pieces D, F |
| 4 | Homes that don't claim; field ground shared by area; the festival search once per tile | architecture | M4 piece W |
| 5 | A `Stage` harness: a readable map, units placed, orders given, outcomes asserted; the two owner cases first | you, architecture, game | M4 piece S |
| 6 | `judge --compare` with a null-perturbation noise floor; an idle led civ as a second control; an economy group | players, game | M4 piece S |
| 7 | A judge rubric inside the review; the PR carries a phone-sized block | you, players | Relay |
| 8 | Close PR 124; carry the player and the trial by hand; strike the third ledger line | you, players | Relay |
| 9 | The play step before the review, Sonnet, on the map the spec names; `player.md` updated for land and armies | players, relay | Relay |
| 10 | Disband all is a button; the scout's three buttons; search by best case with its reason in the panel | you, game | M4 piece T |
| 11 | Feed the scout on its errand; a trail on flat land at 0.75 | game | M4 piece T |
| 12 | Require `merge` in a review's result; a close starts a crew run; one wait at a time | relay | Relay |
| 13 | A milestone is built when none of its pieces can run; tag then; a paused piece goes to the retro | relay | Relay |
| 14 | Findings and ideas classed fix or ask by a rule; a fixes piece per milestone; `ask` waits as `relay:proposed` | you, relay | Relay |
| 15 | A `retro` run kind with a 72-hour window for your notes, ending in a proposal PR that opens the pieces | you, relay | Relay, a try |
| 16 | 6–8 points runs Sonnet medium; the rung below is the canvas on Sonnet high; a kiln with Open 2 and Miss 3 is mis-cut | ladder | Relay |
| 17 | Every new button pressed once by a script that checks the world; the review runs it | ladder | Relay, piece S |
| 18 | Spec-review measures with the piece's earlier settled steps applied | ladder | Relay |
| 19 | Byte budget on `engine/CLAUDE.md` in `check.sh`; one row for `www/play/` | relay | Relay |
| 20 | Screenshots linked at the commit; the crew writes the review's own row; drop "under five is usual" | relay | Relay |
| 21 | `bench --after N`; the festival search per home tile; clans indexed by market | architecture | M4 piece W |
| 22 | Label every `data.rs` number in days or food-turns; notes back to one line | architecture | M4 piece D |
| 23 | Split `units/mod.rs` and `cli/src/judge.rs` when next touched; a brief adding to a file over 600 lines names the split | architecture | Relay, pieces |
| 24 | Clans leaving: 0 on main; tune the run or margin so Standing is losable | game | M4 piece R |

### Not taking

- **CI bands on the scores.** They hold the game where it stands and block deliberate change.
  The review's judge replaces them.
- **Merging PR 124 as it is.** The conflicts are all in the code to drop.
- **Moving the Haiku rungs.** One clean rung-below step and one shipped defect isn't evidence;
  the defect came from an ambiguous brief line.
- **A separate judge step.** The review already runs the sweep; independence costs a cold read.
  Try it only if reviews start praising their own fixes.
- **Rescaling every distance to the day in M4.** It waits on your clock answer (question 1).
- **A vision-model player, or more than one game per play step.** The text view and one game at
  about $0.20 found what the trial found.
