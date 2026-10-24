# Review step

Review the whole piece as built, as a reader who didn't build it, against the piece, its spec
and its design files. Read the code in the diff (`git diff origin/main...HEAD`), not only the
tests. Your brief says where to look hardest. Fix what matters yourself, each fix with a test
that fails without it, and leave nits.

The PR is ready now, so CI runs on every push. Finish `done` when it's green
(`gh pr checks --watch`).
