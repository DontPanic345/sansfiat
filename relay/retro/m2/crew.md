# M2 retro: the crew, pacing, failures and the environment

Evidence: `relay` runs 40691846214, 40781134477, 40825502720, 40857386196, 40968311410 (the five that ran sessions) and 11 no-op runs (40270867726 to 41059123965), their crew.logs and transcripts; step issues #58-#98 and their relay blocks; the Relay log (#12); PRs #59, #68, #75, #82, #89, #94 and their branches; ci.yml runs on `relay/*`. Written under a budget note, so several items are marked **unchecked**.

## Verdict

The M2 relay changes landed and worked. 33 steps over six pieces finished, with no crashes, timeouts, compactions or retries (attempts 0 on every step; the one block was a real test failure). Four of six pieces were merged by the crew. The two the owner merged were stopped by crew-side faults: one review verdict was "don't merge" (#28), and one was a CI race caused by a tracked `relay/runs/result.json` (#57).

Time is the crew's problem. Sessions ran 381.9 min of a 35.4 h span (18%). The other 82% was four waits of 3.3 to 10.8 h, plus owner waits. The 5-hour stop at 50% fired on every working run but the last, and each time the restart was set by the schedule's lateness, not by the window reset.

## 1. Did the M2 changes land (m2.md section 1)?

| Change | Result | Evidence |
|---|---|---|
| Steps made after spec-review | Works. Order in crew.log: `made step spec-review`, then the builds and review after it ends (#54: 13:07:06 then 13:08:39). Only #28's spec-review edited `steps.json`, and only points and why (build-2 4 to 6, review text), not the list. The M1 split-a-build case wasn't exercised. | crew.log; `git diff` of spec vs spec-review commits |
| Spec/Spec-review on the piece | Works. All six pieces carried both lines and ran on the model they named. | step blocks |
| Board, each field on its own | Probably works: zero "couldn't update the board" lines in 5 runs (M1: 11 of 14 step ends). **Unchecked**: my token lacks `read:project`, so I didn't see the board itself or "Waiting on". | crew.logs |
| Step bodies with no comments | Works. All steps #58-#98 have 0 comments. Status line reads "done, $2.80, 11 minutes". | `gh issue list` |
| PR comment per step | Works. Comment counts equal step counts (#59: 8, #68: 6, #75: 6, #82: 6, #89: 4, #94: 5). | PR json |
| Steps table in the PR | Works, with a gap: the review's own row is blank (state "done", no $, minutes or points) because the table is built before the review runs. #96's 5-hour cell is blank (window reset mid-step). | PR #59, #94 bodies |
| Usage before/after | Works, in `meta.usage`. Float noise is still there (`55.00000000000001`, `28.000000000000004`) in the block and the log. | blocks, crew.log |
| `check.sh` in CI | Works. The engine job runs `./check.sh`. Any CI failure on relay branches: one, #28 build-1 (below). CI takes about 3 min (1.8 to 4.0). | ci runs |
| `play.mjs` | **Unchecked** in detail. Sessions made `play-scout.mjs` (#28) and `play-read.mjs` (#57), so the shared driver didn't absorb every piece's play script. | PR #75, #94 text |
| Triggers | The reply-to-blocker path worked: owner comment 10:24:46, relay-events dispatched at 10:25:01, "Picked up" comment, and build-1 ran at 10:25:32. `/relay run`, `retry` and `status`: **unchecked**, not seen in the period. Scheduled runs worked but late (section 2). 6 `relay-events` runs on piece close succeeded. | Relay log, run list |

## 2. Pacing

### Per piece (UTC, 28-30 Oct)

| Piece | Spec start | Merged | Wall | Session min | Merged by |
|---|---|---|---|---|---|
| #54 A | 10-28 12:55 | 10-28 23:33 | 10 h 37 | 86.7 | crew |
| #29 B | 10-28 23:33 | 10-29 06:30 | 6 h 58 | 74.9 | crew |
| #55 D | 10-29 06:52 | 10-29 11:33 | 4 h 41 | 35.8 | crew |
| #28 C | 10-29 06:31 | 10-29 19:10 | 12 h 39 | 65.2 | owner (verdict not merge) |
| #56 F | 10-29 11:34 | 10-29 23:10 | 11 h 36 | 49.2 | crew |
| #57 E | 10-29 23:10 | 10-30 01:40 | 2 h 30 | 70.1 | owner (CI race) |

Pieces #54-#57 were opened at 11:20 on 10-28 and the first session started at 12:55 (the 12:17 slot, 38 min late). M2 total: 35.4 h from first session to last review, 38 h to the last merge.

### The waits (gaps over 5 min between sessions)

| Gap | Length | What it was |
|---|---|---|
| #54 build-1 to build-2 | 9.1 h | 5-hour stop at 69% (13:29). Next run 22:35, the 18:17 slot 4.3 h late. |
| #29 build-1 to build-2 | 5.7 h | 5-hour stop at 71% (00:09). Next run 05:48, the 00:17 slot 5.5 h late. |
| #55 build-1 to #28 build-1 | 3.3 h | 5-hour stop at 55%; #28 build-1 blocked since 06:52, the owner replied at 10:24 (3 h 32 owner wait). |
| #56 build-1 to review | 10.8 h | 5-hour stop at 61% (12:01). The 13:02 run saw 61% again and stopped. Next run 22:45. |
| Owner after #28's review | 8 h | Review done 11:13, merged 19:10. |
| Owner after #57's review | 1 h 17 | Review done 00:23, merged 01:40. |

Step-to-step overhead (creating issues and PRs, two usage probes) totalled 17 min over about 33 steps.

### Why the stops overshoot, and why restarts are late

- A stop is checked only between steps, so one big step carries the window past 50%: #54 build-1 opus medium 28 to 69 (41 points); #29 build-1 sonnet medium 43 to 70 (27); #56 build-1 opus medium 37 to 61 (24). Each came from a Build-1 starting at 28-43%.
- **The schedule runs about 3 times a day, not 4.** 15 scheduled runs in 4.5 days against 18 slots. The 06:17 slot never ran. The 12:17 slot ran 38-46 min late; the 18:17 and 00:17 slots ran 237-355 min late. A 5-hour stop therefore costs 5.7-10.8 h, where the window itself resets in under 5 h.
- The weekly ramp never bound: weekly went 44% to 67% across M2 against limits of 67.9% (day 5) and 81.4% (day 6). The ramp was a non-factor in M2, as it was a blocker in M1.
- "Waiting on: 5-hour usage, until HH:MM" gives the window reset, not when the crew actually restarts. **Unchecked** whether the board shows it.
- The 5-hour reset time isn't in the log. `sessionResetsAt` is read and then dropped, so I couldn't measure how much of each gap was the window and how much was lateness. The `rate_limit_event` frames in transcripts would show it; my grep was still running when I stopped.
- `usage()` takes the "direct" route first; every line says "from the probe route", so the direct route fails each time in CI and no reason is logged. Every probe is a Haiku session of about 15 s, and each step is probed twice (after it, then before the next).

## 3. Failures

- **No crashes, timeouts, init refusals or compactions.** Max context 31% (#96 haiku, 47 min). Longest session 47.2 min of the 90 allowed.
- **One block**: #28 build-1 (haiku medium, 9 min) failed `small_pregames_always_end_with_three_spaced_centres` and wrote `blocked`, with the cause and two options. It had pushed the failing state, so CI failed (40831826589). The crew blocked the piece, labelled it `waiting` and started #55 meanwhile, as designed. The owner's reply (10:24) changed rules and design docs, and told the session to carry on to build-2. The session built both builds (haiku, 24.9 min, summary starts "Build-2 is in"). The crew then ran sonnet build-2 anyway: 1.9 min, $0.07, nothing to do. Wasted but cheap.
- **`relay/runs/result.json` is now tracked in git** although `relay/runs/` is in `.gitignore`. It entered main with #75: the haiku session force-added it (fee8797 "#28 build-2: result.json"). Consequences:
  - `fs.rmSync(RESULT)` before each session leaves the tree dirty; `tidyUp` logged "session left uncommitted changes; dropping them" for #56 review and #57 spec. Harmless so far, but `git reset --hard` there could drop real work.
  - Sessions `git add -A` it into their commits. #57's review committed only `result.json` (0755db4, 00:22:32), which triggered a fresh CI run (40979295645, 00:22:38). Its `gh pr checks --watch` ran before that run registered and showed the previous green one, so the review ended at 00:22:40. At 00:22:55 `autoMerge` saw CI in progress ("CI isn't green"), so the crew didn't merge. The owner merged 1 h 17 later. This is the only M2 merge the crew was meant to do and didn't.
  - The file is in the main diff of #94 and #75.
- **#28's review verdict was not merge**: a real finding (scouts starve on real maps), recorded in `relay/specs/28/review.md`. Not a crew fault; the 8 h that followed was the owner's.
- **Relay log**: five working runs posted; the 11 no-op runs and the 13:02 budget stop on 10-29 posted nothing, as designed. The log can't say why a piece sat for 10 h; it shows no stop time or restart time.
- **Merge conflicts**: none in the crew's pushes. #56's review merged main in (scouts, #75) and fixed a brace, as the review prompt says.
- **Site workflow**: 1 `site` workflow_run failure (40987519480) and 1 `site` delete failure (40987239433) on 10-30 01:40-01:43, right after the #94 merge. **Unchecked** cause.

## 4. The environment sessions ran in

**Mostly unchecked** (I ran out of budget before the transcripts' tool surveys).
- Checks: `engine/check.sh` ran by hand in sessions and in CI at about 3 min (1.8-4.0). CI ran on every push to a relay branch (35 pull_request runs; 34 passed).
- Setup/tool searches, the browser and Chrome path (m2 said `session.md` names Chrome): **unchecked**.
- Every working run installs Claude Code latest (2.1.312 in all five), Rust, wasm-bindgen and a cache; time to first session was about 30-40 s in logs (run start to first "made step"), so setup is not a cost.

## 5. What could come out, and what automatic players need

### Out of relay.mjs and the README
- The board code (`loadBoard`, `setBoard`, Waiting-on text, about 90 lines) works but nothing in the retro depends on it. Keep it until the owner decides he reads it; the cost is its token and its README section.
- Duplicate usage probes (once after a step, again before the next): reuse the "after" reading.
- The direct usage route fails every run; either fix it or drop it to leave one route.
- `README` "Not built yet: the retro after a milestone": still true.
- The README's "Files" and "Running it" are fine; the long "Spend" paragraph overlaps the Pacing text and could halve.

### Automatic players and judges as a step
For a player or judge to run as a step, the crew needs only what a step already has: a `part`/kind in `steps.json`, a prompt file, and a model/effort. Concretely:
- a `play` step kind: runs after the last build, before the review, with no brief, and writes its result to a file under `relay/specs/<n>/` rather than into code;
- the review should receive that file in its prompt (the same "files" path the diff stat already takes);
- the crew must keep running after a "judge" finding of failure rather than blocking, since a bad play result is a finding, not a blocker. That is the same rule as `ideas`.
Details belong with the players angle (`relay/retro/m2/players.md`; **not cross-read**).

### What replaces the coin
Nothing in the crew needs a coin: `Spec` and `Spec-review` are just lines on the piece, and the crew copies whatever they say. A replacement is purely a planner/prompt choice. Wall-clock results (spec-review took 0.8-1.0 min at opus low and 3.7-7.7 at opus medium) are cost-angle material.

## For other angles
- **Prompts**: a session told to commit `result.json` forced it past `.gitignore`; `session.md` should say never to commit it. The review also commits only to change `result.json` when it has nothing else to commit.
- **Cost**: the 5-hour points per step include the owner's own use. #54 spec took 22 points in 10.6 min and build-1 41 in 19.5 min.
- **Briefs**: #28's reply-to-blocker rewrote the design and the next build; build-2 was empty.

## Ideas

| Idea | Value | Cost | Recommendation |
|---|---|---|---|
| `git rm --cached relay/runs/result.json` on main; crew refuses to start if it is tracked | high | small | take |
| Review waits for the CI run of its own head commit (`gh run watch`), not `gh pr checks` right after a push; crew `autoMerge` retries for CI up to a few minutes instead of giving up | high | small | take |
| Log the 5-hour reset time and the next slot in the budget-stop line and in Waiting-on, so a stop says how long it will really last | medium | small | take |
| Have a budget stop at a reset time re-dispatch itself later (a delayed dispatch, or keep the job alive and sleep until the reset, within the 345-min job limit) because schedule lateness cost 5.7-10.8 h per stop | high | medium | try |
| Check the stop before a step using the step's expected cost (points of the model/step) so an Opus step doesn't push 28% to 69% | medium | small | try |
| Raise the 5-hour stop to 80% for unattended runs (every stop was an overshoot anyway, and the weekly ramp never bound) | medium | small | try |
| Log why the direct usage route fails; drop the second probe per step | low | small | take |
| Round usage in the log and block (still floats) | low | small | take |
| Fill the review row of the PR's steps table after the review ends (crew edits the PR) | low | small | later |
| Do not run build-N when a blocker reply already built it (skip a step whose summary came from an earlier session) | low | small | later |
| Investigate the site workflow failure after the #94 merge | low | small | later |
| Cut the board entirely | low | small | drop (works, not costing anything) |
| `play` step kind (section 5) | medium | medium | try, after the players angle |
