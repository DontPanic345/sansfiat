# M3: Relay and process

What changes in how pieces get built, from the M2 retro ([m2-retro.md](m2-retro.md)). It
lands as one hand-built change from a session here, before any M3 piece opens, except where
a line names the piece it waits on. The game pieces are in [m3.md](m3.md); the process these
changes serve is [`docs/process.md`](../process.md).

## 1. What a piece is judged on

Every piece's question is asked in the scores (`process.md`, Scores): Standing, doubt,
decisions, motion, and health as a band. Camps kept and clans joined were M2's questions,
and the game they proved didn't move. From M3:

- The planner writes each piece's question in the scores, with its control, on the piece.
- The judge (piece G) measures them on every PR, and each piece adds its check and moves
  only the bands it means to, saying so in its PR.
- `spec.md` tells the designer: a proof is a run on the retro maps against a control, read
  in the scores; a built test map shows a rule works, not that the piece did.

## 2. Who designs

The coin goes. What replaces it is a rule, so M3's retro has a control for the build
experiments, which is what the coin was for.

- **Spec: Opus medium.** It writes the plan once; depth followed effort, not role.
- **Spec-review: Opus medium, with a different job.** It isn't a second designer from the
  same start; it's the one that runs things. `spec-review.md` says: read the code each
  settled rule touches, run the sweep or a play script where minutes settle a number or
  drop a fallback, and fix the plan where the run disagrees with the reading. M2's catches
  that mattered (B's levy, C's real-map sweep, F's contest rule) all came from a medium
  session that ran something.
- **The review: Sonnet high, unscored.** Opus low when `main` has moved under the piece or
  the goal is judged by play. What it must play or measure goes in `spec.md` under what
  proves the piece, since the review never sees a step's `why`.
- Nothing that judges runs at low effort.

Is spec-review worth keeping? Two mediums cost about $1.50 a piece more than a medium and a
low, on pieces of $8–20. Dropping it saves $1.50 and the catches above. Keep it, and re-ask
at the M3 retro with the new job's results in hand.

Replacing M2's challenge: **each piece runs one build step one rung below the ladder**, and
its `why` says what it expects to see. M2 only ever went up, and none of those steps had a
control. H's spec runs on Sonnet high as the probe for a cheaper designer.

### The ladder

`relay/prompts/spec.md` changes as the retro's briefs angle drafted (its §5 has the text):

- **Four scores, not five.** Importance goes; it carried no information. The ladder shifts
  down a point. Miss scores 3 for "only play, or a run on real maps".
- **Clay, Kiln, Glaze** is a fifth shape: when the kiln's rules are settled but the numbers
  only work together, end with one `glaze` step that tunes them all on real maps and may
  change the kiln. The parts are `clay`, `kiln` and `glaze`; the ladder treats `glaze` as it
  treats `canvas`. A did this in M2 and it fitted best.
- **Henry Ford's When:** a port is settled in its rules, not its numbers; prove them on real
  maps or give them a canvas.
- **Breadth costs minutes, not a bigger model.** A size line in each step's `why` (files,
  rough lines), so the retro can read cost against size.
- **Haiku stays where the ladder puts it.** It did every settled step and cost what Sonnet
  did. Clay and magnum run Opus low under 12 points, as written.
- **Settle by running.** A sweep that takes minutes is cheaper at design time than in a
  build. Run it.
- **A brief names the module that should grow**, not only the code to reuse. "Reuse the
  camp's code in `units.rs`" became a 1,559-line file.
- **Bigger steps.** A build may hold several areas; M2's longest used 31% of its context
  and 47 of its 90 minutes. Cut by area, not by size.

## 3. The design keeps up

`session.md` says only the owner edits `docs/design/`, and the retro then lists fourteen
edits for the owner to type. From M3:

- **Each design file ends in a `## Decided` ledger.** The review appends one line per
  decision the piece made, with the piece's number: a rule it settled, a number now decided,
  a scope line lifted. The prose above the ledger is the owner's, and the review never
  touches it.
- The owner promotes a ledger line into the prose, rewords it, or strikes it, in the weekly
  planning session. Nothing gates on this: a designer reads the file whole, ledger included.
- Builds and designers still don't edit the design; they record decisions, as now.
- A catch-up session writes what M2 already decided (`relay/retro/m2/game.md` §3, rows
  marked T, and §5 items 1, 2, 4, 6, 7, 8, 9, 10, 11, 12) into the ledgers as one PR, so M3's
  designers start from a design that matches main.
- If the ledgers grow faster than the owner promotes, the fallback is a design PR per piece
  that the owner merges; the sessions' side doesn't change.

## 4. The owner's stops

The owner looks at a piece once, on a phone, in a moment between things. The PR description
has to be enough:

- The steps table, the answer to the piece's question in the scores with its control, the
  design edits as a list, the play note's findings, the screenshots a page piece took, and
  the verdict.
- A blocker names its step and the choice it needs, in two or three lines, and the crew
  tells the next session that the earlier attempt's work is on the branch.
- The crew merges on a merge verdict with CI green, as now. It waits for CI on its own head
  commit and retries the merge for a few minutes before leaving it to the owner.

## 5. The play step

Once G's trial game has run and its cost is known, Relay gains a `play` step after the
review:

- A player session plays the merged branch through `sansfiat play` for a hundred turns on
  one retro map, and a judge pass lists problems in `relay/specs/<n>/play.md`, which the PR
  description links. A bad result is a finding, not a block.
- Its model goes on the piece's `**Play:**` line, starting at Sonnet, with Haiku worth a
  trial. `**Play:** none` for a piece that changes nothing a player meets.
- A page piece's play step also runs the page's scripts and attaches their screenshots.
- A milestone play check runs the same on main across the retro maps when the milestone
  closes, before the owner plays.

## 6. Pacing on a Pro plan

- **Restart at the reset.** A 5-hour stop dispatches a run for when the window resets,
  rather than waiting for a six-hourly slot that runs up to five hours late. Each M2 stop
  cost six to eleven hours of wall clock.
- **The next step must fit.** A step starts only if the window plus its expected points (by
  model and part, from M2's medians) stays under 80%. The flat 50% stop goes; every big
  build-1 overshot it anyway.
- The stop logs the reset time, and "Waiting on" shows it.
- The owner's own sessions share the window. A planning session like the retro takes
  10–15 weekly points; the owner plans them for when Relay is between pieces, or accepts
  that Relay waits.

## 7. Crew fixes

- `git rm --cached relay/runs/result.json`; the crew refuses to start a session while
  anything under `relay/runs/` is tracked, and `session.md` says never to commit it.
- The review waits for the CI run of its own head commit; `autoMerge` retries.
- A reply to a blocker names which step it's for, and a reply that says to carry on doesn't
  make one session do two steps' work.
- CI syntax-checks and runs `engine/www/*.js` through the play scripts (P makes them exit
  non-zero on a missing touch point).
- Usage is rounded; 5-hour points survive a window reset; sessions are counted, not
  `attempts`; one usage probe per step boundary.
- A milestone closes with a tag, `m<n>`.

## 8. Prompts

- `soFar` gives the spec's and spec-review's summaries without their decisions, which
  `spec.md` holds. Saves a third of every build prompt.
- `engine/CLAUDE.md` trims to about 6 KB: one row per file, one play-script line, no
  commands `check.sh` already runs. H finishes the job.
- Root `CLAUDE.md` lands, for every session alike.
- `session.md`: decisions are "choices the owner might overrule; how you built it is the
  diff, not a decision"; the summary is "what later steps build on and must know, not a
  list of what the diff shows".
- `review.md`: open the evidence a build names before judging the page or the numbers; a
  rule change or a step's missing work goes in the verdict, not in a quiet fix; drop the
  duplicate steps-table clause.
- `spec-review.md`: the new job (§2), and no path to a file already in the prompt.
- `spec.md`: write `spec.md` and the briefs as the last piece's did, named by the crew.
- `planner.md`: the flow gains the scores, the design rule and the play step; the coin
  paragraph goes.
- `relay/prompts/retro.md`, new: the angles a retro reads from (cost, briefs, crew,
  prompts, game, architecture, players, and one the milestone needs), each to one file
  under `relay/retro/<m>/`, and the retro and the next proposal written from them. The
  planning session reads the retro, not the angles.

## 9. Before a piece opens

- Run each trigger path once: `/relay status`, `/relay run`, a dispatch, a reply to a
  blocker on a closed test issue.
- Run the judge's sweep by hand on main once G merges and read the bands before L opens.
- The owner reads `war.md`, `technology.md` and `page.md`, and the four answers at the top
  of `m3.md`.
