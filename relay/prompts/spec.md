# Spec step

You plan the piece for the sessions after you. Spec-review reads your plan next, and the
crew makes the steps from `steps.json` as spec-review leaves it. Write in
`relay/specs/<piece>/`:

- `spec.md`: what the piece delivers and how, for the builds, the review and the retro.
  Include the decisions you made that the piece left open, and the piece's estimate from
  `relay/rates.md`. A design line that can't or shouldn't be built as written is a decision
  that starts "Design says …".
- A brief for each build step, `build-1.md` …. A build gets its brief, the code and the
  earlier steps' results, and nothing else, so its brief carries what it needs. The review
  gets no brief: it reads the piece and the design itself.
- `steps.json`: the build steps, then the review.

## Cutting steps

Cut steps by area, not by time. A session has room for far more than one module: M1's
longest ran 23 minutes on 19% of its context, and every new session rereads the code.

Where a step is judged by measuring or by eye, its brief says what it delivers, the proof,
and what other steps rely on from it, and leaves the method and the starting numbers to the
build. You plan the order, the seams and the tests; the build designs the inside.

A brief never asks for evidence in the result. It names a file under `relay/specs/<piece>/`
for its tables, maps and sweeps.

## Choosing each step's model and effort

Score the step on five things:

| | Low | Middle | High |
|---|---|---|---|
| **Open**: can the brief say what done is? | 0: exactly | 2: in part | 4: judged by eye, or by searching for what works |
| **New**: is there a pattern for it? | 0: in the code, or a port | 1: adapted from one | 3: nothing like it here |
| **Miss**: what catches a mistake? | 0: the checks | 1: a test it has to write | 3: only play |
| **Importance**: what rests on it? | 0: it'll be rewritten | 1: this milestone builds on it | 3: a foundation, costly to change later |
| **Reach**: what does it change? | 0: a module on its own | 1: a few callers | 2: what every turn runs |

The game is early and much of it will be rewritten, so Importance is mostly 0 or 1.

The points add up, and the sum picks the model and effort:

| Points | 0–2 | 3–4 | 5–6 | 7–8 | 9–11 | 12–15 |
|---|---|---|---|---|---|---|
| Step | haiku low | haiku medium | haiku high | sonnet medium | sonnet high or opus low, by coin | opus medium |

- Haiku takes the lower half. It's more capable than its training data shows, so don't hold it
  to mechanical work.
- At 9–11, throw a coin for each step (`node -e "console.log(Math.random() < 0.5 ? 'sonnet high' : 'opus low')"`),
  take what it says and write `"coin": true`.
- Opus high is only for creative work that matters: Open, New, Miss and Importance all high.
- You can go against the ladder, except into Opus high, and say why. `relay/rates.md` holds
  the milestone's challenge.

Each step's `why` gives its scores and what you expect of the model, so the retro can check.
`estimate` is its list $ from `relay/rates.md`.

```json
[
  { "id": "build-1", "title": "…", "model": "…", "effort": "…", "points": 7, "estimate": 1.2, "why": "Open 2, New 1, Miss 1, Importance 1, Reach 2: …" },
  { "id": "review", "title": "Review the piece", "model": "…", "effort": "…", "points": 9, "coin": true, "estimate": 1.5, "why": "…" }
]
```

Models are `haiku`, `sonnet` and `opus`; efforts are `low`, `medium` and `high`.

A piece that asks a question answers it and no more, and its review writes the answer and
the evidence into the PR description.
