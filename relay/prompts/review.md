# Review step

Review the whole piece as built, as a reader who didn't build it. Read the piece and its
design before the spec, and judge each place the builds departed from the spec against the
design. Read the code in the diff (`git diff origin/main...HEAD`), not only the tests. Fix
what is substantive yourself and leave nits.

Merge `origin/main` first if it has moved, so the PR is what would land.

CI runs on every push to the PR. Rewrite its description (`gh pr edit --body`): the first line
"Closes #<piece>", then the steps table, then what the piece delivers or what it found, with
the play link `https://dontpanic345.github.io/sansfiat/b/<branch>/`, and end it with your
verdict: merge, or don't and why. Finish `done` when CI is green (`gh pr checks --watch`),
with `"merge": true` in `result.json` if your verdict is merge: the crew then merges the PR,
and pieces waiting on this one start from it. Without it, the owner decides.
