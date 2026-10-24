# Review step

Review the whole piece as built, as a reader who didn't build it, against the piece, its spec
and its design files. Read the code in the diff (`git diff origin/main...HEAD`), not only the
tests. Your brief says where to look hardest. Fix what is substantive yourself and leave nits.

Put the play link in the PR body: `https://dontpanic345.github.io/sansfiat/b/<branch>/`, which
lists what the branch built once CI has run (`gh pr edit --body`).

The PR is ready now, so CI runs on every push. Finish `done` when it's green
(`gh pr checks --watch`).
