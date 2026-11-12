# Whole step

This piece is yours from its plan: you design it and build it, and you choose how it ends.
Plan it, build it, play it and judge it the way you think best. The piece's `Spec`,
`Spec-review`, `Play` and `Shape` lines were written for a run of smaller sessions; take from
them what helps.

## The window

You started on a 5-hour usage window that was nearly unused, and it is the budget: on Fable it
lasts about ninety minutes of steady work, not five hours. The prompt gives both windows as you
start and a hook repeats them every fifteen minutes.

- Spend the window on what only you can do: the design, the core, the rules that have to be
  right. Hand on what a cheaper session can do from a brief (below).
- Keep `relay/specs/<piece>/progress.md` current: the plan, and what is done.
- Commit and push before every long run and at least every twenty minutes. When the window runs
  out the session ends where it stands, and the next attempt has only the branch.

## Handing on

You may finish without a verdict and hand the rest to sessions after you. Write
`relay/specs/<piece>/steps.json` as `spec.md` (the prompt file) describes it: any `build-N`
steps, each with a brief `build-N.md` that carries all its build needs, then `review`. The crew
makes those steps, and adds a play step before the review when the piece's `Play:` line names a
model. A review alone is a fair hand-on: a cold reader on Sonnet, and a Sonnet player, judge
what you built on a different axis than you can.

Hand on what is settled. What is still open when you stop is yours to finish, or to say plainly
in `progress.md` and your summary.

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
- The PR is open against `main`, titled as the piece, its body starting `Closes #<piece>.` and
  written for the owner to read once on a phone: the answer in the scores with its control, the
  ledger lines, what the played games showed, and your verdict last. The crew adds the steps
  table.

Finish with `"merge": true` or `false` in `result.json` when you end the piece yourself. On
`true` the crew merges the PR unread, so the verdict is the one a cold reviewer gives: the diff
against `main`, read by someone who didn't write it. Finish without `merge` when `steps.json`
hands on, and the review gives the verdict.
