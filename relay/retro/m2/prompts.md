# M2 retro: the prompts and steering files

What each session was told, set against what it did. Sources: the saved prompts and stream-json
transcripts of 36 sessions in five `relay-runs-*` artifacts (runs 40691846214, 40781134477,
40825502720, 40857386196, 40968311410; 28 to 30 Oct), the `crew.log` results (summary, decisions,
ideas as written), and `engine/CLAUDE.md` over `git log`. Six pieces: #54, #29, #28, #55, #56, #57
(PRs #59, #68, #75, #82, #89, #94). The other run artifacts were empty.
"Measured" means counted from those files; "my read" means judged from reading. Transcript counts
come from scripts matching command text, so treat them as approximate (+/- 1-2).

## Verdict

The M1 fixes held: no table or map reached a summary, builds ran `check.sh` after their last edit,
nobody searched for Chrome, nobody edited `docs/design/`. What is left is mostly size, and it comes
from one place: `soFar` repeats the spec step's and spec-review's summary and decisions in every
later prompt, although `spec.md` already holds them and builds read `spec.md` anyway. After that: a
line that points spec-review at a file already in its prompt, a `result.json` that ended up tracked
on main, build `decisions` that are mostly implementation notes, and `engine/CLAUDE.md`, which
doubled again (5.2 KB to 10.4 KB) and describes the code a second time. The edits are cuts and
one-clause rewordings; two small additions are marked "try".

## 1. Prompt size (measured)

| Kind | n | Whole prompt (avg, range) | Biggest parts |
|---|---|---|---|
| spec | 6 | 19.9 KB (16.2 to 26.0) | design files 5.5 to 14.3 KB; `spec.md` prompt 6.6 KB constant |
| spec-review | 6 | 23.9 KB (18.7 to 28.7) | same, plus the spec step's summary and decisions 3.2 KB |
| build | 18 | 17.2 KB (9.6 to 34.0) | "This piece so far" 10.8 KB avg (62%); the brief 2.4 to 11 KB |
| review | 6 | 49.4 KB (30.0 to 72.4) | so far 9.2 to 30.0 KB; whole spec 7.3 to 20.1 KB; design 5.5 to 14.3 KB; diffstat 3.5 to 5.6 KB |

M1 for comparison: build-5 68 KB, review 103 KB. M2's maxima are 34 KB (#54 build-5) and 72 KB (#54 review).

The steering text itself (`session.md` plus the kind's file) is 3.3 to 4 KB, under 10% of every
build and review prompt. `spec.md` (6.75 KB: shapes, scoring tables) is a third of a spec prompt and
is appended to spec-review too.

Inside "so far" for builds, decisions are 53% (5.7 KB avg) and summaries 47%. The spec and
spec-review entries alone are 4.5 to 6.4 KB in every build prompt, 31% of the average build prompt,
and a build-1 prompt is almost nothing else (#54 build-1: 6.4 of 20.2 KB; #56 build-1: 4.5 of 9.6 KB).
The spec's decisions repeat `spec.md`'s "Settled (decisions the owner can overrule)": `spec.md`
tells the designer to list them there, `session.md` tells it to list them again in `result.json`.
14 of 18 builds `cat` `spec.md` in their first six calls, so they read the list twice.

Later entries (build summaries and decisions) are not the problem: #54 build-5 read 26 KB of them,
against M1's 58 KB.

## 2. Summaries and decisions (measured)

| Kind | Summary avg / max | Decisions avg n / max n | Decision chars avg / max | Ideas avg n |
|---|---|---|---|---|
| spec | 1.2 / 1.8 KB | 8.5 / 15 | 1.9 / 3.8 KB | 1.5 |
| spec-review | 1.1 / 1.8 KB | 3.5 / 7 | 0.8 / 1.6 KB | 0.8 |
| build | 2.0 / 3.7 KB | 6.7 / 11 | 1.9 / 4.1 KB | 2.0 |
| review | 0.8 / 1.2 KB | 1.2 / 2 | 0.3 / 0.6 KB | 2.5 |

(Build rows count #28 build-1 twice: its retry's result, which build-2 then re-wrote unchanged.)

- "One paragraph": held in form (every summary is one block; no tables or maps), not in spirit.
  Builds average 2 KB and the longest (#54 build-1, 3.7 KB) are `Added: a, b, c ...` inventories of
  what the diff shows. 15 of 18 builds name a `relay/specs/<piece>/` file for evidence, so the M1
  "evidence goes in a file" line worked. M1's 20 KB summaries are gone.
- "Decisions the owner might overrule": did not hold for builds. A keyword pass finds 21 of 112
  distinct build decisions that are plainly implementation (a helper, a script name, where a test
  lives): #54 build-1 "Clan embeds Actor as a field `actor`, with #[serde(flatten)] and
  Deref/DerefMut", #57 build-1 "`the_save_version_pins_the_rules` lives in `cli/tests/save.rs`",
  #57 build-2 "Added game.notes(id) on window.game for the play script". My read (unchecked) is a
  third to a half are of that kind. The overrulable ones sit beside them: #28 build-1 "The scout
  walks on its whole labour ..., not on one person's pace as the spec's Settled 8 says for a camp",
  #57 build-1 "`run` with an explicit `--turns` runs past the pre-game's end ... the owner may prefer
  a separate flag". Spec steps are the exception: their decisions are the point.
- Ideas: 65 across 36 sessions. Owner-task ideas repeat because "so far" shows decisions but not
  ideas: #57 filed "design files cite merge4/ paths" / "drop merge4 from ci.yml's matrix" in its
  spec, spec-review, build-2 and review (`ci.yml` appears 16 times in the logs). No session can see
  an earlier idea.

## 3. Orientation, re-reads, searching (measured, approximate)

Builds (18): the first source edit comes after a median 14 tool calls (2 to 27), 0.1 to 5.4 minutes,
with 32 to 131 K tokens of context loaded. That is 5 to 25% of a session by calls; most of it is `cat`
of whole source files, not of prompt files. What is prompt-driven:

- 13 of 18 `cat` their own brief in the first six calls (`ls relay/specs/N/ && cat build-N.md`), though
  the brief is the first thing in the prompt.
- 14 of 18 `cat` `spec.md`. The briefs say "Read `spec.md` Settled 1-4"; intended.
- 11 of 18 `cat` `engine/CLAUDE.md` in the first six calls, some again later (#29 build-2 six times,
  #57 build-1 four) after editing it.
- Spec-review: 5 of 6 read `relay/prompts/spec.md` and/or `spec-review.md` from disk, which the prompt
  already carries. The cause is `spec-review.md`'s last line: "You're done when `steps.json` reads as
  `relay/prompts/spec.md` says." A path invites a `cat`. #28 spec-review ran
  `cat relay/prompts/spec.md | head -50` in call 2 and `cat relay/prompts/spec-review.md` in call 4.
- Spec (6 of 6) pulled the previous piece's `spec.md` / `steps.json` / a brief as a template (#54 read
  `specs/32`, #29 `specs/54`, #28 `specs/29`, #55 `specs/54`+`29`, #56 `specs/55`+`54`, #57 `specs/56`+`55`).
  The prompt gives no format for `spec.md` or the briefs, so each spec copies the last. Four of six also
  opened `docs/proposals/m2.md`; two ran `cat CLAUDE.md` for a root file that doesn't exist. My read:
  3 to 8 calls and 2 to 4 minutes per spec, and it freezes the format.
- Retry after a blocker (#28 build-1, second attempt): about 10 calls finding what the first attempt did
  (`git log --stat`, `tail relay/runs/crew.log`, `head -c 1500` of its own saved prompt). The prompt
  carries the owner's reply but nothing about the first attempt.
- No session searched for Chrome (M1: six of 14). No `printenv` or token in any command.
- All 18 builds ran `check.sh` at least once after their last source edit (my pattern match, not
  confirmed by the crew). `build.md`'s "leave every check passing after your last edit" and
  `session.md`'s "`engine/check.sh` runs every check CI runs" worked as a pair.

## 4. Lines, quoted

### Changed behaviour

- `session.md` "Tables, maps and sweeps go in a file under `relay/specs/<piece>/`, which the summary
  names." 15 of 18 builds name a file; no table in any summary.
- `session.md` "Google Chrome ... `engine/www/play.mjs` drives the page in headless Chrome." 11 of 24
  build and review sessions ran `play.mjs` or a `play-*.mjs`; none looked for a browser.
- `session.md` "`engine/check.sh` runs every check CI runs on the engine." Nobody invented its own set
  (M1 finding 3).
- `spec.md` "the review by coin": all six specs ran the `Math.random` one-liner and wrote "by coin".
  The ladder and "M2's challenge" were followed to the letter (every non-Magnum piece has a Haiku
  step; every piece a step off the ladder). Those are M2 experiments; cost and briefs judge them.
- `review.md` "judge each place the builds departed from the spec against the design": reviews of #28,
  #29 and #57 each list the departures and rule on them (#57: "The build's departures from the spec
  stand"). They take them from the builds' summaries and decisions, not by diffing `spec.md`.
- `review.md` "Merge `origin/main` first if it has moved": #56 review (111 calls, 23.8 min, $2.28)
  merged #75's scouts into the hold rules, fixed a missing brace in main's `select.js`, and changed the
  AI's zoning rule ("ai_zone now zones only if its line ... carries people x food"). Useful, and the
  largest change in M2 that nobody else reviewed; it followed "Fix what is substantive yourself".
- `spec-review.md` "a number that can't pass on the current map": #54 and #28 spec-reviews ran sweeps
  on the unbuilt map (#28 wrote `sweep.md` and dropped the fallback cap). The ones that ran something
  changed the plan; #54, #55 and #57 ran under a minute and changed one or two lines.
- `spec-review.md` "the crew makes the steps from what you leave": no M1-style loss. `steps.json` was
  rewritten by spec-review in 1 of 6 pieces (#28) and the crew used it.

### Did nothing, or nothing visible

- `session.md` "`docs/design/` is the owner's, and only the owner edits it." No session edited it (the
  design commits in #75 and #89 are the owner's). Keep; it's cheap.
- `session.md` "Anyone signed in to GitHub can read your transcript ...": no effect visible.
- `session.md` "`pending` ... (two unfinished attempts block the step)": no session finished
  `pending`. `blocked` was used once (#28 build-1) and worked.
- `review.md` "the first line 'Closes #<piece>', then the steps table": `sessionPrompt` also puts the
  table in "The steps" with the same instruction. Two instructions for one table; reviews did it right.
- The review prompt's "The diff to review": 3.5 to 5.6 KB of `git diff --stat`. Reviews run their own
  `git diff` on call 1 to 3; #57's read one file of it.
- `spec.md`'s shape "Expect:" lines ("Haiku and Sonnet", "Opus clay and a cheap kiln") restate what
  the ladder gives. Briefs angle to judge whether the shapes still earn their space.

### Misled or misread

- `spec-review.md` "reads as `relay/prompts/spec.md` says": a path where the text is already in the
  prompt (section 3).
- `result.json` and "Commit everything ... push": `relay/runs/` is in `.gitignore`, and #55 spec even
  ran `git check-ignore relay/runs/result.json && echo ignored`. But #28 build-1's second attempt ran
  `git add -f relay/runs/result.json` (commit `fee8797`, "#28 build-2: result.json"). After that the
  file is tracked: it reached main with #75 (`aa4e265`) and the PRs for #56 and #57 carry commits
  touching it. The crew deletes the working copy before a session, so sessions see ` D
  relay/runs/result.json` and spend a call restoring it (#28 build-2 `git checkout relay/runs/result.json`,
  #57 review's first command). The prompt did not ask for `-f`; the cure is `git rm --cached`.
- "Act on it." above the owner's reply. #28 build-1's reply ended "Carry on with build-2 from the branch
  as it stands", and build-1 did build-2's work inside its own step ("as the owner's reply directed").
  The next session, build-2, found it done (3 calls, $0.07) and its result is a copy. Crew/briefs.
- #57 build-2's brief: "Name the screenshots in your summary for the review." The review (opus low, 3
  calls, 2.7 min) read one engine file and said so: "I didn't read the page code line by line or open
  the screenshots". Nothing in `review.md` asks a review to open the evidence a build names.
- #28 build-2's brief wrote `./cross-build.sh`; the file is `engine/cross-build.sh`. One failed call.

## 5. `engine/CLAUDE.md`

| Commit | Bytes | Lines |
|---|---|---|
| `170a183` Founding (#48), end of M1 | 10 918 | 59 |
| `267775d` owner's M2 prep (Layout rows cut) | 5 205 | 61 |
| `9cc1215` #59 | 5 462 | 61 |
| `b54e6f2` #68 | 6 385 | 63 |
| `4986c52` #82 | 7 085 | 65 |
| `aa4e265` #75 | 7 759 | 66 |
| `b73abb5` #89 | 8 515 | 67 |
| `70454be` #94 (now) | 10 354 | 74 |

It doubled in six pieces (+0.3 to +0.9 KB each, +1.8 KB on #57), back to where M1 ended. Rows that
regrew: `www/` 302 to 1 362 B, `units.rs` 123 to 663 B, `world.rs` (notes, alerts, SAVE_VERSION). Also:

- Commands: five `play-*.mjs` lines (150 to 300 B each, with per-piece default folders such as
  "(default relay/specs/29/screens)"); `sweep --after 200 --maps retro [...]` (700 B of flags);
  `run --why`; `digest --lead`; `replay`'s SAVE_VERSION.
- The `www/` row lists those play scripts a second time, then feature notes (View > Hidden country,
  `setFocus`, `setHomeLine`, the alerts box, Paint and Unpaint, `lineColour`) that the code says.
- Invariants gained two bullets (~800 B: reasons and alerts are derived; a digest change bumps
  `SAVE_VERSION`). These are live: no code states them.
- About nine builds edited it unprompted (command-text match, unchecked count); five of six pieces' merge
  diffs touch it.

Use over the 24 build and review sessions (measured): `sweep` 19, `cross-build` 12, `play.mjs` 11,
`play-camp` 9, `play-lean` 6, `digest` 6, `run` 6, `bench` 4, `play-scout` 4, `play-hold` 4, `replay` 3,
`play-read` 1; `clear`, `map` and `http.server` 0 (my `map` pattern could miss a binary path). The
`cargo fmt`/`clippy`, `build-web.sh && cross-build.sh` and `check.mjs` lines repeat what `check.sh` runs.

Stale or doubled: per-piece screen folders in the play lines; the play scripts twice; the `clear` and
`http.server` lines. Live: `sweep`, `digest`, `run`, `check.sh`, "Adding a mechanic", Invariants.

## Edits (cuts, rewordings, additions)

Cuts.

1. `relay.mjs` `soFar`: for the `spec` and `spec-review` entries print the summary only (decisions stay
   on the step and the PR, and in `spec.md`). Saves 2.7 to 3.4 KB in every build and review prompt.
2. `spec-review.md`: `You're done when steps.json reads as relay/prompts/spec.md says.` becomes
   `You're done when steps.json reads as the spec step's rules say.`
3. `review.md`: delete "the first line 'Closes #<piece>', then the steps table," (the prompt's "The
   steps" section says it).
4. `engine/CLAUDE.md`, target about 6 KB: drop the `cargo fmt/clippy`, `build-web.sh && cross-build.sh`,
   `check.mjs`, `clear` and `http.server` lines (keep `./check.sh`); replace the five play-script lines
   with one ("`node www/play-<area>.mjs [out-dir]` leads a civ and screenshots; the one for your area is
   in `engine/www/`") and delete default folders; cut the `www/`, `units.rs` and `world.rs` rows to what
   the file owns, one clause each. Keep Invariants.
5. `spec.md` for M3: drop "M2's challenge" and the coin line (the owner's direction replaces the coin).

Rewordings.

6. `session.md`:
   ```diff
   -   - `decisions` are the choices you made that the piece, the design and your brief left
   -     open, and that the owner might overrule.
   +   - `decisions` are choices the piece, the design and your brief left open that the owner
   +     might overrule. How you built it is the diff, not a decision. Under five is usual.
   ```
   and
   ```diff
   -   - `summary` is one paragraph for the later steps: what changed that they build on, and what
   -     they must know.
   +   - `summary` is one paragraph for the later steps: what they build on and must know, not a
   +     list of what the diff shows.
   ```
7. `review.md`: "Fix what is substantive yourself and leave nits." becomes "Fix what is small and
   substantive yourself. A change to a rule, or a step's work missing, goes in your verdict for the
   owner." (#56 review rewrote the AI's zoning alone; M1's #32 review wrote a missing step.)
8. Crew: on a step with a `reply`, add one line: the earlier attempt's commits are on the branch.
   Only if free.

Additions (try, one line each).

9. `spec.md`: `Write spec.md and the briefs as the last piece's did (relay/specs/<n>/).` The crew names
   `<n>`. All six specs did this by hand; it saves calls but locks the format.
10. `review.md`: `Open the evidence a build names (screens, sweeps) before judging the page or the numbers.`
    Costs reading; #57's review skipped it at opus low.

## For other angles

- crew: `relay/runs/result.json` is tracked on main since `aa4e265`; `git rm --cached` it and refuse a
  session that commits `relay/runs/`. A retry after a blocker has no memory of its first attempt, and
  "Act on it." let an owner reply ("Carry on with build-2") become build-1's work.
- briefs: "Name the screenshots in your summary for the review" (#57 build-2) did nothing; #28
  build-2's `./cross-build.sh` path; spec decisions duplicate `spec.md`'s Settled list.
- cost: spec-reviews at $0.30 to $0.34 ran under a minute (#54, #55, #57) and changed little; those
  that ran sweeps ($1.83 #28, $1.52 #29) changed the build.
- game: the #56 review's AI-zone rule change and the #57 build-1 CLI change (`--turns` runs past the
  pre-game's end) had no second reviewer.

## Unchecked

- Counts from regexes over command text (first source edit, brief re-reads, who edited
  `engine/CLAUDE.md`, `check.sh` after last edit): approximate.
- "A third to a half" of build decisions being implementation: my read of 112 first lines, no second reader.
- `map` and `http.server` unused: patterns could miss a binary path.
- The review's inlined "whole spec": no review re-read `spec.md`, so the inline copy was either enough
  or ignored; I can't tell which.
- Anything before 28 Oct (artifacts empty).

## Ideas

| Idea | Value | Cost | Rec |
|---|---|---|---|
| `soFar` prints spec and spec-review summary without decisions (edit 1) | high | small | take |
| `spec-review.md` names "the spec step's rules", not a path (edit 2) | low | small | take |
| Trim `engine/CLAUDE.md` to about 6 KB, one play-script line (edit 4) | medium | small | take |
| `decisions` and `summary` wording (edit 6) | medium | small | take |
| Review: rule changes and missing steps go in the verdict (edit 7) | medium | small | take |
| `review.md` drops the duplicate steps-table clause (edit 3) | low | small | take |
| Untrack `relay/runs/result.json`; refuse commits under `relay/runs/` | medium | small | take |
| Drop "M2's challenge" and the coin line from `spec.md` (edit 5) | medium | small | take |
| Crew says the earlier attempt is on the branch when a reply comes (edit 8) | low | small | try |
| `spec.md` points at the last piece as the format (edit 9) | medium | small | try |
| Reviews open the evidence a build names (edit 10) | medium | small | try |
| `soFar` shows idea titles so sessions skip duplicates (#57 filed one four times) | low | small | later (adds size; or dedupe in the review) |
| Crew refuses an oversized summary as `pending` (M1 idea) | low | small | drop (M2 summaries stayed 0.3 to 3.7 KB) |
| Crew blocks `engine/CLAUDE.md` growth over a size | low | small | later |
| Keep the Chrome/play and `check.sh` lines in session.md | - | - | keep |
