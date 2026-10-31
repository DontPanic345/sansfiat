# Review step

Review the whole piece as built, as a reader who didn't build it. Read the piece and its
design before the spec, and judge each place the builds departed from the spec against the
design. Read the code in the diff (`git diff origin/main...HEAD`), not only the tests. Open
the evidence a build names (screenshots, sweeps) before judging the page or the numbers, and
play or measure what the spec says proves the piece.

Merge `origin/main` first if it has moved, so the PR is what would land.

Fix what is small and substantive yourself, and leave nits. A change to a rule, or a step's
work missing, goes in your verdict for the owner, not in a quiet fix.

## The design ledger

Append the piece's decisions to the `## Decided` ledger at the foot of each design file they
belong to: one line each, a rule a player would notice in the game's words, ending with the
piece's number, such as `(#54)`. A rule it settled, a number now decided in words, a scope
line lifted. How it was built is the diff, not a ledger line. Never edit the prose above the
ledger.

## The PR description

The owner reads it once, on a phone. Rewrite it (`gh pr edit --body`) so it's enough on its
own: the steps table, then the answer to the piece's question in the scores with its
control, the ledger lines you added, the play note's findings if there is one, the
screenshots a page piece took, the play link
`https://dontpanic345.github.io/sansfiat/b/<branch>/`, and last your verdict: merge, or don't
and why.

## Finishing

CI runs on every push to the PR. After your last push, wait for the CI run of that commit
(`gh run list --commit "$(git rev-parse HEAD)"`, then `gh run watch` on it), not the checks
of an earlier one. Finish `done` when it's green, with `"merge": true` in `result.json` if
your verdict is merge: the crew then merges the PR, and pieces waiting on this one start from
it. Without it, the owner decides.
