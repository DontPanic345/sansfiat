# M3 retro: the ladder

Covers pieces H #100 (PR 107), G #101 (PR 118, held part PR 124), L #102 (PR 126), P #103 (PR 134)
and A #104 (PR 142): 38 step issues (#106–#149, less the PRs), 39 sessions. R #105 is unbuilt.
Sources: the `<!-- relay -->` block on each step issue (cost, minutes, usage before and after, peak
context, compactions, summary, decisions); `relay/specs/<n>/steps.json`, `spec.md`,
`spec-review.md`, `review.md`; the PR descriptions; and the stream-json transcripts and
`crew.log` in the artifacts of relay runs 41196527672 (H, G), 41364842651 (L spec to build-3),
41391671599 (L build-4 to P build-3), 41425463303 (P build-4 to A build-2), 41467598377 (A
build-3 to build-5's first session) and 41515434828 (A build-5's second session, review). Every
M3 session's transcript is in these six. Diff sizes are from the GitHub commits API on each
PR's step commits, engine files only, additions plus deletions, images and JSON left out.

"5h" is the 5-hour window's points and "wk" the weekly's. Both are whole numbers and include the
owner's own use in the same hours; that share can't be separated out. Peak context is the
block's figure, the largest turn as a share of the 1M window.

## Verdict

- M3 cost $71.40 list, 769 session-minutes, 434 5-hour points and 33 weekly points (summed by
  step; weekly 2% to 39% over the milestone, so 37 counting the owner's split of G between
  pieces). Five pieces, 23 builds, 38 steps. Against m3.md §3's expectation for these five
  pieces ($67, 365, 32, 20 builds): $ is 7% over, 5-hour points 19% over, weekly on. A is the
  overshoot: one Sonnet high kiln took 49 5-hour points.
- A piece costs $14.28 and 87 5-hour points (M2: $8.05, 46). M3's pieces are bigger, not
  pricier per build: $1.79 a build against M2's $1.72, and Opus is a smaller share (58%
  against 68%).
- The ladder picks the model, and the model sets the cost band. Within a model the points
  don't predict cost: Haiku high at 4–5 points ran $0.17 to $3.98. What does is the piece: H's
  settled refactor ran its seven Haiku fords for $3.07 in all; Haiku at the same points on L, P
  and A ran $1.49–3.98 each.
- The size line was written on every build and is right in rank (estimated lines against
  actual, rank correlation 0.93) but low in size (actual about 1.6x the lines and 1.7x the
  files). It predicts minutes a little and cost hardly at all.
- The rung-below experiment: five of five steps finished in one session and met their done.
  The two Sonnet medium steps at 8 points (L build-1, P build-1) did everything the brief
  asked. One defect shipped from a rung-below step: A build-4 (Haiku medium) made "Disband…
  all" a count from 1, which is owner.md §1.2. Take 8 points to Sonnet medium; the Haiku rung
  has too little evidence to move.
- Spec-review earned its $1.98 a piece (10 5-hour points). Every piece's spec-review changed
  something that would have failed a build or the proof as written. Keep it.
- H's Sonnet high spec was as good as the Opus ones on a settled piece, for $1.72 against
  Opus's $2.04–3.78. Its visible slip was scoring off the scale, which spec-review fixed.
- Reviews caught one bug in code (H's AI post onto a pass) and fixed small things. They missed
  both owner findings: the army's Disband all and the scout's search. No review presses every
  new button.

## 1. Every step

Scores are Open/New/Miss/Reach from the `why`. Ladder is what `relay/prompts/spec.md` gives for
the points and part; for spec, spec-review and the review it is m3-relay.md §2's rule. Ran is
the block's model and effort. Off? is the direction off the ladder. Sess is sessions.

| Piece | Step | Part | Scores | Pts | Ladder | Ran | Off? | Sess | List $ | Min | 5h | wk | Ctx | Cmp | How it went |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| H | spec | | | | opus medium | sonnet high | down (probe) | 1 | 1.72 | 10.2 | 13 | 1 | 18% | 0 | Henry Ford, 7 builds (the proposal said 3); built `baseline/` and `compare.sh` from main; scored two steps New 2, which the scale lacks |
| H | spec-review | | | | opus medium | opus medium | | 1 | 2.25 | 12.6 | 12 | 1 | 12% | 0 | Ran check.sh, compare.sh and main's AI instrumented on five runs and the sweep; four fixes; re-scored b3 7→4 and b4 6→4 (Sonnet to Haiku); found play-camp and play-lean fail on main |
| H | build-1 | ford | 0/0/0/2 | 2 | haiku medium | haiku medium | | 1 | 0.33 | 11.3 | 2 | 0 | 14% | 0 | 28 event sites, bench recorded; 28 files, 997 lines |
| H | build-2 | ford | 0/1/1/2 | 4 | haiku high | haiku high | | 1 | 0.62 | 12.1 | 4 | 0 | 19% | 0 | units.rs into five files with a serde mirror; 3,454 lines; no slip |
| H | build-3 | ford | 0/1/1/2 | 4 | haiku high | haiku high | | 1 | 0.17 | 7.7 | 2 | 1 | 12% | 0 | Actor behaviour and the civ's mirror; 708 lines |
| H | build-4 | ford | 0/1/1/2 | 4 | haiku high | haiku high | | 1 | 1.09 | 17.3 | 7 | 0 | 26% | 0 | Order and World::order; 1,109 lines; four wakes waiting on checks |
| H | build-5 | ford | 0/1/1/1 | 3 | haiku medium | haiku low | down (rung below) | 1 | 0.02 | 5.8 | 0 | 0 | 6% | 0 | The port in 16 turns and 13k output tokens; checks pass |
| H | build-6 | ford | 0/0/1/1 | 2 | haiku medium | haiku medium | | 1 | 0.42 | 10.4 | 3 | 0 | 16% | 0 | Page order calls; added a panel script that presses every button |
| H | build-7 | ford | 0/0/1/1 | 2 | haiku medium | haiku medium | | 1 | 0.42 | 21.8 | 3 | 1 | 16% | 0 | CLI split, 23 files, 2,246 lines; 18 outputs byte-identical |
| H | review | | | | sonnet high | sonnet high | | 1 | 0.58 | 12.3 | 4 | 0 | 9% | 0 | One real fix: the AI's post onto a built pass would panic under `expect`. Merge |
| G | spec | | | | opus medium | opus medium | | 1 | 2.04 | 9.0 | 11 | 1 | 13% | 0 | Easel and Canvas, 3 builds; probe.md measured main |
| G | spec-review | | | | opus medium | opus medium | | 1 | 1.10 | 2.9 | 5 | 0 | 8% | 0 | Tried a nested `claude -p` (not logged in) and an Agent subagent (works); moved the trial to Agent; review to Opus medium |
| G | build-1 | easel | 0/1/1/1 | 3 | haiku medium | haiku medium | | 1 | 1.00 | 36.6 | 6 | 1 | 20% | 0 | Standing, the judge, sweep --judge; 104 turns, 43 of them Monitor or ScheduleWakeup on check.sh |
| G | build-2 | easel | 2/1/1/0 | 4 | haiku high | haiku medium | down (rung below) | 1 | 0.59 | 15.4 | 4 | 0 | 20% | 0 | `sansfiat play`, the view at 34 lines, `judge FILE`; one attempt |
| G | build-3 | canvas | 4/3/3/1 | 11 | opus medium | opus medium | | 1 | 2.80 | 18.3 | 15 | 1 | 14% | 0 | Sonnet trial player and judge ($0.22); found the gift-reach gap and fixed the view; bands from main, later held by the owner (PR 124) |
| G | review | | | | opus low | opus medium | up (crew refuses low) | 1 | 1.04 | 9.6 | 6 | 1 | 8% | 0 | No code change; played 20 turns. Merge. The owner split the PR |
| L | spec | | | | opus medium | opus medium | | 1 | 2.85 | 12.7 | 15 | 1 | 16% | 0 | Easel and Canvas, two easels; 29 settled items |
| L | spec-review | | | | opus medium | opus medium | | 1 | 2.20 | 6.2 | 12 | 1 | 13% | 0 | Ran the guard on small 2: at the spec's numbers it held nothing past the capital; new GUARD numbers, a leaving floor, six smaller fixes |
| L | build-1 | easel | 2/1/3/2 | 8 | sonnet high | sonnet medium | down (rung below) | 1 | 2.51 | 33.1 | 17 | 1 | 18% | 0 | Every target met; flagged that stickier festivals starve the store at the pick |
| L | build-2 | easel | 0/1/1/2 | 4 | haiku high | haiku high | | 1 | 3.98 | 39.1 | 21 | 2 | 49% | 0 | 47 files, 1,615 lines, 185 turns; the founding guard starves; corrected spec item 24 rather than raise the number |
| L | build-3 | easel | 2/1/1/1 | 5 | haiku high | haiku high | | 1 | 2.37 | 23.4 | 16 | 1 | 22% | 0 | Judge and page; handed the page half to a background Haiku subagent; reported play-hold and play-lean failing |
| L | build-4 | canvas | 4/1/3/2 | 10 | opus low | opus low | | 1 | 1.98 | 42.6 | 10 | 1 | 10% | 0 | Three rule changes made claiming pay (14 of 21); camps lost 2→18; left its two played games to the review |
| L | review | | | | opus low | opus medium | up (crew refuses low) | 1 | 2.07 | 11.5 | 10 | 0 | 13% | 0 | No code change; played wide claim against ring on seed 9. Merge |
| P | spec | | | | opus medium | opus medium | | 1 | 3.03 | 9.7 | 16 | 2 | 18% | 0 | Easel and Canvas, three easels by area; 26 touch points; a 573-line spec |
| P | spec-review | | | | opus medium | opus medium | | 1 | 1.94 | 6.3 | 10 | 1 | 11% | 0 | No command printed a save's digest (added `digest --game`); terrain bar 16 ms from measurement; play-hold fails on main; map share |
| P | build-1 | easel | 2/1/3/2 | 8 | sonnet high | sonnet medium | down (rung below) | 1 | 2.02 | 25.3 | 15 | 1 | 18% | 0 | 45 files, 3,373 lines; terrain 4.75 ms against a 16 ms bar; before and after shots match |
| P | build-2 | easel | 2/1/1/1 | 5 | haiku high | haiku high | | 1 | 1.49 | 18.4 | 10 | 1 | 30% | 0 | Screens, router, bar, autosave; 3,148 lines |
| P | build-3 | easel | 2/1/1/1 | 5 | haiku high | haiku high | | 1 | 1.84 | 25.0 | 12 | 1 | 34% | 0 | Panel and 13 scripts; map share 44% at laptop, so it ran layout with `--share 0` for the canvas |
| P | build-4 | canvas | 4/1/3/1 | 9 | opus low | opus low | | 1 | 1.87 | 16.4 | 10 | 0 | 10% | 0 | Map share 44%→73% with four layout changes; a played game's digest matches native |
| P | review | | | | opus low | opus medium | up (crew refuses low) | 1 | 1.60 | 21.5 | 7 | 1 | 10% | 0 | Fixed the end chart's axis. Merge |
| A | spec | | | | opus medium | opus medium | | 1 | 3.78 | 13.3 | 21 | 2 | 20% | 0 | Clay, Kiln, Glaze, 5 builds; measured stores and the scout starving on main |
| A | spec-review | | | | opus medium | opus medium | | 1 | 2.39 | 8.5 | 12 | 1 | 13% | 0 | Ran the retro maps through `play`; four rule changes (surrender at 3, to the cutter, guards don't retreat, the AI's claim skip) and the stack's staging |
| A | build-1 | clay | 2/3/1/2 | 8 | opus low | opus low | | 1 | 3.69 | 36.4 | 19 | 1 | 16% | 0 | fight.rs with presence, parts and counters; the kilns kept its invariants |
| A | build-2 | kiln | 0/1/1/2 | 4 | haiku high | haiku high | | 1 | 1.97 | 26.6 | 12 | 1 | 35% | 0 | Archers, militia, siege, surrender, the fall; 856 lines; caught a militia strength the brief's reading made a no-op |
| A | build-3 | kiln | 2/1/3/2 | 8 | sonnet high | sonnet high | | 1 | 5.10 | 83.0 | 49 | 3 | 33% | 0 | Scout, judge group, stories; the scout's done not met (13 of 14 errands lost); stories half play. M3's dearest step |
| A | build-4 | kiln | 2/1/1/1 | 5 | haiku high | haiku medium | down (rung below) | 1 | 1.98 | 20.0 | 16 | 1 | 32% | 0 | Page and army.mjs (5 s); shipped Disband… all as a count from 1; review renamed camp to army |
| A | build-5 | glaze | 4/1/3/2 | 10 | opus low | opus low | | 2 | 2.82 | 57.0 | 19 | 1 | 5% | 0 | Session 1 (41.8 min) ended itself "out of time" with army.mjs failing; session 2 (15.2 min) fixed it. No capital falls |
| A | review | | | | opus low | opus medium | up (crew refuses low) | 1 | 1.73 | 19.3 | 8 | 1 | 12% | 0 | Re-ran the stories with controls; Gold drifts a point; renamed camp to army on the page. Merge |

No session compacted. The largest peak context is 49% (L build-2, Haiku high); the longest
session is 83 minutes (A build-3) against the 90-minute cap. Only A build-5 needed two
sessions. The crew stopped on the 5-hour budget four times (after L build-3, P build-3, A
build-2 and A build-5's first session), each at 80–83%, and restarted at the reset 2.5–3 hours
later (`crew.log`).

### Totals against m3.md §3

| Piece | Shape | Builds (exp.) | List $ (exp.) | Min | 5h (exp.) | wk sum / range (exp.) | Opus $ | Sonnet $ | Haiku $ | Design $ |
|---|---|---|---|---|---|---|---|---|---|---|
| H | Henry Ford | 7 (3) | 7.62 (8) | 122 | 50 (45) | 4 / 4 (4) | 2.25 | 2.30 | 3.07 | 3.97 |
| G | Easel and Canvas | 3 (4) | 8.57 (12) | 92 | 47 (65) | 4 / 4 (6) | 6.98 | 0 | 1.59 | 3.14 |
| L | Easel and Canvas | 4 (4) | 17.96 (15) | 169 | 101 (80) | 7 / 8 (7) | 9.10 | 2.51 | 6.35 | 5.05 |
| P | Easel and Canvas | 4 (4) | 13.79 (12) | 123 | 80 (65) | 7 / 7 (6) | 8.44 | 2.02 | 3.33 | 4.97 |
| A | Clay, Kiln, Glaze | 5 (5) | 23.46 (20) | 264 | 156 (110) | 11 / 11 (9) | 14.41 | 5.10 | 3.95 | 6.17 |
| M3 | | 23 (20) | 71.40 (67) | 769 | 434 (365) | 33 / 37 (32) | 41.18 | 11.93 | 18.29 | 23.30 |

Expectations are m3.md §3's rows for these five pieces (R's $15, 80, 7 and 4 builds left
out). G is under because its builds were cheap Haiku easels and its review cheap; the owner's
split of G (1 Nov) isn't in any step and sits in the weekly range (10% to 13% between G and L).
Design $ is spec plus spec-review: 33% of M3 (M2: 28%).

| Model | Steps | Sessions | List $ | Share | Min | 5h | 5h per $ |
|---|---|---|---|---|---|---|---|
| Opus | 18 | 19 | 41.18 | 58% | 314 | 218 | 5.3 |
| Sonnet | 5 | 5 | 11.93 | 17% | 164 | 98 | 8.2 |
| Haiku | 15 | 15 | 18.29 | 26% | 291 | 118 | 6.5 |

By role: spec $13.42, spec-review $9.88, builds $41.08 (Opus $13.16 on 5, Sonnet $9.63 on 3,
Haiku $18.29 on 15), reviews $7.02.

### M2 against M3

| | M2 (6 pieces, 35 steps) | M3 (5 pieces, 38 steps) |
|---|---|---|
| List $ | 48.31 | 71.40 |
| 5h points | 277 (about 287) | 434 |
| Weekly points | 23 | 33 (37 by range) |
| Session-minutes | 382 | 769 |
| Per piece: $, 5h, wk | 8.05, 46, 3.8 | 14.28, 87, 6.6 |
| Builds per piece | 2.8 | 4.6 |
| $ per build | 1.72 (29.19 / 17) | 1.79 (41.08 / 23) |
| Opus share of $ | 68% | 58% |
| Haiku builds: steps, median $ | 7, 0.71 | 15, 1.00 (1.91 outside H) |
| Longest session, peak context | 47 min, 31% | 83 min, 49% |
| Sessions needing a second | 1 (C build-1) | 1 (A build-5) |

M3's pieces carried more builds and more code (P's three easels changed 9,376 engine lines; M2's
largest build was B build-1). A piece takes about 0.9 of a 5-hour window and 7 weekly points,
which at half the week for Relay is about seven pieces a week of points, but the four budget
stops cost 11 hours of wall clock in M3's 2.4 days of runs.

## 2. Did the ladder predict?

Rank correlations over the 23 builds (Spearman; sizes over the 20 builds whose `why` gives
lines):

| Over | n | Points ~ $ | Points ~ 5h | Points ~ min | Est. lines ~ $ | Actual lines ~ $ | Actual files ~ min | Output tokens ~ $ |
|---|---|---|---|---|---|---|---|---|
| All builds | 23 | 0.75 | 0.69 | 0.50 | 0.35 | 0.02 | 0.46 | 0.26 |
| Non-Opus | 18 | 0.78 | 0.79 | 0.51 | 0.53 | 0.26 | 0.69 | 0.71 |
| Haiku | 15 | 0.70 | 0.72 | 0.33 | 0.60 | 0.28 | 0.58 | 0.96 |

**Across models, points predict cost because they pick the model.** Within Haiku the 0.70 is
the piece, not the points. Haiku high at 4–5 points:

| Step | Pts | Engine files, lines | $ | Min | 5h | Output tokens |
|---|---|---|---|---|---|---|
| H build-3 | 4 | 11, 708 | 0.17 | 7.7 | 2 | 42k |
| H build-2 | 4 | 14, 3,454 | 0.62 | 12.1 | 4 | 91k |
| H build-4 | 4 | 19, 1,109 | 1.09 | 17.3 | 7 | 117k |
| P build-2 | 5 | 23, 3,148 | 1.49 | 18.4 | 10 | 165k |
| P build-3 | 5 | 24, 2,855 | 1.84 | 25.0 | 12 | 178k |
| A build-2 | 4 | 21, 856 | 1.97 | 26.6 | 12 | 142k |
| L build-3 | 5 | 21, 986 | 2.37 | 23.4 | 16 | 228k |
| L build-2 | 4 | 47, 1,615 | 3.98 | 39.1 | 21 | 250k |

H's fords move code under a byte-identical proof (`compare.sh`); nothing has to be decided.
L's and A's steps write new rules whose starting numbers turn out wrong on the real maps, and
the step runs sweeps to find that out. The ladder's scores don't see this: Open 0 and Miss 1
for L build-2, the same as H build-4. Haiku's cost tracks its own output tokens (0.96), which
is how much it thinks, and that rises with what the step has to work out, not with lines.

**Within Opus nothing predicts** (five builds: 11 points $2.80, 10 $1.98, 9 $1.87, 8 $3.69,
10 $2.82). The clay at 8 was the dearest.

**The size line was written and is right in rank, low in size.** Every build's `why` has one;
two canvases give files and no lines. Estimated lines against actual rank at 0.93, files at
0.71. Median actual over estimate is 1.6x in lines and 1.7x in files: tests, pinned digests,
`engine/CLAUDE.md` rows and the page check are left out of the estimate. The size line
predicts minutes better than cost (actual files ~ minutes 0.69 outside Opus) and doesn't
predict cost within Haiku. "Breadth costs minutes, not a bigger model" holds: L build-2's 47
files took 39 minutes on Haiku high and finished.

**Where the ladder's top bit:** A build-3 (Sonnet high, kiln at 8) cost $5.10, 83 minutes and 49
5-hour points, more than any Opus step. Its scores were Open 2 and Miss 3 in a kiln: it held
unsettled work (which map a story plays on, whether the scout lives), and its done (the scout
survives its first errand) wasn't met. The shape rule only flags a kiln at 11 or more.

## 3. The one-rung-below experiment

| Piece | Step | Ladder | Ran | Expected (`why`) | What happened | $, min, 5h |
|---|---|---|---|---|---|---|
| H | build-5 ford, 3 pts | haiku medium | haiku low | Port the 70-line script verbatim in one attempt | Did, in 16 turns; cross-build and the led baseline agree; the review found nothing | 0.02, 5.8, 0 |
| G | build-2 easel, 4 pts | haiku high | haiku medium | Grammar, resume and judge in one attempt; a view complete but plain, under 80 lines | Did: 34 lines at the pick, 48 at +100. The canvas's trial then found the view listed clans the gift can't reach. That came from the spec's wording ("in its market's reach"), not the build | 0.59, 15.4, 4 |
| L | build-1 easel, 8 pts | sonnet high | sonnet medium | Rules clean, targets in two or three sweeps; might stop at the first variant that passes | Every target met with a tuning table (`build-1-tuning.md`); flagged the store starving at the pick, which build-2 and the canvas then dealt with | 2.51, 33.1, 17 |
| P | build-1 easel, 8 pts | sonnet high | sonnet medium | Faithful move and the atlas; maybe terrain above the bar or a placeholder worse than flat colour | Terrain 4.75 ms against 16; shots match by eye; old scripts pass but the one failing on main | 2.02, 25.3, 15 |
| A | build-4 kiln, 5 pts | haiku high | haiku medium | Panels and feed land; may struggle to get army.mjs to a fight quickly | army.mjs reaches a fight in 5 s. But "Disband… all" is a count from 1 that sends `{type:'disband', n}` (`engine/www/panels.js` `disband`), where the spec's "whole army" is `n` none. The owner found it (owner.md §1.2). The review renamed the page's "camp" strings it left | 1.98, 20.0, 16 |

None failed or needed a second session. Each `why` said what the retro would learn if it
failed, so a failure would have been readable; none came.

What it can't say: there is no control. Nothing measures what the step would have cost a rung
up. Haiku medium (A build-4, $1.98) cost the same as Haiku high on the step before it (A
build-2, $1.97), and Haiku low's $0.02 is on the cheapest step of M3. The rungs tried below
were the cheap ones; the dear rung (Opus low on canvas, glaze and clay: $10.36 on four steps)
wasn't tried.

Is the rung below the ladder now? For 8 points, yes: both Sonnet medium steps did what Sonnet
high would be asked to, at $2.02–2.51, against Sonnet high's one step at $5.10 (a different
step, so not a control). For the Haiku rungs, one clean step and one shipped defect isn't
enough; the defect came from an ambiguous brief line ("plus all") that Haiku high could have
read the same way.

## 4. Haiku

Haiku ran 15 steps: every ford, five easels and two kilns. All finished in one session.

**A build-2 (#145, Haiku high kiln, 4 points).** Built archers and bows, militia, the pressed
capital, siege, surrender, the fallen capital and every rule that skips a fallen civ: 21 files,
856 lines, with tests for each and the invariants kept. Its decision that militia fight at the
same strength attacking as defending caught a brief reading that would have made the rule a
no-op. What later steps changed of it: the glaze widened `line::cut_by_rival` and stopped
surrender while marching or fighting, both from running the stories, which is the glaze's job.
The review found nothing in it. The `why` expected it to miss a reader of the capital in the
fall; no step reports one.

**A build-4 (#147, Haiku medium kiln, 5 points, the rung below).** Built the page for armies:
stacks drawn with their size, Raise and Disband by kind, the scout's switches, the new feed
kinds, Why for a fight, and army.mjs. The defects: Disband… all as above, and "camp" left in
the feed and hints, which the review renamed. Glaze session 1 broke army.mjs by a rule change,
not a page fault.

**Against Sonnet steps of similar size:**

| Step | Model | Engine files, lines | $ | Min | 5h | Output tokens | Turns |
|---|---|---|---|---|---|---|---|
| A build-2 (#145) | haiku high | 21, 856 | 1.97 | 26.6 | 12 | 142k | 113 |
| A build-4 (#147) | haiku medium | 10, 510 | 1.98 | 20.0 | 16 | 127k | 138 |
| L build-2 (#129) | haiku high | 47, 1,615 | 3.98 | 39.1 | 21 | 250k | 185 |
| L build-1 (#128) | sonnet medium | 30, 711 | 2.51 | 33.1 | 17 | 62k | 98 |
| P build-1 (#136) | sonnet medium | 45, 3,373 | 2.02 | 25.3 | 15 | 66k | 56 |
| A build-3 (#146) | sonnet high | 29, 1,173 | 5.10 | 83.0 | 49 | 121k | 132 |

Outside H, a Haiku build costs about what a Sonnet medium build does ($1.49–3.98 against
$2.02–2.51) and takes as many 5-hour points (10–21 against 15–17), while writing two to four
times the output tokens. As in M2, Haiku isn't cheaper per step on new work; it is much
cheaper on settled refactors (H: $0.02–1.09). It finishes, and on #145 its judgement was sound.

L build-3 (Haiku high) handed the page half of its brief to a background Haiku subagent
(transcript, run 41364842651). The work landed and checked; the block's cost includes it.

## 5. Designers

### Spec

| Piece | Model | $ | Min | 5h | Output tokens | Settled items | Spec-review's changes |
|---|---|---|---|---|---|---|---|
| H | sonnet high | 1.72 | 10.2 | 13 | 61k | everything but naming | 4 fixes, 2 steps re-scored down a model |
| G | opus medium | 2.04 | 9.0 | 11 | 37k | 20 | 2 (the trial's sessions, the review's effort) |
| L | opus medium | 2.85 | 12.7 | 15 | 47k | 29 | 2 that mattered, 6 smaller |
| P | opus medium | 3.03 | 9.7 | 16 | 52k | 26 touch points | 4 |
| A | opus medium | 3.78 | 13.3 | 21 | 66k | 25+ | 4 rule changes and the stack's staging |

H's Sonnet spec found the fact that shaped the piece (the digest hashes the log, so the led
game's digest moves once and a state digest proves the rest) and built the baselines the
builds ran against. Its slips: two steps scored New 2, which the scale lacks, and a done
("each play runs to the end") that couldn't pass on main. P's Opus spec made the same kind of
slip (play-hold already failing on main). Spec-review fixed both. On a settled piece the
Sonnet designer was as good. The probe says nothing about an unsettled piece; there is no
probe.md for H.

### Spec-review

| Piece | $ | Min | Bash calls | What it ran | What it changed | What a later step hit that it could have caught |
|---|---|---|---|---|---|---|
| H | 2.25 | 12.6 | 44 | check.sh, compare.sh, the AI with logging on five runs and the sweep, the plays | Serde values for the mirror; the refusals' order and text; a plays baseline (two fail on main); sweep.rs split; two steps to Haiku | The review found the AI's post can land on a built pass and panic. Spec-review settled "every AI order passes" from sampled runs, which an `expect` then relied on |
| G | 1.10 | 2.9 | 22 | A nested `claude -p`, `relay/usage.mjs`, an Agent subagent | The trial on Agent subagents; the review at Opus medium | None in the build. The bands on main's numbers, which the owner held (split.md), weren't questioned by it or the review |
| L | 2.20 | 6.2 | 35 | The guard's division on small 2 at the pick; the store a turn | GUARD.guarding 3.5 and inside 0.05 (the spec's held nothing); the leaving floor; flip window; six smaller | It measured the store on main (5–9 a turn). Build-1, earlier in the same piece, cut that to about 1–3, so the founding guard starved (build-2) and the canvas changed rules to feed it, at camps lost 3→18 |
| P | 1.94 | 6.3 | 41 | The six old scripts, the map's share at three sizes, terrain timing, save sizes | `digest --game`; a 16 ms bar; play-hold rebuilt in build-3; how the share is counted | None: build-3's 44% share is what it predicted, and the canvas fixed it |
| A | 2.39 | 8.5 | 38 | Each story's opening through `play` on main | Surrender at 3 to the cutter; guards don't retreat; the AI's claim skip; the stack staged | The stack still never reaches a capital (the glaze: the store fills three packs at most). It measured the store with the camp away at 46 by +35; the stories ran on A's later rules (build-3's foraging, the guard kept by the AI's rule), and why the two disagree isn't measured |

Every piece's spec-review changed something that would have failed a build or the proof as
written (G's trial couldn't have started; L's guard held nothing; P's played proof had no
command; A's surrender could never fire). It cost $9.88 in all, 14% of M3, and 51 5-hour
points. Keep it.

Its blind spot is the piece's own earlier steps: it measures main, and in L and A a step
earlier in the piece moved what it measured.

### The review

The prompt says Sonnet high, or Opus low when the piece is judged by play. The crew refuses a
judging step at low effort, so G, L, P and A ran Opus medium ($1.04–2.07, 7–10 5-hour points;
H's Sonnet high review $0.58). The text and the crew disagree.

## 6. Shapes

| Piece | Suggested | Ran | Where the hard part sat | Two sessions or compaction | Review changed |
|---|---|---|---|---|---|
| H | Henry Ford | Henry Ford, 7 fords (more steps by area than the proposal's 3) | Nowhere: settled. All fords Haiku | None | Code: the AI's post. Verdict merge |
| G | Easel and Canvas | Same | The canvas: the trial found the view's gap and set the bands | None | Note on a band. Merge; the owner then split the PR and held the bands (PR 124) |
| L | Easel and Canvas | Same, two easels | The canvas: three rule changes made claiming pay. It also changed rules beyond its numbers, and left the played games to the review | None | No code. Merge, with three things for the owner (camps lost, Red's guard, play-hold) |
| P | Easel and Canvas | Same, three easels by area | The canvas: the map's share and the layout | None | The end chart's axis. Merge |
| A | Clay, Kiln, Glaze | Same | Split: the clay held the fight rule; build-3, a kiln, held the stories and the scout (unsettled, its done unmet); the glaze tuned what the economy allowed | Glaze, two sessions | The page's "camp" strings. Merge |

No designer changed the suggested shape. The canvas held the hard part in G, L and P. In A
the glaze held only part of it: a kiln at 8 points with Open 2 and Miss 3 carried the stories
and the scout, and was the dearest step of M3. The clay held: the kilns built on presence, the
parts and the counters and kept the invariants (build-2's and build-5's summaries).

A build-5's first session ended itself at 41.8 minutes ("I'm out of time, so I'm committing
now") with `pending` and army.mjs failing, while the 90-minute cap had 48 minutes left and the
5-hour window stood at 79% (its last rate-limit event). The second session finished in 15
minutes for $0.69.

**What the reviews missed that the owner found playing (owner.md §1):**

- **Disband all** (§1.2). `disband()` in `engine/www/panels.js` gives the army an `all` option
  with a count that starts at 1, and sends `{type:'disband', n}`; the engine reads `n` none as
  the whole army (A spec item 3). Built by A build-4 (Haiku medium); army.mjs raises and fights
  but never disbands; the review played a fight through play.mjs and didn't press Disband.
- **The scout's search** (§1.3). No step or review watched a scout search on a real map. A
  build-3 measured only whether the scout survives its first errand (`scout.md`: 13 of 14
  lost); P's and A's page scripts check the scout's panel and buttons, not its route.

Both are the kind a scripted outcome test would catch (owner.md §1.5), and the first is the
kind a review that presses each new button once would catch.

## 7. The ladder's next text

Edits to `relay/prompts/spec.md`, as the proposal can take them.

1. **The ladder table.** Replace the row with:

   | Points | 0–1 | 2–3 | 4–5 | 6–8 | 9–10 | 11–12 |
   |---|---|---|---|---|---|---|
   | Step | haiku low | haiku medium | haiku high | sonnet medium | sonnet high | opus medium |

   with the line under it: "`canvas`, `glaze` and `clay` run Opus low from 8 to 10."
   Evidence: L build-1 and P build-1 at 8 on Sonnet medium met every target (§3). A's clay at 8
   stays Opus low.

2. **The rung below.** Replace "Each piece runs one build step a rung below the ladder, and its
   `why` says what it expects to see." with: "Each piece runs its `canvas`, `glaze` or `clay`
   a rung below, Sonnet high in place of Opus low, and its `why` says what it expects to see.
   A Henry Ford piece runs its largest ford a rung below." The cheap rungs saved nothing to
   measure; the four Opus low steps are $10.36 of M3's builds.

3. **The mismatch rule.** Replace "A `ford`, `easel` or `kiln` step at 11 or more holds
   unsettled work to settle or move." with: "A `ford`, `easel` or `kiln` step with Open 2 and
   Miss 3, or at 9 or more, holds unsettled work: move it to the canvas or glaze, or settle it
   here by running." Evidence: A build-3 (§2).

4. **The review.** Replace "The review isn't scored. It runs Sonnet high, or Opus low when
   `main` has moved under the piece or the goal is judged by play." with: "The review isn't
   scored. It runs Sonnet high, or Opus medium when `main` has moved under the piece or the
   goal is judged by play." It then matches the crew and "nothing that judges runs at low
   effort".

5. **What the review must play.** Add to "What proves the piece": "Every new button or order
   the player meets is pressed once in a script that checks its effect on the world, and the
   review runs it." Evidence: Disband all (§6).

6. **The size line.** Replace "and its size (files, rough lines)" with "and its size (files,
   rough lines, counting tests, pins and `engine/CLAUDE.md`)". Estimates run 1.6x low.

7. **Expect lines.**
   - Henry Ford: "Haiku. A refactor proved byte for byte is the cheapest work there is."
   - Easel and Canvas: "Haiku easels, Sonnet medium for an easel at 6–8, and an Opus low
     canvas."
   - Clay, Kiln and Glaze: add "A kiln that has to run stories on real maps to see its own
     rule work is part of the glaze."

8. **The designer.** Under "Choose a shape" add: "Measure on `main` with the piece's earlier
   settled steps applied where a later step relies on the number." This is for spec-review's
   prompt as much as the spec's: L's store and A's packs moved under it (§5).

9. **Henry Ford's spec** may run on Sonnet high (m3-relay.md §2's rule, for the planner): H's
   spec was as good as the Opus ones on a settled piece (§5).

## For other angles

- **crew**: the review's ladder (Opus low) and `checkSteps` (no low for a judging step)
  disagree; four of five reviews ran Opus medium. A build-5's first session stopped itself
  "out of time" at 42 minutes with 48 left on the cap; check whether the prompt or the 5-hour
  reading made it think so. Four budget stops each waited 2.5–3 hours for the reset; the
  restart-at-reset worked every time. The fit check's expected points for a Sonnet high kiln
  (A build-3: 49) are far above M2's medians.
- **prompts**: L build-3 (Haiku) delegated half its brief to a background subagent. G build-1
  (Haiku) spent 43 of 104 turns on Monitor and ScheduleWakeup waiting for check.sh. The `why`
  for each build says what the retro learns on failure; that line read well and should stay.
- **game**: the economy stops every stack before a capital (A glaze, review); camps lost 3→18
  and warriors starved 19→53 from L's fed guard, half undone by A's foraging (53→26).
- **players**: owner.md §1.2 and §1.3 come from a step and a scout no script exercises; §1.5's
  outcome tests would catch both.
- **architecture**: H moved 3,454 and 2,246 lines on Haiku for under $1.10 each; settled
  refactors are cheap enough to run whenever a file swells.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| 6–8 points runs Sonnet medium | §3: L b1, P b1 met every target | medium | small | take |
| The rung-below step is the canvas, glaze or clay, on Sonnet high | §3: cheap rungs saved nothing to measure; Opus low builds $10.36 | high | small | try |
| A kiln or easel with Open 2 and Miss 3 is mis-cut | §2: A build-3, $5.10, 49 5h, done unmet | medium | small | take |
| The review's text says Opus medium, matching the crew | §5: four of five reviews | low | small | take |
| Every new button pressed once in a script that checks the world; the review runs it | §6: Disband all | high | medium | take |
| Spec-review measures with the piece's earlier settled steps applied | §5: L's store, A's packs | medium | medium | try |
| Keep spec-review at Opus medium | §5: every piece had a change that would have failed a build | high | $2 a piece | take |
| Henry Ford specs on Sonnet high | §5: H's spec as good, $1.72 | low | small | try |
| Size line counts tests, pins and CLAUDE.md | §2: estimates 1.6x low | low | small | take |
| Drop the size line, since it doesn't predict cost | §2 | low | small | drop (it predicts minutes, ranks right, and costs one clause) |
| Move the Haiku rungs down (4–5 to Haiku medium) | §3: G b2 clean, A b4 shipped a defect | low | small | later (one more milestone's data) |
| Score "settled by a byte-identical proof" as its own low band | §2: H's fords at a tenth of L's Haiku cost at equal points | low | small | later |
| Scripted outcome tests for units (owner.md §1.5) as part of what proves a piece | §6: the scout's search | high | medium | take (game and players angles size it) |
