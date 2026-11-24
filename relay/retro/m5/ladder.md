# M5 retro: the ladder

This covers the 23 pieces M5 ran and its fixes piece: 114 step issues with figures (#344 to #510)
in 118 sessions, plus J (#352), which the owner built by hand in their own session and which has
no figures. The sources:

- the `<!-- relay -->` block on each step issue: model, effort, points, part, `why`, cost,
  minutes, usage before and after, context, sessions, summary;
- the crew's log, "Relay log: M5" (#345), and the piece issues' comments;
- the stream-json transcripts and `crew.log` of six relay runs: 43202663985 (L's first whole
  session), 43819728610 (C build-2), 44217665860 (P), 44239124993 (P's play and review, V, Y's
  spec), 44327129485 (X's spec-review, build-1 and build-2) and 44382984870 (X build-3 to
  review). Those hold 8 of the 25 Haiku sessions. The other runs' transcripts weren't read;
- diff sizes from the GitHub commits API, one call per step commit on each PR: engine files only,
  additions plus deletions, with JSON and images left out. All the M5 branches are deleted except
  `relay/341` and `relay/342`, so the PRs' commits are the only record.

"5h" means the 5-hour window's points and "wk" the weekly window's. Both are whole numbers, and
both include the owner's own use in the same hours, which can't be split out.

Names: the M5 proposal's L #336, J #352, S #353, R #354, C #337, G #355, X #356; the pieces it
moved to M6 but which ran in M5 anyway, P #338, V #339, Y #340, N #341, H #342; two pieces
opened mid-milestone, #426 ("What a clan carries", called **K** here) and #427 ("Settling, read",
**Q**); the eight map-art pieces #439 to #446 (**A1** to **A8**); and the fixes piece F #350.

## Verdict

- **Cost.** M5 cost $229.09 at list price, 2,693 session-minutes, at least 1,684 5-hour points
  and 135 weekly points (at least 156 by the window's range). That counts 22 merged pieces,
  N (closed and kept as `proof/341`), H (stopped after its clay, PR #418 open), the fixes piece
  and the check. m5.md §4 expected about $210 and fourteen windows for seven Fable pieces. The
  plan changed halfway, so the figures can't be checked against it as a whole (§1).
- **A whole piece on Fable cost $11.41, not $28 to $32.** The five whole Fable pieces with
  figures (L, S, R, C, G) cost $57.03 together, at 113 5-hour points and 9.4 weekly points each.
  That is 38% of the expected list dollars but 75% of the expected window. Their whole sessions
  did 74% of the pieces' dollars and 79% of their engine lines. What they handed on was the
  page, the red tests and the tuning (§2).
- **On the window, Fable is the dearest model.** In M5 Fable took 11.0 five-hour points per list
  dollar (all six sessions fall between 10.5 and 11.4). Opus took 5.9, Sonnet 7.7 and Haiku 6.9.
  In M4 every model took 5.0 to 5.5. A 5-hour window bought about $9 of Fable and $17 of Opus.
  In pieces, that is 0.9 of a whole Fable piece, 1.4 stepped game pieces (P, V, Y, X: 70 points
  each) or 1.7 art pieces (59 each). The owner's change of 19 Nov (stepped on Opus by default,
  Fable no longer a model a step can name) is what this shows.
- **The ladder predicted better than in M4.** Points rank with cost at 0.70 over 45 builds
  (M4: 0.48), and the size line's estimate ranks with the actual lines at 0.94 (M4: 0.75). Lines
  run 1.17x the estimate and files 1.30x. Only one build ran off the ladder: G build-1, a glaze
  at 6 points on Sonnet high, written by G's whole step.
- **Haiku still re-arms a Monitor.** V build-1 called Monitor 89 times and X build-2 124 times.
  Those two sessions cost $6.29, 28% of all Haiku spend, even though m5-relay.md §6 and
  `build.md` both say to wait on one call. That covers only the 8 Haiku transcripts read.
- **Design is cheaper, and a bigger share.** An Opus spec averaged $3.28 (M4: $4.37) and
  spec-review $1.41 (M4: $2.84). Builds averaged $1.67 (M4: $1.87). In the 13 finished stepped
  pieces, design was 45% of cost (M4: 35%), because the builds got cheaper. The fixes piece ran
  its spec on Opus for $5.62. M4's ran on Sonnet for $0.24.
- **The bound rule held.** N build-2 (Sonnet high canvas) reached its target with a home rule
  but would have turned two real-map tests red. It stopped and asked instead of loosening them,
  as m5-relay.md §6 says. The owner paused N and then closed it.

## 1. What ran

The plan changed twice during the milestone, and the ladder's figures follow those changes:

1. **12 to 17 Nov: whole on Fable.** L, S, R, C and G ran whole on Fable medium as m5.md
   planned. J was built by hand in the owner's session ("built by hand", #357), so it has no
   figures. Its PR #358 is 594 engine lines.
2. **17 to 19 Nov: the M6 pieces, stepped.** P, V, Y, N and H had been "moved to M6" and paused,
   but ran with Opus specs, as did X, re-cut from whole to stepped. N blocked at build-2 and was
   closed as `proof/341`. H stopped after its clay when K and Q were put ahead of it.
3. **19 Nov on: stepped on Opus by default** (a17cb46), a whole piece always ends in a review
   (0bc90df), a whole step is one window (59cd80f). K and Q ran whole on Opus medium. The
   eight map-art pieces (#439 to #446) were opened on 23 Nov and ran stepped, followed by the
   fixes piece.

So M5 holds three models of a piece: whole on Fable, whole on Opus and stepped on Opus. They are
compared below, but on different work.

## 2. Every step

How to read the table:

- **Ladder** is what `relay/prompts/spec.md` gives for the points and part: 2–3 Haiku medium,
  4–5 Haiku high, 6–8 Sonnet medium, 9–10 Sonnet high, 11+ Opus medium; `canvas`, `glaze` and
  `clay` Sonnet high at 8–10. A blank Ladder means the step isn't scored (whole, spec, review,
  play).
- **Lines** and **files** are the step's engine diff. For a whole step, they are the diff of its
  commits.
- **5h** is the block's figure. A* marks a step the window reset during, where the figure is a
  floor (§8).
- No session compacted. The highest peak context was 38% (X build-2, Haiku high).

| Piece | Step | Part | Pts | Ladder | Ran | Sess | List $ | Min | 5h | wk | Ctx | Lines / files (est.) | How it went |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| L | whole | | | | fable medium | 2 | 12.84 | 61 | 135 | 11 | 13% | 1,712 / 53 | Session 1 spent the window 0→102% in 47 min ($9.62) and its uncommitted work was dropped; session 2 committed the rules with ~30 tests red |
| L | build-1 | ford | 6 | sonnet medium | sonnet medium | 1 | 0.61 | 26 | 4 | 0 | 8% | 237 / 24 (300 / 14) | The red tests, tests only |
| L | play | | | | sonnet medium | 1 | 0.28 | 2 | 2 | 1 | 5% | | |
| L | review | | | | sonnet high | 1 | 0.89 | 29 | 6 | 0 | 12% | | Merge |
| J | whole | | | | (owner, by hand) | | | | | | | 594 / 33 (PR) | No play, no review |
| S | whole | | | | fable medium | 1 | 8.34 | 32 | 95 | 8 | 22% | 1,350 / 36 | The engine half; check.sh not run whole |
| S | build-1 | ford | 6 | sonnet medium | sonnet medium | 1 | 1.37 | 20 | 9 | 1 | 14% | 433 / 18 (350 / 8) | The page |
| S | play | | | | sonnet medium | 1 | 0.23 | 1 | 2 | 0 | 4% | | |
| S | review | | | | sonnet high | 1 | 0.49 | 8 | 3 | 0 | 9% | | Merge |
| R | whole | | | | fable medium | 1 | 7.20 | 56 | 81 | 7 | 19% | 1,051 / 45 | All of it, check.sh green; handed on only play and review |
| R | play | | | | sonnet medium | 1 | 0.38 | 3 | 3 | 0 | 6% | | |
| R | review | | | | sonnet high | 1 | 0.45 | 10 | 3 | 1 | 8% | | Merge |
| C | whole | | | | fable medium | 1 | 6.03 | 29 | 66 | 5 | 20% | 1,075 / 12 | The core rule; suite red, no sweep, no PR |
| C | build-1 | easel | 8 | sonnet medium | sonnet medium | 1 | 1.62 | 43 | 11 | 1 | 14% | 482 / 26 (500 / 10) | Suite, pass test, page |
| C | build-2 | canvas | 8 | sonnet high | sonnet high | 2 | 3.77 | 80 | 26 | 2 | 15% | 243 / 10 | Session 1 exited 1 with no result at 45 min |
| C | play | | | | sonnet medium | 1 | 0.28 | 2 | 2 | 0 | 5% | | |
| C | review | | | | sonnet high | 1 | 0.50 | 8 | 4 | 1 | 9% | | Merge |
| G | whole | | | | fable medium | 1 | 7.63 | 45 | 87 | 7 | 20% | 1,434 / 62 | Rules and judge; page, pins, check.sh handed on |
| G | build-1 | glaze | 6 | sonnet medium | sonnet high | 1 | 3.10 | 85 | 20 | 1 | 21% | 335 / 37 (350 / 12) | **Off: up.** A glaze under 8; the whole neither re-cut nor re-scored |
| G | play | | | | sonnet medium | 1 | 0.24 | 1 | 2 | 0 | 4% | | |
| G | review | | | | sonnet high | 1 | 0.78 | 14 | 5 | 1 | 11% | | Merge |
| P | spec | | | | opus medium | 1 | 1.84 | 8 | 10 | 1 | 10% | | Magnum re-cut to Easel and Canvas |
| P | spec-review | | | | opus medium | 1 | 1.17 | 4 | 6 | 0 | 8% | | |
| P | build-1 | easel | 7 | sonnet medium | sonnet medium | 1 | 1.15 | 17 | 8 | 1 | 12% | 407 / 14 (400 / 8) | The live driver |
| P | build-2 | canvas | 11 | opus medium | opus medium | 1 | 9.66 | 39 | 44 | 4 | 26% | 523 / 15 (500) | Played the page in Chrome, fixed three. 404 turns. M5's dearest build |
| P | play | | | | sonnet medium | 1 | 0.76 | 26 | 5 | 0 | 8% | | |
| P | review | | | | opus medium | 1 | 1.43 | 16 | 8 | 1 | 10% | | Merge |
| V | spec | | | | opus medium | 1 | 2.42 | 8 | 12 | 1 | 13% | | Henry Ford |
| V | spec-review | | | | opus medium | 1 | 1.57 | 5 | 8 | 0 | 10% | | |
| V | build-1 | ford | 5 | haiku high | haiku high | 1 | 2.33 | 20 | 12 | 1 | 31% | 1,451 / 18 (700 / 10) | 89 Monitor calls |
| V | build-2 | ford | 4 | haiku high | haiku high | 1 | 0.30 | 11 | 3 | 1 | 15% | 423 / 8 (450 / 6) | |
| V | build-3 | ford | 3 | haiku medium | haiku medium | 1 | 1.89 | 34 | 10 | 0 | 29% | 530 / 14 (— / 12) | 159 Bash calls, 304 turns |
| V | build-4 | ford | 3 | haiku medium | haiku medium | 1 | 0.78 | 23 | 5 | 1 | 18% | 211 / 9 (— / 9) | |
| V | review | | | | sonnet high | 1 | 0.61 | 15 | 4 | 0 | 9% | | Merge |
| Y | spec | | | | opus medium | 1 | 1.75 | 5 | 9 | 1 | 11% | | Henry Ford |
| Y | spec-review | | | | opus medium | 1 | 1.42 | 9 | 8 | 1 | 8% | | |
| Y | build-1 | ford | 3 | haiku medium | haiku medium | 1 | 0.05 | 20 | 0 | 0 | 8% | 2,359 / 10 (1,200 / 10) | data as a folder: a move |
| Y | build-2 | ford | 3 | haiku medium | haiku medium | 1 | 0.12 | 16 | 1 | 0 | 11% | 225 / 17 (200 / 12) | |
| Y | build-3 | ford | 3 | haiku medium | haiku medium | 1 | 0.14 | 27 | 1 | 0 | 12% | 264 / 8 (320) | |
| Y | review | | | | sonnet high | 1 | 0.30 | 13 | 2 | 0 | 6% | | Merge |
| N | spec | | | | opus medium | 1 | 3.52 | 28 | 18 | 1 | 17% | | |
| N | spec-review | | | | opus medium | 1 | 1.40 | 13 | 7 | 1 | 9% | | |
| N | build-1 | easel | 4 | haiku high | haiku high | 1 | 0.56 | 26 | 3 | 0 | 17% | 215 / 12 (250 / 10) | |
| N | build-2 | canvas | 10 | sonnet high | sonnet high | 1 | 2.73 | 70 | 22 | 2 | 21% | 0 | Blocked rather than loosen two real-map tests; piece closed |
| X | spec | | | | opus medium | 1 | 1.90 | 13 | 14 | 1 | 11% | | Clay, Kiln and Glaze |
| X | spec-review | | | | opus medium | 1 | 1.90 | 10 | 15 | 0 | 11% | | |
| X | build-1 | clay | 10 | sonnet high | sonnet high | 2 | 4.02 | 98 | 38 | 3 | 25% | 1,114 / 25 (750 / 12) | Wrote done, then hit the 90-min cap; second session $0.13 |
| X | build-2 | kiln | 5 | haiku high | haiku high | 1 | 3.96 | 49 | 26 | 1 | 38% | 755 / 33 (600 / 12) | 124 Monitor calls, 438 turns |
| X | build-3 | kiln | 4 | haiku high | haiku high | 1 | 0.77 | 20 | 5 | 1 | 20% | 373 / 15 (400 / 9) | |
| X | build-4 | glaze | 9 | sonnet high | sonnet high | 1 | 2.54 | 85 | 17 | 1 | 19% | 153 / 10 (150 / 3) | |
| X | play | | | | sonnet medium | 1 | 0.36 | 2 | 3 | 0 | 6% | | |
| X | review | | | | sonnet high | 1 | 0.55 | 12 | 4 | 1 | 10% | | Merge |
| H | spec | | | | opus medium | 1 | 5.57 | 50 | 31 | 3 | 21% | | M5's dearest spec |
| H | spec-review | | | | opus medium | 1 | 3.25 | 40 | 20 | 2 | 15% | | |
| H | build-1 | clay | 8 | sonnet high | sonnet high | 1 | 5.16 | 81 | 44 | 4 | 29% | 1,364 / 49 (750 / 22) | The herd on the actor. The rest of H waits |
| K | whole (1) | | | | opus medium | 1 | 8.18 | 118 | 50 | 3 | 24% | 3,472 / 172 (both, with main merged) | Review: not merge |
| K | review | | | | opus medium | 1 | 0.81 | 12 | 6 | 1 | 7% | | Not merge |
| K | whole (2) | | | | opus medium | 1 | 5.76 | 81 | 59 | 7 | 19% | (above) | The owner's widened piece |
| K | review ×2 | | | | opus medium | 2 | 1.41 | 23 | 9 | 0 | 8% | | Not merge; the owner merged |
| Q | whole | | | | opus medium | 1 | 5.69 | 51 | 39 | 2 | 20% | 1,008 / 40 | All of it; handed on play and review |
| Q | play | | | | sonnet medium | 1 | 0.37 | 5 | 4 | 0 | 6% | | |
| Q | review | | | | opus medium | 1 | 1.47 | 14 | 10 | 1 | 10% | | Merge |
| A1–A8 | spec ×8 | | | | opus medium | 8 | 26.61 | 94 | 152 | 13 | 10–21% | | $2.06 to $5.20 |
| A1–A8 | spec-review ×8 | | | | opus medium | 8 | 9.51 | 33 | 52 | 4 | 7–11% | | $0.83 to $1.71 |
| A1–A8 | easels ×13 | easel | 4–5 | haiku high | haiku high | 13 | 10.65 | 288 | 84 | 6 | 14–25% | 6,252 / 95 (5,640 / 114) | $0.31 to $1.53 |
| A8 | build-2 | easel | 8 | sonnet medium | sonnet medium | 1 | 1.14 | 45 | 8 | 1 | 11% | 184 / 8 (350 / 10) | |
| A1–A8 | canvases ×8 | canvas | 9–10 | sonnet high | sonnet high | 9 | 16.07 | 295 | 129 | 10 | 9–22% | 716 / 39 (1,620 / 73) | "Judged beside main"; A8's ran past the job's deadline into a second session |
| A1–A8 | reviews ×8 | | | | opus medium | 8 | 8.42 | 104 | 47 | 3 | 6–10% | | All merge |
| F | spec | | | | opus medium | 1 | 5.62 | 9 | 31 | 2 | 16% | | The crew writes Sonnet medium for a fixes spec; #350 says Opus |
| F | spec-review | | | | opus medium | 1 | 0.95 | 3 | 5 | 0 | 7% | | |
| F | build-1 | ford | 4 | haiku high | haiku high | 1 | 0.80 | 28 | 5 | 1 | 20% | 505 / 14 (250 / 11) | |
| F | build-2 | ford | 2 | haiku medium | haiku medium | 1 | 0.06 | 9 | 0 | 0 | 10% | 59 / 6 (40 / 5) | |
| F | review | | | | sonnet high | 1 | 0.34 | 10 | 2 | 0 | 7% | | Merge |
| retro | check | | | | sonnet medium | 1 | 0.32 | 2 | 3 | 0 | 5% | | Small 2, 110 turns |

The art pieces are grouped. The full rows are in the step blocks #447 to #509.

### Totals by kind of piece

| Kind | Pieces | List $ | Each $ | Min | 5h | Each 5h | wk | Each wk |
|---|---|---|---|---|---|---|---|---|
| Whole on Fable (L, S, R, C, G) | 5 | 57.03 | 11.41 | 555 | 566 | 113 | 47 | 9.4 |
| Whole on Opus (Q; K twice) | 2 | 23.69 | Q 7.53, K 16.16 | 303 | 177 | Q 53, K 124 | 14 | |
| Stepped game pieces, merged (P, V, Y, X) | 4 | 45.69 | 11.42 | 604 | 278 | 70 | 21 | 5.3 |
| Stepped art pieces (A1–A8) | 8 | 72.40 | 9.05 | 860 | 472 | 59 | 37 | 4.6 |
| Stepped, unfinished (N, H) | 2 | 22.19 | | 309 | 145 | | 13 | |
| Fixes (F) | 1 | 7.77 | | 58 | 43 | | 3 | |
| Check | | 0.32 | | 2 | 3 | | 0 | |
| M5 | 22 + F | 229.09 | | 2,693 | ≥1,684 | | 135 | |

| Model | Steps | Sessions | List $ | Share | Min | 5h | 5h per $ (M4) | wk per $ |
|---|---|---|---|---|---|---|---|---|
| Opus | 47 | 47 | 113.23 | 49% | 788 | 670 | 5.9 (5.3) | 0.47 |
| Sonnet | 37 | 40 | 51.41 | 22% | 1,108 | 395 | 7.7 (5.3) | 0.62 |
| Haiku | 25 | 25 | 22.41 | 10% | 573 | 155 | 6.9 (5.5) | 0.54 |
| Fable | 5 | 6 | 42.04 | 18% | 224 | 464 | 11.0 (5.0) | 0.90 |

By role: whole $61.67 (9 sessions), spec $49.23 (15), spec-review $21.17 (15), builds $75.35 (45
steps, 48 sessions), reviews $18.45 (22), play $2.90 (8), check $0.32.

### M4 against M5

| | M4 (7 pieces and fixes) | M5 (22 merged, 2 unfinished, fixes) |
|---|---|---|
| List $ | 149.36 | 229.09 |
| 5h points | ≥785 | ≥1,684 |
| Weekly points by step (by range) | 68 (73) | 135 (≥156) |
| Session-minutes | 2,084 | 2,693 |
| $ per build | 1.87 | 1.67 (75.35 / 45) |
| Opus spec, mean $ | 4.37 | 3.28 |
| Spec-review, mean $ | 2.84 | 1.41 |
| Design share of a finished stepped piece | 35% | 45% |
| Points ~ $ over builds (rank) | 0.48 | 0.70 |
| Estimated ~ actual lines (rank), actual / est. | 0.75, 1.12x | 0.94, 1.17x |
| Actual / estimated files | 1.5x | 1.30x |
| Haiku builds: steps, median $ | 21, 0.82 | 25, 0.76 |
| Steps needing a second session | 5 | 4 |
| A whole piece on Fable | R: $31.66, 160 5h | $11.41, 113 5h (five) |

Weekly by range: 70% to 97% from 12 to 13 Nov, then 2% to 89% from 15 to 20 Nov, then 5% to 47% from 23 to
24 Nov. The ramp stopped the crew from 13 to 15 Nov and from 20 to 23 Nov.

## 3. The whole step: what a window bought

m5-relay.md §1 asked what each whole step did itself, what it handed on, and the cost against
M4's stepped pieces.

| Piece | Whole $ / 5h / min | Engine lines, whole | What it handed on | Handed-on builds $ | Play + review $ | Piece $ / 5h |
|---|---|---|---|---|---|---|
| L | 12.84 / 135 / 61 (2 sessions) | 1,712 | ~30 red tests (ford, Sonnet medium) | 0.61 | 1.17 | 14.62 / 147 |
| S | 8.34 / 95 / 32 | 1,350 | the page, check.sh whole (ford, Sonnet medium) | 1.37 | 0.72 | 10.43 / 109 |
| R | 7.20 / 81 / 56 | 1,051 | nothing but play and review | 0 | 0.83 | 8.03 / 87 |
| C | 6.03 / 66 / 29 | 1,075 | suite and page (easel); retro maps and numbers (canvas) | 5.39 | 0.78 | 12.20 / 109 |
| G | 7.63 / 87 / 45 | 1,434 | page, pins, sweep, check.sh (glaze) | 3.10 | 1.02 | 11.75 / 114 |
| Five | 42.04 / 464 / 224 | 6,622 (79%) | | 10.47 | 4.52 | 57.03 / 566 |
| Q (Opus) | 5.69 / 39 / 51 | 1,008 | nothing but play and review | 0 | 1.84 | 7.53 / 53 |
| K (Opus) | 13.94 / 109 / 198 (2 rounds) | 3,472 with main | review only, three times | 0 | 2.22 | 16.16 / 124 |

- **A Fable whole session spends one window in 30 to 60 minutes.** Its sessions used 66 to 102
  points. L's first, with two helpers, reached 102% at 47 minutes. It died with nothing
  committed, and the crew then dropped its uncommitted work (f840e80 fixed both). Every later
  Fable whole session ended inside its window.
- **What a whole leaves.** A Fable whole session builds the rules, the planner, the judge group
  and a first sweep. It hands on what needs the whole test suite or the page: in four of five
  pieces, check.sh hadn't run whole when it finished. Only R finished everything.
- **Against stepped.** A whole Fable piece cost the same list dollars as a stepped game piece
  ($11.41 against $11.42) but took 1.6x the 5-hour points (113 against 70) and 1.8x the weekly
  (9.4 against 5.3). Q's Opus whole cost $7.53 and 53 points for a reading-only piece. The
  pieces differ, so this compares cost, not quality. The quality evidence is the reviews: all
  five Fable pieces merged on their first review. K's Opus whole failed its review twice.
- **m5.md's estimate was off in both directions.** It took M4's R ($28 to $32, 140 to 160 points)
  as the measure. In M5 a Fable dollar cost twice the window it did in M4, while the pieces cost
  under half the dollars. The window, not the dollars, decides how many Fable pieces fit.

## 4. Did the ladder predict?

These are Spearman rank correlations over the 45 builds with figures. Sizes come from each
`why`'s "about N files, M lines"; three `why`s gave no lines and two gave no files.

| Over | n | Points ~ $ | Points ~ 5h | Points ~ min | Est. lines ~ $ | Actual lines ~ $ | Est. ~ actual lines | Est. ~ actual files | Actual files ~ min |
|---|---|---|---|---|---|---|---|---|---|
| All builds | 45 | 0.70 | 0.71 | 0.50 | 0.14 | 0.05 | 0.94 | 0.67 | 0.23 |
| Non-Opus | 44 | 0.68 | 0.69 | 0.49 | 0.11 | 0.03 | 0.94 | 0.67 | 0.22 |
| Haiku | 25 | 0.38 | 0.48 | 0.05 | 0.47 | 0.45 | 0.95 | 0.77 | 0.25 |

- **Points predict cost again.** The canvases, clays and glazes sit on Sonnet high at 8 to 10
  and the easels on Haiku at 4 to 5. So the top of the scale buys a dearer model, which it
  didn't in M4, when the canvases ran a rung below.
- **Size predicts nothing about cost** (0.05), as in M4. The size line ranks well and runs a
  little low: 1.17x the lines and 1.30x the files. The big misses were H's clay (750 lines and 22
  files estimated, 1,364 and 49 actual), V build-1 (700 to 1,451) and Y build-1 (1,200 to 2,359,
  a pure move). G build-1 touched 37 files against 12 estimated, the pins and page scripts the
  M4 retro said the size line should count.
- **Within Haiku, points predict little** (0.38), and minutes not at all (0.05). Haiku's cost is
  its turns: X build-2 took 438 turns and V build-3 304.

### Off the ladder

One build, G build-1: a glaze at 6 points on Sonnet high. By the ladder a 6 is Sonnet medium,
and the mismatch rule says a glaze under 8 isn't holding the hard part: re-cut or re-score. G's
whole session wrote the step and did neither. It cost $3.10 over 85 minutes, about what a Sonnet
high glaze at 9 cost in X ($2.54). `whole.md` points at `spec.md` for `steps.json`, but not at
the scoring rule in so many words.

`spec.md`'s `steps.json` example still shows a canvas on `"model": "opus", "effort": "low"`,
which the ladder text above it forbids. No M5 step copied it.

## 5. The rungs

### Haiku

25 steps, $22.41, median $0.76 (M4: 21 steps, $0.82). The 13 art easels at 4 to 5 points cost
$0.31 to $1.53, with lines near their estimates. Haiku took judgement work and held.

The Monitor problem the M4 retro found is still there. In the eight transcripts read:

| Step | $ | Turns | Monitor | Bash |
|---|---|---|---|---|
| V build-1 (high) | 2.33 | 280 | 89 | 79 |
| V build-2 (high) | 0.30 | 82 | 2 | 25 |
| V build-3 (medium) | 1.89 | 304 | 1 | 159 |
| V build-4 (medium) | 0.78 | 177 | 0 (2 ScheduleWakeup) | 95 |
| X build-2 (high) | 3.96 | 438 | 124 | 146 |
| X build-3 (high) | 0.77 | 174 | 2 | 77 |

The two re-armers are 28% of all Haiku spend. V build-3's 159 Bash calls look like polling by
another route. Telling Haiku to wait on one call hasn't changed what it does. Taking the Monitor
tool away from a Haiku session would.

### Sonnet

- **Canvases at Sonnet high:** 10 steps, $22.57. The art canvases cost $0.69 to $2.75 for 46 to
  135 lines, 0.44x their estimated lines. They score 9 because "judged by eye" and "only play" add 7 before New and Reach, so
  they land on Sonnet high whatever they hold. C's canvas ($3.77, two sessions) and N's ($2.73)
  were the game's. N's stopped at the bound, as it should.
- **Clays at Sonnet high:** X's ($4.02) wrote done and then sat to the 90-minute cap. H's ($5.16)
  was M5's dearest Sonnet step, at 1,364 lines.
- **Fords and easels at Sonnet medium:** 5 steps at $0.61 to $1.62.
- **Reviews at Sonnet high:** 9, $0.55 each. All merged.
- **Play:** 8 steps, $2.90. P's play ran in the browser for 26 minutes and $0.76, the rest in
  about two minutes for $0.23 to $0.38.

### Opus

- **Spec:** 15, mean $3.28, from Y's $1.75 to F's $5.62 and H's $5.57. On settled pieces design
  outweighs the build: Y's design was $3.17 against $0.31 of builds, V's $3.99 against $5.30, and
  F's $6.57 against $0.86. The fixes piece's lines said Opus where the crew writes Sonnet medium.
  M4's Sonnet fixes spec cost $0.24.
- **Spec-review:** 15, mean $1.41. It halved from M4.
- **P's canvas at 11, Opus medium:** $9.66 in 39 minutes, 404 turns, M5's dearest build. It played
  the page in Chrome and fixed the three worst things. Nothing ran against it to say whether a
  lower rung would have done.
- **Reviews:** 13, $1.04 each. All eight art reviews and P's ran on Opus because the
  goal is judged by play or by eye, as `spec.md` says.
- **Whole:** K and Q, $19.63 over three sessions (above).

### Fable

Six sessions, all whole, $42.04, 11.0 five-hour points a dollar. Since 0bc90df no step can name
it.

## 6. Second sessions

| Step | Why | Second session |
|---|---|---|
| L whole (Fable) | The window ran out at 47 minutes, with nothing committed | $3.22 |
| C build-2 (Sonnet high) | Exit 1, no result, at 45 minutes (window at 15%): cause not read | $1.67, 35 min |
| X build-1 (Sonnet high) | Wrote `done`, then didn't end, and hit the 90-minute cap | $0.13, 8 min |
| A8 build-3 (Sonnet high) | The job's deadline | $?, in the next run |

None came from a stale pin, which caused four of M4's five. V moved the page scripts and tests
off the retro picks. Pins still moved: K's whole retuned `cases.rs` and `scout_errand.rs` for
new picks, and G's glaze touched 37 files. But each step that moved them counted and carried
them in its own session.

## 7. Specs, briefs and steps.json against the builds

- **The briefs held.** 45 builds and one block (N build-2), which was the brief's own stop rule
  doing its job. No build re-cut its piece, and none reports loosening a bound (K's whole retuned `cases.rs` and `scout_errand.rs` for new picks, and its review read them).
- **The shapes held, with one exception.** P was proposed as Magnum and its spec re-cut it to
  Easel and Canvas. That was right for cost: a $1.15 easel under a $9.66 canvas, where a magnum
  would have run Opus for the whole.
- **Whole steps' `steps.json`.** The five Fable wholes wrote six builds between them: two fords,
  an easel, a canvas and a glaze. Five were on the ladder. Their briefs worked: every handed-on
  build finished in one session except C's canvas.
- **Expect lines.** Easel and Canvas expects "Haiku easels, Sonnet medium for an easel at 6–8,
  and a Sonnet high canvas", which is what every Easel and Canvas piece ran. Clay, Kiln and Glaze
  expects a Sonnet high or Opus medium clay and glaze and a cheap kiln: X ran just that. Henry
  Ford expects Haiku, as V, Y and F ran (L's and S's fords at 6 went to Sonnet medium by points).

## 8. What the blocks can't measure

- **J** has no block figures: the owner built it in their own session.
- **5-hour points across a reset.** A1 build-2 reads 49 to 1, recorded as 1. At Sonnet's 7.7 a
  dollar it took about 5. Other resets fell between steps. M5's total is a floor, a few points
  low.
- **Weekly points by step** (135) undercount the range (at least 156). The difference is J, the
  owner's own sessions on the crew (four Relay commits on 19 Nov) and rounding.
- **Owner's share.** Fable's 11 points a dollar is steady across six sessions, which argues
  against the owner's use explaining it. It can't be ruled out.
- **Monitor and turns** were read for 8 of 25 Haiku sessions. The other runs' transcripts are
  still downloadable until about 1 December.
- **A whole session's helpers** are in its cost: L's first session ran two Agent helpers (161
  helper messages).

## 9. The ladder's next text

These are edits to the prompts, written so the proposal can take them as they stand.

1. **The example in `spec.md`.** In the `steps.json` block, replace
   `"part": "canvas", "model": "opus", "effort": "low", "points": 10` with
   `"part": "canvas", "model": "sonnet", "effort": "high", "points": 9`. Nothing runs Opus low (§4).
2. **The whole step scores its builds.** In `relay/prompts/whole.md`, after "Write
   `relay/specs/<piece>/steps.json` as `spec.md`", add: "Score each build on `spec.md`'s ladder,
   the mismatch rule included: a canvas, glaze or clay under 8 is re-cut or re-scored, and its
   `why` says which." Evidence: G build-1 (§4).
3. **Haiku waits in the foreground.** In `relay/prompts/build.md`, replace "wait on it with one
   call" with "run it in the foreground with a ten-minute timeout: one call, no Monitor, no sleep
   loop". In the ladder's notes, keep "Haiku's cost is its turns", and add "and a Haiku session
   has no Monitor tool" if the crew takes the idea below. Evidence: 89 and 124 Monitor calls,
   28% of Haiku spend (§5).
4. **Fixes specs stay on Sonnet.** In `relay/README.md`'s fixes line, after "on Sonnet medium",
   add "for its spec and spec-review too: its findings are settled". Evidence: F's Opus spec
   $5.62 for $0.86 of builds; M4's Sonnet spec $0.24 (§5).
5. **The size line.** Keep it. It ranks at 0.94. Change "(files, rough lines, counting tests,
   pins and `engine/CLAUDE.md`)" to "(files, rough lines, counting tests, pins, the page scripts
   it retunes and `engine/CLAUDE.md`; a move counts its lines twice)". Evidence: files 1.30x low,
   Y build-1's move at 2x (§4).
6. **The whole line.** In `relay/README.md` and the planner, beside the Whole line: "A whole
   session runs Opus medium. On Fable a whole piece took 1.6x the 5-hour points of a stepped one
   for the same list dollars." The code already says no step names Fable (0bc90df); this gives
   the reason (§3).
7. **Nothing else moves.** The points table, the Sonnet high band for canvas, glaze and clay,
   Opus medium from 11, and the expect lines all predicted what ran (§4, §7).

## For other angles

- **relay**: L's first Fable session spent the window in 47 minutes and lost its work. f840e80
  fixed the loss, and every later whole ended inside its window. X build-1 wrote `done` and then
  ran to the 90-minute cap, and C build-2 exited 1 with no result: both are worth reading in
  their transcripts. The weekly ramp stopped the crew for about two days twice (13 to 15 Nov and
  20 to 23 Nov). #350's Spec line says Opus where the crew writes Sonnet medium.
- **architecture**: V moved the page scripts and tests off the retro picks, and no second session
  came from a stale pin (M4: four). Pins still move with a pick-moving change (K, G), but inside
  the step. H's clay ran 1.8x its lines estimate on the actor.
- **game**: N is closed as `proof/341` and H's PR #418 is open after its clay. K's whole failed
  its review twice, and the owner merged it. The art pieces were the cheapest pieces of M5
  ($9.05 each) and every one merged on its first review.
- **players**: the play steps cost $0.23 to $0.38 in about two minutes, except P's, which played
  the page in Chrome for 26 minutes and $0.76. Whether the play step should play the page is the
  players angle's call.
- **retro and proposal**: a stepped game piece costs about $11 and 70 five-hour points, and an
  art piece $9 and 59. A whole Fable piece takes 113 points. At about 5 weekly points a stepped
  piece, the 135 weekly points M5 spent would buy about 25 stepped pieces.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| A whole step runs Opus, not Fable: same list $, 1.6x the window | §3 | high | none (built in 0bc90df) | take, with the reason written down |
| The crew takes the Monitor tool away from Haiku sessions | §5: 89 and 124 calls, 28% of Haiku $ | high | small | take |
| `build.md` says to run check.sh in the foreground, one call, no Monitor or sleep loop | §5 | medium | small | take |
| `whole.md` scores its builds on the ladder, mismatch rule included | §4: G build-1 | low | small | take |
| Fix `spec.md`'s steps.json example (opus low canvas) | §4 | low | small | take |
| A fixes piece's spec and spec-review run Sonnet medium, as the crew writes | §5: $5.62 against $0.24 | medium | small | take |
| The size line counts a move's lines twice and the page scripts it retunes | §4: files 1.30x low | low | small | take |
| Try a Sonnet high spec on one Henry Ford piece, with Opus spec-review | §5: Y's design $3.17 against $0.31 of builds | medium | small | try |
| Score a "judged by eye" canvas that finds no rule (art) lower than one that tunes | §5: art canvases at 9, $0.69–2.75 | low | small | later |
| The crew records a reset in a step's usage | §8: one step | low | small | drop (one step in M5; the floor is a few points) |
