# M1 retro: the crew and the sessions' environment

Pieces #31 (The map and the page, PR #39) and #32 (Founding, PR #48). Evidence is the `relay`
runs 40100535820, 40109339917, 40131547026, 40162811649, 40189094743, 40211144705, 40232162428
and 40232705905, their `relay-runs-*` artifacts (crew.log, prompts, transcripts), the Relay log
(#12), the step issues #38–#53, and the CI and site runs on `relay/31` and `relay/32`.

The short version: the sessions ran cleanly. All 14 steps finished on their first attempt, with
no crashes, timeouts, blocks or compactions. The longest session took 23 minutes of the 90
allowed, and context peaked at 19%. What went wrong was crew-side: one ordering bug that lost a
step, and pacing that kept the crew idle far longer than it worked.

## 1. Steps are made from the spec's `steps.json` before spec-review can change it

On #32, spec-review split build-3 into build-3 (logic) and build-4 (look). It added
`relay/specs/32/build-4.md` and a build-4 entry in `steps.json` (f196817). By then the crew had
already made every later step from the spec's `steps.json`. crew.log of 40211144705 has
`made step spec-review (#49)` through `made step review (#53)` at 16:30:44–16:31:04, and
spec-review only started at 16:31:11. So build-4 never existed and never ran. Build-3's summary
(#52) says "Left for build-4: drawing capitals/civ land/units/routes, feed KINDS …". The review
(#53) then wrote build-4's 240 lines unbriefed (a7e451d, "the look build-4 was to give") and
reviewed its own work. The PR #48 body says "build-4 never landed".

Nothing noticed: no check compares the steps that were made with `steps.json` at review time.
#31's spec-review didn't change the step list, so it hit only #32.

**Take out:** make the later steps after spec-review rather than after the spec (`plan` in
`relay.mjs`). The spec step's `steps.json` check stays as it is. Spec-review is then the last
word on the plan, which is what the README already says it is.

## 2. Pacing kept the crew idle far longer than it worked, and the owner had to override it by hand

| Piece | Session time | Wall clock (spec start → merge) |
|---|---|---|
| #31 | ~110 min over 8 sessions | 22 h (24 Oct 14:09 → 25 Oct 12:21) |
| #32 | ~53 min over 6 sessions | 8.5 h (25 Oct 12:21 → 20:55) |

- **The weekly ramp at the start of the week.** On day 1 the weekly limit is 95 × 1/7 = 13.6%.
  Run 40131547026 stopped after one step at "weekly usage 14.000000000000002% ≥ 13.6% (day 1)".
  40162811649 (04:41) stopped on the same rule. #31 then sat for 14 h (21:14 → 11:29) with the
  week at 14% used.
- **The 5-hour stop at 50%.** It ended 40100535820 at 62% (one opus build-2 alone took it from
  44% to 62%), 40189094743 at 55% and 40211144705 at 50%. The check runs only between steps, so
  an opus step overshoots it.
- **The schedule runs late.** `17 */6` started runs 2.5–5 h after their slot: 18:17 → 20:49 and
  20:51, 00:17 → 04:37 and 04:41, 06:17 → 11:00 and 11:29, 12:17 → 15:57 and 16:19. A budget stop
  therefore costs at least one late slot. After the 12:21 stop, #32's spec waited until 16:19,
  when the 5-hour window had dropped to 0%.
- **The owner's override.** After 40211144705 stopped at 50% (17:04), #32's review sat waiting
  until the owner pushed 23644e2 ("raise the usage limits to 90% for one run"), dispatched
  40232162428 (20:44), and pushed 23e4009 ("back to the normal usage limits") a minute later.
  These are the only two crew commits on main in the period, and together they change nothing.
  The other two commits are the game pieces themselves (c0e3a19, 170a183). The scheduled run
  40232705905 queued behind the dispatch and found nothing to do.

Two whole pieces used about 18 points of the weekly window (9% → 27%). The pacing protected far
more headroom than M1 needed.

**Take out:** drop the day-of-week ramp in `pace` and use one flat weekly cap. Raise the 5-hour
stop to a level the owner is happy to leave unattended, so a run-at-90% commit is never needed
again. Both are deletions or constant changes, with no new mechanism.

## 3. `www/check.mjs` is still not in CI, and sessions can't put it there

`check.mjs` checks the data the page depends on, and both pieces extended it. #31 build-5 added
it to `ci.yml`, but the push was refused because the App can't write workflow files. The session
then tried `git checkout HEAD~1 -- .github/workflows/ci.yml && git commit --amend`, which auto
mode denied as "Git Destructive" (the transcript's `permission_denials`). It finished with
a589cb3, "leave ci.yml for the owner (the runner can't push workflows)". The same gap then
appears in #31 build-5's ideas, in #32 spec's ideas ("CI still doesn't run www/check.mjs"), in
the PR #48 known gaps, and in `engine/CLAUDE.md`. It is still open: the engine job ends at
`./cross-build.sh`. Every session that touched the page ran `check.mjs` by hand, so nothing
slipped through in M1. A later page step that forgets will slip through.

**Fix without touching the crew:** have `cross-build.sh`, which CI already runs, finish with
`node www/check.mjs`. Sessions can maintain that file. Otherwise, the owner adds the one line
to `ci.yml`.

## 4. CI caught nothing, and the crew's "mark ready so CI runs" step is dead code

CI ran 16 times on the two branches (40101712346 … 40232749601), once per push, and all 16 runs
passed. The sessions ran every check in `engine/CLAUDE.md` before each push (fmt, clippy,
`cargo test`, `build-web.sh`, `cross-build.sh`, `check.mjs`). CI repeated them as a second
opinion. That's the right outcome, but it shows CI isn't the safety net here: the sessions' own
discipline is.

CI also ran while the PRs were still drafts, from the spec onwards (40101712346 at 14:15 on
24 Oct, with #39 opened as a draft at 14:15:46). `runStep`'s comment "CI runs on the whole piece
once the PR is ready" is wrong, because `pull_request` fires on drafts.

**Take out:** the `markPullRequestReadyForReview` call in `runStep` and `draft: true` on the
PR, plus the README sentence about them. Keep them only if the owner uses draft as their own
"not reviewed yet" signal. The review sessions waited for CI correctly: #32's review ended with
`gh pr checks 48 --watch` and finished 30 s after CI, and the crew merged 5 s later.

## 5. Board updates failed on every finished step

11 of the 14 step ends in these runs log "couldn't update the board". They fail either with
"Number values cannot exceed 8 decimal places" (the cost is unrounded, e.g.
0.9322537999999999) or with "Column value must be a valid value for number column". One field's
failure aborts the rest of the loop in `setBoard`. The cost angle (`cost.md`) has the details.
In M1 nobody read the board, and everything on it is already on the step issues.

**Take out:** the board's number fields, or the board altogether (`setBoard`, `loadBoard`,
`RELAY_BOARD_TOKEN`, the README section). The log's float noise ("55.00000000000001%") is the
same rounding issue, but cosmetic.

## 6. Summaries carry evidence dumps into every later prompt

The step summaries on #31 build-2 and build-3 are 18 KB each, mostly ASCII maps and sweep
tables. Every later step's prompt carries every earlier summary, so "This piece so far" grew to
58 KB of #31 build-5's 68 KB prompt, and #31's review prompt reached 103 KB. #32's summaries
stayed at 2–3 KB and its prompts at 17–26 KB (56 KB for the review, with the diff). The cost
was small, since context peaked at 18–19% with no compactions. But it is all reading that no
later step needed.

**Fix in the prompt, not the crew:** one line in `prompts/session.md` under `summary`: what
later steps need to know, with evidence (maps, sweep output) in the commit or the PR.

## 7. Playing the page: every session improvises its own headless Chrome driver

Four sessions played the page: #31 build-5, #31 review, #32 build-3 and #32 review. Each wrote
its own Node/CDP driver from scratch in `/tmp` (`/tmp/shots`, `/tmp/review`, `/tmp/drive`) to
launch Chrome, click, right-click via `Input.dispatchMouseEvent`, and take screenshots. They then
read 35 full-size PNGs (0.5–0.67 MB each, about 17 MB of tool results), by far the largest
outputs in M1. The drivers worked: the reviews played the pick, orders and hunger through the
page as the pieces asked. #32 spec had to find Chrome at `/usr/bin/google-chrome` itself, and
#32 review's ideas propose "www/play.mjs so later reviews don't rewrite them".

**Game-side, not crew:** commit a small play script beside `check.mjs` with clicks by tile
(`map.screenOf`) and a screenshot at a fixed, smaller size. Name it, and Chrome's path, in
`engine/CLAUDE.md`. That gives each page session one less thing to build, and smaller images.

## 8. Navigation and tool economy: fine, with small repeated waste

- **Orientation.** Every build session opened the same way: `cat engine/CLAUDE.md`, `ls` of the
  tree, `wc -l`, the spec, then whole source files via `cat` (15–29 KB outputs). That was 6–23
  navigation calls before the first edit, and almost no use of Read or Grep. It was cheap and
  it worked. There's no root `CLAUDE.md`, so `engine/CLAUDE.md` arrives only when read, and every
  session read it first. Nothing to change unless prompts grow.
- **Tests.** `cargo test` takes 70–120 s (`opt-level = 3` dev profile; #32 build-2 measured
  ~73 s). Build sessions ran it 10–11 times (#31 build-1, build-2, build-3; #32 build-2). That
  makes it the slowest tool by far, but still minutes against the 90-minute cap.
- **Minor misfires, one turn each.** Calls to a lowercase `bash` tool ("No such tool available")
  in five sessions. `sleep 60` and `sleep 100` blocked while reviews waited on background tests
  (#31 review then used Monitor). A few `ls` calls on paths that don't exist.
- **Reading CI logs.** No session needed CI logs, because CI never failed (see 4). Whether they
  can read them through the `gh` wrapper is untested.

## 9. What worked, and needs no change

- No crashes, timeouts, retries, blocked steps, holds or `/relay` commands on #31 or #32. Each
  step's `attempts` stays 0 because it counts only unfinished attempts.
- The `After` hold (30350b2): #32's spec step was made at 12:21:27, six seconds after PR #39
  merged.
- Merge on verdict: both PRs were merged by the crew with CI green (c0e3a19, 170a183). The site
  built `/b/relay/31/` and `/b/relay/32/` previews after each CI run, and removed them on close.
- Runs stopped by the budget cost about 40 s each (40109339917, 40162811649) and post nothing to
  the Relay log. That's fine, although the log can't show why a piece sat still for a day.

## Owner's hands in the period

Two commits and one dispatch to get past the 5-hour stop (23644e2, 40232162428, 23e4009). That
was all. The check.mjs line for `ci.yml` (3) is still waiting on the owner.
