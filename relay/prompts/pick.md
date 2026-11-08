# Pick step

Two sessions built the same brief from the same commit, each on its own branch: version A and
version B. You judge the work, not who did it, and land the better of it on the piece's branch
for the steps after you to build on.

## Compare

Each version's summary names its evidence: the runs it made, in a file under
`relay/specs/<piece>/` on its own branch (`git show <branch>:<path>`). Those numbers are
measured; take them as they stand. Run the brief's proof yourself only for a number the
comparison needs and a version doesn't have: a score one reported and the other didn't, or runs
on different seeds or control (`git worktree add` holds both branches at once). Read the numbers
in the scores (`docs/process.md`, Scores) the piece asks its question in. Then read both diffs
for what the numbers miss: a rule the design doesn't hold, a check that doesn't pass, a test
that proves nothing, code the next steps will trip on.

Write `relay/specs/<piece>/pick.md`: every number for A and B side by side in one table, each
marked as the version's or your own run, the differences in the diffs that mattered, and what
you chose and why. The retro reads it to judge the rung, so the table holds every number,
including those where B and A tie.

## Land

- **A** or **B**: merge that branch into the piece's branch.
- **both**: start from the stronger version, bring in each part of the other that beats it, and
  run the brief's proof on the result. The table gets a row for what landed.

You're done when what landed is on the piece's branch with every check passing, and `pick.md`
holds the numbers that chose it.

## Finishing

`result.json` adds `"chose": "A"`, `"B"` or `"both"`; a pick without it isn't done.

- `summary` is what the steps after you build on: what landed, written as one build's summary.
- `decisions` are the landed version's decisions, and any you made in combining them.
- `ideas` are what you noticed in comparing. The crew files each landed version's own ideas.
