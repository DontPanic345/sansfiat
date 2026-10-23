# Spec step

Write the piece's spec, a brief for each of its steps and its `steps.json`, from the piece,
its design file and the code as it is now. Put them in `relay/specs/<piece>/`.

## The scope

The piece's **In scope** list is the whole job, and its **Out of scope** list is fenced
off. Every step must serve an in-scope line. Work the piece needs that isn't in scope is a
design question, not a step.

## `spec.md`

For Alan's sign-off and the piece's reviewer:

- What the piece delivers, and which parts of the design it builds.
- `## Scope map`: a table of each step against the in-scope line it serves. Every in-scope
  line appears at least once.
- `## Design questions`: each rule or number the piece needs that the design doesn't
  give, with two or three options and your recommendation. Write "None" when there are
  none. Alan answers these in `docs/design/` before building starts.
- `## Design`: the types, functions, files touched and tests, enough for the review.

## The briefs: `build-1.md` … and `review.md`

One for each build step, each small enough for one session, and one for the review. A
build session works from its brief and the code alone, so each brief is self-contained:

- It starts `# <step>: <title>`, says what to build, and lists the tests that prove it.
- It carries all the design it needs, copied in from the spec and the design file: names,
  signatures, rules, numbers. Briefs repeat design wherever two steps need the same part.
- It says what's out of scope for the step.

The review brief names what this piece's review should look at hardest.

## `steps.json`

The build steps then the review, at most four build steps:

```json
[
  { "id": "build-1", "title": "…", "model": "sonnet", "effort": "medium", "why": "…" },
  { "id": "review", "title": "Review the piece", "model": "sonnet", "effort": "high", "why": "…" }
]
```

If the piece needs more than four build steps, stop at four and propose splitting it under
Design questions.

Size each step's model and effort to its work, with the budget in mind, and give the
one-line reason in `why`:

- **Opus**: rules that are subtle or interact with other mechanics, changes across
  the engine, or a large step. Also the review of a merge.
- **Sonnet**: a contained step, or a proof's or polish's review.
- **Haiku**: mechanical work with an exact check.

Efforts are `low`, `medium` or `high`. Alan checks every step's model and effort at sign-off.
