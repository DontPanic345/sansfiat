# M4 retro: the ladder

This covers pieces S #151 (PR 159), E #152 (PR 178), T #153 (PR 204), D #154 (PR 226), F #155 (PR 245), W #156 (PR 268),
R #157 (PR 294) and the fixes piece X #162 (PR 301): 62 step issues, plus R's stepped spec #289 (closed when R
turned whole), and 69 sessions. The sources:

- the `<!-- relay -->` block on each step issue: cost, minutes, usage before and after, context, sessions, `why`,
  summary, and for a twin `branch`, `chose` and `picked`;
- `relay/specs/<n>/steps.json`, the briefs, `spec-review.md`, `pick.md` and `canvas.md`;
- R's unrun plan, `relay/specs/157/steps.json` at 2739064 (PR 290);
- the stream-json transcripts and `crew.log` in the artifacts of the 18 relay runs from 42284235074 (S spec) to
  42960440614 (this retro's check). Every M4 session's transcript is in them;
- diff sizes from the GitHub commits API on each PR's step commits: engine files only, additions plus deletions,
  with images and JSON left out. The twins' A branches were measured with `git diff` against their base.

"5h" means the 5-hour window's points and "wk" the weekly window's. Both are whole numbers, and both include the
owner's own use in the same hours, which can't be split out.

## Verdict

- **Cost.** M4 cost $149.36 at list price, 2,084 session-minutes, at least 785 5-hour points and 68 weekly points
  (73 by the window's range). m4.md §3 expected $107, 660 and 50 for the pieces. The pieces alone were $148.11, 39%
  over, with 5-hour points 17% over and weekly 34% over. E, T, W and R account for the overshoot. Only F came in
  under.
- **Per piece.** A stepped piece cost $19.41 and about 103 5-hour points (M3: $14.28 and 87). Builds cost about the
  same as in M3 ($1.87 a build against $1.79). Design is what grew: an Opus spec averaged $4.37 (M3: $2.93) and
  spec-review $2.84 (M3: $1.98). Design was 35% of a stepped piece's cost.
- **List $ is now the window's meter.** Every model takes 5.0 to 5.5 five-hour points per list dollar (M3: Opus 5.3,
  Sonnet 8.2, Haiku 6.5). The ladder sets cost only through which model it picks.
- **Points predict cost less than in M3:** rank 0.48 over the builds (M3: 0.75), and 0.34 within Haiku. Haiku's
  cost tracks its own tool calls (rank 0.98). Four Haiku sessions re-armed a Monitor on `check.sh` 41 to 155 times.
  Those four cost $12.53, 46% of all Haiku spend. E build-2, scored 4, cost $5.68 with 155 Monitor calls.
- **Opus low lost both blind picks.** In F, Sonnet high (the twin) found why gatherers starve and changed one rule,
  while Opus low tuned until the bound passed. The pick took B's rules and canvas. In W, Opus medium (the twin)
  traced the town starving to a rule and fixed a judge bug, while Opus low named "crowding". The pick took B
  unchanged. E, T and D's canvases also ran on Sonnet high, a rung below. All three finished and reported honestly.
  D took two sessions and 146 minutes, and missed two bounds.
- **The twin cost $9.28 (6% of M4)** and gave the first controlled comparisons of models in four milestones.
- **The size line now has the right size:** lines 1.12x and files 1.5x the estimate (M3: 1.6x and 1.7x). It ranks
  right (0.75 for lines, 0.89 for files). It still predicts nothing about cost (0.02).
- **Five steps needed a second session** (M3: one). Four of the five were stale premises: page scripts and tests
  pinned to a retro pick, which moves with any float the pre-game reads.
- **R's whole step on Fable medium cost $28.20** and 142 5-hour points, $31.66 and 160 with its discarded stepped
  spec. That is about 1.3x what its stepped plan would have cost at M4's medians. Its first session spent a whole
  5-hour window (1% to 102%) in 89 minutes and died on an API error. A whole piece on Fable takes two windows, not
  one.

## 1. Every step

How to read the table:

- **Ladder** is what `relay/prompts/spec.md` gave for the points and part when the step was made. At M4's start:
  6–8 points ran Sonnet medium and 9–10 Sonnet high; `canvas`, `glaze` and `clay` ran Opus low from 8 to 10, and
  each piece ran its canvas a rung below (Sonnet high) until the twin replaced that from F on.
- **Off?** is the direction the step ran off the ladder, and **Sess** is its sessions.
- **List $** and **Min** are the block's figures, summed across sessions.
- **5h** marked "≥" means the 5-hour window reset during the step, so the block's reading is a lower bound (§8).
- No session compacted.

| Piece | Step | Part | Pts | Ladder | Ran | Off? | Sess | List $ | Min | 5h | wk | Ctx | How it went |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S | spec | | | sonnet high (probe) | sonnet high | | 1 | 1.61 | 11.2 | 13 | 1 | 17% | Henry Ford with 7 fords (the proposal said 3) |
| S | spec-review | | | opus medium | opus medium | | 1 | 3.03 | 11.3 | 15 | 2 | 13% | Found that the scout's red case was the wrong bug, and re-scored build-3 to New 2; four fixes |
| S | build-1 | ford | 4 | haiku high | haiku medium | down (largest ford) | 1 | 0.25 | 18.1 | 2 | 0 | 14% | Parser, Stage, retro cache; 7 files, 819 lines (est. 6, 900) |
| S | build-2 | ford | 2 | haiku medium | haiku medium | | 1 | 0.04 | 14.8 | 0 | 0 | 8% | Fixtures on the stage; 336 lines |
| S | build-3 | ford | 5 | haiku high | haiku high | | 1 | 0.16 | 7.8 | 2 | 0 | 12% | cases.rs: owner's invariant green, search order red, river bridge red |
| S | build-4 | ford | 3 | haiku medium | haiku medium | | 1 | 0.18 | 6.5 | 1 | 0 | 12% | Sim.stage and the page arm |
| S | build-5 | ford | 5 | haiku high | haiku high | | 1 | 0.47 | 19.5 | 3 | 0 | 16% | Judge split byte for byte, idle control, economy group |
| S | build-6 | ford | 4 | haiku high | haiku high | | 1 | 0.31 | 15.8 | 2 | 0 | 14% | `judge --compare`, the floor, `bench --after` |
| S | build-7 | ford | 4 | haiku high | haiku high | | 1 | 1.38 | 23.4 | 10 | 1 | 30% | buttons.mjs over 34 cases; it found Keep ±1, which X fixed |
| S | review | | | opus medium | opus medium | | 1 | 1.72 | 16.1 | 8 | 1 | 11% | Same; merge |
| E | spec | | | opus medium | opus medium | | 1 | 4.12 | 13.4 | 21 | 2 | 20% | Five builds (proposal 3). Its probe: no story takes a capital |
| E | spec-review | | | opus medium | opus medium | | 1 | 1.91 | 5.3 | 10 | 0 | 11% | Fell before Conquest, the turn count, the limit's settle probe |
| E | build-1 | easel | 4 | haiku high | haiku high | | 1 | 2.40 | 56.1 | 12 | 1 | 29% | The start; 253 turns, 41 of them Monitor |
| E | build-2 | easel | 4 | haiku high | haiku high | | 1 | 5.68 | 35.2 | 25 | 2 | 45% | The end; 365 turns, 155 of them Monitor. M4's dearest Haiku step |
| E | build-3 | easel | 3 | haiku medium | haiku medium | | 1 | 0.38 | 19.4 | 2 | 1 | 15% | The judge reads the end |
| E | build-4 | easel | 5 | haiku high | haiku high | | 1 | 2.23 | 22.5 | 11 | 1 | 30% | End screen; 91 Monitor calls |
| E | build-5 | canvas | 9 | opus low | sonnet high | down (rung below) | 1 | 2.95 | 35.3 | 20 | 1 | 22% | "No, not yet": 0 of 60 games end early. Limits kept. Just what its `why` predicted |
| E | play | | | sonnet medium | sonnet medium | | 1 | 0.18 | 1.6 | 1 | 0 | 4% | |
| E | review | | | opus medium | opus medium | | 1 | 1.67 | 23.7 | 9 | 1 | 11% | Not shown; merge |
| T | spec | | | opus medium | opus medium | | 1 | 4.40 | 17.8 | 23 | 2 | 20% | Moved the flat trail into the first easel |
| T | spec-review | | | opus medium | opus medium | | 1 | 4.05 | 23.0 | 20 | 2 | 17% | Ran a rough feeding rule on the errands; new starting numbers |
| T | build-1 | easel | 3 | haiku medium | haiku medium | | 2 | 1.28 | 66.8 | 7 | 0 | 8% | Flat trail. Session 1 ended pending with 7 page scripts failing on moved picks |
| T | build-2 | easel | 4 | haiku high | haiku high | | 1 | 1.45 | 54.6 | 9 | 0 | 26% | Search by best case; 1,195 lines (est. 750) |
| T | build-3 | easel | 6 | sonnet medium | sonnet medium | | 1 | 1.92 | 48.5 | 13 | 1 | 15% | The scout's feeding and wander |
| T | build-4 | easel | 5 | haiku high | haiku high | | 1 | 1.02 | 29.0 | 7 | 1 | 24% | The scout's page |
| T | build-5 | canvas | 10 | opus low | sonnet high | down (rung below) | 1 | 2.20 | 78.7 | 14 | 1 | 17% | 28 of 28 scouts alive; work at 24%, so an ask. Its `why` predicted both |
| T | play | | | sonnet medium | sonnet medium | | 1 | 0.22 | 1.4 | 2 | 0 | 4% | |
| T | review | | | opus medium | opus medium | | 1 | 2.07 | 29.1 | 10 | 1 | 13% | Fixed Move to… not staying |
| D | spec | | | opus medium | opus medium | | 1 | 3.94 | 35.0 | 21 | 2 | 17% | Re-cut Clay to Easel and Canvas: the clay scored 3 |
| D | spec-review | | | opus medium | opus medium | | 1 | 2.60 | 19.5 | 13 | 1 | 12% | The ring's proof moved to staged armies |
| D | build-1 | easel | 3 | haiku medium | haiku medium | | 1 | 0.32 | 17.0 | 2 | 0 | 14% | split and Ladder, digest-safe; 958 lines (est. 450) |
| D | build-2 | easel | 6 | sonnet medium | sonnet medium | | 1 | 3.33 | 84.7 | unread | unread | 22% | Every number's unit, DAY. It let scout_errand lose 5 |
| D | build-3 | easel | 5 | haiku high | haiku high | | 1 | 1.17 | 33.6 | 7 | 0 | 19% | The ring |
| D | build-4 | canvas | 10 | opus low | sonnet high | down (rung below) | 2 | 6.28 | 146.4 | 41 | 4 | 18% | Missed 2 bounds and let scout_errand lose 4; found 2 real search bugs. Session 1 ended pending on stale pins. M4's dearest build |
| D | play | | | sonnet medium | sonnet medium | | 1 | 0.17 | 1.1 | 1 | 0 | 4% | |
| D | review | | | opus medium | opus medium | | 1 | 2.04 | 48.0 | 12 | 1 | 12% | Not shown; merge |
| F | spec | | | opus medium | opus medium | | 1 | 4.79 | 22.5 | 24 | 2 | 20% | Re-cut Clay to Easel and Canvas |
| F | spec-review | | | opus medium | opus medium | | 1 | 2.58 | 26.8 | 14 | 1 | 13% | Starving is a rule's work, not a number's |
| F | build-1 | easel | 4 | haiku high | opus medium | up (owner) | 2 | 3.60 | 96.6 | 18 | 2 | 7% | A Haiku session (60 min, $2.39) blocked on scout_errand. The owner moved the step to Opus with "fix or remove the test" |
| F | build-2 | easel | 3 | haiku medium | opus medium | up (owner) | 1 | 3.45 | 47.2 | 17 | 1 | 15% | Judge's fields group |
| F | build-3 | easel | 4 | haiku high | haiku high | | 1 | 0.82 | 20.3 | 5 | 1 | 20% | The page shows fields |
| F | build-4 (A) | canvas | 10 | opus low | opus low | | 2 | 2.69 | 100.2 | 13 | 1 | 5% | Tuned until it passed. Only its page check landed |
| F | build-4-twin (B) | canvas | 10 | rung below | sonnet high | | 1 | 1.88 | 88.1 | ≥1 | 1 | 16% | Found the transient and one rule; landed |
| F | build-4-pick | | | opus medium | opus medium | | 1 | 1.02 | 15.8 | 6 | 0 | 7% | Both |
| F | play | | | sonnet medium | sonnet medium | | 1 | 0.26 | 1.7 | 1 | 0 | 5% | |
| F | review | | | opus medium | opus medium | | 1 | 1.70 | 32.7 | 9 | 1 | 11% | Not shown (the gain is in AI play); merge |
| W | spec | | | opus medium | opus medium | | 1 | 4.59 | 22.2 | 24 | 2 | 19% | Sketched the canvas's rule |
| W | spec-review | | | opus medium | opus medium | | 1 | 2.87 | 15.4 | 15 | 1 | 13% | Scale by flat search; re-scored build-2 to Haiku medium |
| W | build-1 | easel | 4 | haiku high | haiku high | | 1 | 2.22 | 40.1 | 11 | 1 | 28% | 42 Monitor calls |
| W | build-2 | easel | 3 | haiku medium | haiku medium | | 1 | 0.77 | 38.2 | 5 | 0 | 19% | Turn at 0.34 of main's |
| W | build-3 | easel | 5 | haiku high | haiku high | | 1 | 1.99 | 37.4 | 11 | 1 | 31% | 32 files, 2,156 lines (est. 22, 1,320 with the moves) |
| W | build-4 (A) | canvas | 10 | opus low | opus low | | 1 | 3.35 | 68.4 | ≥6 | 2 | 13% | Named crowding; nothing landed |
| W | build-4-twin (B) | canvas | 10 | rung above | opus medium | | 1 | 5.31 | 60.7 | 45 | 4 | 21% | Found the divide_land rule and a judge floor bug; landed whole |
| W | build-4-pick | | | opus medium | opus medium | | 1 | 1.07 | 15.3 | 10 | 1 | 8% | B |
| W | play | | | sonnet medium | sonnet medium | | 1 | 0.22 | 1.1 | 1 | 0 | 4% | |
| W | review | | | opus medium | opus medium | | 1 | 2.14 | 26.9 | 10 | 1 | 12% | Better; merge |
| R | spec (#289) | | | opus medium | opus medium | | 1 | 3.46 | 13.1 | 18 | 2 | 16% | Easel and Canvas with a twinned canvas. Closed when R went whole |
| R | whole | | | fable medium | fable medium | | 2 | 28.20 | 132.3 | 142 | 12 | 12% | Session 1 hit 102% of the window at 89 min (API error, $24.31); session 2 took 43 min |
| X | spec | | | sonnet medium | sonnet medium | | 1 | 0.24 | 1.0 | 2 | 0 | 5% | Five findings as four fixes |
| X | spec-review | | | sonnet medium | sonnet medium | | 1 | 0.19 | 1.7 | 2 | 0 | 4% | Errands restaged; raised build-1 to Sonnet medium |
| X | build-1 | ford | 6 | sonnet medium | sonnet medium | | 1 | 0.33 | 20.7 | 2 | 0 | 5% | The scout trails the cheaper path |
| X | build-2 | ford | 2 | haiku medium | haiku medium | | 1 | 0.02 | 22.2 | 0 | 0 | 5% | Keep ±1 |
| X | review | | | sonnet high | sonnet high | | 1 | 0.22 | 22.4 | 2 | 0 | 5% | Same; merge |
| retro | check | | | sonnet medium | sonnet medium | | 1 | 0.25 | 1.6 | 2 | 1 | 5% | Small 2, 103 turns |

Twin labels: A and B are the branch letters each run recorded (`relay/<n>-build-4-a` and `-b`). In both pieces the
twin ran as B.

The largest peak context was 45% (E build-2, Haiku high). The longest sessions were R whole's first (89.3 min), F's
twin (88.1) and D build-2 (84.7), against the 90-minute cap; the whole's job has its own 300.

### Totals against m4.md §3

| Piece | Shape run | Builds (exp.) | List $ (exp.) | Min | 5h (exp.) | wk (exp.) | Design $ | Builds $ | Twin + pick $ | Review + play $ |
|---|---|---|---|---|---|---|---|---|---|---|
| S | Henry Ford | 7 (3) | 9.15 (8) | 145 | 56 (50) | 5 (4) | 4.64 | 2.79 | | 1.72 |
| E | Easel and Canvas | 5 (3) | 21.52 (12) | 212 | 111 (70) | 9 (6) | 6.03 | 13.64 | | 1.85 |
| T | Easel and Canvas | 5 (3) | 18.61 (12) | 349 | 105 (70) | 8 (6) | 8.45 | 7.87 | | 2.29 |
| D | Easel and Canvas (re-cut from Clay) | 4 (3) | 19.85 (15) | 385 | ≥97 (90) | ≥8 (7) | 6.54 | 11.10 | | 2.21 |
| F | Easel and Canvas (re-cut from Clay, Kiln, Glaze) | 4 (5) | 22.79 (24) | 452 | ≥108 (160) | 10 (11) | 7.37 | 10.56 | 2.90 | 1.96 |
| W | Easel and Canvas | 4 (4) | 24.53 (18) | 326 | ≥138 (110) | 13 (8) | 7.46 | 8.33 | 6.38 | 2.36 |
| R | Whole (planned as Easel and Canvas) | 0 (4) | 31.66 (18) | 145 | 160 (110) | 14 (8) | 3.46 | 28.20 whole | | |
| Pieces | | 29 (25) | 148.11 (107) | 2,014 | ≥775 (660) | 67 (50) | 43.95 | 82.49 | 9.28 | 12.39 |
| X | Henry Ford | 2 | 1.00 | 68 | 8 | 0 | 0.43 | 0.35 | | 0.22 |
| Check | | | 0.25 | 2 | 2 | 1 | | | | |
| M4 | | | 149.36 | 2,084 | ≥785 | 68 | | | | |

m4-relay.md §6 expected 60 to 70 weekly points for the milestone with its loop. It took 68 by step, and 73 by the
window's range (52% to 70% before the weekly reset on 7 Nov, then 0% to 55%). The play steps cost what §4 said
they would: about $0.20 and one 5-hour point each. The fixes piece came in under its 4 weekly points, at 0.

| Model | Steps | Sessions | List $ | Share | Min | 5h | 5h per $ |
|---|---|---|---|---|---|---|---|
| Opus | 25 | 27 | 74.17 | 50% | 806 | ≥391 | 5.3 |
| Sonnet | 17 | 18 | 22.45 | 15% | 548 | ≥118 | 5.3 |
| Haiku | 21 | 22 | 24.54 | 16% | 598 | 134 | 5.5 |
| Fable | 1 | 2 | 28.20 | 19% | 132 | 142 | 5.0 |

The check is in the Sonnet row. F build-1's Haiku session ($2.39) sits in its block under Opus, and so in the Opus
row here; the transcripts put Haiku's own session spend at $26.93.

By role: spec $27.15 (8, R's included), spec-review $17.23, builds $54.64 (31), twins $7.19, picks $2.09, play
$1.05, reviews $11.56, whole $28.20, check $0.25.

### M3 against M4

| | M3 (5 pieces, 38 steps) | M4 (7 pieces and fixes, 63 steps) |
|---|---|---|
| List $ | 71.40 | 149.36 |
| 5h points | 434 | ≥785 |
| Weekly points | 33 (37 by range) | 68 (73 by range) |
| Session-minutes | 769 | 2,084 |
| Per stepped piece: $, 5h, wk | 14.28, 87, 6.6 | 19.41, 103, 8.8 |
| Builds per stepped piece | 4.6 | 4.8 |
| $ per build | 1.79 | 1.87 (54.29 / 29) |
| Opus spec, mean $ | 2.93 | 4.37 (E to W) |
| Spec-review, mean $ | 1.98 | 2.84 |
| Design share of a stepped piece | 33% | 35% |
| Opus share of $ | 58% | 50% (Fable 19%) |
| Haiku builds: steps, median $ | 15, 1.00 | 21, 0.82 (1.23 outside S) |
| Longest session, peak context | 83 min, 49% | 89 min, 45% |
| Sessions needing a second | 1 | 5 |

## 2. Did the ladder predict?

These are rank correlations (Spearman) over the 29 ladder builds. Twins and X are left out, and the sizes come
from each `why`'s "about N files, M lines".

| Over | n | Points ~ $ | Points ~ 5h | Points ~ min | Est. lines ~ $ | Actual lines ~ $ | Est. ~ actual lines | Est. ~ actual files | Actual files ~ min |
|---|---|---|---|---|---|---|---|---|---|
| All builds | 29 | 0.48 | 0.48 | 0.48 | −0.03 | 0.02 | 0.75 | 0.89 | 0.51 |
| Non-Opus | 25 | 0.58 | 0.65 | 0.46 | 0.08 | 0.10 | 0.73 | 0.91 | 0.67 |
| Haiku | 20 | 0.34 | 0.43 | 0.12 | 0.31 | 0.25 | 0.71 | 0.93 | 0.79 |

**Points predict cost less well than in M3, for three reasons:**

1. The canvases moved down to Sonnet high, which costs about what Opus low did. The top of the scale no longer buys
   a dearer model.
2. The owner moved two low-scored easels up to Opus.
3. Haiku's spend at 3 to 5 points ran from $0.16 to $5.68.

**Haiku's cost is its turns.** Over Haiku's 23 sessions, list $ against tool calls ranks at 0.98. What raised the
turn count in M4 was waiting:

| Step | Pts | Monitor calls | Bash calls | $ | Min |
|---|---|---|---|---|---|
| E build-2 | 4 | 155 | 203 | 5.68 | 35.2 |
| E build-4 | 5 | 91 | 100 | 2.23 | 22.5 |
| W build-1 | 4 | 42 | 88 | 2.22 | 40.1 |
| E build-1 | 4 | 41 | 209 | 2.40 | 56.1 |
| The other 19 Haiku sessions | 2–5 | 0–12 | 14–163 | 0.02–2.39 (14.40 in all) | |

E build-2 re-armed the same `until grep -q "^EXIT" /tmp/check2.log; do sleep 5; done` Monitor 80 times, and a
second one 44 times (transcript, run 42320809361). Every re-arm is a turn that re-reads the cached context. M3 saw
the same thing on a smaller scale: G build-1 took 43 Monitor turns of 104. The ladder can't see this. It is the
build prompt's job (ideas).

**Within Opus and Sonnet, no single measure predicts cost.** The six Sonnet builds ran $0.33 to $6.28, and the four
Opus builds $2.69 to $3.60. The dearest steps are the ones that retuned page scripts and pins after the picks
moved (§7).

**The size line now has the right size.** Each build's `why` gave one, and M3's change ("counting tests, pins and
`engine/CLAUDE.md`") moved the median actual over estimate from 1.6x to 1.12x in lines. Files are still 1.5x low:
the page scripts retuned to new seeds are left out. Canvases are the exception, with about 0.3x their estimated
lines, because a tuning step changes numbers, not code. The line ranks right and predicts minutes (files ~ minutes
0.67 outside Opus). It doesn't predict cost.

**Each `why`'s "I expect" line mostly held:**

| Step | Expected | Happened |
|---|---|---|
| E build-5 | No story ends a game early; record how near | The same: 3 of 21 conquest games take one capital, none ends early |
| T build-5 | Close the last losses; an ask if the work stays under a quarter | 28 of 28 alive, work 24%, and an ask |
| D build-4 | Pass every bound or send the ask with the nearest | Missed 2 bounds, and loosened scout_errand to allow 4 lost rather than ask first |
| F twin | Sonnet high meets the number bounds and stops at the rule bound; Opus low more likely finds the cause | **Reversed:** Sonnet high found the cause, Opus low tuned |
| W twin | Opus medium traces the starving to a rule; Opus low names it | The same |

## 3. The rung below and the twin

M3 asked for the canvas a rung below. M4 ran it on E, T and D, then replaced it with the twin from F on
(m4-relay.md §7).

### Sonnet high canvases

| Step | Pts | $ | Min | 5h | Done met? | What it found |
|---|---|---|---|---|---|---|
| E build-5 | 9 | 2.95 | 35.3 | 20 | Yes: an honest "no" | The march runs out of food before the capital; limits kept against the +600 leader |
| T build-5 | 10 | 2.20 | 78.7 | 14 | Alive yes, work no (24%, an ask) | 38 settings; wander was the lever |
| D build-4 | 10 | 6.28 | 146.4 | 41 | 4 of 6 bounds | Two real search bugs fixed (a trail a rounding short; a route's trails wearing away behind a slow scout); stale pins and seeds took session 2 |
| F build-4-twin | 10 | 1.88 | 88.1 | ≥1 | Every bound, with room | The starving was a growth transient; one rule changed |

M3's Opus low canvases cost $1.87 to $2.82. Sonnet high's median here is $2.58. The rung costs the same and does
the same.

### The two twins

| | F build-4 (A: Opus low) | F twin (B: Sonnet high) | W build-4 (A: Opus low) | W twin (B: Opus medium) |
|---|---|---|---|---|
| List $ | 2.69 | 1.88 | 3.35 | 5.31 |
| Minutes | 100.2 (2 sessions) | 88.1 | 68.4 | 60.7 |
| 5h | 13 | ≥1 (window reset) | ≥6 (window reset) | 45 |
| Settings run | 11 | 19 | 6 | not counted |
| Bounds | All pass, the starving bound only less its floor (raw 1.262 over 1.25) | All pass with room (raw 1.222) | Misses towns' maps' floor and 4b, and also the town-moves bound | Misses town maps (4 of 7) and 4b |
| Found the cause the brief asked for | No: tuned until the bound passed | Yes: a growth transient; fields sown only where they yield | No: "crowding" | Yes: `divide_land` left mothers on tiles they didn't hold; the judge's floor key read a missing path |
| Engine lines | 54 | 45 | 273 | 328 |
| Landed | fields.mjs's farmer wait | rules, numbers, canvas.md | nothing | everything |
| Pick | both (B's engine byte for byte) | | B | |

Sources: `relay/specs/155/pick.md`, `relay/specs/156/pick.md`, and the four summaries.

What this says, from two pairs:

- **Opus low is the weakest canvas rung.** Both times it tuned numbers until a bound read pass and stopped. Both
  times the other version traced the miss to a rule. That is the canvas's work: F's spec-review said in advance
  that its starving bound was "a rule's work, not a number's".
- **Effort mattered more than the model.** Sonnet high beat Opus low on the same brief at 70% of its cost. Opus
  medium beat Opus low at 1.6x.
- **The pick worked.** Both picks ran their own check where a number was missing or the versions disagreed. F
  merged one part of A (a page check that actually proved something). W tested A's away-rule on top of B and
  declined it with numbers. Each cost about $1 and 15 minutes.

What it can't say: two pairs on two pieces, both against Opus low. Nothing in M4 compares Sonnet high with Opus
medium on the same brief.

**What the twin cost:** $2.90 on F and $6.38 on W, counting the twin and its pick. That is $9.28, or 6% of M4 and
8% of the stepped pieces. Each twin also added about 75 to 105 minutes of wall clock before the next step.

## 4. Haiku

Haiku ran 21 steps: all seven of S's fords, thirteen easels in E, T, D, F and W, X's second ford, and F build-1's
first session. All but two finished in one session. T build-1 ran pending on the page scripts (§7), and F build-1
blocked on a test that was a snapshot of one float path, which the owner's reply settled.

- **The fords were cheap again.** S's seven fords cost $2.79 in all ($0.04 to $1.38). A settled refactor proved by
  digests is still the cheapest work there is. S build-1 ran a rung below (Haiku medium at 4) and landed the
  parser and the stage for $0.25.
- **The easels are where Haiku's spend went:** $0.32 to $5.68, median $1.28. The dear ones were the waiting turns
  (§2), not judgement.
- **Defects.** No Haiku step shipped a defect a later step reported in M4. The two `fix` findings X took (Keep ±1,
  and the scout's trail on the possible path) were bugs on main. S's Haiku-built buttons.mjs and cases.rs found
  them. Whether the owner's play finds a Haiku defect, the game angle will say.

| Step | Model | Engine files, lines | $ | Min | 5h | Turns |
|---|---|---|---|---|---|---|
| W build-3 | haiku high | 32, 2,156 | 1.99 | 37.4 | 11 | 153 |
| T build-2 | haiku high | 19, 1,195 | 1.45 | 54.6 | 9 | 135+ |
| D build-1 | haiku medium | 12, 958 | 0.32 | 17.0 | 2 | 47 |
| T build-3 | sonnet medium | 17, 926 | 1.92 | 48.5 | 13 | 81+ |
| D build-2 | sonnet medium | 33, 975 | 3.33 | 84.7 | unread | 120+ |
| F build-1 | opus medium (after Haiku) | 20, 782 | 3.60 | 96.6 | 18 | 41 + 181 |
| F build-2 | opus medium | 12, 708 | 3.45 | 47.2 | 17 | 73+ |

For new code of 700 to 2,000 lines, a Haiku easel cost $0.32 to $1.99, Sonnet medium $1.92 to $3.33, and Opus
medium about $3.50.

The two owner-moved easels in F cost $7.05 on Opus, against about $1 to $2.40 each for Haiku easels of the same
size. Whether Opus bought anything there can't be measured: there is no control. F build-1's blocker was raised
by its Haiku session, which was the right call.

"Turns" marked "+" are a lower bound: the result frame of a session that waited on a Monitor covers only its last
wake.

## 5. Designers

### Spec

| Piece | Model | $ | Min | 5h | Shape | Builds planned (proposal) |
|---|---|---|---|---|---|---|
| S | sonnet high | 1.61 | 11.2 | 13 | Henry Ford, kept | 7 (3) |
| E | opus medium | 4.12 | 13.4 | 21 | Easel and Canvas, kept | 5 (3) |
| T | opus medium | 4.40 | 17.8 | 23 | Easel and Canvas, kept | 5 (3) |
| D | opus medium | 3.94 | 35.0 | 21 | Clay and Kiln re-cut to Easel and Canvas | 4 (3) |
| F | opus medium | 4.79 | 22.5 | 24 | Clay, Kiln, Glaze re-cut to Easel and Canvas | 4 (5) |
| W | opus medium | 4.59 | 22.2 | 24 | Easel and Canvas, kept | 4 (4) |
| R | opus medium | 3.46 | 13.1 | 18 | Easel and Canvas, kept (then whole) | 4 (4) |
| X | sonnet medium | 0.24 | 1.0 | 2 | Henry Ford, kept | 2 |

- **The Opus specs cost 49% more than M3's.** Each one ran probes on the retro maps: E's settle probe, T's flat
  trail values, F's y8 probe, W's sketch of the canvas's rule. That is what M3's "measure on main" and "run it now"
  lines asked for, so the cost is bought, not wasted.
- **The mismatch rule fired twice.** D's and F's designers scored the proposal's clay at 3 and 4, and re-cut the
  piece to Easel and Canvas, as `spec.md` tells them to.
- **The Sonnet specs held.** S's Sonnet high spec ran seven fords that all landed; spec-review re-scored one step
  and found the red case was the wrong bug. X's Sonnet medium spec cost $0.24. That is two settled pieces in M4,
  after H in M3. No unsettled piece has been specced on Sonnet.

### Spec-review

Every spec-review changed something that would have failed a build or the proof as written:

- S: the red case was the wrong bug.
- E: Fell before Conquest, and the turn count.
- T: the feeding rule as worded loses 2 to 5 errands.
- D: there were no armies to measure the ring on.
- F: starving is a rule's work, and bound 4 has room.
- W: "trade 1.1x" couldn't pass, and the scale is a data structure.
- X: the errands no longer reproduce.

It cost $17.23, 12% of M4. Keep it at Opus medium. X's Sonnet medium spec-review ($0.19) found its stale errands,
which is enough for a fixes piece.

M3 said spec-review's blind spot was the piece's own earlier steps. In M4 it was the picks. Every rule change moved
the retro picks, and the pinned tests and seeded page scripts with them (§7), and no spec-review counted that work.

### The review

All six stepped reviews ran Opus medium ($1.67 to $2.14, 8 to 12 five-hour points), as the text now says. X's ran
Sonnet high for $0.22.

The verdicts: S same, E not shown, T fixed a bug (Move to… didn't stay), D not shown, F not shown, W better, X
same. Every one merged. What the reviews missed that the owner found playing waits on the owner's notes; the game
angle has them.

## 6. The whole step

R ran as `whole` on Fable medium after its Opus spec had planned it as four builds and a twinned canvas (#289, PR
290, closed).

| | Stepped plan (#289's steps.json) at M4's medians | Whole (#293) |
|---|---|---|
| Steps | spec, spec-review, haiku medium easel, 2 sonnet medium easels, opus low canvas with an opus medium twin, pick, play, review | 1 step, 2 sessions |
| List $ | about $23.75 (spec 3.46, spec-review 2.84, easels 0.8 + 2 × 2.6, canvas 3.0, twin 5.3, pick 1.05, play 0.2, review 1.9) | 28.20 (+3.46 for the discarded spec) |
| 5h | about 125 | 142 (+18) |
| Minutes | about 330, over about 4 crew runs | 132 |
| Wall clock | | 23:28 to 03:25, with a 1.7-hour wait for the window |

- Session 1 started at 1% of the 5-hour window and ended at 89.3 minutes on an API error, at 102%, having spent
  $24.31 (`crew.log`, run 42923500975). Of that, $23.18 was Fable and $1.13 Sonnet subagents (six spawned).
- Session 2 started at the reset and finished in 43 minutes for $3.89.
- Fable medium spends about 1.1 five-hour points a minute, so a "session of up to five hours" can't run in one
  window. The README's rule (start under 10%) gives it about 90 minutes.

What it built: led over idle +0.049 Standing on 21 of 21 civs, and three Sonnet players. Its ending question was
not shown. It reviewed itself: no session but its own read the piece before the merge. One whole piece costs about
1.2x its stepped plan, or 1.3x with the discarded spec. On quality there is one point and no control.

## 7. Shapes and second sessions

Every stepped piece but S ran Easel and Canvas. The canvas held the hard part in all five. In E and T it was a
search judged on the retro maps, and its done was an honest report. In D, F and W it was finding a cause.

Five steps needed a second session:

| Step | Why | Second session |
|---|---|---|
| T build-1 (Haiku medium) | 7 page scripts tuned to main's pick fail on the flat trail's maps | $0.05, 21.8 min: retuned seeds |
| D build-4 (Sonnet high) | check.sh red at 83 min: pins and page scripts on the new pick | $2.34, 63.3 min |
| F build-1 (Haiku, then Opus) | Blocked: scout_errand is a snapshot of one float path (a 1e-12 nudge loses 5 of 28) | $1.21 on Opus after the owner's reply, plus 5.7 hours waiting |
| F build-4 (Opus low) | check.sh red on three page checks the new rules moved | $0.62, 23.8 min |
| R whole (Fable) | The 5-hour window ran out | $3.89, 43 min |

Four of the five come from one cause: tests and page scripts pinned to a retro map's pick or a seed's play, which
any float change moves. D build-2's summary says the same ("picks move with any float"), and its and D build-4's
loosened scout_errand to pass, from 0 lost to 5 and then 4. That is a rule's bound weakened by a build. No score on
the ladder sees this work: a step that moves the pick pays a retune whatever its points.

## 8. What the blocks can't measure

- **5-hour points across a reset.** Three steps started before the window reset and ended after it. D build-2 has
  no "after" reading. F's twin reads 43 to 1, and W build-4 47 to 6. The block records the end reading less the
  start, floored, so those steps undercount. At 5.2 points a dollar they took about 17, 10 and 17, so M4's 5-hour total is
  about 37 points low: about 822, not 785.
- **Output tokens.** Not measured. A session that waited on a Monitor gets a result frame covering only its last
  wake (E build-5: 2 turns and 1,344 output tokens in a $2.95 session). The streamed messages carry only partial
  counts. M3's "Haiku cost tracks its output tokens" can't be checked. Tool calls stand in for it here.
- **R's discarded spec** (#289) is in no step of R's whole issue. Its $3.46 is counted above by hand.
- **A model change by the owner** keeps both models' spend under the new model in the block. F build-1's $3.60 is
  $2.39 Haiku and $1.21 Opus.
- **The owner's share** of each window can't be separated out.

## 9. The ladder's next text

These are edits to `relay/prompts/spec.md`, written so the proposal can take them as they stand.

1. **The canvas rung.** Replace "`canvas`, `glaze` and `clay` run Opus low from 8 to 10." with:
   "`canvas`, `glaze` and `clay` run Sonnet high from 8 to 10, and Opus medium from 11." Evidence: Opus low lost
   both blind picks; Sonnet high canvases finished at Opus low's cost (§3).
2. **Magnum.** Replace "A `magnum` step runs on Opus: low under 11 points, medium from 11." with "A `magnum` step
   runs Opus medium." Nothing that tunes or traces a cause should run Opus low (§3).
3. **The twin.** Keep it for one more milestone, with its rung fixed. Replace "The twin's rung is the question: a
   rung below asks whether the step needs its rung, and a rung above asks whether more would have paid." with:
   "The twin runs Opus medium against the ladder's Sonnet high: does a cause-finding canvas pay for Opus? Its
   `why` says what you expect each to do." Rename the section from "an M4 experiment" to "an M5 experiment".
   Evidence: M4 compared both rungs only against Opus low (§3).
4. **Expect lines.**
   - Easel and Canvas: "Haiku easels, Sonnet medium for an easel at 6–8, and a Sonnet high canvas."
   - Clay and Kiln: "Sonnet high or Opus medium clay and a cheap kiln."
   - Clay, Kiln and Glaze: "Sonnet high or Opus medium clay and glaze, and a cheap kiln."
   - Henry Ford keeps "Haiku" (S: seven fords for $2.79).
5. **Reach.** Under the scores, add: "A step that changes a number or a rule the pre-game reads moves every retro
   pick: the tests pinned to a pick and the page scripts tuned to a seed move with it. Its brief names them, and
   its `why` counts them in the size line." Evidence: four of five second sessions (§7).
6. **A bound is not a build's to move.** Under the mismatch rule, add: "A brief says which tests carry a rule's
   bound. A build that can't meet one stops with an ask; it doesn't loosen the test." Evidence: scout_errand from 0
   lost to 5 and 4 (§7).
7. **The size line.** Replace "(files, rough lines, counting tests, pins and `engine/CLAUDE.md`)" with "(files,
   rough lines, counting tests, pins, the page scripts it retunes and `engine/CLAUDE.md`)". Files run 1.5x low.
8. **The designer.** Add to the ladder's notes: "Haiku's cost is its turns: a step that waits on long runs costs
   as much as its waiting." This is for the build prompt as much as the ladder (ideas).

For `relay/prompts/whole.md` and the README: "A whole session on Fable medium spends a 5-hour window in about 90
minutes: plan for two sessions, and commit before the window runs out." Evidence: §6.

## For other angles

- **relay**:
  - Haiku re-arms a Monitor on `check.sh` dozens of times; E build-2 did it 155 times. The build prompt could say to
    wait on one foreground call or one Monitor.
  - The 5-hour reading across a reset undercounts (D build-2, F's twin, W build-4).
  - R's #289 spec was closed with its spend outside R's whole step.
  - The weekly ramp stopped the crew three times, for about 16, 24 and 15 hours, which was most of M4's 4.3 days of
    wall clock. Five 5-hour stops waited 1 to 2.5 hours each.
  - The play steps play 100 turns in 1.1 to 1.7 minutes for about $0.20, mostly idle (95 to 98 of 100 turns on F
    and W). They are cheap, but whether they play is the players angle's call.
- **architecture**: tests and page scripts pinned to a retro pick or a seed cost four second sessions and one owner
  stop (§7). Fixtures on the stage (S) rather than on the pick would end that.
- **game**: two builds loosened scout_errand (0 lost to 5, then 4) to pass. F's and W's picks took the version that
  found a rule (fields sown only where they yield; `divide_land`). E's and R's "not shown" endings are a fact of
  the economy, not of a rung.
- **players**: every play step played small 2 as civ 1. "Three seeds" ran only in R's whole session.
- **retro and proposal**: a stepped piece now costs about $19 and 103 five-hour points, and a whole piece on Fable
  about $28 to $32 and 140 to 160. At about 9 weekly points a stepped piece and the weekly ramp as it is, about five
  pieces fit in a week.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| Canvas, glaze and clay run Sonnet high at 8–10, Opus medium from 11; no Opus low | §3: Opus low lost both picks | high | small | take |
| Magnum runs Opus medium only | §3 | low | small | take |
| Keep the twin one more milestone, fixed at Sonnet high against Opus medium | §3: no pair compares those two | high | about 6% of a milestone | take |
| The build prompt says to wait on a long check with one call, never by re-arming a Monitor | §2: 4 Haiku sessions, $12.53, 46% of Haiku spend | high | small | take |
| A brief names the pinned tests and seeded scripts a pick-moving change retunes, and the size line counts them | §7: 4 of 5 second sessions | medium | small | take |
| A build may not loosen a test that carries a rule's bound; it asks instead | §7: scout_errand 0 → 5 → 4 | medium | small | take |
| Stage the fixtures off the retro pick, so a float change doesn't move them | §7 | high | medium | take (architecture sizes it) |
| The crew records a reset in a step's usage and reads 5h points as (100 − before) + after | §8: three steps unread | medium | small | take |
| A whole on Fable plans for two windows; whole.md says to commit before the window ends | §6: session 1 died at 102% | medium | small | take |
| Try a Sonnet high spec on one unsettled piece, with spec-review on Opus medium | §5: Opus specs at $4.37; Sonnet held on S, X and H | medium | small | try |
| Count the page scripts in the size line's files | §2: files 1.5x low | low | small | take |
| Drop spec-review on Henry Ford pieces | §5: S's spec-review found the wrong bug | | | drop (it earned its cost again) |
| Whole for every piece | §6: 1.2–1.3x stepped, one point, no independent review | | | later (one more whole piece with a stepped twin of it) |
| Score "settled by a byte-identical proof" as its own band | §4: S's fords at $0.04–1.38 | low | small | later |
