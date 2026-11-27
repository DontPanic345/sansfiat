# M6: Relay and process

What changes in how pieces get built, from the M5 retro ([m5-retro.md](m5-retro.md), Process)
and its angles (`relay/retro/m5/relay.md` §6, `ladder.md` §9, `players.md` §5). No notes from
you came in. Sections 1 to 3 are built in this PR and land on main when it merges, before any
M6 piece opens. Section 4 is left for you to decide. #503 (Relay on Codex) is a separate plan,
and if you take it, it goes in here as its own section.

## 1. Built in this PR: the crew

- **Merge falls back when squash is refused.** The crew merges by squash, and on a 405 that
  names squash it merges with a merge commit (`mergePr`). L's merge waited 49 hours on that
  setting. A 405 for any other reason, such as a conflict, still stops the merge.
- **The play step sees the ticked ideas too.** Its prompt lists every line on the ideas issue,
  open and ticked. A ticked line that still happens is said against its own title, not filed as
  a new idea. The gift that reaches no one came back four times in M5 because the play step saw
  only the open lines.
- **A Haiku session has no Monitor tool** (`barredTools`, `--disallowedTools Monitor`). Two Haiku
  builds re-armed it 89 and 124 times, 28% of Haiku's spend, even though `build.md` said not to.
- **A piece with no milestone holds the milestone open.** The eight art pieces were opened
  with no milestone, so `m5` was tagged at the fixes piece's merge, before #445 and #446 had
  closed. The built check now counts open pieces with no milestone as well.

Each change has a test in `relay.test.mjs`.

## 2. Built in this PR: the prompts

From `ladder.md` §9 and `relay.md` §6, as written there:

- `spec.md`'s `steps.json` example scores a canvas Sonnet high at 9, not Opus low. Nothing runs
  Opus low.
- `whole.md`: a whole step scores its builds on the ladder, with the mismatch rule. G's build-1
  was a glaze at 6 on Sonnet high, written by a whole that didn't score it.
- `build.md`: `check.sh` runs in the foreground with a ten-minute timeout: one call, no Monitor,
  no sleep loop.
- `spec.md`'s size line counts the page scripts a build retunes, and counts a move's lines
  twice. Files ran 1.30x the estimate.
- `README.md`: the fixes piece's spec and spec-review run Sonnet medium. F's Opus spec cost
  $5.62 for $0.86 of builds.
- `README.md` and `planner.md`: a whole session runs Opus medium, and they give the reason. On
  Fable a whole piece took 1.6x the 5-hour points of a stepped one for the same list dollars.
- `review.md`: a verdict that leaves the merge to you is `"merge": false`, with the choice in
  the PR. Prose isn't a verdict. The crew already holds a review without the field; K's reviews
  wrote it as prose and cost a session.
- `play.md` and `spec.md`: the spec names the retro map where the piece's rule bites, and the
  play step plays it. With none named, it plays medium 8, then small 6, then small 2. All nine
  of M5's play steps played small 2.

## 3. Built in M6 by a piece

- **The judge plays the Reach player** (W). Every piece's judge then has a led game that plays
  as well as the view lets it, beside the AI and idle. No M5 judge saw the claim, because none
  had one.
- **Proofs on the stage** (T). The review can watch a rule's proof on the Tests screen.

## 4. For you to decide

- **Hand changes to the crew go through a PR.** You changed the crew by hand six times
  mid-milestone. One of them, 92939e3, dropped the weekly check, and the week went from 70% to
  97% in 22 hours. A small PR leaves a line the retro can count. It's a habit, not code.
- **Pieces opened mid-milestone.** M5 opened 23 pieces against eight, and for 58% of its hours
  nothing was ready, waiting on you. `m6.md` holds to nine. A piece you open partway through
  goes after them unless you give it an earlier `After` line.
- **#503, Codex beside Claude.** Decide it before W opens. It changes the pacing and the costs
  in `m6.md` §6.

## 5. Not built

- **One dispatch per batch of label events** (91 cancelled runs in 2.5 minutes on 12 Nov). They
  cost nothing but noise, so it waits until it gets in the way.
- **Keeping a killed step's `done` result when its commit passed** (X build-1). It cost one
  session in M5.
- **The whole step's start bar raised to 75%.** No M6 piece runs whole.
- **Counting 5-hour points across a reset.** One step in M5, and the reading is a floor.
- **A Sonnet high spec on a Henry Ford piece.** T runs a Sonnet spec, which tries it once.
