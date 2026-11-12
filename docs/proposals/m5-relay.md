# M5: Relay and process

What changes in how pieces get built, from the M4 retro ([m4-retro.md](m4-retro.md)) and your
notes (`relay/retro/m4/owner.md`). All of it is built in this PR, by hand, and lands on main
when the PR merges, before any M5 piece opens. `relay/README.md` is how it runs now. The
crew's GitHub paths below are covered by its tests and have not yet run for real: M5's first
piece is their first run.

## 1. The experiment: a whole piece on Fable

Every M5 piece has a `Whole: fable medium` line. Its first session plans and builds it.

- **A fresh window.** A whole step starts only while the 5-hour window is under 10%, and only
  in a job with five hours left.
- **Usage in the session.** The prompt gives both windows as the session starts, and a hook
  repeats them every fifteen minutes. At 70% it says to commit and push; at 90% to stop and
  hand back.
- **Handing on.** The session may end the piece itself with a merge verdict, or write
  `steps.json` with builds, each with a brief, and a review, and finish without one. The crew
  makes those steps and puts the play step before the review. It is Clay and Kiln with the
  spec inside the clay.
- **A session the limit ends costs no attempt.** The next waits for the reset and is told why
  the last ended and the branch's last commit.

The retro reads what a window bought: what each whole step did itself, what it handed on, and
the cost against M4's stepped pieces.

## 2. The twin and the pick are gone

With them: the twin's branches, the coin, the pick's prompt and the blind A and B. The ladder
keeps what they found: `canvas`, `glaze` and `clay` run Sonnet high from 8 to 10 points and
Opus medium from 11, a `magnum` runs Opus medium, and nothing runs Opus low.

## 3. Above the line and below it

What reaches an issue, a PR or a comment is written for you to read: the PR description, the
piece's status line, the log, an idea. Everything a session writes for other sessions stays in
files on the branch: the spec, the briefs, evidence, `notes.md`. The crew posts no per-step
comments on the PR, and a step's summary and decisions stay in its block.

## 4. Ideas

- An idea is something you would have to decide. A builder's note goes in the piece's
  `notes.md`, and what a designer could settle, it settles.
- Each milestone has one ideas issue, "Ideas from M<n>", a line an idea. A `fix` still goes to
  the fixes piece.
- A spec, whole or play session is shown the open lines. A designer takes the obvious ones
  into its piece and names them; the crew ticks them. The play step doesn't file one already
  open.
- The proposal ends every idea: taken, dropped or kept. `m5.md` does that for the 91
  `relay:proposed` issues open now, and merging it closes each one it takes or drops.

## 5. Trust, and verify another way

A session doesn't re-run what the session before it just ran. It trusts the saved result and
checks on another axis: a cold read of the goal against the diff, a different test, a played
game. The review reuses the builds' saved sweeps.

## 6. Tests

- A test a build adds runs in seconds. `engine/check.sh` fails when any test file takes over a
  minute.
- The test that ran the whole sweep to check JSON keys is a report now: `check.sh` on CI went
  from 12m24s to 9m22s.
- A build runs the tests its change touches while it works, and `check.sh` whole once, waiting
  on it with one call.
- A test that carries a rule's bound is not a build's to loosen.
- Sessions stay on the one-hour cache. The five-minute one rewrites the whole context on any
  wait longer than that, and `check.sh` is longer.

## 7. The log, the PR and the wait

- One log issue a milestone, "Relay log: M<n>". Each run's report links the step it ran and
  gives both windows, the week beside the share the day allows. Closes #216.
- A PR is a draft until its verdict.
- The review puts the judge's block above the steps table.
- A weekly stop starts a wait for its lift, as a 5-hour stop does, five hours at a time.

## 8. The ledger

A design file's `## Decided` is its decision record: a few lines, each a choice that shapes
the game. How a rule is built isn't in it, and a detail is pinned only once it has churned.
The review adds a line only at that weight.

## 9. The play step

Its prompt carries the piece and the open ideas, with no build summaries. It plays the civ
its piece's number gives, so a milestone's pieces don't all play small 2 as civ 1. Piece P
tries a session that plays the page in a browser; if that works, the retro proposes it for the
play step.

## 10. Not built

- Counting 5-hour points across a window's reset (#322): the reading stays a floor.
- Dropping a build's `decisions` field: `session.md` now says what counts as one.
- A phone check of a PR description: yours, once.
