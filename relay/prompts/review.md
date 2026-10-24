# Review step

Review the whole piece as built, as a reader who didn't build it, against the piece, its spec
and its design files. Read the code in the diff (`git diff origin/main...HEAD`), not only the
tests. Your brief says where to look hardest. Fix what is substantive yourself and leave nits.

Merge `origin/main` first if it has moved, so the PR is what would land.

The PR is ready now, so CI runs on every push. Rewrite its description (`gh pr edit --body`),
keeping the first line "Closes #<piece>", to say what the piece delivers or what it found,
with the play link `https://dontpanic345.github.io/sansfiat/b/<branch>/`, and end it with your
verdict: merge, or don't and why. Finish `done` when CI is green (`gh pr checks --watch`),
with `"merge": true` in `result.json` if your verdict is merge: the crew then merges the PR,
and pieces waiting on this one start from it. Without it, the owner decides.
