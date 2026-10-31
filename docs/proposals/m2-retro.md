# M2 retro

M2's six pieces: #54 (A), #29 (B), #55 (D), #28 (C), #56 (F), #57 (E). Eight sessions each read them
from one angle. Their findings and evidence are in `relay/retro/m2/`: `cost.md`, `briefs.md`,
`crew.md`, `prompts.md`, `game.md`, `architecture.md`, `players.md` and `history.md`. What M3 does
about them is in [m3.md](m3.md).

Relay ran clean: 35 steps and every piece merged. No step crashed, timed out or compacted, and the
largest context any session used was 31%. M2 cost $48.31 at list, about $8 a piece against M1's $12.84,
and the week went from 44% to 67%. What went wrong is the game, not the relay. Every piece passed
its own question, but after the pick the world stands still and the player has nothing to decide.

## Game

### A frozen world

Measured on main, on the seven retro maps with every civ on the AI (`game.md` §1):

- **The pre-game is sound.** 7 of 7 maps end by three centres on their own, with people levelled off.
- **After the pick, nothing moves.** The world's people change under 3% in 300 turns. On medium 8 no
  civ's clans, people or joins change from turn +100 to +300.
- **The AI never cuts a line**, never sends its scout anywhere and never answers a cut. 7 of 21 AI
  scouts starve within 8 turns of the pick, sitting at their capital.
- **Joining is a lever, but you set it once.** Gift 6 draws 14.7 clans against rivals' 4.5, and it
  saturates there. Joins are over by +100.
- **Two regressions went unseen until now.** Camps kept 200 turns fell from 21/21 (#29) to 15/21
  after #75. Joins at the default gift fell from 183 (#55) to 84. No step re-runs an earlier
  piece's question.

What the game needs, in order: a goal, an AI that uses its army, a world that keeps moving after
the pick, and units that survive their first errand.

### Your play notes

`game.md` §4 has each note with its cause in the code. In short:

| # | Note | Cause | Proposal |
|---|---|---|---|
| 1 | Clan routes through desert | Desert costs 1 to cross, like plains | Desert costs 2. Foraging on the walk would break "every walk costed the same way" |
| 2 | A market has a radius | A leftover M1 ring on the page (`map-view.js:144`); the engine uses path cost | Drop the ring; selecting a festival shows who walks there |
| 3 | Clan 10 flips on and off its festival | A margin for switching festivals, none for stopping (`markets.rs:95`) | Stickiness both ways |
| 4 | Festivals few and far | Spacing 8, reach 20 | Spacing 5–6, reach 12–14, swept against the pre-game ending |
| 5 | No trail under homes; every worked tile trailed | Home never counts as walked; any tile walked 10 turns gets one. 72–79% of held tiles are trailed | Trails by traffic: people-walks per tile, home included, above a threshold |
| 6 | Clans join outside the civ's ring | Joining is by leaning at the capital's festival; the 2-step ring is an M1 stand-in no design file defines | You define a civ's land |
| 7 | Can't move the camp; terms muddled | Right-click only, said only when idle | A "Move to…" button. Unit = camp or scout; warriors are the camp's people |
| 8 | A unit panel at the bottom | Per-kind panels on the right | `page.md`: one panel shape, one button row |
| 9 | Scout starves in the sealed valley; route fades | Forages only its own tile; `strip` zeroes the tile; no refill; route clears when done; nobody walks it | The camp's switches, forage round it, strip only what's taken, refill, keep the trail while it's used, raise a new scout |
| 10 | Space should end the turn | No key for it | Add it |
| 11 | Gold circles and a thick ring at the capital | Dead M1 "work tiles", and the civ's 2-step ring | Remove the circles; keep the ring only once a civ's land is defined |

### Questions for you

These need your call before M3's pieces can be written:

1. **What is a civ's land?** Line blocking, forage vs pillage and the AI's post all use the 2-step
   ring today.
2. **May people and clans leave a civ** through festival moving and merging? They do now, and
   `founding.md` calls it out of scope. #57 alerts on it.
3. **The first way to win.** Influence (share of the world's people) is countable today and gives
   joining a purpose.
4. **The scout's pace:** the camp marches at one person's pace, the scout on its whole labour. Pick one rule.

`game.md` §3 lists every M2 decision by design file, marked take, reject or ask, and §5 has 14
design edits with their evidence.

## Process

### 1. Shapes fit five pieces of six

A's Clay and Kiln, closed by a Canvas, held up best: B's camp and F's zone are built on `Actor`,
`divide` and `Walks`. Every canvas held its piece's hard part. C, as Henry Ford, didn't fit. A port's rules
were settled but its numbers weren't, and its one unsettled item was proved only on built test
maps. Build-2 made 8 design decisions, and the review found scouts starve on real maps (`briefs.md` §1).

### 2. Effort mattered more than role

The coin always paired an Opus medium with an Opus low, so it couldn't tell spec from spec-review.
What it did show: depth followed effort. Medium sessions took 18–37 turns, read the code and ran
sweeps. Low sessions took 6–14 turns, in under 3 minutes. The medium spec-reviews made M2's
biggest catches. F's contest rule would otherwise have left no line able to be cut, and C's
measured trail costs before any build ran. Design is 28% of the cost (`cost.md` §3, `briefs.md` §2).

### 3. Haiku saves nothing at these sizes

Haiku did every settled step it was given, including C's build-2, which had been scored up to
Sonnet. But it cost the same per step as Sonnet ($0.71 vs $0.74) and used as many 5-hour points
per $, with 3–4× the output and twice the clock. Within a model, points predicted nothing: the
same score ran $0.43 to $4.28. Size is missing from the scores. All six off-ladder steps went
*up*, and none had a control (`cost.md` §2).

### 4. The review is blind to the designer's plan

The review gets the steps table without each step's `why`, so the checks designers put there were
lost: D's and E's reviews skipped the play checks. Four reviews ran 3–4 minutes and changed nothing.
F's, which played the merged piece, was the only one to change code. All six designers scored the
review 9 (`briefs.md` §3).

### 5. Pacing

Sessions ran 18% of M2's 35 hours. A big build-1 hit the 5-hour stop on 4 of 5 working runs. The
six-hourly schedule fires about three times a day, up to 5.5 hours late, so a stop cost 6–11 hours,
not 5. The weekly ramp never bound (`crew.md` §2).

### 6. Crew faults

- `relay/runs/result.json` was force-added by a #28 build and merged. #57's review committed only
  that file, which raced CI, so the crew left the merge to you. It's untracked now.
- Your reply on #28, "carry on with build-2", made build-1's Haiku session build build-2 too.
  The real build-2 then ran empty.
- A missing brace in `select.js` broke main's page after #75. CI runs no page code.
- The Pages build has failed since #94: `tools/site.sh` globbed for prototypes that no longer
  exist. Fixed.

### 7. What sessions read

The M1 fixes held: summaries stayed small and every build ran `check.sh`. What's left
(`prompts.md`):

- The spec's and spec-review's summaries and decisions are 31% of a build prompt, repeating
  `spec.md`.
- A third to half of build decisions are implementation notes.
- `engine/CLAUDE.md` doubled to 10.4 KB.
- 13 of 18 builds re-read their own brief.

### 8. The code

The actor model held as a design but not as a module (`architecture.md`). `actors.rs` was never
touched after #59. What the camp and scout share went into `units.rs`, now 1,559 lines and 23% of
core, with spoiling written four times and eating three. Briefs said "reuse the camp's code in
`units.rs`", and sessions read that as "same file".

## Ideas, ranked

Across the eight angles: about 140 ideas, of which these 20 are worth doing next. Value over
cost; each angle's file has the rest with its own call.

| # | Idea | From | Where |
|---|---|---|---|
| 1 | `sweep --judge` in CI: each piece's question as a check, against its control | players, game | M3 piece |
| 2 | Spec-review at Opus medium every piece; never judge at low | briefs, cost | Relay |
| 3 | Untrack `result.json`; crew refuses if tracked; review waits for its own CI; merge retries | crew | Relay |
| 4 | Restart a run when the 5-hour window resets; start a step only if its expected points fit under 80% | crew, you | Relay |
| 5 | A goal: Influence | game | Design, then M3 |
| 6 | The AI uses its army: contests, pillages, cuts and answers a cut | game | M3 piece |
| 7 | Scouts that live: switches, forage round, refill, raise again, keep a used trail | game, you | M3 piece |
| 8 | Festival stickiness both ways | game, you | M3 piece |
| 9 | Units: one vocabulary, a Move button, a bottom panel, space ends the turn, no dead circles or rings | game, you | M3 piece |
| 10 | Split `units.rs`; spoil, eat and walk on `Actor`; one event constructor | architecture | M3 piece |
| 11 | The review's must-plays live in `spec.md`; the review isn't scored | briefs | Relay |
| 12 | `soFar` drops the spec's and spec-review's decisions; trim `engine/CLAUDE.md` to 6 KB | prompts | Relay |
| 13 | Prove every unsettled item on real maps; run a quick sweep at design time | briefs | Relay |
| 14 | `sansfiat play`: a text view and a JSON action loop; one hand-run LLM player and judge | players | M3 piece |
| 15 | Trails by traffic | game, you | M3 piece |
| 16 | Good and bad years | history, game | M4 |
| 17 | Fixed-source goods (salt, obsidian, shells) at festivals; salt slows spoiling | history | M4 |
| 18 | Syntax-check `www/*.js` in CI | game, crew | Relay |
| 19 | A brief names the module that should grow, not just the code to reuse | architecture | Relay |
| 20 | One step a piece one rung *below* the ladder, replacing M2's challenge | cost, briefs | Relay |

### Not taking

- **Raising the 5-hour stop to 80% outright.** #4 gates on what the next step is expected to take instead.
- **Cutting the board.** It works now, and costs nothing.
- **A vision-model player** driving the page by screenshots. The text view is cheaper and catches more.
- **Shells or beads as currency.** As an ornament good with its own demand, later, they fit vision rule 2;
  as a unit of account they break the game's premise (`history.md`).
