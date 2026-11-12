# Review step

Review the whole piece as built, as a reader who didn't build it. Read the piece and its
design before the spec, and judge each place the builds departed from the spec against the
design. Read the code in the diff (`git diff origin/main...HEAD`), not only the tests. Open
the evidence a build names (screenshots, sweeps) before judging the page or the numbers, and
play or measure what the spec says proves the piece.

Merge `origin/main` first if it has moved, so the PR is what would land.

Fix what is small and substantive yourself, and leave nits. A change to a rule, or a step's
work missing, goes in your verdict for the owner, not in a quiet fix. So does a band, a test floor
or a fail check that encodes a rule: name it in the verdict as a rule.

## The judge

Judge the change as the judge's verdict below says, from `sansfiat sweep --judge --json` on the
branch and on `main` and `sansfiat judge --compare` on the two, and read the play note
(`relay/specs/<piece>/play.md`) when there is one. Where an earlier step saved those sweeps under
`relay/specs/<piece>/` and the engine hasn't changed since (`git log` says), read its files: run
a sweep only for a commit nobody swept. The same holds for the buttons script and the tests: CI
runs them on your push. Your own checking goes where the builds' couldn't: the diff read cold
against the goal, an earlier piece's question of this milestone that this one could have moved,
a seed or a case no step ran.

## The design ledger

The `## Decided` ledger at the foot of a design file is its decision record: the few choices
that shape the game and that a later piece must not undo by accident. Add a line only for a
decision of that weight: a rule of the game chosen over another the design allowed, in the
game's words, with the reason in a clause, ending with the piece's number, such as `(#54)`.
Most pieces add one line or none. How a rule is built, a number, what the AI does this
milestone and anything a later piece is free to change are the code as built, not ledger
lines. Never edit the prose above the ledger.

## The PR description

The owner reads it once, on a phone. Rewrite it (`gh pr edit --body`) so it's enough on its
own: the judge's block (the verdict and a line per score), then the steps table, then the answer to
the piece's question in the scores with its control, the ledger lines you added, the play
note's findings if there is one, the screenshots a page piece took, linked at the commit and not
the branch, the play link
`https://dontpanic345.github.io/sansfiat/b/<branch>/`, and last your verdict: merge, or don't
and why.

## Finishing

CI runs on every push to the PR. After your last push, wait for the CI run of that commit
(`gh run list --commit "$(git rev-parse HEAD)"`, then `gh run watch` on it), not the checks
of an earlier one. Finish `done` when it's green, with `"merge": true` in `result.json` if
your verdict is merge: the crew then merges the PR, and pieces waiting on this one start from
it. With `"merge": false`, the owner decides. The crew rewrites the steps table with your row.
