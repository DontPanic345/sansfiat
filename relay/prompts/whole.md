# Whole step

This piece is yours whole: one session takes it from its plan to its merge verdict, and yours is
the only session it gets. Plan it, build it, play it and judge it the way you think best. The
piece's `Spec`, `Spec-review`, `Play` and `Shape` lines were written for a run of smaller
sessions; take from them what helps.

## The window

The session has a 5-hour usage window that was close to empty when you started.
`node relay/usage.mjs` prints how much of the window is used. The window may run out. 

- Keep `relay/specs/<piece>/progress.md` current: the plan, what is done.
- Commit and push regularly, a second attempt has only the branch.

## Done

The piece is done when every line here holds:

- Every clause of the Goal and every Decided line is in the game, each shown by a test or a
  number.
- The Goal's question is answered in the scores: `sansfiat sweep --judge` on the branch and on
  `main`, the old and new numbers beside the control, in a file under `relay/specs/<piece>/`.
- The games the Goal asks of a player are played through `sansfiat play` by a player who knows
  only the text view, and scored with `sansfiat judge`.
- `engine/check.sh` passes on your last commit, and CI is green on it.
- Each decision a player would notice is one line in the `## Decided` ledger at the foot of its
  design file, in the game's words, ending `(#<piece>)`.
- The PR is open against `main`, titled as the piece, its body starting `Closes #<piece>.` and
  written for the owner to read once on a phone: the answer in the scores with its control, the
  ledger lines, what the played games showed, and your verdict last. The crew adds the steps
  table under the Closes line.
- `result.json` has `"merge": true` or `false`. On `true` the crew merges the PR unread, so the
  verdict is the one a cold reviewer gives: the diff against `main`, read by someone who didn't
  write it.
