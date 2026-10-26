# M1 retro

#31 (The map and the page) and #32 (Founding). Five sessions each read them from one angle; their
findings, with the evidence, are in `relay/retro/m1/`: `cost.md`, `briefs.md`, `crew.md`,
`prompts.md`, `game.md`. What M2 does about them is in [m2.md](m2.md).

Both pieces ran clean: 14 steps, each finished in one session, with no crash, timeout, block or
compaction. The longest session was 23 minutes and context peaked at 19%. $25.68 at list, and the
week went from 9% to 25%. What went wrong was one ordering bug in the crew, pacing that left the
crew idle far longer than it worked, and a founding whose civs can't feed their one warrior.

## Process

### 1. Spec-review's step changes are lost

The crew makes every step from `steps.json` as soon as the spec closes (`relay.mjs:751`), before
spec-review starts. On #32, spec-review split the page step into build-3 and build-4; build-4 was
never made, and the review wrote its 240 lines unbriefed and then judged its own work. #31's
spec-review edit to a `why` was dropped the same way.

The split itself was wrong: it split "to fit 90 minutes" when the whole page in #31 took 14. And
the spec told spec-review where to look: #32's spec-review `why` reads "checks that the decisions
match `founding.md`", and that is exactly where it then missed the disband going to a rival civ.

Spec-review earned its place on #31: the whole-map re-roll fired on 5 of 40 small seeds, the
pooled hidden-country floor kept a test from failing, and `opt-level = 3` cut `cargo test` from
1m50s to 10s. On #32 it caught three real things and buried them under edits the compiler would
have made.

### 2. Pacing

#31's sessions ran 110 minutes; the piece took 22 hours. #32's ran 53 and took 8.5. The waits:

- Day 1's weekly limit is 13.6%. #31 stopped at 14% and sat 14 hours.
- The 5-hour stop at 50% is checked between steps, so one Opus step overshoots it (44% → 62%).
- The schedule starts 2.5–5 hours late, so every budget stop costs at least one slot.

The 50% stop stays: an Opus build takes about 20% of the window in a session elsewhere, and the
weekly window is the real limit. Raising it to 90% for one run was an exception to get #32 to
review. Both are now in `relay/README.md`.

### 3. The board

11 of 14 step ends failed to write to it. The crew sets each field in turn inside one `try`, with
Status last, and the unrounded cost (`0.9322537999999999`) fails first, so Status never moves
either. Nothing else is wrong with it. M2 keeps it and fixes it.

### 4. Cut

- **Draft PRs.** CI runs on drafts anyway, so `draft: true` and `markPullRequestReadyForReview` do
  nothing.
- **`tokens`** in the step comment and status. 87–97% is cache reads, so it reads as size and
  isn't. Cost is the size signal.
- **Prompt no-ops** (`prompts.md` §8): "Commit work that passes its checks as you go", the
  prototype `npm test` line, the pinned `wasm-bindgen-cli` version, "(not committed)", "the
  retro reads it", and the crew's closing "Do this one step" line.

### 5. Models and effort

The three Opus sessions cost $11.65 of $25.68, and the two measured took 18 points each of the
5-hour window. #31's build-2 and build-3 fixed their briefs by measuring; Sonnet's build-4 did
the same kind of fix for $1.89. #32's Opus spec still got six fixes from a Sonnet spec-review.
Nothing in either piece shows Sonnet failing where Opus then succeeded. All 14 steps ran at
high, Haiku never ran, and #32's whys copy #31's rather than choose. So M1 can't say what
medium, low or Haiku would have done.

A bug: the piece's `Spec:` line is read to the end of the line, and both were hard-wrapped, so the
spec step's `why` was cut mid-sentence.

### 6. What a session writes

- **Summaries.** #31's build-2 and build-3 put river tables and ASCII maps in their summaries
  (20 KB each), so "This piece so far" was 58 of build-5's 68 KB. Build-4 wrote its sweep to
  `evidence.md` and stayed at 7 KB.
- **Decisions** turned into a hand-off channel ("Card fields are snake_case in JSON").
- **The review checks against a spec the builds rewrote.** Builds correcting the spec worked,
  but nobody judged whether 11 deltas in 20 is "most".

### 7. Checks

`www/check.mjs` isn't in CI. The App can't push workflows, so #31's build-5 was refused, and the
gap was handed to you four times. We can push from here.

### 8. Engine chores

- Four sessions each wrote their own headless-Chrome driver in `/tmp` and read 35 full-size
  screenshots (17 MB, the biggest outputs in M1).
- #32 asked no question about the main game, so nobody ran a civ for long, and the starving
  warriors went unmeasured even though build-2 saw them.
- `engine/CLAUDE.md` is 11 KB and grows about 20 lines a piece, mostly Layout rows restating
  signatures.

### Not taking

- A `design: true` flag on decisions and a crew step that deduplicates ideas into the PR body
  (`game.md` §1, §3). Rewording decisions does the first. For the second, the retro reads the
  ideas already.
- A review rule to run every per-turn loop for N turns. That belongs in the proposal: a piece that
  adds an income or a cost names the question ("can a civ keep a warrior 200 turns?"), and the
  spec measures it.
- The crew refusing a long summary. Try the wording first.

## Game

### Civs can't feed their one warrior

`game.md` ran the engine on main from the pick for 200 turns, on seven maps with all three civs
on the AI: **14 of 21 civs lost their warrior to hunger**, the first 22 turns after the pick. The
levy brings in 1–3 food a turn and the store spoils 10% a turn, so it holds about 10 against
one warrior; a second drains it. Three sessions said so as an idea.

`founding.md` already promises the fix: the capital "works the tiles next to it". The spec drew
those tiles and nothing works them. Every unit M1 still has to build (scout, army, supply) eats
from this store.

There's a weaker signal under it: civ clans shrank after founding on 3 of 5 maps while unaligned
clans grew, perhaps because the levy takes their buffer. It needs a run with no levy before
anyone acts on it.

### Your play notes

- **Territory.** The page paints a tile for the first clan by id whose round or edge touches it
  (`map-view.js` `setDynamic`). So clan 2's edge covers clan 6's own home after the division on
  small seed 0, turn 18.
- **Rounds over mountains.** The page draws a round as straight lines between its tiles, but each
  leg walks around impassable ground.
- **Penned clans never move on.** On small seed 1, clan 7 starts starving 58 times in 230 turns,
  never three turns running, because one fed turn resets `hungry` (`clans.rs:232`).

### Design edits

Proposals for you to make or reject; each is in `game.md` with its evidence. You took the Units lines in `founding.md` (made with a full pack, eats from it, disbands to its own civ) and the rename of map.md's section to "Hidden country"; the rest stay in the code.

- **`founding.md`**
  - The store's numbers as starting values: cap 50, levy half, spoil 10%, order spoil → levy →
    feed → cap.
  - "Empty at founding" replaced by why the endowment exists.
  - Disbanding goes to the nearest clan **of its civ** (the review's fix).
  - Making a warrior costs no food. A warrior holds no land, blocks nothing and stacks freely
    until the army piece.
  - A civ's land is its capital's territory plus its clans' rounds. #27 builds on that, so it's
    worth settling before its interview.
  - The store feeds units and takes the levy without carrying, as a stand-in until supply lines.
- **`map.md`**
  - The numbers the builds settled: crossing +2, marsh 3, floodplain fertility 2.2, floodplain
    coast becomes plains.
  - Mountains impassable.
  - Starving clans grouped into one feed line.
- **`scouting.md` against `founding.md`**: does a civ start with a scout, a warrior or both? It has
  to be settled before #28.

## Your questions

### What sessions write, and where

A session's deliverable is code. The prose it's asked for is `spec.md`, the briefs and
`steps.json` from the spec; `result.json` (`summary`, `decisions`, `ideas`) from every step; and
the PR description from the review. The crew then copies it around:

| Text | Written by | Where it goes | Who reads it |
|---|---|---|---|
| `spec.md`, briefs, `steps.json` | spec; corrected by spec-review and the builds | `relay/specs/<n>/` on the branch | Each build gets its brief. The review gets its brief and the whole spec |
| `summary` | every step | The step's hidden block, a ✅ comment on the step with stats added, a comment on the PR with decisions and ideas added, and **every later step's prompt** | Later steps, you |
| `decisions` | every step | The hidden block, the PR comment, every later prompt | Later steps, you |
| `ideas` | every step | The PR comment | You |
| Step's model, effort and `why` | spec | The step issue's body, under "Step `build-2` of #32" | You, if you open the step |
| PR description | review | The PR | You |
| Blocked, merged and review-done notes | crew | The piece issue | You |
| Run report | crew | Relay log, #12 | You |

**The two messages you saw are one summary**, posted twice: with stats on the step, with
decisions and ideas on the PR. Nobody needs it on the step.

**Why they write so much.** `session.md` names three readers for `result.json` and gives no size.
So the summary turns into a commit log: #32's build-2 summary is about 2,300 characters listing
every function it added, and it goes into four later prompts.

**What's handed on** is the branch (commits, the spec folder) and every earlier step's summary
and decisions. A build's prompt, #32's build-2 at 24 KB, is: one paragraph on what a Relay
session is; its brief (about 200 lines); "This piece so far", the earlier summaries and
decisions; `build.md` and `session.md`; and "Do this one step". The brief does the work. "This
piece so far" is where it bloats: 58 of 68 KB in #31's build-5.

**Briefs as the step's body** cost the sessions nothing. They never read the issue, and the brief
is in their prompt already.

### The ASCII in #31's build-2

It wasn't Opus looking for a job. Its brief said "shown in your result as ASCII … put the table
in your result". The spec put the evidence in the wrong place, and the build did as it was told.

### Were the Opus builds specced into a corner?

Partly, yes. #31's spec ran on Sonnet and wrote build-2 as pseudocode: the distance field, where
rivers start, the walk, the tie-breaks and every starting number. Opus kept the algorithm and
fixed what broke when it ran: the sources (hills only gave small maps no long river) and the
delta threshold (12 to 10). It was implementing Sonnet's design, not designing.

Whether it would have done better unspecced, M1 can't say. Every miss in both pieces was in a
number or rule the spec chose before any code ran, and every one was found by running it. No
spec on any model can measure what doesn't exist yet. So I'd put the problem less as the
apprentice planning for the master and more as the plan going further than planning can: the
spec wrote the how for work that's only judged by measuring.

Where I'd challenge the framing: the plan's other parts held up. The order of steps, the
interfaces between them, and the tests worked on both pieces, and spec-review's catches on #31
came from reading those against the code. That's planning work, and Sonnet did it well.

### Usage data

Not enough yet. The crew probes the 5-hour and weekly usage before each step and writes it only
to `crew.log`, which expires with the run's artifact after 14 days. The probe reads the whole
account, so your own sessions in the same hours land on Relay's steps. Measured from M1: Opus
took about 5 five-hour points per list $, Sonnet about 7, and both about 0.7 weekly points per $.

### Worth playing

At https://dontpanic345.github.io/sansfiat/engine/:

- `?size=small&seed=2`: *Run to end* stops at the pick. Lead Red, make a warrior, march it across
  a river, then watch Blue and Gold's warriors starve. Does the feed tell you why?
- `?size=medium&seed=8`: make several warriors and watch the store. Is "n / 50" worth showing
  when it never nears 50?
- `?size=large&seed=8`: at fit zoom, can you find the rivers and the hidden country?
