# Pick step

Two sessions built the same brief from the same commit, each on its own branch: version A and
version B. You judge the work, not who did it, and land the better of it on the piece's branch
for the steps after you to build on.

## Measure

Run the brief's proof on both branches, the same runs with the same seeds and control on each
(`git worktree add` holds both at once). Read the results in the scores (`docs/process.md`,
Scores) the piece asks its question in. A summary says what a session believes; a run says what
its branch does. Then read both diffs for what the runs miss: a rule the design doesn't hold, a
check that doesn't pass, a test that proves nothing, code the next steps will trip on.

Write `relay/specs/<piece>/pick.md`: every run on A and B side by side in one table, the
differences in the diffs that mattered, and what you chose and why. The retro reads it to judge
the rung, so the table holds every number you measured, including those where B and A tie.

## Land

- **A** or **B**: merge that branch into the piece's branch.
- **both**: start from the stronger version, bring in each part of the other that beats it, and
  run the brief's proof again on the result. The table gets a row for what landed.

You're done when what landed is on the piece's branch with every check passing, and `pick.md`
holds the runs that chose it.

## Finishing

`result.json` adds `"chose": "A"`, `"B"` or `"both"`; a pick without it isn't done.

- `summary` is what the steps after you build on: what landed, written as one build's summary.
- `decisions` are the landed version's decisions, and any you made in combining them.
- `ideas` are what you noticed in comparing. The crew files each landed version's own ideas.
