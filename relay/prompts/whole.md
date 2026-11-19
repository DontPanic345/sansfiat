# Whole step

This piece is yours from its plan: you design it and build it, and a review after you gives
its verdict. Plan it, build it, play it and judge it the way you think best. The piece's `Spec`,
`Spec-review`, `Play` and `Shape` lines were written for a run of smaller sessions; take from
them what helps.

## The window

You started on a 5-hour usage window that was at most half used, and what is left of it is the
budget: the piece is yours for this one window. The prompt gives both windows as you start and a
hook repeats them every ten minutes, and tells you to commit at 70% and stop at 85%. Helpers
spend the same window.

- Spend the window on what only you can do: the design, the core, the rules that have to be
  right. Hand on what a cheaper session can do from a brief (below).
- Keep `relay/specs/<piece>/progress.md` current: the plan, and what is done.
- Commit and push before every long run and at least every twenty minutes.
- At 70%, plan the hand-on. At 85%, hand on: write a brief for each part still to build, list
  them in `steps.json` before the review, push, and finish `done`. Finish `pending` only when
  the window ends you before you can.

## Handing on

You always finish by handing on. Write `relay/specs/<piece>/steps.json` as `spec.md` (the
prompt file) describes it: any `build-N` steps, each with a brief `build-N.md` that carries all
its build needs, then `review`. The crew makes those steps, and adds a play step before the
review when the piece's `Play:` line names a model. A review alone is the usual hand-on: a cold
reader judges what you built on a different axis than you can, and gives the merge verdict.

Hand on what is settled. What is still open when you stop goes in a build's brief as a question
with what you found, and in your summary.

## Done

You are done when every line here holds, or when it is handed on in `steps.json` to a step
that will make it hold:

- Every clause of the Goal and every Decided line is in the game, each shown by a test or a
  number.
- The files you grew keep `docs/architecture.md`'s rules: you split one you took past about 800
  lines.
- The Goal's question is answered in the scores: `sansfiat sweep --judge` on the branch and on
  `main`, the old and new numbers beside the control, in a file under `relay/specs/<piece>/`.
- The games the Goal asks of a player are played through `sansfiat play` by a player who knows
  only the text view, and scored with `sansfiat judge`.
- CI is green on your last commit.
- Each decision of the weight the design ledger holds (`review.md`, The design ledger) is one
  line in the `## Decided` ledger of its design file.
- The PR is open against `main`, titled as the piece, its body starting `Closes #<piece>.`
  The review writes its description and its verdict.

Finish `done` with `steps.json` ending in `review`, and no `merge` in `result.json`: the verdict
is the review's.
