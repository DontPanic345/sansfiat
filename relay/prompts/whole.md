# Whole step

This piece is yours from its plan to its merge verdict, in one session. No spec, spec-review,
build, play or review step runs before or after you, so nothing is checked unless you check it.

How you do it is your choice. The steps other pieces run are in `relay/prompts/` (`spec.md`,
`spec-review.md`, `build.md`, `pick.md`, `play.md`, `review.md`), the player and the judge are in
`relay/players/`, and what those steps left for earlier pieces is in `relay/specs/`. Follow their
shapes, part of them, or anything else: subagents, your own order, your own proofs. The piece's
`Spec`, `Spec-review`, `Play` and `Shape` lines say what a run of steps would have used; read
them as suggestions.

## What the piece is owed

- Its Goal's question answered in the scores, on the retro maps, beside its control, with the old
  and new numbers (`review.md`, The judge).
- The games its Goal or its `Play` line asks for, played as `play.md` and `players/player.md` say,
  by a player who hasn't read the code.
- Evidence in files under `relay/specs/<piece>/`.
- Every check passing after your last edit.
- Its decisions in each design file's `## Decided` ledger (`review.md`, The design ledger).
- A PR. Open it once the branch has commits (`gh pr create --base main`, titled as the piece,
  its body starting `Closes #<piece>.`), and before you finish rewrite its description as
  `review.md` says. The crew puts the steps table under the Closes line.
- CI green on your last commit (`review.md`, Finishing).
- A merge verdict, `"merge": true` or `false` in `result.json`. On `true` the crew merges and
  later pieces build on it, with no one reading in between. Give the verdict as a reader who
  didn't build the piece would.

## The clock and the window

The session has the cap this prompt starts with, and one 5-hour usage window that was close to
empty when you started. `node relay/usage.mjs` prints how much of the window is used. The window
may run out before the clock does; the session then ends where it stands, and a second attempt,
the last, starts cold from the branch once the window resets.

- Keep `relay/specs/<piece>/progress.md` current: the plan, what is done and proven, what is
  next, the numbers so far. Your context compacts over a session this long, and a second attempt
  has only the branch.
- Commit and push each part as it works.
- Leave the last of the clock and the window for the proof, the PR and CI.
- A smaller piece that is proven and says what it left out is worth more than the whole piece
  unproven. What it left out goes under `ideas`.
