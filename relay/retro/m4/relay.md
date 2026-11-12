# M4 retro: Relay

The crew, the workflows, the waits, the prompts and the loop without the owner, for M4's eight
pieces and this retro's first steps.

Evidence: the 32 `relay.yml` runs from 42279350480 (6 Nov 19:24) to 43129906988 (this one), listed
in [`relay/runs.txt`](relay/runs.txt); the `crew.log` of every run that kept one, merged in
[`relay/crew.log.txt`](relay/crew.log.txt) (the artifacts expire on 20 Nov); the 70 saved prompts
and transcripts in those artifacts; the relay blocks on step issues #158–#314; PRs 159, 178, 204,
226, 245, 268, 294, 301 and their bodies; `relay-events` runs; `ci` and `site` runs since 6 Nov;
the timelines of #155 and #157; `relay.mjs`, `relay.yml`, `relay/prompts/` and their `git log`.
Times are UTC. "My read" marks a judgement; everything else is counted. Cost per step and per
model is the ladder angle's (`ladder.md`) and isn't repeated here.

## Verdict

The crew ran M4 clean and without the owner's hands on the pieces. 69 sessions over eight pieces,
eight merges by the crew on the first try, each within 25 s of its review. No crash, no timeout,
no compaction. One session died on the 5-hour limit (R's whole, at 102%), one step blocked (F
build-1), and five steps took a second session. The milestone-built rule tagged `m4` and opened
this retro two seconds after the fixes piece merged. Of the M3 retro's 18 crew and prompt fixes,
16 landed and work; one landed and changed nothing (build decisions still average 4.5); one is
unused (`/relay ready`).

The owner stopped being the bottleneck: 5% of M4's wall clock against 58% in M3. The weekly ramp
took its place. M4 ran 104.7 hours from first session to last merge: sessions 33%, the weekly
ramp 52%, 5-hour waits 9%, the one blocked step 5%. Every weekly stop came at the ramp's line
(14% against 13.6, 28/27.1, 42/40.7, 55/54.3), so the crew spent each day's share and the ramp,
not the crew, set M4's pace. What the crew lacks is a restart when the ramp lifts: a weekly stop
starts no wait run, so the crew sat 2.5, 15.9 and 14.5 hours past the lift, until the owner or the
daily schedule (3.6 to 7.3 hours late) started it. That cost no points, but it put M4's finish
about 15 hours late and this retro's angles another 16.

The loop without the owner ran end to end. It also shows where it leaks. 6 findings were `fix`
and 77 `ask`, so 93% of what sessions noticed went to the owner as `relay:proposed` issues. 85
are open and none has been promoted. The play steps file the same asks piece after piece under
new titles (four clusters of four), because nothing acts on an ask until the retro. And the
owner changed the crew three times mid-milestone by hand (the twin, the whole step, the
handover), outside the one Relay change m4-relay.md planned.

## 1. The crew in M4

### Runs

| Kind | Runs | Ids |
|---|---|---|
| Ran sessions | 14 (and this one) | 42284235074, 42320809361, 42344509296, 42365743989, 42375878008, 42475829825, 42512834771, 42528831144, 42731568415, 42736828759, 42923500975, 42947799559, 42952767977, 42960440614; 43129906988 |
| `wait` runs | 5, plus 3 duplicates cancelled within a minute | 42306770851, 42335658217, 42361578962, 42767156711, 42935458094; cancelled 42306692118, 42361540011, 42767092997 |
| Queued behind a working run, then re-stopped | 7 | 42304949489, 42346442922, 42765612338 (piece closes, 5-hour); 42407439254, 42542101113 (piece closes, weekly); 42784677816 (a wait's run, weekly); 42935366133 (the whole's handover, 102%) |
| Scheduled no-ops | 2 | 42279350480 (before the pieces opened), 42418039480 (weekly) |

The duplicate waits that bred in M3 now cancel: the `relay-wait` group keeps the newest. A piece's
close starts a run every time (8 of 8), which queues behind the working run and re-stops if the
window is spent; harmless.

### Per piece

| Piece | First session | Merged | Wall | Session min | List $ | Inside the wall |
|---|---|---|---|---|---|---|
| S #151 | 6 Nov 19:58 | 6 Nov 22:28 | 2 h 30 | 144.5 | 9.15 | |
| E #152 | 6 Nov 22:28 | 7 Nov 06:18 | 7 h 50 | 212.5 | 21.52 | two 5-hour stops, 4 h 14 |
| T #153 | 7 Nov 06:18 | 7 Nov 13:12 | 6 h 54 | 348.9 | 18.61 | one 5-hour stop, 1 h 00 |
| D #154 | 7 Nov 13:12 | 7 Nov 19:41 | 6 h 29 | 385.3 | 19.85 | |
| F #155 | 7 Nov 19:41 | 9 Nov 00:47 | 29 h 06 | 451.9 | 22.79 | weekly 15 h 46; blocked 5 h 41 |
| W #156 | 9 Nov 00:47 | 10 Nov 06:00 | 29 h 13 | 325.7 | 24.53 | weekly 23 h 42 |
| R #157 | 10 Nov 06:00 | 11 Nov 03:26 | 21 h 26 | 145.4 | 31.66 | 5-hour 2 h 29 and 1 h 44; weekly 14 h 45 |
| X #162 | 11 Nov 03:26 | 11 Nov 04:37 | 1 h 11 | 68.0 | 1.00 | |
| M4 | 6 Nov 19:58 | 11 Nov 04:37 | 104 h 39 | 2,082 | 149.11 | |

Session minutes and dollars are the `end` lines of `crew.log`, R's including its discarded stepped
spec #289. M3 for comparison: 57 h 15, 769 session minutes, $71.40.

### Where the hours went

| | M4 hours | Share | M3 share |
|---|---|---|---|
| Sessions | 34.7 | 33% | 22% |
| Weekly ramp stops | 54.2 | 52% | 0% |
| 5-hour stops | 9.5 | 9% | 20% |
| The owner (F build-1 blocked) | 5.7 | 5% | 58% |
| Crew overhead (step issues, merges, CI waits, queueing) | 0.6 | 1% | |

### The 5-hour stops

| Stop | Window | Reset | Next session | Before it |
|---|---|---|---|---|
| 6 Nov 22:42 | 79% | 00:50 | 00:52:01 | E spec |
| 7 Nov 03:48 | 80% | 05:50 | 05:52:01 | E build-5 |
| 7 Nov 09:52 | 82% | 10:50 | 10:52:01 | T build-3 |
| 10 Nov 06:13 | 90% | 08:40 | (weekly stop at 08:42) | R spec #289, Opus, 18 points from 72% |
| 11 Nov 00:58 | 102% | 02:40 | 02:42:04 | R whole session 1, killed by the limit |

Every restart came two minutes after the logged reset, as in M3. The flat 75% line made the same
kind of decision `EXPECTED_POINTS` did and nobody had to keep a table: the window ended between
79% and 90% at every stepped stop. It doesn't guard a whole step. That step starts under 10% and
can still spend the window (below).

### The weekly ramp

The weekly window reset on 7 Nov between 08:42 and 09:03 (T build-2's readings, 70% to 0%, and
the ramp's day numbers), so each ramp day begins near 09:00. The ramp lets the window reach
95% × day / 7.

| Stop | Reading | Lifts | Restarted | By | Past the lift |
|---|---|---|---|---|---|
| 7 Nov 19:41, after D merged | 14% ≥ 13.6 (day 1) | 8 Nov ~09:00 | 8 Nov 11:27 | the owner, from Actions, right after pushing the twin | 2.5 h |
| 9 Nov 01:10, after W's spec | 28% ≥ 27.1 (day 2) | 9 Nov ~09:00 | 10 Nov 00:52 | the owner, from Actions | 15.9 h |
| 10 Nov 08:42, at the wait's restart | 42% ≥ 40.7 (day 3) | 10 Nov ~09:00 | 10 Nov 23:27 | the schedule, 5 h 10 late | 14.5 h |
| 11 Nov 04:40, after this retro's check | 55% ≥ 54.3 (day 4) | 11 Nov ~09:00 | 12 Nov 00:47 | the schedule, 6 h 30 late | 15.8 h |

`runSteps` computes the lift as `until` and puts it on the board, but it calls `dispatchWait` only
for a 5-hour stop (`relay.mjs:1593`). A weekly stop waits for the schedule (`17 18 * * *`), which
fired 1 h 07, 3 h 37, 3 h 52, 7 h 18, 5 h 10 and 6 h 30 late in M4, and once a day.

Each stop came within 1.3 points of the line, so every ramp day's share was spent before the next
day began. My read: the lag cost wall clock and no points. R's whole and the fixes piece could
have started near 09:00 on 10 Nov instead of 23:27, and this retro's angles near 09:00 on 11 Nov
instead of 00:47 on 12 Nov. The weekly window stood at 55% the whole 20 hours, so the owner spent
none of it either.

The ramp is what paces Relay now. M4 took 68 weekly points (73 by the window's range, `ladder.md`),
about five ramp days. At about 9 weekly points a stepped piece, a ramp day carries one and a half
pieces.

### Failures

- **Second sessions.** Five steps took two (`ladder.md` §7 has why). Four were stale pins and page
  scripts. Every pending note said what was left and the next session finished it: T build-1
  (21.8 min, $0.05), D build-4 (63.3 min), F build-4 (23.8 min), R whole (43 min). T build-1's first
  session, on Haiku medium, chose `pending` at 45 of its 90 minutes with seven page scripts to
  retune. It ran `date -u` and didn't cite time. The second session did the retune in 22 minutes.
- **The one blocked step.** F build-1 on Haiku high blocked at 13:19 on 8 Nov: `scout_errand` is a
  snapshot of one float path. The owner answered at 18:58–19:00 with `/relay model build-1 opus
  medium`, `/relay model build-2 opus medium` and "Fix or remove the test". The reply ran a crew
  run at once (42512834771). 5 h 41 of wall, and the only owner stop on a piece.
- **The whole's death.** R whole session 1 started at 1% and ran 89.3 minutes to the 5-hour
  limit (`api_error: usage_limit_reached`, 429; 127 turns, $24.31). The crew logged `exit 1`,
  `result null`, "session left uncommitted changes; dropping them", and counted an attempt. The
  session had committed four times, the last at 00:26, so 31 minutes of work went. The second
  attempt's prompt said "The last attempt didn't finish. Pick up from here." with no note and no
  word on why. It finished in 43 minutes. A second death on the limit would have blocked the step.
- **Crashes, timeouts, compactions:** none. Longest stepped session 88.1 minutes (F's twin)
  against the cap of 90. Highest context 45% (E build-2, Haiku high).
- **The usage probe** failed once (15:50 on 7 Nov, after D build-2: direct HTTP 403, the probe had
  no windows), so D build-2 has no points.
- **Deadline handovers.** Four ("no time for another step in this job" three times, and the whole
  needing a 300-minute job). Each next run was already queued or dispatched, so none cost time.
- **CI.** 8 failures in 84 finished `ci` runs since 6 Nov. Seven are mid-step pushes on `relay/152`–`155`
  that were green before their review. One is on main: `save.mjs` timed out waiting for turn 242
  on the merge of E (42346427221, 06:18 on 7 Nov). The PR's head was green, and the next main run
  passed with nothing changed in between, so it is a flake in a page script.
- **Merges.** 8 of 8 by the crew on `merge: true`, 10–25 s after the review ended, each review
  having watched CI on its own head commit. No merge waited for the owner.
- **Pages.** 85 `site` runs succeeded and 21 were cancelled by newer ones; none failed.
- **The board.** `RELAY_BOARD_TOKEN` now reaches `relay-events`. No board error in any log. What
  the board shows: unchecked.

### Mid-milestone changes to the crew

m4-relay.md says the Relay change "lands as one hand-built change … before any M4 piece opens"
(#150, 6 Nov 19:57). Three more landed by hand while M4 ran:

| Commit | When | What | What it cost |
|---|---|---|---|
| 85868ed | 8 Nov 11:23, before F's spec | The twin and the blind pick | Planned in m4-relay.md §7 ("From #155 on"), built late. F and W twinned. The twin cost $9.28 (`ladder.md` §3) |
| 1c3f219, 79a9847, 7f16d67 | 10 Nov 05:00–08:55 | The `whole` step; whole.md; its note on the 5-hour window | R re-cut by hand: the owner closed spec-review #292 and removed its steps (09:19–09:28 on 10 Nov). R's Opus spec, $3.46, was discarded |
| 688d98e | 10 Nov 09:15 | A crew hands over when main's crew code moves | Never fired: no log line in M4 |

All three fell inside weekly stops, so none cost wall clock. My read: the twin was a planned
experiment and worth it. The whole was the owner's own call on R, so it is owner time M4 didn't
count. The handover rule is what makes hand changes safe while the crew runs.

## 2. The prompts

### The M3 retro's fixes (m3-relay.md §5)

| # | Fix | Landed | Did it change behaviour | Evidence |
|---|---|---|---|---|
| 1 | `merge` required | yes | Yes: 8 of 8 reviews and the whole carry it; no review went `pending` | crew.log `merged PR` ×8 |
| 2 | A close starts a run | yes | Yes: 8 dispatches, no restart by hand after a merge | runs.txt |
| 3 | One wait at a time | yes | Yes: 3 duplicates cancelled in under a minute | 42306692118, 42361540011, 42767092997 |
| 4 | Screenshots at the commit | yes | Yes: 5 of 5 page PRs link `blob/<sha>` | PR 178, 204, 226, 245, 268 |
| 5 | The crew writes the review's row | yes | Yes: PR 159's review row reads 16 minutes (16.1 actual) | PR 159 |
| 6 | Drop "Under five is usual" | yes | No: build decisions average 4.5 a step (M3 4.5), max 9. Exactly five fell from 13 of 22 builds to 8 of 33, and 2 of 33 builds wrote none, against "a build that follows its brief usually has none" | step blocks |
| 7 | `engine/CLAUDE.md` byte budget | yes | Yes: 8,176 of 8,192 bytes at M4's end, where M3 went 5.2 to 20.1 KB. But 16 bytes are left (`architecture.md`) | `check.sh` |
| 8 | A rule in a band goes in the verdict | yes | Yes: D's and F's reviews put the `scout_errand` loosening under "A test floor that encodes a rule" for the owner | PR 226, 245 |
| 9 | Flat 75% start | yes | Yes: stops at 79–90%, no table to fit | §1 |
| 10 | Milestone built; tag | yes | Yes: `m4` on 7417e2d, retro #306 opened at 04:37:24 | crew.log |
| 11 | The play step and the check | yes | They ran: 5 play steps at 1.1–1.7 minutes and about $0.20, all on small 2 as civ 1, and the check. What they found is the players angle's | step blocks |
| 12 | Fix or ask, the fixes piece | yes | It ran (§3) | |
| 13 | The retro run | yes | Running (§4) | |
| 14 | `/relay ready` | yes | Unused: 0 of 85 proposed issues promoted | |
| 15 | Start time and cap in the prompt | yes | Yes: 70 of 70 transcripts run `date -u`. No session quit early for time | transcripts |
| 16 | Board token in `relay-events` | yes | No board error logged | |
| 17 | A daily schedule | yes | It fired 6 times, 1–7 hours late. It started work 3 times, after weekly stops that nothing else would restart | runs.txt |
| 18 | The log rounds usage | yes | Yes | crew.log |

### The M4 lines (m4-relay.md §3–§9)

| Line | What it changed | Evidence |
|---|---|---|
| The judge's block "at the top of the PR description" (§3, `review.md`) | The reviews from D on lead their own text with "Judge: …". The crew's review prompt also says "Put this table at the top of the PR description, under Closes" (`relay.mjs:475`). So every PR opens with the 11-column, 10–14-row steps table and the verdict comes under it. E's and T's PRs have no judge block | PR 178, 204, 226 |
| "Re-run the earlier pieces' questions" (`review.md`) | Done in full by W's review ("#155's farming scores all hold or rise"). Not in S, E or T, where there was little to re-run | PR 268 |
| Every new button pressed by a script (§3) | `buttons.mjs` exists (S). It found Keep ±1, which X fixed | #162 |
| The summary is "what later steps must know, not the diff" (§9, `session.md`) | Build summaries average 1.8 KB (M3 2.1), max 3.5 KB. "So far" in a build prompt averages 9.4 KB (M3 10.4), max 23.7 | step blocks, prompts |
| whole.md: "a session of up to five hours" with a note on the window | The note was added on 10 Nov (7f16d67). Session 1 still spent the window in 89 minutes: Fable medium takes about 1.1 five-hour points a minute (`ladder.md` §6) | crew.log |
| whole.md: "Commit and push regularly, a second attempt has only the branch" | Followed: four commits in session 1. The crew dropped the last 31 minutes | PR 294 commits |

### What a session reads

| Kind | n | Prompt (avg, range) | "So far" (avg, max) |
|---|---|---|---|
| spec | 8 | 29.0 KB (15.4–41.6) | none |
| spec-review | 7 | 30.2 KB (17.9–44.1) | 1.5 KB, 2.0 |
| build | 37 | 19.1 KB (7.5–32.6) | 9.4 KB, 23.7 |
| pick | 2 | 36.1 KB (29.4–42.8) | 13.9 KB, 17.0 |
| play | 5 | 36.4 KB (32.3–39.9) | 18.4 KB, 22.2 |
| review | 7 | 68.0 KB (25.6–86.9) | 17.8 KB, 27.9 |
| whole | 2 | 31.6 KB | none |
| retro check | 1 | 19.2 KB | none |

Prompt sizes are flat against M3 (builds 18.4 KB, reviews 69.5). The play step is the outlier: half
its prompt is every build's summary and decisions, for a session that plays 100 turns in under
two minutes through `sansfiat play` with `player.md`. My read: the player needs the piece's goal
and what is new to press, not the builds' decisions.

### Lines that misled

1. The crew's review prompt, "Put this table at the top of the PR description" (`relay.mjs:475`),
   against `review.md`'s order: the judge's block, then the table. Every M4 PR puts the verdict
   under the table, which is the part of the body the owner reads on a phone.
2. `session.md`, "A build that follows its brief usually has none": 31 of 33 builds wrote some,
   4.5 on average.
3. `whole.md`, "a session of up to five hours": one Fable window lasts about 90 minutes of it.
4. The crew's "The last attempt didn't finish. Pick up from here." after a session the usage limit
   killed: it doesn't say why, what was dropped, or the last commit.

## 3. The loop without the owner

### Fix or ask

| | Fix | Ask |
|---|---|---|
| builds (incl. twins) | 3 | 26 |
| reviews and the whole | 2 | 20 |
| spec and spec-review | 1 | 11 |
| play steps | 0 | 11 |
| picks | 0 | 4 |
| this retro so far (check, ladder, architecture) | 0 | 13 |
| total | 6 | 85 |

- **The fixes piece worked.** The crew opened #162 at the first `fix` (20:22 on 6 Nov) and
  appended six findings (two the same Keep ±1 bug, by different titles). It ran last, as a Henry
  Ford piece on Sonnet and Haiku: spec in 1.0 minute, spec-review 1.7, two builds, a Sonnet high
  review. 71 minutes and $1.00, merged with "same" on every score and each repro a test. M3's
  analogue, H, cost $7.62.
- **Asks pile up.** 85 `relay:proposed` issues are open (#160–#328), none promoted. The
  title check skips none of them, but the same ask comes back under a new title:

  | What | Issues |
  |---|---|
  | A gift shows no effect / doesn't say whom it reaches | #220 (T play), #239 (D play), #296 (R), #320 (check) |
  | Nothing to decide for most of 100 turns | #199 (E play), #286 (W play), #299 (R), #317 (check) |
  | A scout's route order shows no result | #219 (T play), #222 (T review), #297 (R), #319 (check) |
  | `scout_errand` and the scout's first errand | #241, #256, #265 |
  | The starving bounds read noise | #257, #259, #261 |
  | The camp hold waits on R | #189, #192, #201 |

  The play steps meet the same game each piece, on the same map and civ, and nothing acts on an
  ask until the retro. So each play step mostly re-files what the last one found.
- **Classes held to the rule.** My read of the four-cluster asks: each is a new lever, a page
  readout no line names, or a "less fun" with no line, so `ask` is right under the rule as written.
  The rule sends almost everything a player meets to the owner, because the design rarely names
  a readout.

### The owner's stops in M4

| Stop | M3 | M4 |
|---|---|---|
| Blocker replies | 0 | 1 (F build-1, 5 h 41) |
| Merges left to the owner | 1, 33 h, by a missing flag | 0 |
| Restarts by hand | 1 after a merge | 2 after weekly stops (8 Nov 11:27, 10 Nov 00:52) |
| Crew changes by hand mid-milestone | 0 | 3 (§1) |
| Re-cutting a piece by hand | 1 (G's split) | 1 (R to whole, its steps removed) |
| Starting the retro | by hand, after 3.4 idle days | automatic, 2 s after the last merge |
| Proposed issues to read | 68 ideas, none filed | 85 issues, none promoted |

The owner also opened #216 (7 Nov): the Relay log issue (#12) should be one per milestone, link
each update to its step issue, and give the weekly limit beside the 5-hour one. The crew still
posts every run to #12.

## 4. The retro run so far

| Step | Ran | Minutes | List $ | 5h | Note |
|---|---|---|---|---|---|
| check | 11 Nov 04:37 | 1.6 | 0.25 | 2 | Then a weekly stop at 55% (day 4): 20 h to the next run |
| ladder | 12 Nov 00:48 | 11.6 | 3.54 | 19 | |
| architecture | 12 Nov 01:00 | 10.0 | 2.80 | 14 | |
| relay | 12 Nov 01:10 | this session | | | |

The three angles each read the artifacts with the crew's `gh` (`actions: read` works). The
72-hour wait starts when this step ends, so with no owner notes `game` runs near 01:30 on 15 Nov.
The oldest artifacts (S, 6 Nov) expire on 20 Nov, after `retro` and `proposal` would usually run.
This file keeps the crew logs in `relay/` in case they don't.

## 5. What could come out

- **The play step's "so far".** Give the play step the piece's goal and what's new, not every
  build's summary and decisions: half of a 36 KB prompt. Try.
- **The play step on the same map every piece.** Five play steps and the check all played small 2
  as civ 1, and four findings repeat across them. Either vary the map, or drop the play step from a
  piece whose `Play:` question the check already covers. The players angle's call.
- **The spec's decisions on spec-review.** Specs write 9.6 decisions on average (max 16). Only
  spec-review sees them, and it keeps 2.2. Nothing to cut; noted for the briefs.
- Not the daily schedule: it was the only thing that restarted the crew after two weekly stops.
- Not the crew's CI wait before merging: every review waited for its own CI, and the crew's check
  still costs one call.

## 6. Fixes for the M5 Relay change

1. A weekly stop starts a wait for its lift, as a 5-hour stop does. The wait job caps at 340
   minutes, so it sleeps up to its cap, and the run it starts waits again if the lift is further
   off (§1: 2.5 + 15.9 + 14.5 + 15.8 hours past the lift).
2. A session the usage limit ends (`api_error: usage_limit_reached`) doesn't count as an attempt.
   The crew commits nothing for it, but it tells the next session why it ended and gives the last
   commit (R whole session 1: 102%, 31 minutes dropped, "Pick up from here").
3. A whole step plans for two windows. whole.md's "up to five hours" says a Fable window lasts
   about 90 minutes, and to commit before each test run (`ladder.md` §6).
4. The crew's review prompt puts the steps table under the judge's block, not at the top (every
   M4 PR; `relay.mjs:475` against `review.md`).
5. The 5-hour reading across a reset (`ladder.md` §8, #322): the crew has the reset time, so it
   counts (100 − before) + after.
6. Builds wait on a long check with one call, not by re-arming a Monitor (`ladder.md` §4, #321:
   four Haiku sessions, $12.53).
7. Repeated asks: before filing an ask, the crew gives the session the open `relay:proposed`
   titles from the same piece's earlier steps, or files the play step's findings as comments on
   one issue per piece. My read: the second is simpler (four clusters of four).
8. `session.md`'s build decisions: say which kinds count (a rule, a number or a scope line the
   brief left open), with an example of what doesn't, or drop the field for builds. "Usually has
   none" didn't move the average.
9. #216: one Relay log issue per milestone, each line linking its step, with the weekly window
   and the ramp's line beside the 5-hour reading.
10. `save.mjs`'s wait for turn 242 timed out once on main (CI run 42346427221). Give
    it the wait the other scripts have, or mark it flaky. My read: `fix` if it repeats.

## For other angles

- **Ladder:** cost per step, per model and per piece are yours, and the five second sessions. The
  crew's minutes and dollars here match your table to the cent.
- **Architecture:** `engine/CLAUDE.md` has 16 bytes left under its budget, so the next module forces
  a rewrite. The four second sessions from pinned picks are the case for staging fixtures off the
  retro pick.
- **Game:** two builds loosened `scout_errand`, and D's and F's reviews put it under "a test floor
  that encodes a rule" (PR 226, 245). The owner merged both; the ledger should say which bound
  stands.
- **Players:** five play steps and the check all played small 2 as civ 1, in 1.1–1.7 minutes. The
  gift, the scout's route and "nothing to decide" each came up four times (§3). R's whole played
  three seeds; no play step did.
- **Retro and proposal:** the weekly ramp, not the crew, is Relay's pace: one and a half stepped
  pieces a ramp day. 85 proposed issues need a triage. Most are play findings that repeat, so a
  proposal can close a cluster with one piece. The fixes piece is cheap ($1.00), so a looser `fix`
  rule for page readouts would cost little.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| A weekly stop starts a wait for its lift | §1 weekly table; `relay.mjs:1593` | About 15 h off a milestone's finish and off a retro's start; no restarts by hand | A few lines; the wait chains past 340 minutes | take |
| A usage-limit death isn't an attempt, and the next prompt says why | R whole, run 42923500975 | A whole can't block on its own window | Read `api_error` in the result frame | take |
| Steps table under the judge's block | PR 159–301; `relay.mjs:475` | The verdict is the first thing on a phone | One line in the crew's review prompt | take |
| whole.md plans two windows | crew.log 11 Nov 00:57; `ladder.md` §6 | An honest plan; less work dropped | One paragraph | take |
| Count 5-hour points across a reset | `ladder.md` §8 | Right points on about 1 step in 20 | Small | take |
| One wait call on a long check | `ladder.md` §4 | Haiku cost cut by up to half | One line in `build.md` | take |
| Play findings as comments on one issue per piece, or the earlier titles in the prompt | §3 clusters | Fewer duplicate asks for the owner | Small | try |
| The play step's prompt without the builds' "so far" | §2 sizes | Half its prompt | Small | try |
| A per-milestone Relay log with the weekly line (#216) | the owner's issue | The status without scrolling | Small | take |
| A build's decisions say what counts, or builds drop the field | step blocks: 4.5 a build | Smaller "so far", decisions the owner reads | One paragraph | try |
| Vary the play step's map or civ | five play steps on small 2, civ 1 | Findings that don't repeat | One line in `play.md` | try (players angle) |
| A looser `fix` rule for page and text-view readouts | 6 fix against 85 ask; X cost $1.00 | Readout bugs fixed without the owner | The rule's wording; the owner's call | ask the owner |
| `save.mjs` timeout | CI 42346427221 | One flaky check fewer | Small | later, if it repeats |
| Phone check of a PR description | unchecked since M3 | Knows whether the owner's stop reads | The owner looks once | later |
