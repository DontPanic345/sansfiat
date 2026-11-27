# M5 retro: Relay

The crew, the workflows, the waits, the prompts and the loop without the owner, for M5's
pieces (the eight game pieces, the five M6 pieces the owner released, the two the owner opened
mid-milestone, the eight art pieces, the fixes piece) and this retro's first steps.

Evidence: the 215 `relay.yml` runs from 12 Nov 09:00 to this one, listed in
[`relay/runs.txt`](relay/runs.txt); the `crew.log` of every run with an artifact, merged in
[`relay/crew.log.txt`](relay/crew.log.txt) (the artifacts expire from 26 Nov); the 115 saved
prompts and transcripts in those artifacts; the relay blocks on step issues #344–#512; the
piece issues #336–#446 and their comments and label events; PRs #349–#504; `ci` runs since
12 Nov; the Relay log #345; the ideas issues #351 and #480; `git log` of `relay/` and
`.github/`. Times are UTC. "My read" marks a judgement; everything else is counted. Cost per
step, per model and per window is the ladder angle's (`ladder.md`) and isn't repeated here.

## Verdict

M5's crew changes did what they were built for once the owner had fixed them twice by hand.
The fresh-window whole step, the usage hook, the per-milestone log, the ideas issue, the play
step's lean prompt and the weekly lift all ran. But M5's first piece found four crew bugs in
its first day, and the owner changed the crew six times by hand mid-milestone (three in M4),
re-cut the milestone, built one piece in their own session and opened ten pieces outside the
proposal. 17 of M5's 21 merges were the crew's, each within a minute of its review; the four
the owner made are a squash setting (L), a hand build (J) and two `merge: false` verdicts (X,
K) that waited 4.5 and 5.2 days.

The owner, not the crew or the window, set M5's pace. M5 ran 291.9 hours from L's first
session (12 Nov 10:42) to #446's merge (24 Nov 14:38). Sessions filled 44 of them (15%). The
5-hour window cost about 61 hours of stops; every restart came two minutes after its reset.
The weekly ramp cost 17.3 hours in two stops, each restarted at 09:02, two minutes past its
lift: the M4 retro's fix works. The rest, about 168 hours (58%), the crew had nothing ready:
waiting on the owner's merge, re-cut, play, release of the next pieces, or new pieces.

The loop without the owner is quieter than M4's: 8 `fix` and 29 ideas against M4's 6 and 85,
on one ideas issue instead of 85 issues. It still repeats: the gift that reaches no one is four
lines on #351 and the check's first finding again.

## 1. The crew in M5

### Runs

| Kind | Runs | Note |
|---|---|---|
| Ran sessions | 74 with artifacts (and this one) | 118 sessions |
| Cancelled | 134 | 91 in 2.5 minutes at 09:28–09:30 on 12 Nov, when the proposal's merge opened eight pieces and each label fired a dispatch; the rest are waits and duplicates the concurrency groups replaced |
| Scheduled | 23 | Every six hours since 92939e3 (12 Nov); 3 cancelled |
| Failed | 1 | 43250613590, 15:42 on 12 Nov, the `relay` job with no failed step logged; cause not read |

The 91-run storm cost nothing (each was cancelled in seconds) and is the price of one dispatch
per label event.

### Per piece

| Piece | First session | Merged | Wall | Inside the wall |
|---|---|---|---|---|
| L #336 | 12 Nov 10:42 | 15 Nov 09:10, by the owner | 70 h 28 | The whole died on the limit (47 min dropped), a 9.2 h stop, blocked 9.7 h, the merge refused (squash) 49.3 h |
| J #352 | 15 Nov 09:12 | 15 Nov 10:55, by the owner | 1 h 43 | Built by hand in the owner's session, no play, no review |
| S #353 | 15 Nov 10:57 | 15 Nov 16:22 | 5 h 25 | 5-hour stop 4.4 h |
| R #354 | 15 Nov 20:52 | 16 Nov 02:06 | 5 h 14 | Waited 4.5 h before it for a window under 10% (it read 14%) |
| C #337 | 16 Nov 02:06 | 16 Nov 08:23 | 6 h 17 | 5-hour stop 3.5 h; build-2 died (exit 1) and its work was kept |
| G #355 | 17 Nov 00:14 | 17 Nov 06:54 | 6 h 40 | 5-hour stop 4.2 h |
| P #338 | 18 Nov 01:42 | 18 Nov 05:24 | 3 h 42 | 5-hour stop 1.9 h |
| V #339 | 18 Nov 05:24 | 18 Nov 07:23 | 1 h 59 | |
| Y #340 | 18 Nov 07:23 | 18 Nov 11:10 | 3 h 47 | 5-hour stop 2.2 h |
| N #341 | 18 Nov 11:11 | closed 19 Nov 10:42 as `proof/341` | | build-2 blocked on two red real-map tests |
| X #356 | 18 Nov 13:31 | 24 Nov 02:35, by the owner | 8 h 10 to the review | `merge: false` (health past its floor); then 5.2 days to the owner's merge |
| H #342 | 18 Nov 22:40 | open (PR #418) | | spec, spec-review, build-1; held when K and Q took its place |
| K #426 | 19 Nov 12:32 | 24 Nov 02:11, by the owner | | Two whole sessions and three reviews, all `merge` false or empty |
| Q #427 | 19 Nov 17:32 | 19 Nov 18:44 | 1 h 12 | Waited 2.8 h before it for a window under 50% |
| Art #439–#446 | 23 Nov 07:04 | 24 Nov 14:38 | 31 h 34 for eight | Five 5-hour stops, 15.4 h |
| Fixes #350 | 24 Nov 03:42 | 24 Nov 04:42 | 1 h 00 | |

M4 for comparison: eight pieces in 104.7 hours, every merge the crew's.

### Where the hours went

| | M5 hours | Share | M4 share |
|---|---|---|---|
| Sessions | 44 | 15% | 33% |
| 5-hour stops (incl. waits for a fresh window) | 61 | 21% | 9% |
| Weekly ramp stops | 17 | 6% | 52% |
| Nothing ready: the owner | 168 | 58% | 5% |
| Crew overhead | under 1 | | 1% |

The owner's 168 hours, by when they fell:

| From | To | Hours | What |
|---|---|---|---|
| 12 Nov 20:57 | 13 Nov 06:41 | 9.7 | L's whole blocked; the owner fixed the crew (f840e80) and handed on by hand |
| 13 Nov 07:50 | 15 Nov 09:10 | 49.3 | L's merge refused ("Squash merges are not allowed", HTTP 405); the owner re-cut M5 (22ed7a4) and merged. The week stood at 97% of its window on 13 Nov, so little could have run anyway |
| 16 Nov 08:23 | 17 Nov 00:12 | 15.8 | The owner's play after C (m5.md §2), then `/relay ready` on G |
| 17 Nov 06:54 | 18 Nov 01:08 | 18.2 | X and the five M6 pieces waited to be released; the owner readied them at 01:08 |
| 19 Nov 18:44 | 20 Nov 01:15 | 6.5 | K's `merge` empty; the owner's `/relay run` |
| 20 Nov 10:47 | 23 Nov 07:03 | 68.3 | Nothing open after K's second review; the week reset about 21 Nov 09:00; the owner opened the eight art pieces at 07:03 on 23 Nov |

My read: some of the 168 hours were the owner choosing to stop, as m5.md planned for the
play after C, and some were the owner's own work on the game (the re-cut, J, K and Q's
issues, the map art). Two were crew faults: L's refused merge, and the blocked whole.

### The 5-hour stops

| | Stops | Hours | Restarted |
|---|---|---|---|
| A stepped step at 75% | 13 | 38.9 | Every one 2 minutes after its reset |
| A whole waiting for a fresh window (under 10%, then under 50% from 19 Nov) | 5 | 11.7 | 2 minutes after the reset |
| Spent by the owner between reset and restart | 1 | 9.2 | 12 Nov: reset 15:40, but the window read 99% at 19:53 |

The waits for a fresh window: 38% (1.2 h), 19% (1.8 h), 14% (4.5 h), 65% (1.4 h) and 56%
(2.8 h). The 14% wait held R for a whole window to spare 14 points. 0bc90df moved the bar to
50% on 19 Nov and made a whole one window that hands on at 85%.

### The weekly ramp

92939e3 (12 Nov, with the M5 change) dropped the weekly start check. The week went 70% to 97%
in 22 hours (12 Nov 09:28 to 13 Nov 07:50): the first two days of M5 spent the week. c9ab453
(18 Nov) put the ramp back.

| Stop | Reading | Lifts | Restarted | Past the lift |
|---|---|---|---|---|
| 18 Nov 23:31 | 68% ≥ 67.9 (day 5) | 19 Nov ~09:00 | 09:02 | 2 min |
| 20 Nov 01:16 | 82% ≥ 81.4 (day 6) | 20 Nov ~09:00 | 09:02 | 2 min |

M4 sat 2.5 to 15.9 hours past each lift. The wait for the lift (the M4 retro's fix 1) took that
to two minutes.

### Failures

- **L's whole, session 1** (12 Nov 10:42, Fable medium, on a fresh window after a 1.2-hour wait) died
  on the limit at 102% after 47.2 minutes, `exit 1`, `result null`. The crew logged "dropping
  them": the M4 fix that a limit death costs no attempt and keeps the work didn't fire, because
  the 429 was in a result frame before the helpers' empty frames, and the crew read only the
  last. **Session 2** ended its turn while a helper ran; the helper was stopped ten minutes
  later and the session ended with no result in 14.2 minutes. Two endings without a result:
  blocked at 20:57. f840e80 (13 Nov 01:01) closed all three: any result frame with the 429
  counts, helpers finish (`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS=0`), and an unfinished step's
  work is committed. Since then: C build-2 (`exit 1`, 45 min) had its work committed and
  finished in a second session; X build-1 ran past 90 minutes (`exit 143`) with a done result
  the crew didn't take, and the second session re-verified in 8.2 minutes; #446 build-3 went
  `pending` at the job's end and finished in the next run. No work was dropped after 13 Nov.
- **Merges.** L's merge failed on the repository's merge setting (HTTP 405); by 15 Nov squash
  merges were allowed again, with no change in `relay.mjs`. Three merged PRs left their piece
  open (#354, #338, #340) because a review rewrote the description without "Closes #N";
  c9ab453 restores the line. The crew merged 17 of 17 `merge: true` verdicts from 15 Nov on.
- **A review with no `merge`.** K's reviews #430 and #436 wrote "merge left to the owner"
  without the field; the crew re-ran #436 as #437 ($0.45), which wrote `merge: false`. The
  rule holds, at the cost of a session.
- **The one blocked step on a piece:** N build-2, which wouldn't loosen two real-map tests (the
  M5 rule "a test that carries a rule's bound is not a build's to loosen" working). The owner
  paused N, then closed it into a settling piece.
- **Crashes and compactions:** none. Highest context 38% (X build-2, Haiku high).
- **CI.** 16 failures in 166 finished `ci` runs since 12 Nov, none on main (M4: one flake on
  main). All 16 are mid-step pushes on piece branches.

### Mid-milestone changes to the crew

| Commit | When | What | Why |
|---|---|---|---|
| 92939e3 | 12 Nov 09:27 | Drop the weekly start check; hook every ten minutes; cron every six hours | With the M5 change. It spent the week in a day |
| f840e80 | 13 Nov 01:01 | A session ends whole or leaves its work; the hook speaks at 70% and 85% from the stream | L's whole ended twice without a result |
| c9ab453 | 18 Nov 22:46 | The weekly ramp back; PRs restore "Closes" | The week, and #354, #338, #340 left open |
| a17cb46 | 18 Nov 22:53 | Pieces run stepped on Opus by default | The Fable whole cost 1.6x a stepped piece's 5-hour points (`ladder.md`) |
| 0bc90df | 19 Nov 10:00 | A whole always ends in a review; starts under 50%; Fable can't be named | |
| 59cd80f | 19 Nov 10:07 | A whole is one window and hands on at 85% | |

The handover rule ("the crew changed on main: a fresh run carries on") fired twice, at 23:30 on
18 Nov and 11:04 on 19 Nov, each right after a change, and cost nothing. It is what made six
hand changes safe while the crew ran.

The owner also changed the milestone: the re-cut (22ed7a4, 15 Nov) moved P, V, Y, N, H and K
to M6 and added J, S, R and C; then the owner released P, V, Y, X, N and H on 18 Nov, opened K
and Q on 19 Nov and the eight art pieces (from the map-art roadmap, PR #438) on 23 Nov. M5 opened 23 pieces and merged 21 where m5.md proposed eight. #503 proposes Relay on Claude and Codex for M6.

## 2. The prompts

### The M5 change (m5-relay.md) against what it did

| § | Line | What it changed | Evidence |
|---|---|---|---|
| 1 | A whole piece on Fable | Five pieces (L, S, R, C, G) ran whole on Fable and every one handed on to builds, play and review. Then a17cb46 and 0bc90df ended it: same list $ as a stepped piece, 1.6x the 5-hour points | `ladder.md`; §1 |
| 1 | A fresh window (under 10%) | Held the whole 11.7 hours across five waits. Its session still died at 102% once (L) | §1 stops |
| 1 | Usage in the session; the hook | The hook's text isn't in the stream-json transcripts, so how sessions answered it is unmeasured there. It reached this session at 70%. 0 of 118 sessions after L died on the limit | transcripts; crew.log |
| 1 | Handing on | Worked: every whole wrote `steps.json` and the crew made the steps, play before review | crew.log |
| 1 | A limit death costs no attempt | Failed on L (one frame read), fixed in f840e80, not needed again | §1 |
| 2 | Twin and pick gone | Gone: no pick prompt in M5 | prompts |
| 3 | No per-step PR comments | Yes: the crew posts only the PR link and the verdict on a piece | piece issues |
| 4 | One ideas issue | Yes: #351 holds 26 lines, 11 ticked by designers. A second, "Ideas" #480, opened for the art pieces (which have no milestone) with 2 lines | #351, #480 |
| 4 | A designer takes the obvious ones | 14 `taken` across spec, whole and build steps; the fixes spec took 3 | crew.log |
| 5 | Trust, and verify another way | Reviews reused builds' sweeps ("the sweeps in relay/specs/426/ are current"). Not counted further | #430, #437 |
| 6 | Tests run in seconds; a rule's bound isn't a build's | N build-2 blocked rather than loosen two tests | #341 |
| 7 | One log a milestone, both windows | Yes: #345, 30 reports, each step linked, the week with the ramp's line from 19 Nov | #345 |
| 7 | Draft until the verdict; judge above the table | Yes on the PRs read (#409, #429) | PRs |
| 9 | The play step without builds' summaries | Yes: play prompts 23.3 KB (M4 36.4), "so far" empty | below |
| 9 | Play the civ the piece's number gives | Not checked across all eight; the check still played small 2 as civ 1 | `play.md` |

### What a session reads

| Kind | n | Prompt (avg, range) | "So far" (avg, max) | M4 prompt |
|---|---|---|---|---|
| spec | 15 | 26.8 KB (21.4–43.9) | none | 29.0 |
| spec-review | 15 | 26.7 KB (22.1–41.3) | 1.4 KB, 1.7 | 30.2 |
| build | 47 | 17.3 KB (11.9–22.2) | 5.1 KB, 12.8 | 19.1 |
| play | 8 | 23.3 KB (20.0–27.1) | none | 36.4 |
| review | 21 | 49.1 KB (37.0–73.7) | 8.3 KB, 16.7 | 68.0 |
| whole | 9 | 32.2 KB (27.0–40.7) | 0.5 KB, 4.9 | 31.6 |

Every kind is smaller than in M4; builds' "so far" fell from 9.4 to 5.1 KB, so the decisions
line in `session.md` ("what counts as one") moved it where "usually has none" didn't.

### Lines that misled

1. `whole.md` and m5-relay.md §1, a fresh window "under 10%": it waited a full window for 14%
   and still couldn't stop a Fable session spending the whole window in 47 minutes. Gone in 0bc90df.
2. The review prompt lets a review write "merge left to the owner" as prose: two of K's three
   reviews did, with no field (#430, #436).
3. The review rewrites the PR description and drops "Closes #N" (fixed in c9ab453).
4. The milestone-built rule tagged `m5` at the fixes piece's merge (04:42 on 24 Nov), while #445
   and #446 were still building, so `m5` lacks the last two art pieces and the retro issue
   opened ten hours before its first step could run.

## 3. The loop without the owner

### Fix or ask

| | Fix | Idea |
|---|---|---|
| play steps | 8 | 12 |
| wholes | 0 | 6 |
| builds | 0 | 5 |
| specs and spec-reviews | 0 | 3 |
| reviews | 0 | 3 |
| total | 8 | 29 |

- **The fixes piece worked again.** Opened at the first `fix` (07:10 on 13 Nov), it ran last
  on Opus spec and spec-review, two Haiku builds and a Sonnet review: 58 session minutes and $7.77 (M4 $1.00, on Sonnet). The spec on Opus is most of the rise ($5.62, `ladder.md`).
- **Fewer repeats, not none.** #351 holds 26 lines where M4 opened 85 issues, and designers
  ticked 11. But the same thing still comes back under new words: "gave 0.0 stays zero" (ticked),
  "The gift reaches no one on the small maps", "The store falls with a gift that gave nothing",
  and the check's "Gift never reached a clan". Two lines say a settlement's store feeds nothing.
  The play step is told the open lines; it isn't told the ticked ones, so a fix that didn't move
  what a player sees comes back as new.

### The owner's stops in M5

| Stop | M4 | M5 |
|---|---|---|
| Blocker replies | 1 | 2 (L's whole, by a crew fix and a hand hand-off; N, by pausing and closing it) |
| Merges left to the owner | 0 | 4 (L by the setting, J by hand, X and K on `merge: false`) |
| Restarts by hand | 2 | 1 (`/relay run` on K, 20 Nov) |
| Crew changes by hand mid-milestone | 3 | 6 |
| Re-cutting a milestone or piece | 1 | 2 (the M5 re-cut; N closed into settling) |
| Pieces built by hand | 0 | 1 (J) |
| Pieces opened outside the proposal | 0 | 10 (K, Q, eight art) |

## 4. The retro run so far

| Step | Ran | Minutes | List $ | 5h | Note |
|---|---|---|---|---|---|
| check | 24 Nov ~14:38 | 2.0 | 0.32 | 3 | The retro issue opened at 04:43, but its steps waited for #446 |
| ladder | 24 Nov | 10.0 | 2.59 | 20 | |
| architecture | 24 Nov | 13.7 | 3.01 | 22 | |
| relay | 24 Nov 15:05 | this session | | | Started at 64%; the 70% hook came early |

The artifacts from 12 Nov expire on 26 Nov, about when `game` runs if the owner sends no notes.
This file keeps the crew log and the run list in `relay/`.

## 5. What could come out

- **The fresh-window bar for a whole.** It held 11.7 hours of M5 for one death it didn't prevent.
  The 85% hand-on (59cd80f) does what it was for. Now that pieces run stepped by default, the
  50% bar costs only when a `Whole:` line is used. Keep, but it could go to 75% like a step.
- **The play step on every piece.** Eight play steps cost little ($0.23–0.76) and filed 8 of the
  8 fixes. Keep.
- Not the handover on a crew change: it is what let six hand changes land safely.
- Not the six-hour cron: no restart waited on it in M5, but it is the fallback.

## 6. Fixes for the M6 Relay change

1. A review's verdict is the `merge` field or the step isn't done, and its prompt says "merge
   left to the owner" is `merge: false` with the choice in the PR (#430, #436).
2. The milestone is built when every piece with the milestone's label is closed, not at the
   fixes piece's merge (the `m5` tag at 04:42, before #445 and #446).
3. The play step is told the ideas a designer ticked this milestone as well as the open ones,
   so it files "still no effect" against the ticked line, not a new one (the gift, four times).
4. The crew checks the repository's merge setting before the first merge of a milestone, or
   falls back to a merge commit when squash is refused (L, 49 hours).
5. One dispatch per batch of label events, not per event (91 cancelled runs in 2.5 minutes).
6. A step killed past its cap that wrote `done` keeps its result if its last commit passed
   (X build-1: the second session only re-verified).
7. Hand changes to the crew go through a small PR the crew's next run reads, so the retro
   counts them; six in M5, and 92939e3 spent a week in a day.

## For other angles

- **Ladder:** your costs match the crew log; the log's sessions sum to 2,636 minutes before the
  last run's (this run's) sessions. The Fable whole's end (a17cb46, 0bc90df) came from your M5
  figures before this retro, from the owner.
- **Architecture:** N and H stopped on the store and the jobs (`proof/341`, PR #418); your clay
  list is where they pick up.
- **Game:** X and K merged on the owner's word with `merge: false` and worse health; their
  ledger lines should say which numbers the owner kept.
- **Players:** the gift that reaches no one is four lines on #351 and the check's first
  finding; the settlement's store, two.
- **Retro and proposal:** the owner is M5's pace, at 58% of the wall. M5 opened 23 pieces and merged 21 against m5.md's eight, ten of them opened mid-milestone; the proposal should say whether M6 plans for
  that or holds to its list. #503 (Relay on Claude and Codex) is open and changes who builds.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| A review isn't done without the `merge` field; prose isn't a verdict | #430, #436, #437 | One session saved; a verdict the crew can act on | One line in `review.md` and the check | take |
| The milestone is built when its labelled pieces close | `m5` tag at 04:42 on 24 Nov | The tag and the retro on what was built | Small | take |
| The play step sees ticked ideas too | #351, the gift four times | Fewer repeats for the owner | Small | take |
| Merge falls back when squash is refused | #336, HTTP 405 | No 49-hour stop on a setting | Small | take |
| One dispatch per batch of label events | 91 cancelled runs, 12 Nov | Quieter Actions | Small | later |
| A killed step's `done` result kept when its commit passed | X build-1 | One session | Small | try |
| Hand crew changes through a PR the retro can count | six in M5 | The retro sees what changed and why | The owner's habit | ask the owner |
| The whole's start bar to 75% | §1, 11.7 h of waits | Hours back when a `Whole:` line is used | One number | try |
