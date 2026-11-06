# M3 retro: players and the judge

Angle: what the judge told the owner in M3, what to do with PR 124, the review-time judge the
owner asks for (owner.md §5), the play step for M4, and what the AI is as an opponent.
Measured = read from a PR, an issue block, a spec file or a run of mine on main (040bc81,
same engine as ec1e47b). Estimated = mine, marked. My runs are in the scratchpad, not the repo;
the figures that matter are quoted here.

## Verdict

**The judge worked as a report and never as a check, and the game it reports on has no
opponent yet.** Every M3 piece with a question gave old and new numbers against a control, and
two findings changed decisions (the trial's dead gift, PR 118; L's lost camps, PR 126). But
`sweep --judge` plays only AI against AI, the AI never leaves its own land, and on main a led
civ that gives no order for 100 turns ends within about a point of the AI playing the same civ
on three maps I ran. So "beat the AI" means "beat doing nothing", and M3's question, a game,
can't be answered before R. The CI bands in PR 124 would have caught no regression in M3: run
on each merged main they fail L's deliberate cost and warn on two numbers whose meaning L
changed. **Close PR 124, carry the player, the trial's record and two ledger lines into M4 by
hand, drop the bands, and put a judge with a written rubric into the review**, fed by a play
step that runs before it and by a `judge --compare` table that does the arithmetic.

## 1. What the judge told the owner in M3

### Each piece's answer

| Piece | Question in the scores? | Control | Old → new given? | What decided |
|---|---|---|---|---|
| H #100 (PR 107) | none: a refactor, proved by digests (process.md, Pieces) | main | retro sweep byte-identical, digests, speed 1.558 → 1.544 ms | merge; the right proof for a refactor |
| G #101 (PR 118) | the tool itself; Standing defined | all-AI control for the trial | trial Red 10.0% vs control 8.9% | the trial's dead-gift finding fixed the view; the owner split the bands off (relay/specs/101/split.md) |
| L #102 (PR 126) | Standing (solo gain), doubt (lead changes) | no-claims, main, `--solo` | lead changes 7 on 4 → 12 on 5 (no-claims 3 on 3); solo +0.0170; camps lost 3 → 18, starved 19 → 53 | merged with the army cost named; play found seed 2's guard starves in 7 turns (relay/specs/102/review.md) |
| P #103 (PR 134) | decisions, counted as levers with a button (14 → 15 of 15) | main | yes, a table; digest unchanged | merge; the scores barely apply to a page piece, and the play notes (scout hunger, Move-to reach) carried the findings |
| A #104 (PR 142) | Standing moved by fighting | `--no-fight`, per story | skirmish Red +2.9, stack −0.1, surrender +0.6; sweep starved 53 → 26, lost 18 → 6 | merge; the review measured drift with a civ outside the fight (Gold ±1.0) and called two of three stories drift |

Every piece with a question asked it against a control and gave numbers on both sides, which is
what m3-relay.md §1 asked. Two gaps:

- **A's stories are one map.** All three run on small seed 2, Red against Blue
  (relay/specs/104/stories.md), because medium seed 9's corridor doesn't overlap. CLAUDE.md
  says "the retro maps show the piece does"; here one retro map did.
- **Drift was found late and by hand.** The engine is deterministic, so "drift" is the
  divergence of clan dynamics once a run differs at all, not random noise. PR 142's review
  measured it with Gold (in no fight, moved −1.0 to +0.9 points). Nobody measured it for L, and
  the trial's "beat the AI by 1.1 points" (play.md) is inside it. No noise floor exists for the
  sweep's seven-map totals: **unmeasured**.

### Findings that changed something, and noise

| Finding | Source | Effect |
|---|---|---|
| A gift reaches only clans that trade at your market; the view said "in reach" | trial, relay/specs/101/play.md | view fixed in PR 118 |
| Army numbers aren't health | split, PR 118 | `armies` group split from `health`; bands held |
| Claims cost 15 more lost camps | PR 126, review-judge.txt | owner merged with it; `GUARD.start` 2 offered as the fix |
| On small seed 2 the led civ's guard starves whatever it claims | relay/specs/102/review.md | the review moved its play to seed 9 |
| An AI civ starves to 0% in a played game with no war | relay/specs/102/review.md | none recorded; the sweep's AI-only floor is 1.0% |
| No stack reaches a capital: the store fills about three packs | PR 142 | left for the economy or R |
| Stack and surrender's Standing moves are within drift | PR 142, relay/specs/104/review.md | not claimed as answers |
| A's top Standing 0.100 → 0.096, lead changes 12 on 5 → 10 on 6 | PR 142 | reported; it's the forage change moving everything, of drift size |

### What `sweep --judge` on main never shows

Measured on main (`sweep --judge`, 65 s wall, identical to relay/specs/104/glaze-judge.txt):

- **No fight.** people fallen 0.00, tiles taken 0, armies surrendered 0, claimed tiles taken 0.
  A's whole rule set is silent on the maps the judge plays.
- **No capital falls, no game ends.** R is unbuilt; `phase` has no `Over`.
- **Clans hardly leave.** clans left 0 (1 after L); joined 130. Losable Standing (m3.md answer 2)
  exists in the rules and not in the AI's games.
- **No led civ.** Every civ is on the AI, so the levers the player has (gift, keep, claim shape,
  raise, pillage, zone) are measured only in stories and played games.
- **Standing on land is 0.001**, because a clan on held land joins within about 11 turns
  (relay/specs/102/review.md).

The doubt the sweep reports (10 lead changes on 6 maps) is AI civs growing at different rates,
not a contest.

## 2. PR 124: close it, carry four things into M4

### What's in it

`git diff main...origin/claude/wonderful-lamport-aduj1e --stat` shows 45 files, because the
branch is the unsquashed #101; the held work is the one commit 98d3a12 (22 files, +834 −39):
`engine/judge.toml`, the band check in `cli/src/judge.rs` (+167), the `toml` dependency,
`sweep --judge` in `check.sh`, three `vision.md` Decided lines, `relay/players/player.md` and
`judge.md`, `relay/specs/101/play.md`, `bands.md`, `bands-*.txt` and `trial/` (lines,
findings, judge output), and band lines in both CLAUDE.md files.

### How bad the conflicts are

Cherry-picking 98d3a12 onto main in a scratch worktree: **10 hunks in 5 files**, all in code
the band check touches. `cli/src/judge.rs` 4 (the module doc, the `Total` doc, `judge()`'s
signature against L's `--solo`, and the band tests against L's `GameJudged`), `sweep.rs` 2
(`--bands` against `--solo`/`--no-claims`), `main.rs` 1 (USAGE), `engine/CLAUDE.md` 2,
`cli/tests/play.rs` 1 (main's test no longer reads the trial save). Mechanical, about an hour
of a session, but every one of them is in the part the owner doesn't want.

The prompts and the trial's record conflict with nothing: those paths don't exist on main.
The trial's `g.json` (on main) is save version 2 and main is 7, so it no longer replays: it is
a record, not a fixture.

### What the bands would have done

Run against each merged main with the PR's `judge.toml` (my script over `--json`, and by hand
for the post-L text in relay/specs/104/main-judge.txt):

| Main after | Fails | Warns |
|---|---|---|
| G (bands.md) | none | none |
| L | `armies.warriors_starved` 53 (> 40), `armies.camps_lost` 18 (> 8) | `ai_orders_per_100` 22.21 (> 5.0), `levers_repeated` 2.95 (> 0.9) |
| P | as L (no core file changed) | as L |
| A (now) | none | `camps_lost` 6, `ai_orders_per_100` 21.86, `levers_repeated` 3.10 |

The two fails are the cost L named and the owner merged. The orders warns are L adding claim
orders to the count (claim 15.98 of 21.86 a hundred turns), so the band no longer measured what
its note says. Health (panics, digests, centres) never moved. The owner's "CI would have caught
nothing" (owner.md §5) holds: the bands blocked one deliberate change and caught none by
accident. They hold the game where it stood (relay/retro/m2/players.md said so).

### The three options

| Option | Keeps | Costs | Verdict |
|---|---|---|---|
| Change now and merge | player.md, judge.md, record, ledger | resolve 10 hunks, then delete most of what they resolve; still a PR of mostly docs with a stale judge.md | no: the code is the part to drop |
| Close | nothing | loses the player the owner likes and the trial's cost record m3-relay.md §5 depends on | no |
| **Remake into M4** | player.md, the record, ledger lines 1–2 | a docs-only change; the judge prompt rewritten | **yes** |

### Steps

1. Close PR 124 with a comment pointing at the M3 retro. Leave the branch.
2. In M4's Relay change (the hand-built change m3-relay.md's opening describes), take the
   paths that don't conflict:
   `git checkout origin/claude/wonderful-lamport-aduj1e -- relay/players/player.md relay/specs/101/play.md relay/specs/101/trial/g.lines.jsonl relay/specs/101/trial/judge-findings.md relay/specs/101/trial/judge.txt`
   and remove `relay/specs/101/split.md`'s line about where the held work is.
3. Drop `judge.toml`, the band code, the `toml` dependency, the `check.sh` line, the CLAUDE.md
   band lines, `bands.md` and `bands-*.txt`. bands.md's one durable fact goes in a line of
   play.md: halving the gift leaves health untouched and moves every score by a fifth or more,
   so the scores see an economy change and health doesn't.
4. Add a `## Decided` to `vision.md` with lines 1 and 2 (Standing counts clans and units; the
   half-point lead margin): PR 118 flagged both as describing merged code. Strike line 3 (owner.md §5).
5. Update player.md for what M3 added: land, the guard, armies and archers, pillage, fights,
   the scout's orders (§4 below).
6. Replace `judge.md` with the review-time rubric (§3); keep its four problem kinds for the play
   note, where they worked.

## 3. The review-time judge

The owner wants a judge "with taste" that "confirms the game has got better by the measures that
matter to the game, not to an engine" (owner.md §5). Taste here is a written rubric applied to
numbers a command has already compared, so the model's job is judgement, not arithmetic.

### What it reads

1. **`sansfiat judge --compare main.json pr.json`** (to build): every metric of the two
   `sweep --judge --json` runs side by side, with the change and a noise floor per metric, and
   the change flagged only past it. The floor comes from a **null perturbation**: the PR's sweep
   once more with one harmless difference at the pick (an extra rng draw, or the pick a turn
   later), so drift is measured on every run instead of found by hand. About 20 s more CPU
   (estimated from the 40 s sweep).
2. **The piece's question and control** from the issue, and its stories' runs against their
   controls (`story … --no-X`, `judge FILE`).
3. **The play note** from the play step (§4): one played game, `judge FILE` beside the AI
   control and beside an idle led civ (below).
4. Not the code. The review has read it.

### Taste, as rules

The judge calls a change **better** only when all of these hold:

- The piece's question moves past the noise floor in the score it names, against its control,
  on more than one retro map.
- The move is in the first four scores in their order (Standing, doubt, decisions, motion), and
  health stays in band. Camps kept and clans joined are machinery (process.md, Scores).
- The mechanism is the one the piece claims. A Standing gain because a rival starved, or
  because a thinner guard eats less (PR 142's skirmish), is **indirect**, and the judge says so.
- The lever was exercised on the retro maps. A rule that fires 0 times there (fights, captures,
  capitals falling on main) is **unexercised**, not better.
- A led player beats an idle led civ by more than the floor. Otherwise the lever is dead,
  whatever the AI-only sweep says.

It refuses to call good: moves within drift; totals that move because a count's meaning changed
(L's claim orders); a single built map; a gain only in AI-against-AI play; anything it can't
point at a number for. Praise isn't a finding (relay/retro/m2/players.md).

### What it writes

One block at the top of the PR description, readable on a phone, and the full table in
`relay/specs/<n>/judge.md`:

```
Judge: better | same | worse | not shown
Standing  0.100 → 0.096 (floor 0.010)  same
Doubt     12 on 5 → 10 on 6 (floor 2)  same
...       question: skirmish Red +2.9 vs --no-fight (floor 1.0)  better, indirect
Unexercised: fights 0, capitals fallen 0 on the AI maps
```

A verdict of worse or not shown doesn't block the merge; the owner reads it at the merge, which
is his stop (process.md, The owner's hours).

### Where it runs, model and cost

**Inside the review, not a new step.** Every M3 review already ran the sweep and wrote the
answer, on Opus medium: #116 $0.58 (Sonnet high), #123 $1.04, #132 $2.07, #140 $1.60, #149
$1.73; 10–22 minutes, 4–10 5-hour points, 0–1 weekly (the issues' relay blocks). With the
compare table doing the arithmetic, the judge section adds an estimated $0.3–0.6 and 2–3
5-hour points. A separate step would cost a cold read of the piece again (estimated $0.5–1)
for independence; try that only if reviews start praising their own fixes. The review's
"judging step isn't run at low effort" rule (#123, #132) already keeps it at Opus medium.

Relay order changes from m3-relay.md §5: **play before review**, so the review's judge reads
the played game. The full chain per piece: builds → play (Sonnet, about $0.2) → review with
judge (Opus medium, about $1.6–2.3). Estimated $0.5–0.9 and 3–4 5-hour points on top of an M3
piece's $12–22.

## 4. The play step for M4

| | Proposal | From |
|---|---|---|
| Model | Sonnet player, Sonnet judge pass at medium; a Haiku player on one piece as a trial | trial: $0.12 and $0.08, under 4 and 3 minutes (play.md) |
| Map | the retro map where the piece's lever applies, named by the spec: small seed 9 for land and armies (seed 2's guard starves at once, relay/specs/102/review.md) | |
| Turns | 100 from the pick, one game; a milestone check runs three maps | m3-relay.md §5 |
| Controls | `judge FILE` against all-AI (has) and against an idle led civ (`play` with `end 100`, no tokens) | my runs below |
| Writes | `relay/specs/<n>/play.md`: the four problem kinds with turn and evidence, the could-not-tell note, Standing against both controls | judge.md's form worked |
| Cost | about $0.2 list and 1 5-hour point (estimated from M3 reviews' roughly 5 points a dollar; the trial's points weren't recorded) | |

What player.md still needs:

- **The world as it is now**: land and its guard, armies of warriors and archers, pillage,
  fights, surrender, the scout's orders. It still says "a capital, a camp of warriors, a scout".
- **A rule to chase a dead number.** The trial's player raised its gift twice while "gave 0.0"
  and never asked why. The view now says when a gift can't reach a clan, but on my idle game
  (small seed 2, Red, +100) it shows `keep 15.0 in force`, a store of 10.8 and `gave 0.0` with
  no line saying the keep is what stops the gift. The prompt should say: a number that stays at
  0 after an order gets `why` on its subject before the next order.
- **Try every lever family once**: the trial never raised (judge-findings.md); 103 of 108 turns
  had no order.
- **The could-not-tell note per lever**, not one at the end.

### Findings into pieces without the owner (owner.md §6)

A play note's finding that names a missing or wrong reason, a dead lever or a refused order the
view offered, with evidence, opens an issue labelled `finding` with the piece's number. The
crew opens a fix piece from it automatically when it touches only the view, the page or a reason
(no rule, no number in `data.rs`), and the same finding appears in two played games. Anything
that changes a rule goes to the next retro's list instead. This keeps the owner out of "the view
doesn't say why" and in "should the gift work this way".

## 5. The AI as the owner's opponent

### What it does (`core/src/units/ai.rs`, `claim.rs`, `civs.rs`)

Each turn, for each civ not led: send its stand-in claim when it differs; its guard rule (lose a
person after a hungry turn, else raise one while a claimed tile is thin and the stores hold
`GUARD.keep_turns` turns); raise a camp of `UNIT.start` if it has none; post it up to
`UNIT.post` steps toward the nearest rival capital, **stopping at the edge of its own land**;
zone its line home. Its gift stays at founding's `gift_default` 2.0 and it never sets keep,
pillages, raises archers or a second army, or marches out. Hence the sweep's zeros in §1.

### What a led player faces

Measured: a led civ given no order for 100 turns from the pick, `judge FILE` against the same
map with every civ on the AI:

| Map, civ | Idle led | AI playing it | Rank |
|---|---|---|---|
| small 2, Red | 3.6% | 2.8% | 3 / 3 |
| small 9, Blue | 13.0% | 12.9% | 2 / 2 |
| medium 9, Gold (civ 3) | 2.5% | 3.5% | 2 / 3 |

Within about a point either way: the AI's orders are worth about as much as none. Nobody marches
on the led civ; no tile of its claim is taken (0 in all six games); its losses are hunger. On
small seed 2 the idle Red's feed shows warriors disbanding from hunger at turns 239–241; at +100
its guard is 0.0 people, its claim is 24 tiles with 1 held, and the default gift never reached anyone (`since founding gave 0.0`).

### Is "a game" answerable before R?

No. M3's question needs a led game in doubt (m3.md, R) and a way to lose. Today the led civ's
Standing moves only with its own economy and the rivals' growth; the only doubt is which AI
grows faster, and nothing ends. What the judge and the player can answer before R: whether each
lever moves Standing for a led civ beyond drift and beyond idle. That is the bar M4's pieces can
use until R lands.

## Integration tests (owner.md §1.5)

For the `architecture` angle to design. How they relate: the stories (`sansfiat story`) are
already scripted units on a real map through the real engine; an integration test is a story
with an assertion, run in CI in seconds. They catch "the scout never went near the mountains"
(owner.md §1.3), which no score sees. That is where CI earns its time, not bands: tests say a
rule does what it says on a real map; the judge says whether the game got better; the play
note says what a player couldn't tell.

## For other angles

- **game:** idle led civ against the AI-played civ, three maps, within about a point (table in
  §5). Strike vision.md ledger line 3; take lines 1–2 from PR 124. The view's keep line doesn't
  say the keep stops the gift.
- **cost:** M3 reviews $0.58–2.07, 10–22 minutes, 4–10 5-hour points (#116, #123, #132, #140,
  #149). The trial's plan points weren't recorded; play.md says the crew holds them.
- **architecture:** integration tests as stories with assertions; `judge --compare` and the null
  perturbation live in `cli/src/judge.rs`.
- **prompts:** player.md describes M2's world; the review prompt needs the judge rubric and the
  compare table; play moves before review.
- **crew:** a `finding` label and the rule for auto-opening view-only fix pieces; play before
  review in the step order.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| Close PR 124; carry player.md, the trial record and ledger lines 1–2 by hand; drop the bands | §2, owner.md §5 | high | small | take |
| Strike "an army mustn't starve its own side" | owner.md §5 | medium | none | take |
| `judge --compare A.json B.json`: old, new, change and floor per metric | §3 | high | small | take |
| Null-perturbation run for a measured noise floor | PR 142's Gold, §3 | high | small | take |
| Judge rubric inside the review (better / same / worse / not shown, phone block) | owner.md §5, §3 | high | small | take |
| Idle led civ as a second control in `judge FILE` | §5 runs | high | small | take |
| Play step before review, Sonnet, the spec names the map | m3-relay.md §5, play.md | high | small | take |
| Update player.md for land and armies; chase-a-dead-number rule; try each lever | trial, §4 | high | small | take |
| `finding` issues; auto-open view-only fix pieces | owner.md §6 | medium | medium | try |
| Haiku as the player on one piece | m2 players | medium | small | try |
| Stories on two retro maps, not one | PR 142, stories.md | medium | small | take |
| Separate judge step for independence from the reviewer | §3 | low | medium | later |
| CI bands on the scores | PR 124 | low | small | drop |
| Integration tests: scripted units on real maps, asserted in CI | owner.md §1.5 | high | medium | take (architecture designs) |
| The view says when keep stops the gift | idle game, §4 | medium | small | try |
| Milestone play check on main, three maps, before the owner plays | m3-relay.md §5 | medium | small | later, once R makes a led game contested |
