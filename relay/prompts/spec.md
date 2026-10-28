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

## Resolve what you can

Ambiguity is cheap to settle here and costly in a build, so settle all you can before you
cut any steps. It comes in two kinds:

- **What we want**: the rules, the intent, how it should feel to play. Reading the design and
  the code settles it; where the design doesn't say, ask. Write each answer as a decision.
- **What works**: a threshold, a share, a starting number, or the method for something
  judged by measuring or by eye. Only running settles it, so don't guess: name it, say what
  proves it, and give it to a build. Its brief says what it delivers, the proof, and what
  other steps rely on from it, and leaves the method and the starting numbers to the build.

What you leave unsettled is what a step's **Open** score measures. Every point you settle
here makes the piece cheaper.

## Choosing a shape

The shape is how you arrange what's left unsettled. Cut steps by area, not by time: a
session has room for far more than one module. M1's longest ran 23 minutes on 19% of its
context, and every new session rereads the code.

### Henry Ford: the production line

- **When:** everything is settled, or what's left is in features that matter little, and the
  piece splits along seams you can name.
- **Cut:** steps by area, in order, each with an exact done.
- **Expect:** Haiku and Sonnet.

### Easel and Canvas

- **When:** important work couldn't be settled, such as tuning, feel, or anything the player
  meets directly, and it needs groundwork under it.
- **Cut:** easel steps first, all settled, built for what the painter needs. Then one painter
  step holding all of the important unsettled work. Its brief says what the easel provides
  and what proves the painting, and that it may change the easel.
- **Expect:** a cheap easel and an Opus painter.

### Clay and Kiln

- **When:** the hard, important part is the shape at the centre: a model, a core algorithm,
  or an interface everything hangs off. Once that shape exists, the work after it is settled.
- **Cut:** one clay step first. It shapes the core and leaves what the kiln steps rely on:
  interfaces, invariants and tests. Then the kiln steps, all settled, each building on the
  clay. A kiln step doesn't reshape the clay. If the clay doesn't hold, it makes the smallest
  fix that works and records it as a decision, so the review sees it.
- **Expect:** Opus clay and a cheap kiln.

To choose between Canvas and Kiln, ask where the unsettled work sits. On top, judged at the
end on what the others built: Easel and Canvas. Underneath, a shape the others build on:
Clay and Kiln.

### Magnum

- **When:** the design and the build can't be pulled apart. The piece is one tangled thing,
  and splitting it would cost more seams than it saves.
- **Cut:** one build step.
- **Expect:** Opus. A Magnum piece is exempt from the milestone's Haiku challenge.

### Freestyle

- **When:** none of the shapes above fit. Say in one sentence why, then arrange the steps
  however the piece needs.

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

Take what the ladder gives; don't choose a model from the shape. The shape says what to
expect, and when the scores disagree, the cut is usually wrong:

- A painter or clay step scoring under 9 isn't holding the hard part. Either the piece is
  really Ford, or the hard part is in another step.
- A Ford, easel or kiln step scoring 12 or more holds unsettled work that should be settled
  or moved.

Re-cut or re-score, and write which in the step's `why`.

`spec.md` names the shape and in one sentence why, and says what you settled, what you left
unsettled and which step holds it. Each step in `steps.json` carries the `shape`. Its `why`
gives its scores, what you expect of the model, and any mismatch with the shape and what you
did about it, so the retro can check. `estimate` is its list $ from `relay/rates.md`.

```json
[
  { "id": "build-1", "title": "…", "shape": "canvas", "model": "…", "effort": "…", "points": 7, "estimate": 1.2, "why": "Open 2, New 1, Miss 1, Importance 1, Reach 2: …" },
  { "id": "review", "title": "Review the piece", "shape": "canvas", "model": "…", "effort": "…", "points": 9, "coin": true, "estimate": 1.5, "why": "…" }
]
```

Models are `haiku`, `sonnet` and `opus`; efforts are `low`, `medium` and `high`.

A piece that asks a question answers it and no more, and its review writes the answer and
the evidence into the PR description.
