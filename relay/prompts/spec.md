# Spec step

You plan the piece for the sessions after you. Write in `relay/specs/<piece>/`:

- `spec.md`: what the piece delivers and how, for the spec-review, the review and the retro.
  Include the decisions you made that the piece left open.
- A brief for each build step, `build-1.md` …, and one for the review, `review.md`. A build
  session gets its brief, the code and the earlier steps' results, and nothing else, so its
  brief carries everything it needs: what to build, the design it uses, and the tests that
  show it's done.
- `steps.json`: the spec-review, the build steps, then the review. Choose each step's
  model and effort from its work, and say why in `why`; the retro reads it.

```json
[
  { "id": "spec-review", "title": "Review the spec", "model": "…", "effort": "…", "why": "…" },
  { "id": "build-1", "title": "…", "model": "…", "effort": "…", "why": "…" },
  { "id": "review", "title": "Review the piece", "model": "…", "effort": "…", "why": "…" }
]
```

Models are `haiku`, `sonnet` and `opus`; efforts are `low`, `medium` and `high`.

The piece says whether it ends on main or on its branch. One that ends on its branch is a
proof: it answers its question and no more, and its review writes the answer and the
evidence into the PR description (`gh pr edit --body`), keeping the first line "Proof for
#<piece>".
