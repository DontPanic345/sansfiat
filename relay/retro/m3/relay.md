# M3 retro: Relay

The crew, the prompts, the owner's stops, and the loop without the owner (owner.md §6).

Evidence: `relay` runs 41195968196 to 42176501453 (34 runs, 31 Oct 11:13 to 6 Nov 06:46 UTC),
their logs, and the run artifacts of the six that ran sessions (41196527672, 41364842651,
41391671599, 41425463303, 41467598377, 41515434828: 39 saved prompts and transcripts); step
issues #106–#149 and their relay blocks; PRs 107, 118, 124, 126, 134, 142 and their comments;
the timelines of #101, #102 and #105; `ci` and `site` runs on `relay/*` and main; `relay.mjs`,
`relay.yml`, `relay-events.yml`, `site.yml`; `engine/CLAUDE.md` over `git log`. Times are UTC.
"My read" marks a judgement; everything else is counted.

## Verdict

The crew ran M3 clean. 38 steps over five pieces in 39 sessions: no crash, timeout, compaction
or blocker, one step that took two attempts (#148). Restart at the reset works: four 5-hour
stops, each crew run back 2.5 minutes after the logged reset. The 80% gate held the window to
80–83% at every stop. The crew merged four of five pieces on the first try, each review having
waited for its own CI.

What cost time was the owner, not the crew. Of M3's 57 hours from first session to last merge,
sessions ran 12.8 h (22%), 5-hour waits took 11.2 h (20%), and one owner wait took 33 h (58%).
That wait came from a missing flag: #101's review wrote "Verdict: merge" in the PR and its
summary but left `"merge": true` out of `result.json`, so the crew told the owner the verdict
wasn't merge. After the owner merged, the crew didn't start again until the owner typed
`/relay run`. Then Relay sat idle for 3.4 days (13 no-op runs) with #105 paused and nothing to
start the retro. The owner is right that they are the bottleneck (owner.md §6), and most of
it is fixable in the crew.

Of the prompt changes, `soFar` without the spec's decisions worked and the review's CI wait
worked. `engine/CLAUDE.md` was trimmed to 5.2 KB and grew back to 20.1 KB in five pieces.
"Under five is usual" capped build decisions at exactly five instead of making them owner
choices. The loop in §4 needs four things the crew lacks: a play step, a fixes piece the crew
fills from findings by a rule it can apply, a milestone that closes with a piece left open, and
a retro run.

## 1. The crew in M3

### Runs

| Kind | Runs | Ids |
|---|---|---|
| Ran sessions | 6 | 41196527672, 41364842651, 41391671599, 41425463303, 41467598377, 41515434828 |
| `wait` runs | 7: 4 needed, 3 duplicates | 41375151652, 41406100328, 41442195714, 41491472241; duplicates 41406162263, 41442302129, 41491568884 |
| Crew runs that only re-stopped | 3 | 41397498165 and 41475929139 (schedule, queued behind a working run), 41425466701 (from a duplicate wait) |
| Cancelled while queued | 1 | 41467600849, replaced in the concurrency queue by 41475929139 |
| No-op after the last merge | 1 | 41515440230, from a duplicate wait |
| Trigger checks (m3-relay.md §9) | 3 | 41195968196 (`status`), 41195970343, 41196323734 |
| Scheduled no-ops | 13 | 41201342373 … 42176501453 |

The duplicates breed: a scheduled run queues in the `relay` concurrency group behind a working
run, runs after it stops, sees the same 5-hour stop and starts a second `wait`. Two waits fire
two crew runs at the reset; the second queues behind the first, re-stops and starts another
wait. Every stop after the first had two waits. It costs nothing but runner minutes and log
noise, because the crew runs are serialised.

The schedule (`17 */6`) fired 15 times in 24 slots, up to 3 h 44 m late (the 18:17 slot at
22:01 on 1 Nov). In M3 it started no work: every restart came from a wait run or the owner.

### Per piece

| Piece | First session | Merged | Wall | Session min | 5-hour | Weekly | List $ | Merged by |
|---|---|---|---|---|---|---|---|---|
| #100 H | 31 Oct 11:22 | 31 Oct 13:28 | 2 h 06 | 121.5 | 50 | 4 | 7.62 | crew |
| #101 G | 31 Oct 13:28 | 1 Nov 23:56 | 34 h 28 (review done 15:02) | 91.8 | 47 | 4 | 8.57 | owner, split (PR 124 held, open) |
| #102 L | 2 Nov 00:01 | 2 Nov 05:57 | 5 h 56 | 168.6 | 101 | 7 | 17.96 | crew |
| #103 P | 2 Nov 05:57 | 2 Nov 10:41 | 4 h 43 | 122.6 | 80 | 7 | 13.79 | crew |
| #104 A | 2 Nov 10:41 | 2 Nov 20:37 | 9 h 56 | 264.1 | 156 | 11 | 23.46 | crew |
| M3 | 31 Oct 11:22 | 2 Nov 20:37 | 57 h 15 | 768.6 | 434 | 33 | 71.40 | |

5-hour and weekly are the step blocks' rounded points, summed. The weekly window went from 2%
(#106 before) to 39% (#149 after); the 4 points between 31 Oct 15:02 and 2 Nov 00:01 were the
owner's. m3.md §3 expected about $80, 450 and 40 for six pieces; five cost $71, 434 and 33.

### The waits

| Gap | Length | What it was |
|---|---|---|
| #101 review → #102 spec | 32 h 59 | The crew read "not merge" (no flag); the owner split #101 and merged at 23:56 on 1 Nov; `/relay ready` (unknown command) at 23:59, `/relay run` at 00:00 |
| #102 build-3 → build-4 | 3 h 05 | 5-hour stop at 81%, reset 05:00, session 05:02:37 |
| #103 build-3 → build-4 | 2 h 38 | stop at 83%, reset 10:00, session 10:02:33 |
| #104 build-2 → build-3 | 2 h 55 | stop at 81%, reset 15:00, session 15:02:35 |
| #104 build-5 attempt 1 → 2 | 2 h 35 | stop at 80%, reset 20:00, session 20:02:34 |
| #104 merged → now | 3.4 days | #105 paused (by the owner, 1 Nov 22:57); nothing left to run; 13 no-op runs |

Step to step inside a run took 10–20 s, plus about 30 s to make a piece's step issues. The
restart lag is the wait job's 120 s margin plus setup.

### The gate

No step started at or over 80%. Every stop came with the window already at 80% or more, so the
expected-points term never decided a stop; it decided four late starts:

| Step | Start | Expected | End |
|---|---|---|---|
| #102 build-3 haiku high | 65% | 6 | 81% |
| #103 build-3 haiku high | 71% | 6 | 83% |
| #104 build-2 haiku high | 69% | 6 | 81% |
| #104 build-5 opus low | 65% | 13 | 80% (pending) |

The expected points are off by half or double. M3's medians (step blocks; they include whatever
the owner ran at the time):

| Model, effort | `EXPECTED_POINTS` | M3 median (n) | Range |
|---|---|---|---|
| opus medium | 25 | 12 (13) | 5–21 |
| opus low | 13 | 15 (4) | 10–19 |
| sonnet medium | 8 | 16 (2) | 15–17 |
| sonnet high | 10 | 13 (3) | 4–49 |
| haiku high | 6 | 11 (8) | 2–21 |
| haiku medium | 5 | 3.5 (6) | 2–16 |

A flat "start under 75%" makes the same eight decisions in M3 (highest start 71%, every stop at
80% or more) and drops the table. Neither protects against a 49-point step (#146 sonnet high,
83 minutes) starting late in a window.

The weekly ramp didn't stop a step, but it came close: 26% against 27.1% at 06:59 on 2 Nov
(day 2), and 38% against 40.7% at 20:02 (day 3). It stays.

### Failures

- **Attempts.** #148 (#104 build-5, opus low) wrote `pending` after 41.8 of its 90 minutes,
  saying "I'm out of time"; the prompt says only that the crew stops a session at 90. Its note
  named the failing script and what to do; the second attempt, told "The last attempt didn't
  finish", finished in 15.2 minutes. Cost of the split: $0.69 and one session's start-up.
- **Crashes, timeouts, compactions:** none. Longest session 83.0 of 90 minutes (#146); highest
  context 49% (#129, haiku high); everything else 35% or under.
- **CI.** Two failures, both on mid-step pushes to `relay/104` (41475951444 during build-3,
  41491391958 on build-5's WIP), green before the review. Every review waited for its own head
  commit (4–5 `gh run watch` in each review transcript).
- **Merges.** Four `merge: true`, four merges on the first try, 10–12 s after the review ended.
  The retry path wasn't exercised.
- **Pages.** Every merge rebuilt main's site (e.g. 41522109572 after #142). The cancelled `site`
  runs are superseded by newer ones (`cancel-in-progress`); none failed. But see screenshots in
  §2: they break when the branch is deleted.
- **The board.** Status changes landed (project events on #101's timeline). `relay-events` runs
  without `RELAY_BOARD_TOKEN` and logs that the board won't be updated, so a piece the owner
  closes isn't moved. Whether "Waiting on" shows the reset: unchecked (no read of the board).
- **The log** still prints `14.000000000000002%`; the blocks are rounded.

### What could come out of `relay.mjs`

- `EXPECTED_POINTS` and `expectedPoints`, for a flat 75% start line (above). Try for M4.
- The six-hourly schedule, once a piece's close starts a run (§5): a daily run is enough to catch
  a dropped dispatch.
- Not the crew's CI wait before merging: the reviews made it redundant in M3, but it costs one
  call and guards against a review that doesn't wait.

## 2. The prompts

### The §8 changes

| Change (m3-relay.md §8) | Did it change behaviour | Evidence |
|---|---|---|
| `soFar` without the spec's and spec-review's decisions | Yes, at the start. Build-1's "so far" is 2.9–3.5 KB (M2: 4.5–6.4 KB) | the five build-1 prompts |
| | But builds' decisions refill it: "so far" averages 10.4 KB, 56% of an 18.4 KB build prompt (M2: 10.8 KB, 62%); 74–77% for #100 build-5 to build-7 | 24 build prompts |
| `engine/CLAUDE.md` to about 6 KB | Trimmed, then regrew 3.9×: 5.2 KB (abfa2cd) → 8.9 (#107) → 10.4 (#118) → 12.4 (#126) → 19.0 (#134) → 20.1 KB (#142). Layout is 14.7 KB; 18 added lines are `www/play/*` rows of 200–460 B each, where §8 wanted one play-script line | `git log -- engine/CLAUDE.md` |
| Root `CLAUDE.md` | Landed, 2.2 KB | `CLAUDE.md` |
| Decisions are "choices the owner might overrule" | Partly. Build decisions fell from 6.7 a step (max 11) to 4.5 (max 5); 13 of 22 builds wrote exactly five. My read: 41 of 98 are owner choices; #100's builds 1 of 26 ("Event setters take `impl Into<Option<T>>`"), #104's 16 of 23 ("A retreat is one step back") | step blocks #109–#148 |
| The summary is "what later steps build on" | No change. Builds average 2.1 KB, max 3.8 KB (M2: 2.0, 3.7), and still list files and functions (#129: "`core/src/claim.rs` holds a civ's claim (`Civ.claim`, sorted with its capital…") | step blocks |
| `review.md`: open the evidence first; a rule change goes in the verdict | Evidence, yes: every review played or measured (#101 played 20 turns, #103 led Red 30 turns on the page, #102 played seed 9). Rule in the verdict: missed once. #101's bands made "an army mustn't starve its own side" a CI failure, filed as a ledger line; the owner split it out and struck the line (owner.md §5, `relay/specs/101/split.md`) | review transcripts, PR 118, 124 |
| `spec-review.md`'s new job | Every spec-review ran the engine (23–44 Bash calls each, most of them runs). What it caught is the briefs angle's | spec-review transcripts |

### What a session reads

| Kind | n | Prompt (avg, range) | Biggest parts |
|---|---|---|---|
| spec | 5 | 25.4 KB (16.0–35.4) | design files |
| spec-review | 5 | 27.8 KB (18.2–37.8) | design files; "so far" 1.4–1.7 KB |
| build | 24 | 18.4 KB (10.0–28.4) | "so far" 10.4 KB avg |
| review | 5 | 69.5 KB (46.5–86.1) | the whole spec 19–33 KB, "so far" 13–24 KB, the design 3–22 KB |

M2's reviews averaged 49 KB, max 72. Reviews used 8–13% of their context, so size costs tokens,
not room. `engine/CLAUDE.md` reached 25 of 39 transcripts (its "Adding a mechanic" text appears
in them), so its 20 KB lands on most sessions.

### The reviews' PR descriptions

All five carry the steps table, the answer in the scores against main, the ledger lines and the
verdict. They run 728–1,304 words and 47–83 lines. Page pieces carry screenshots (PR 126: 2,
PR 134: 19 thumbnails in a 4-column table, PR 142: 3). With no play step, the reviews' own play
went in as "What the play found" (134) and "Played: a wide claim is thin" (126).

- **Screenshots break after the merge.** They link
  `raw.githubusercontent.com/…/relay/<n>/relay/specs/<n>/screens/…`; the branch is deleted on
  merge, so the same image is 404 on `relay/103` and 200 on `main`. A link at the commit lasts.
- **The review's own row is invented.** It writes "~25" minutes for #123 (actual 9.6) and "~35"
  for #140 (21.5), because the table is built before the review ends.
- **On a phone:** unchecked. My read: the 11-column steps table and the 4-column screenshot table
  scroll sideways; the rest reads.

### Lines that misled

1. `review.md` "with `"merge": true` in `result.json` if your verdict is merge", while
   `session.md`'s schema has no `merge`. #101's review wrote "## Verdict: merge" in the PR and
   "verdict merge" in its summary, and no flag. The crew posted "its verdict isn't merge". 33 h.
2. `session.md` "Under five is usual": read as a quota. Five is the mode and the maximum.
3. `review.md` "the screenshots a page piece took": no word on where they link, so they link the
   branch.
4. `review.md` "the steps table": the review fills its own row with a guess.
5. `session.md` "The crew stops a session after 90 minutes", with no clock: #148's first attempt
   quit at 42 minutes, "out of time".

## 3. The owner's stops

| Stop | M3 |
|---|---|
| Blockers | 0 |
| Merges by the crew | 4 (PR 107, 126, 134, 142) |
| Merges left to the owner | 1 (PR 118), by the missing flag, not by the verdict |
| Still open | PR 124 (#101's held half) and #105 R (paused) |

What the owner did that the process gives to a session or the crew:

- **Restarted the crew after a merge.** `/relay ready`, then `/relay run` (#102, 23:59 and
  00:00). The close event ran (41364521322) but starts nothing.
- **Caught a rule hidden in a band.** G's fail bands on camps kept and warriors starved set the
  opposite precedent to the owner's #28 call. `review.md` says a rule change goes in the
  verdict; the review called it a ledger line. The split took an interactive session and about 4
  weekly points.
- **Decided where the milestone ends.** The owner paused #105 on 1 Nov, before L, P and A ran.
  The crew has no rule for a paused piece at a milestone's end, so `m3` was never tagged and the
  owner played `ec1e47b` by hand.
- **Started the retro.** Relay finished on 2 Nov 20:37 and waited 3.4 days for it.
- **Read 68 ideas.** Sessions wrote 68 ideas across 39 sessions; none became an issue (the README
  says they don't). At least eight repeat across steps: the guard starving camps (#102 build-4,
  #102 review, #104 spec-review), clans that never leave (#102 builds 2, 3, 4), the hungry
  scout's reason (#103 build-4, #103 review), play-camp and play-lean (#100 spec-review, review).

## 4. The loop without the owner

`docs/process.md`'s loop is vision, design, proposal, pieces, Relay, play, retro. Relay already
runs pieces without the owner. Play and retro run with the owner, and findings reach new work
only through the owner. The design below moves play, retro and findings into Relay, and leaves
the owner the calls that are taste.

### (a) Play

- **A `play` step after the review, before the merge**, as `process.md` says. The crew makes it
  when the review closes done. It plays the branch (main merged in) through `sansfiat play` for
  100 turns on one retro map with `relay/players/player.md`, judges with `judge.md`, and writes
  `relay/specs/<n>/play.md`; the crew links it in the PR. A page piece also runs `www/play/all.mjs`
  and attaches its screenshots. The piece's `**Play:**` line names the model, Sonnet medium by
  default, `none` for a piece the player doesn't meet. A bad result is a finding, not a block:
  the crew merges on the review's verdict after the play step closes.
- It needs the player and judge prompts, which sit in PR 124. They merge first (the owner likes
  the player, owner.md §5).
- Every finding in `play.md` is one line with a class, `fix` or `ask` (rule (c)), and a repro: a
  `sansfiat play` lines file or a play script that shows it on main.
- **The milestone play check** runs the same on main across the seven retro maps when the
  milestone is built (below), with `sweep --judge`, into `relay/retro/<m>/play.md`.
- Findings become work: `fix` lines go to the milestone's fixes piece; `ask` lines become issues
  labelled `relay:proposed`, which never run until the owner or a merged proposal makes them
  pieces. Sessions' `ideas` take the same two classes and go the same way, so no idea is lost and
  none repeats (the crew skips a title already open).

### (b) The fixes piece and the retro run

**The fixes piece.** `process.md` wants pieces big by area, not fixes around edges, so `fix`
findings don't each become a piece. The crew keeps one open `kind:piece` per milestone, "Fixes
for M<n>", and appends each `fix` finding to its body with its repro. It runs after the
milestone's other pieces, on Sonnet medium spec and spec-review, Ford-shaped builds. Each repro
becomes a test, which is the kind of test owner.md §1.5 asks for. M3's analogue is #100 H: 4
weekly points, 50 5-hour points.

**When a milestone is built.** A milestone is built when none of its pieces can run: each is
merged, closed, or `relay:paused`, and its fixes piece is merged. Then the crew tags main `m<n>`
and opens a `kind:retro` issue for it. A paused piece stays open. The retro reads it, and the
proposal says what happens to it: move it to the next milestone as it stands, remake it, or
close it with its proof tag. For #105: M3 is built at `ec1e47b`, and R goes into M4's proposal.
`/relay retro` on a milestone's last piece or on its retro issue starts the retro early.

**The retro run.** Its steps come from a fixed `steps.json`, run by the same crew and pacing as a
piece's:

| Step | Model | Writes |
|---|---|---|
| `check` | Sonnet medium | the milestone play check, `relay/retro/<m>/play.md` |
| `relay`, `briefs`, `architecture` | Opus medium (owner.md §4) | their angle files; these don't need the owner's play |
| owner's play | none: the crew waits up to 72 h for the owner's notes as a comment on the retro issue, or `/relay retro go` | `relay/retro/<m>/owner.md` from the comment |
| `game`, `players` | Opus medium | their angle files, with the owner's notes |
| `retro` | Opus medium | `docs/proposals/<m>-retro.md` |
| `proposal` | Opus medium | `<next>.md` and `<next>-relay.md`, opened as one PR for the owner |

Angles are sessions like any step: they read the run logs and artifacts with the crew's `gh`.
The artifacts last 14 days, so the retro has to run soon after the milestone is built. The App
may need `actions: read` for them (unchecked).

**When the proposal PR merges**, the crew opens its pieces as the issues Relay reads (the
proposal already writes each one in that form), labelled `relay:ready`, in the next milestone.
The Relay change (`<next>-relay.md`) is a hand-built PR, as now, until the crew can build
itself, which this retro doesn't propose.

### (c) What may start without the owner

A finding is `fix`, and runs without the owner, when all four hold:

1. **It names what it breaks**: a line of a design file (prose or Decided ledger), a piece's Goal
   or Decided line, a touch point in `page.md`, or health (a panic, a digest mismatch, a check
   that fails on main).
2. **It has a repro that fails on main**: a `sansfiat play` lines file, a play script or a test.
3. **The fix restores that line**: it edits no design prose and adds no ledger line except one
   restating the line it restores.
4. **It fits one Ford build**: no new order, unit kind, module or score, and no number in
   `data.rs` except one a design or ledger line already fixes.

Anything else is `ask`: a new rule or a changed one, a design file's prose, a new lever or
system, a band or what the judge counts, two design lines that conflict, and any finding that
says the game is less fun without naming a line it breaks.

Applied to owner.md §1: 2 (Disband all on an army disbands one: `founding.md:72` says Disbanding
"takes all kinds") and 3 (the scout skips mountains: `scouting.md:58` says it "checks the nearby
mountains before crossing") are `fix`. 1 (Route to… is a new order), 4 (a trail always matters
is a rule change) and 5 (a test harness is a new system) are `ask`. Applied to M3's 68 ideas, my
read is about 14 `fix` (the page enabling Raise when the engine refuses it, #103 spec-review;
play-hold and play-camp failing; the feed missing fight icons, #104 builds 1 and 2; `line::cutter`
naming a rival that cuts nothing, #104 spec-review) and the rest `ask` or repeats.

### (d) The owner's stops

| Now | With the loop |
|---|---|
| Open each piece after reading the proposal | Merge the proposal PR; the crew opens its pieces |
| A blocker's reply | Same |
| A merge the crew wouldn't make | Same, and fewer: `merge` is required (§5) |
| Restart the crew after a merge | Gone: a close starts a run |
| Turn ideas into issues | Gone: `fix` runs in the fixes piece; `ask` waits in `relay:proposed`, which the owner promotes with `/relay ready` (the command the owner already typed) or leaves to the next proposal |
| Play main and dictate, then start the retro | Play and dictate as a comment on the retro issue, within 72 h, or not at all |
| Promote or strike ledger lines in planning | Same, in the proposal PR: the proposal session drafts the promotions as a design edit beside it |

The owner's hours become one merge a milestone, one play, and replies. What stays the owner's
is taste: the design's prose, every `ask`, and the next milestone's shape.

### (e) What it costs

| Line, per M3-sized milestone | Weekly | 5-hour | Source |
|---|---|---|---|
| Five pieces | 33 | 434 | measured |
| Five play steps | 2–3 | 15–30 | estimated: G's trial was $0.12–0.20 a 100-turn game (#122); M3 ran 6 five-hour points and 0.46 weekly points per list dollar; a session's overhead on top |
| Fixes piece | 4 | 50 | #100 H as the analogue |
| Milestone play check | 1–3 | 10–40 | seven games at the play step's cost |
| Retro: 5 angles, retro, proposal | 15–35 | 150–400 | estimated: an M3 Opus-medium spec cost 1–2 weekly and 11–21 five-hour points; an angle reads 2–3× more. This retro's own cost would measure it: unmeasured here |
| Total | 55–78 | 660–950 | |

M3's pieces took 37% of a week in 57 hours, then Relay sat idle. Half a week for Relay, as
`process.md` plans, is about 47 points, so an M3-sized milestone with its loop takes 1.2–1.7 of
Relay's half-weeks, or most of one whole week. The retro is the biggest new line, but it isn't
new spend: the owner runs those sessions today from the same week (`process.md`: 10–15 points
for a retro or proposal session). What changes is that it runs while the owner is away, in the
3.4 days Relay sat idle. On a week that tight, four angles instead of seven is the lever
(owner.md §4: "the top set of retro angles that lead to changes").

## 5. Fixes for the M4 Relay change

1. A review's `result.json` must carry `"merge": true` or `false`; the crew treats a review
   without it as `pending` (#101: "Verdict: merge" in PR 118, no flag, 33 h owner wait).
2. A piece's close starts a crew run (#102: owner's `/relay ready` and `/relay run`, 1 Nov 23:59).
3. One `wait` at a time: the wait job in its own concurrency group with `cancel-in-progress`, or
   the crew skips a wait when one is queued (7 waits for 4 stops).
4. The review links screenshots at the commit, not the branch (PR 126, 134, 142: 404 after merge).
5. The crew writes the review's own row of the steps table after the review (PR 118 "~25" for
   9.6 minutes, PR 134 "~35" for 21.5).
6. `session.md`: drop "Under five is usual"; say a build that follows its brief usually has none
   (13 of 22 builds wrote exactly five; #100's builds, 1 owner choice in 26).
7. `engine/CLAUDE.md`: one row for `www/play/` and a byte budget that `check.sh` enforces (5.2 →
   20.1 KB in five pieces; 18 play-script rows).
8. `review.md`: a band, a test floor or a fail check that encodes a rule goes in the verdict as a
   rule (#101's band on armies starving; owner.md §5 struck it).
9. Replace `EXPECTED_POINTS` with a flat 75% start line, or refit it from M3's medians (opus
   medium 25 against 12; haiku high 6 against 11; same eight decisions in M3).
10. A milestone is built when none of its pieces can run; tag `m<n>` then (no `m3` tag; #105
    paused).
11. The `play` step and the milestone play check, with `relay/players/` merged from PR 124
    (README: "Not built yet").
12. Findings and ideas classed `fix` or `ask`; `fix` to the milestone's fixes piece, `ask` to
    `relay:proposed` issues (68 ideas, none kept; 8 repeats).
13. The `retro` run kind and the proposal PR that opens its pieces on merge (Relay idle 3.4 days).
14. `/relay ready` as a command that labels a proposed issue ready (the owner typed it on #102).
15. The prompt gives the session its start time and says to check `date` before giving up on time
    (#148's first attempt quit at 42 of 90 minutes).
16. `relay-events` gets `RELAY_BOARD_TOKEN`, so a close moves the board (log of 41364521322).
17. The schedule goes to once a day (13 scheduled no-ops; no work started from one in M3).
18. The crew's log rounds usage, as the blocks do (`14.000000000000002%` in 41364842651).

## For other angles

- **Cost:** per-piece totals and the expected-points table in §1 come from the step blocks; the
  5-hour medians by model and effort are there for the ladder. The retro's own cost, per angle,
  is the figure §4(e) lacks.
- **Briefs:** every spec-review ran the engine; whether its runs changed the plan is yours. Build
  decisions at five per step, a third of them implementation notes, are in `soFar` for every
  later build.
- **Game:** #101's held half (PR 124) waits on the owner's call on bands; the play step needs its
  `relay/players/` prompts merged. owner.md §1.2 and §1.3 are bugs against `founding.md:72` and
  `scouting.md:58`.
- **Architecture:** `engine/CLAUDE.md`'s Layout is 14.7 KB; the M3 `www/` growth (18 play rows) is
  where it went.
- **Players:** the review sessions' own play notes ("A hungry scout's panel gives no reason", "Move
  to… doesn't warn") repeat as ideas in builds and reviews; they are the play step's first
  findings.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| Require `merge` in a review's result | #101, PR 118, crew comment 15:02 31 Oct | Removes the one owner stop M3 had for no reason (33 h) | A few lines in `applyResult` and `review.md` | take |
| A piece's close starts a crew run | #102 comments 23:59, 00:00 | No restart by hand after an owner merge | One dispatch in the close handler | take |
| One wait run at a time | runs 41406162263, 41442302129, 41491568884 | Half the wait runs, a clean log | A concurrency group | take |
| Screenshots linked at the commit | PR 134, `raw…/relay/103/…` 404 | PRs read after merge, and in the retro | One line in `review.md` | take |
| Crew writes the review's row | PR 118, PR 134 | Honest steps table | Edit the PR body after the review | take |
| Drop "Under five is usual" | 13 of 22 builds at five | Smaller `soFar`, decisions the owner reads | One line | take |
| Byte budget on `engine/CLAUDE.md` in `check.sh` | 5.2 → 20.1 KB | Stops the regrowth that trims haven't | A `wc -c` check | take |
| A rule in a band goes in the verdict | split.md, owner.md §5 | The owner sees precedents before merge | One line in `review.md` | take |
| Flat 75% start line instead of `EXPECTED_POINTS` | §1 gate tables | Less to keep calibrated, same M3 decisions | Delete a table | try |
| Milestone built with a paused piece; tag then | #105, no `m3` tag | The retro can start on its own | `nextWork` and the tag rule | take |
| The `play` step and milestone check | m3-relay.md §5, owner.md §6 | Findings without the owner's play | A step kind, PR 124 merged | take |
| `fix`/`ask` findings; a fixes piece per milestone | §4(c), 68 ideas | Bugs fixed without the owner; ideas kept | Ideas to issues, one standing piece | take |
| The `retro` run and a proposal PR that opens pieces | owner.md §6, 3.4 idle days | The loop runs while the owner is away | The biggest change here: a run kind, angle prompts, `actions: read` | try in M4, on four angles |
| `/relay ready` command | #102, 23:59 | The owner's words work | One command | take |
| Start time in the prompt | #148 | One fewer early quit | One line | try |
| Board token in `relay-events` | 41364521322 | Board right on close | One secret | take |
| Daily schedule | 13 no-ops | Quieter Actions list | One cron line | take, after the close starts a run |
| Round the log | 41364842651 | Readable log | One call | take |
| Phone check of a PR description | unchecked here | Knows whether the owner's stop works on a phone | The owner looks once | later |
