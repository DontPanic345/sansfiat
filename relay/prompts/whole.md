# Whole step

This piece is yours whole: one session takes it from its plan to its merge verdict, and yours is
the only session it gets. Plan it, build it, play it and judge it the way you think best. The
piece's `Spec`, `Spec-review`, `Play` and `Shape` lines were written for a run of smaller
sessions; take from them what helps.

The cap this prompt starts with is sized for the whole piece, so spend it on the whole piece.
Keep your plan and your numbers in `relay/specs/<piece>/` and push as each part works: your
context compacts over a session this long, and the files are what you reread.

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
