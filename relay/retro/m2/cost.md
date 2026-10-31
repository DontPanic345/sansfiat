# M2 retro: models, effort and cost

Covers pieces A #54 (PR #59), B #29 (#68), D #55 (#82), C #28 (#75), F #56 (#89), E #57 (#94): 35 step
issues, 41 sessions. Sources: the `<!-- relay -->` block on each step issue (cost, minutes, usage
before/after), `relay/specs/<n>/steps.json` (scores, whys), and the stream-json transcripts and
`crew.log` of relay runs 40691846214, 40781134477, 40825502720, 40857386196 and 40968311410 (28-30 Oct;
every M2 session ran in these five; the other runs were pacing no-ops). "5h" is the 5-hour window's
points, "wk" the weekly's. Both are whole numbers and include the owner's own use in the same hours.
Peak context is the largest single turn in the transcript as a share of the 1M window. Output tokens
and turns are summed over a session's result frames (a session can wake more than once).

## Verdict

- M2 cost $48.31 list for six pieces ($8.05 a piece, M1: $12.84), 382 session-minutes, 277 5-hour points,
  23 weekly points (44% to 67%). One piece is about 46 5-hour points and 4 weekly points.
- The ladder's points picked the model, and the model picked the cost. Within a model they did not
  predict anything: five-point steps ran from $0.43 to $4.28 and 3 to 27 5-hour points.
- Haiku finished all seven of its steps and was not cheaper. It cost the same per step and per point as
  Sonnet, used 3-4x the output tokens and about twice the minutes.
- All six off-ladder steps went up (two to Opus medium, four to Sonnet medium). Every one finished in a
  session and none has a control. The two Opus medium steps cost $10.23 and the four Sonnet medium
  steps $5.99; nothing shows the lower rung would have failed.
- Unchecked: weekly points are whole numbers (23 in all), the owner's own use is inside every usage
  figure, E build-1's 5h points are estimated, and the "idle gap" and tool-call counts come from the
  transcripts I downloaded (five runs) without re-reading the sessions themselves.
- The coin has nothing to tell apart. Every piece had one medium and one low, in opposite roles, so a
  role's effect cannot be separated from the piece. What it does show: medium costs 3-5x low in both
  roles and does 3-5x the work, a medium spec-review is where the sweeps and structural fixes came from,
  and a low spec-review still caught a real problem each time but read very little.
- M3: spec Opus low and spec-review Opus medium, fixed, the planner raising the spec to medium for a
  foundation. Spend the experiments on the builds, not the designers (section 4).

## 1. Every step

Scores are Open/New/Miss/Importance/Reach from the step's `why` (they sum to Points in every case).
Ladder is what the points and part give by `relay/prompts/spec.md`. Sessions is how many times the step
ran. Spec and spec-review ran by coin, so they have no points.

| Piece | Step | Part | Scores | Pts | Ladder | Ran | Off? | Sess | List $ | Min | 5h | wk | Peak ctx | Cmp | How it went |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A | spec | | | | coin | opus medium | | 1 | 2.80 | 10.6 | 22 | +1 | 16% | 0 | Clay and Kiln, 5 builds, 15 decisions; 53k output tokens |
| A | spec-review | | | | coin | opus low | | 1 | 0.34 | 1.0 | 2 | 0 | 4% | 0 | Read 5 things, ran the sweep; per-size bars, one brief line |
| A | build-1 | clay | 2/3/1/3/2 | 11 | opus low | opus medium | up | 1 | 5.66 | 19.5 | 41 | +3 | 24% | 0 | Actor model in; kilns recorded no clay fixes. Run stopped at 69% |
| A | build-2 | kiln | 0/1/1/1/2 | 5 | haiku high | haiku high | | 1 | 0.50 | 10.1 | 3 | 0 | 17% | 0 | Depletion, trails, taming; no problems |
| A | build-3 | kiln | 0/3/1/1/2 | 7 | sonnet medium | sonnet medium | | 1 | 0.74 | 5.9 | 5 | 0 | 9% | 0 | Festivals, people moving, merging; no problems |
| A | build-4 | kiln | 2/1/1/0/1 | 5 | haiku high | haiku high | | 1 | 0.71 | 10.0 | 4 | +1 | 19% | 0 | Page territories, archive; 90 turns, 11 decisions |
| A | build-5 | canvas | 4/1/1/1/2 | 9 | opus low | opus low | | 1 | 2.76 | 25.4 | 13 | +1 | 13% | 0 | Tuned the pre-game: 30/30 end by centres. Changed rules the brief didn't name |
| A | review | | 2/1/3/1/2 | 9 | coin | sonnet high | | 1 | 0.68 | 4.2 | 4 | 0 | 11% | 0 | No code change; verdict merge |
| B | spec | | | | coin | opus low | | 1 | 0.80 | 2.5 | 4 | +1 | 7% | 0 | Easel and Canvas, 3 builds |
| B | spec-review | | | | coin | opus medium | | 1 | 1.52 | 4.5 | 8 | 0 | 10% | 0 | 7 decisions; moved the levy to the harvest beyond need |
| B | build-1 | easel | 0/1/1/1/2 | 5 | haiku high | sonnet medium | up | 1 | 4.28 | 27.0 | 27 | +2 | 24% | 0 | Rewrote units, founding, levy across core; one pass, 139 turns. Run stopped at 71% |
| B | build-2 | easel | 0/1/1/0/1 | 3 | haiku medium | haiku medium | | 1 | 0.79 | 10.4 | 4 | 0 | 20% | 0 | Camp on the page, sweep, archive; no problems |
| B | build-3 | canvas | 4/1/3/1/2 | 11 | opus low | opus low | | 1 | 2.51 | 27.5 | 13 | +1 | 12% | 0 | 21/21 camps hold. Changed the levy again (the harvest-beyond-need share was always 0), added CIV.pull |
| B | review | | 2/1/3/1/2 | 9 | coin | opus low | | 1 | 0.56 | 3.0 | 3 | 0 | 6% | 0 | No code change; verdict merge |
| D | spec | | | | coin | opus medium | | 1 | 1.79 | 5.6 | 9 | 0 | 13% | 0 | Easel and Canvas, 3 builds |
| D | spec-review | | | | coin | opus low | | 1 | 0.33 | 0.9 | 2 | 0 | 4% | 0 | Flagged CIV.store_cap 20 leaves 5 food spare; left to build-1 |
| D | build-1 | easel | 0/1/1/1/2 | 5 | haiku high | sonnet medium | up | 1 | 0.98 | 9.8 | 7 | +1 | 12% | 0 | One pass, replay and cross-build green |
| D | build-2 | easel | 2/1/1/0/1 | 5 | haiku high | haiku high | | 1 | 0.43 | 9.3 | 3 | 0 | 17% | 0 | Leaning panels; no problems |
| D | build-3 | canvas | 4/1/1/1/2 | 9 | opus low | opus low | | 1 | 0.58 | 7.1 | 3 | +1 | 5% | 0 | One number: store_cap 20 to 60, which spec-review had flagged. Answered the question |
| D | review | | 2/1/3/1/2 | 9 | coin | sonnet high | | 1 | 0.45 | 3.1 | 3 | 0 | 9% | 0 | No code change; verdict merge |
| C | spec | | | | coin | opus low | | 1 | 0.63 | 2.6 | 3 | +1 | 5% | 0 | Henry Ford, 3 builds |
| C | spec-review | | | | coin | opus medium | | 1 | 1.83 | 7.7 | 10 | 0 | 11% | 0 | Built a scratch copy, ran the pre-game sweep; 5 decisions, rest_run/walkable/scout fixes |
| C | build-1 | ford | 0/0/1/1/2 | 4 | haiku medium | haiku medium | | 2 | 1.71 | 34.0 | 11 | +1 | 27% | 0 | Attempt 1 ($0.36, 9 min) blocked on a spacing test its change broke; owner replied; attempt 2 ($1.35, 25 min) fixed it, then built the scout |
| C | build-2 | ford | 2/1/1/1/1 | 6 | haiku high | sonnet medium | up | 1 | 0.07 | 1.9 | 1 | 0 | 3% | 0 | Nothing left: build-1's second session had built it |
| C | build-3 | ford | 0/1/1/0/1 | 3 | haiku medium | haiku medium | | 1 | 0.52 | 8.8 | 4 | +1 | 17% | 0 | Scout page; no problems |
| C | review | | 2/1/3/1/2 | 9 | coin | sonnet high | | 1 | 1.11 | 10.2 | 7 | 0 | 12% | 0 | Found scouts starve across country; verdict not merge; owner pushed 4 commits and merged |
| F | spec | | | | coin | opus low | | 1 | 0.45 | 1.6 | 2 | 0 | 4% | 0 | Magnum, 1 build |
| F | spec-review | | | | coin | opus medium | | 1 | 1.01 | 3.7 | 5 | +1 | 7% | 0 | Ran the sweep; contest rule (first holder wins meant an AI line could never be cut) |
| F | build-1 | magnum | 4/3/1/1/2 | 11 | opus low | opus medium | up | 1 | 4.57 | 20.1 | 24 | +2 | 19% | 0 | One pass, 6 decisions. Run stopped at 61% |
| F | review | | 2/1/3/1/2 | 9 | coin | opus low | | 1 | 2.28 | 23.8 | 11 | 0 | 12% | 0 | Merged #75, fixed what broke (scouts, AI zone starving, a missing brace on main), played the cut. Verdict merge |
| E | spec | | | | coin | opus medium | | 1 | 1.93 | 5.3 | 10 | +1 | 12% | 0 | Henry Ford, 2 builds |
| E | spec-review | | | | coin | opus low | | 1 | 0.30 | 0.8 | 2 | 0 | 4% | 0 | Added a fourth digest pin, best-of-three bench |
| E | build-1 | ford | 0/1/1/1/2 | 5 | haiku high | haiku high | | 1 | 1.71 | 47.2 | n/a | +1 | 31% | 0 | 127 turns, 24 min active and a 21-minute idle gap; digests unchanged. 5h window reset mid-step, so no points |
| E | build-2 | ford | 2/1/1/0/1 | 5 | haiku high | sonnet medium | up | 1 | 0.67 | 14.1 | 5 | +1 | 10% | 0 | One pass; play-read passes |
| E | review | | 2/1/3/1/2 | 9 | coin | opus low | | 1 | 0.31 | 2.7 | 2 | 0 | 4% | 0 | No code change; verdict merge |

No session compacted. The largest peak context is 31% (E build-1, Haiku); the longest session is 27.5
minutes against the 90-minute cap. The block's `attempts` is 0 on every step, including C build-1 which
ran twice; the Sess column comes from `crew.log`.

### Totals

| Piece | Shape | List $ | Min | 5h pts | wk (first to last) | Opus $ | Sonnet $ | Haiku $ | Design share ($) |
|---|---|---|---|---|---|---|---|---|---|
| A | Clay and Kiln | 14.19 | 87 | 94 | 44 to 50 | 11.56 | 1.42 | 1.21 | 22% |
| B | Easel and Canvas | 10.46 | 75 | 59 | 50 to 55 | 5.39 | 4.28 | 0.79 | 22% |
| D | Easel and Canvas | 4.56 | 36 | 27 | 57 to 61 | 2.70 | 1.43 | 0.43 | 46% |
| C | Henry Ford | 5.87 | 65 | 36 | 55 to 60 | 2.46 | 1.18 | 2.23 | 42% |
| F | Magnum | 8.31 | 49 | 42 | 61 to 64 | 8.31 | 0 | 0 | 18% |
| E | Henry Ford | 4.92 | 70 | 19 + about 10 | 64 to 67 | 2.54 | 0.67 | 1.71 | 45% |
| All | | 48.31 | 382 | 277 (about 287) | 44 to 67 | 32.96 | 8.98 | 6.37 | 28% |

C and D ran interleaved, so their weekly ranges overlap; use the totals. Design share is spec plus
spec-review. E's missing 5h is estimated from its $ at Haiku's rate.

| Model | Sessions | List $ | Share of $ | Minutes | 5h pts | Pts per $ (median) | Pts per $ (aggregate) |
|---|---|---|---|---|---|---|---|
| Opus | 20 | 32.96 | 68% | 176 | 189 | 5.2 | 5.7 |
| Sonnet | 8 | 8.98 | 19% | 76 | 59 | 6.7 | 6.6 |
| Haiku | 7 | 6.37 | 13% | 130 | 29 (6 sessions) | 6.2 | 6.2 |

By role: spec $8.40, spec-review $5.33, builds $29.19 (Opus $16.08 on 5 steps, Haiku $6.37, Sonnet $6.74),
reviews $5.39.

### M1 against M2

| | M1 (2 pieces) | M2 (6 pieces) |
|---|---|---|
| List $ per piece | 12.84 | 8.05 |
| Session-minutes per piece | 79 | 64 |
| Sessions per piece | 7 | 5.8 |
| Opus share of $ | 45% | 68% |
| Opus 5h pts per $ | about 5 | 5.2 |
| Sonnet 5h pts per $ | about 7 | 6.7 |
| Weekly pts per piece | 8 | 3.8 |
| Longest session, peak context | 23 min, 19% | 27.5 min, 31% |
| Efforts | all high | low and medium throughout |

The cost per point is where M1 found it, so list $ is still a fair size signal across models (Opus
slightly dearer per $ than the table implies, 1.2x Sonnet, down from 1.4x). Pieces got cheaper because
there are fewer, smaller non-Opus sessions and no high effort, not because Opus use fell: Opus is more of
the bill than in M1, and four Opus sessions (A build-1, F build-1, A build-5, B build-3) are $15.50.

## 2. Did the points predict?

**Not within a model.** Rank correlation of points with list $ is 0.42 over the 16 builds that have
both, but that is the model choice: among the 11 non-Opus builds it is -0.27. Five-point steps:

| Step | Model | $ | Min | 5h |
|---|---|---|---|---|
| A build-2 (kiln rules) | haiku high | 0.50 | 10.1 | 3 |
| A build-4 (page) | haiku high | 0.71 | 10.0 | 4 |
| D build-2 (page) | haiku high | 0.43 | 9.3 | 3 |
| E build-1 (engine reasons) | haiku high | 1.71 | 47.2 | n/a |
| E build-2 (page) | sonnet medium | 0.67 | 14.1 | 5 |
| D build-1 (engine) | sonnet medium | 0.98 | 9.8 | 7 |
| B build-1 (units rewrite) | sonnet medium | 4.28 | 27.0 | 27 |

What the points leave out is volume. B build-1 rewrote units, founding and the levy across most of core,
and scored the same 5 as D build-1's trader. Reach (0-2) is about what a change touches, not how much
there is to write. The reviews all score 2/1/3/1/2 = 9, so for them the points carry nothing and the coin
is the whole decision.

**The ladder's top bites hard and was predictable.** Opus medium on a clay and on a magnum took 41 and 24
5-hour points; Sonnet medium on B build-1 took 27. Those three produced three of the four budget stops
(69%, 71%, 61%). Resumption waited 9.1, 5.7 and 10.7 hours.

**How well it went tracks the part, not the points.**

- Clay and Kiln held: the three kilns recorded no fix to the clay, and the camp (B build-1) went on the
  actor as an embedded field. The clay was the most expensive step in M2 and what the piece needed.
- Each canvas answered its question by measuring and changed something its easel could not see (A: the
  end rule and six numbers; B: the levy's form and CIV.pull; D: store_cap). Opus low was enough for all
  three, including D's, which ran 7 minutes and $0.58.
- The magnum finished in one session; the review that followed it ($2.28) was where the real bugs were.
- Ford and easel steps at 3-6 points finished with no review finding against them.

**Haiku.** It ran seven steps: A build-2 and build-4, B build-2, D build-2, C build-1 and build-3, E
build-1. All finished; one blocked once.

- C build-1 attempt 1 blocked on a real failing test (its change moved a capital 7 steps from another).
  That was a correct block. The owner replied with a fix to the rules, and attempt 2 then carried on and
  built the scout, which was build-2's work. The review found no fault with the scout as a build.
- No review found a defect that traces to a Haiku step. Reviews of A, B, D and E changed no code.
- E build-1 was the largest: 127 turns, 154k output tokens, 31% context, $1.71, and 47 minutes on the clock
  (24 of them active; a 21-minute gap sits at 23:44-00:05 in the transcript).
- Haiku wrote 58k to 154k output tokens a step (median 61k, more than half thinking). The Sonnet builds that
  were not rewrites wrote 16k to 32k. So per step Haiku cost what Sonnet did (median $0.71 against $0.74),
  and each Haiku page step ($0.43 to $0.79) took about 9 to 10 minutes against Sonnet's E build-2 at $0.67
  and 4 of active time on a bigger page. Per list $, Haiku bought 6.2 points and Sonnet 6.7.
- Haiku took 34% of session-minutes for 13% of the $.

M2's challenge ("at least one Haiku step per piece") has been answered: Haiku works on settled work at
3-5 points. It did not save money or points, so "cheaper" in M3 is not a matter of putting Haiku in more
steps.

**Off the ladder.** All six went up one rung (or two). None went down.

| Step | Ladder to run | Expected | What happened |
|---|---|---|---|
| A build-1 clay | opus low to opus medium | Uses most of the 90 minutes; leaves an actor interface with no fixes beyond small; kiln fixes comparable with B's | 19.5 min, $5.66, 41 5h pts, one session, check.sh green. Kilns recorded no clay fixes. As expected, except the time (22% of the cap). No control, so medium's part is unknown. It forced the first stop |
| B build-1 easel | haiku high to sonnet medium | Sonnet lands it in one pass with replay and cross-build right; Haiku would need a second attempt | One pass, $4.28, 27 min, 139 turns. Not tested against Haiku. Scope, not model, made it the dearest non-Opus step |
| D build-1 easel | haiku high to sonnet medium | Same as B | One pass, $0.98, 9.8 min. Not tested |
| C build-2 ford | haiku high to sonnet medium | Sonnet lands the scout in one pass; Haiku would miss a camp-only assumption and loop | Haiku's second build-1 session built the scout, review found it sound, and the Sonnet session found nothing to do ($0.07). The one test of the claim went against it, by accident |
| F build-1 magnum | opus low to opus medium | One attempt in 90 minutes, check.sh and cross-build green, at most one change to a Settled rule | 20.1 min, one attempt, green, 6 decisions, none changing a Settled rule. As expected; no control |
| E build-2 ford | haiku high to sonnet medium | One attempt; play-read passes; screenshots read cleanly where Haiku would leave the map too busy | One attempt, passes, $0.67, 14.1 min. Not tested |

Five of the six expectations were met and none could fail the ladder, because the ladder's choice never ran.
The "where Haiku would likely need a second attempt" claims were made three times and tested once, by
accident, and Haiku passed. Cost of the six bumps: not separable from a control; the Sonnet four came to
$5.99 and the Opus two to $10.23.

## 3. The coin

| Piece | Spec | Spec-review | Spec $ / min / out tokens | Review $ / min / out tokens | Review tool calls | Review decisions | Pair $ |
|---|---|---|---|---|---|---|---|
| A | medium | low | 2.80 / 10.6 / 53k | 0.34 / 1.0 / 3.2k | 5 | 1 | 3.14 |
| D | medium | low | 1.79 / 5.6 / 29k | 0.33 / 0.9 / 3.0k | 6 | 1 | 2.12 |
| E | medium | low | 1.93 / 5.3 / 28k | 0.30 / 0.8 / 3.0k | 6 | 2 | 2.23 |
| B | low | medium | 0.80 / 2.5 / 13k | 1.52 / 4.5 / 26k | 23 | 7 | 2.32 |
| C | low | medium | 0.63 / 2.6 / 12k | 1.83 / 7.7 / 34k | 28 | 5 | 2.46 |
| F | low | medium | 0.45 / 1.6 / 8k | 1.01 / 3.7 / 19k | 17 | 5 | 1.46 |

- **Medium costs 3-5x low in both roles.** Specs: $2.17 against $0.63 on average. Reviews: $1.45 against
  $0.32. Medium specs wrote 28-53k output tokens to low's 8-13k, but their spec.md files are only 1.4x
  larger (11.8-20.4k characters against 7.4-13.2k); the rest was thinking.
- **The pairs cost about the same.** Medium-spec pairs average $2.50, low-spec pairs $2.08. Since each
  piece had one of each, the coin never changed what a design cost much.
- **The low spec-review reads little.** Every low review ran 5-6 commands in about a minute (spec,
  briefs, a few greps, one patch) and wrote about 3k tokens, though `spec-review.md` says to read the code
  first. Medium reviews ran 17-28 commands: read the code, built a scratch copy, ran sweeps (C: the
  pre-game with real trail costs; F: `--after 200` on the retro maps).
- **What each found.** Medium reviews: B (the levy didn't touch growth), C (a built trail would wear away
  the turn it finished; walkable; the scout misread from `scout/`; its sweep let build-1 only re-run),
  F (first-holder-wins would make an AI line uncuttable, so the review's required cut was impossible).
  Low reviews: A (per-size speed bars), D (store_cap 20 leaves 5 food spare, which is what build-3 then
  had to change), E (watched digests never run the player's orders; bench best of three). The medium ones
  were structural; the low ones were real and smaller, and D's was named but left to a build.
- **What the medium reviews did not stop.** B's levy fix was wrong again (build-3 found clans' surplus is
  always 0) and C's scouts starve across country (found by the review). Both are measurements, which the
  canvas and the review make; no spec-time reading gets them.
- **Outcomes don't separate.** All six merged, five of them on a merge verdict from the crew. Reviews with a
  low spec-review (A, D, E) and with a medium one (B, C, F) both changed no code except F's. F's
  fixes came from merging #75 into it, not from the plan.
- **What can't be told apart:** whether a medium spec beats a low spec (a low spec was never read by a low
  review), whether a medium review beats a low one on the same spec, and whether it matters at all.
  There were no (medium, medium) or (low, low) pieces, three pieces a side, and piece size, shape and
  design vary together. Designer cost is $1.46-$3.14 a piece, and was 18-46% of it.
- **The second coin (the review, Sonnet high or Opus low at 9 points).** Sonnet: $0.68, $0.45, $1.11
  (4.2, 3.1, 10.2 minutes). Opus low: $0.56, $2.28, $0.31 (3.0, 23.8, 2.7 minutes). Five of six changed no
  code. The one that did (F) was the one with a merge to do and a page to play.

## 4. M3: what replaces the coin

**Recommendation: fix the designers at spec Opus low and spec-review Opus medium, and let the planner raise
the spec to medium on the piece line when the piece is a foundation (a clay, or an actor model like A).
Vary the builds, not the designers.**

- Designers are cheap: $13.73 of $48.31 (28%), and the swing between any choice here is about $1-2 a piece.
  Pushing them to Sonnet saves cents; the owner's "cheaper" lives in builds, where Opus is $16 of $29.
- The review is the reader that runs the code. Only the medium reviews built a scratch copy and measured
  before the builds started, and their finds (B, C, F) are the ones a build would otherwise hit. The
  spec writes the plan once, and a low spec was never the reason a build failed.
- A fixed pair gives M3's retro a control for the build experiments, which is what the coin was for in M2.
  A coin with no cross cell never answered anything.
- Raise the spec to medium for a foundation, by the owner or planner on the piece line (A's spec took 52k
  tokens and 10.6 minutes for the model everything stands on), and say so in the line, as the ladder does
  for steps.
- Keep the spec from choosing spec-review's model (M2's rule); the piece line does.

Alternatives and why not:

- *Fixed Opus medium both:* about $1.5 a piece more, no evidence it buys anything, and no cheaper push.
- *Designer ladder with the five scores:* the scores are per step. A piece-level version (Open, Importance)
  would be a guess on three or four data points; the planner's line does the same job.
- *Designer picks spec-review's model:* reverses M2's intent (the reviewer looks where the designer
  didn't), and designers would pick low for their own pieces.
- *Sonnet designer on a Ford piece* as a probe: possible once (C and E cost $2.46 and $2.23), but it saves
  under a dollar and makes the control worse. Later.

Make the review a rule too: no coin. Default Sonnet high; Opus low when `main` has moved since the spec
(F's fixes) or the piece's goal is judged by play. That is the review's own Miss-and-Reach question, and the
scoring table gives 9 to every review.

## 5. The balance

**Where more expensive would have paid.**

- **A play/judge step, not a dearer model.** The only review that changed the code (F, Opus low, 23.8 min,
  $2.28) merged main and played the cut. Five other reviews read and ran the checks and found nothing. If
  builds outrun the owner's testing, the spend that pays is a step that plays the merged piece (automatic
  players and judges are on the table). Cost: a review-sized session, $1-2.5, 10-25 minutes, 5-12 5h pts,
  about +20% a piece.
- **Spec-review at medium when a sweep can settle something.** The sweeps in C and F changed the builds
  before they ran (C's build-1 became a re-run, F's contest rule). Low reviews on D and E named issues but
  left them to builds. A rule in `spec-review.md`: run the cheap measurement the spec defers, when the
  piece asks a question or has a canvas.
- **Nothing else.** No step failed for want of a model. The three canvases and the clay were Opus and
  answered; the Sonnet and Haiku builds finished one-pass.

**Where cheaper would have done.**

- **The clay and the magnum at Opus low (the ladder).** The bumps cost $10.23 and 65 5h pts, and
  drove two of the stops. No control says medium helped. Try the ladder as written once.
- **Haiku's saving is not in the model.** At 3-6 points Haiku and Sonnet cost the same, and Haiku takes
  twice the clock. A cheaper push that might work is effort: Sonnet low for settled page work, and Haiku
  medium for what the ladder gives Sonnet medium (7-8). Nothing in M2 ran either.
- **Reviews at Sonnet medium** when main hasn't moved and there's nothing to play: five of six changed
  nothing. Saves under a dollar, so low priority.
- **A canvas that moves one number (D build-3, $0.58)** could be tried at Sonnet high; saves little.

**Proposed ladder changes** (`relay/prompts/spec.md`):

1. Replace M2's "at least one Haiku step" with "at least one step one rung below the ladder (cheaper), and
   its `why` says what it expects". All six M2 off-ladder steps went up, so the retro has no comparison.
   Going up stays allowed when the `why` names what the lower rung would miss.
2. Add a size line to the `why` for any step touching more than a handful of files (a rewrite across
   core), and tell the planner to expect $3-5 and 25+ minutes from those. Points set the rung; size sets
   the bill. Keep the five scores; don't make a sixth.
3. Cap clay and magnum at Opus low in the table unless the points reach 12. Medium is for 12+ only.
4. Reviews: no coin. Sonnet high by default; Opus low when main moved or the goal is judged by play.
5. Reviews score 9 every time; drop their scoring or score the piece's play risk instead.

## 6. Points per $, and what a piece takes

| | 5h pts per $ (median) | 5h pts per $ (aggregate) | Minutes per $ | Typical step |
|---|---|---|---|---|
| Opus | 5.2 | 5.7 | 5.3 | $0.3 to $5.7 |
| Sonnet | 6.7 | 6.6 | 8.5 | $0.45 to $4.28 |
| Haiku | 6.2 | 6.2 | 20.4 | $0.43 to $1.71 |

Small steps are noisy: a $0.30 session reads 2 points and rounds to a third of its value either way.
Use the aggregates. Weekly points per $ overall: 23 over $48.31 is 0.48 (M1: 0.62).

| Piece type (M2) | List $ | Minutes | 5h pts | wk pts |
|---|---|---|---|---|
| Median piece | 7.1 | 67 | 39 | 4 |
| Cheapest (D, E) | 4.6-4.9 | 36-70 | 19-27 | 3-4 |
| Dearest (A) | 14.2 | 87 | 94 | 6 |
| Average of six | 8.05 | 64 | 46 | 3.8 |

- **A 5-hour window:** the 50% gate stopped runs four times (69%, 71%, 55%, 61%) after a heavy step. One
  step can take 25-40 points, so a window holds one heavy step and a few light ones; a median piece is
  about three-quarters of a window, A is two, D is a quarter.
- **The week:** 100 weekly points is about 12 windows' worth (12 5h pts per weekly pt). At 3.8 a piece
  that is 26 pieces a week if the owner used none. Weekly ran 44% to 67% over 35 hours, nowhere near the
  ramp (67.9% on day 5), and never stopped a run; the 5-hour gate did. With the owner using half the week,
  plan on about 13 M2-sized pieces, and treat 6-8 as comfortable. The weekly figures are whole points
  over 23 in all, so they are good to a point or two per piece.
- **Wall clock is the 5h gate's, not the work's:** A took 10.5 hours from PR to merge for 87 session-
  minutes. Four stops cost 9.1, 5.7, 3.2 and 10.7 hours of waiting.

## For other angles

- **crew:** the block's `attempts` is 0 for every step, including C build-1 (two sessions) and the PR
  steps table; the review row has blank cost and minutes because the table is written before the review
  runs. `fiveHour` is null when the window resets mid-step (E build-1: before 23, after 0), losing the
  step's points; usage values still carry floats such as 28.000000000000004. C build-1's first, blocked
  session is summed into its cost and minutes with no mark.
- **crew:** the stops follow heavy single steps; A's stop at 69% held the piece's remaining five steps for 9
  hours.
- **prompts:** `spec-review.md` says to read the code first; at Opus low the review reads five or six
  commands' worth. The review's scoring in `spec.md` gives 9 every time.
- **briefs:** the review-by-role finds table above is the cost angle's view; the structural fixes are
  B's levy, C's three, F's contest rule.
- **game:** C's review finding (scouts starve on 7 of 16 capital-to-capital routes) was not fixed; B's levy
  rule changed twice; D's store_cap went 20 to 60.
- **players:** F's review (23.8 min, $2.28, played the cut) is the template for a judge step.

## Ideas

| Idea | Value | Cost | Rec |
|---|---|---|---|
| Fix designers at spec Opus low, spec-review Opus medium; planner raises spec to medium for a foundation | medium | small | take |
| Drop the review coin; Sonnet high default, Opus low when main moved or play judges the goal | medium | small | take |
| Replace the Haiku challenge with a step one rung below the ladder, with an expectation in the `why` | high | small | take |
| A play/judge step after the review (automatic players), $1-2.5 and 10-25 minutes | high | medium | try |
| Spec-review at medium runs the sweep the spec defers | medium | small | take |
| Cap clay and magnum at Opus low unless 12+ points; run one clay at low as the control | medium | small | try |
| Try Sonnet low on settled page work; Haiku medium at 7-8 | medium | small | try |
| Add a size line to `why` and expected cost/minutes by part to the planner's notes | medium | small | take |
| Keep Haiku only where wall time is free (settled engine rules, pages); it is no cheaper | low | small | take |
| Drop the review's scoring or score play risk | low | small | take |
| Sonnet high canvas for a canvas that is only a number search (D build-3) | low | small | later |
| Sonnet designer on a Ford piece as a probe | low | small | later |
| Record 5h points across a window reset (use the max, or probe at the reset) | low | small | take (crew) |
| Count sessions, not `attempts: 0`, in the block and the PR table | low | small | take (crew) |
| Plan weekly budget at 4 weekly pts, 46 5h pts and $8 a piece (A-sized: 6, 94, $14) | medium | small | take |
