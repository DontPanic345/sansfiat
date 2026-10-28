# Spec step

You are the piece's designer. You plan it for the sessions after you: spec-review reads your
plan next, and the crew makes the steps from `steps.json` as spec-review leaves it. Write in
`relay/specs/<piece>/`:

- `spec.md`: what the piece delivers and how, for the builds, the review and the retro. It
  names the shape and why, says what you settled, what you left unsettled and which step
  holds it, and lists the decisions you made that the piece left open. A design line that
  can't or shouldn't be built as written is a decision that starts "Design says …".
- A brief for each build step, `build-1.md` …. A build gets its brief, the code and the
  earlier steps' results, and nothing else, so its brief carries what it needs. A brief names
  a file under `relay/specs/<piece>/` for any tables, maps and sweeps it produces. The review
  reads the piece and the design itself, so it has no brief.
- `steps.json`: the build steps, then the review.

## Settle what you can

Ambiguity is cheap to settle here and costly in a build, so settle all you can before you
cut any steps. It comes in two kinds:

- **What we want**: the rules, the intent, how it should feel to play. Reading the design and
  the code settles it; where they're silent, decide. Write each as a decision the owner can
  overrule on the PR. Finish `blocked` only for a choice that would change the piece's goal.
- **What works**: a threshold, a share, a starting number, or the method for something
  judged by measuring or by eye. Only running settles it. Name it, say what proves it, and
  give it to a build. Its brief says what it delivers, the proof, and what other steps rely
  on from it, and leaves the method and the starting numbers to the build.

What you leave unsettled is what a step's **Open** score measures.

## Choose a shape

The shape is how you arrange what's left unsettled. The piece's `Shape:` line suggests one:
take it, or change it and say why in `spec.md`. Cut steps by area: a session has room for far
more than one module, and every new session rereads the code.

Each build step plays a **part** in its shape.

### Henry Ford: the production line

- **When:** everything is settled, or what's left is in features that matter little, and the
  piece splits along seams you can name.
- **Parts:** `ford` steps by area, in order, each with an exact done.
- **Expect:** Haiku and Sonnet.

### Easel and Canvas

- **When:** important work couldn't be settled, such as tuning, feel, or anything the player
  meets directly, and it needs groundwork under it.
- **Parts:** `easel` steps first, all settled, built for what the painter needs. Then one
  `canvas` step holding all the important unsettled work. Its brief says what the easel
  provides, what proves the painting, and that it may change the easel.
- **Expect:** a cheap easel and an Opus painter.

### Clay and Kiln

- **When:** the hard, important part is the shape at the centre: a model, a core algorithm,
  or an interface everything hangs off. Once it exists, the work after it is settled.
- **Parts:** one `clay` step first. It shapes the core and leaves what the kiln relies on:
  interfaces, invariants and tests. Then `kiln` steps, all settled, each building on the
  clay. Each kiln brief says to keep the clay's shape, and where the clay doesn't hold, to
  make the smallest fix that works and record it as a decision.
- **Expect:** Opus clay and a cheap kiln.

Where does the unsettled work sit? On top, judged at the end on what the others built:
Easel and Canvas. Underneath, a shape the others build on: Clay and Kiln.

### Magnum

- **When:** the design and the build can't be pulled apart. The piece is one tangled thing,
  and splitting it would cost more seams than it saves.
- **Parts:** one `magnum` step.
- **Expect:** Opus.

## Choose each step's model and effort

Score the step on five things:

| | Low | Middle | High |
|---|---|---|---|
| **Open**: can the brief say what done is? | 0: exactly | 2: in part | 4: judged by eye, or by searching for what works |
| **New**: is there a pattern for it? | 0: in the code, or a port | 1: adapted from one | 3: nothing like it here |
| **Miss**: what catches a mistake? | 0: the checks | 1: a test it has to write | 3: only play |
| **Importance**: what rests on it? | 0: it'll be rewritten | 1: this milestone builds on it | 3: a foundation, costly to change later |
| **Reach**: what does it change? | 0: a module on its own | 1: a few callers | 2: what every turn runs |

The game is early and much of it will be rewritten, so Importance is mostly 0 or 1.

The points add up, and the sum climbs the ladder:

| Points | 0–2 | 3–4 | 5–6 | 7–8 | 9–11 | 12–15 |
|---|---|---|---|---|---|---|
| Step | haiku low | haiku medium | haiku high | sonnet medium | sonnet high; opus low for `canvas` and `clay`; the review by coin | opus medium |

- Haiku takes the lower half, judgement work included: it's more capable than its training
  data shows.
- A review at 9–11 takes a coin
  (`node -e "console.log(Math.random() < 0.5 ? 'sonnet high' : 'opus low')"`): take what it
  says, and write "by coin" in its `why`.
- A `magnum` step runs on Opus: low under 12 points, medium from 12.
- Opus medium is the top. You can go against the ladder, up to Opus medium, and say why.

The ladder picks the model; the shape says what to expect. When the scores and the part
disagree, the cut is usually wrong:

- A `canvas` or `clay` step under 9 isn't holding the hard part. Either the piece is Henry
  Ford, or the hard part is in another step.
- A `ford`, `easel` or `kiln` step at 12 or more holds unsettled work to settle or move.

Re-cut or re-score, and write which in the step's `why`.

## M2's challenge

- Each piece runs at least one step on Haiku, except a Magnum piece.
- Each piece runs at least one step off the ladder, and its `why` says what it expects to
  happen.

## steps.json

Each build step carries its `part`. Its `why` gives its scores, what you expect of the model,
and any mismatch with the part and what you did about it, so the retro can check.

```json
[
  { "id": "build-1", "title": "…", "part": "easel", "model": "…", "effort": "…", "points": 7, "why": "Open 2, New 1, Miss 1, Importance 1, Reach 2: …" },
  { "id": "review", "title": "Review the piece", "model": "…", "effort": "…", "points": 9, "why": "…" }
]
```

Models are `haiku`, `sonnet` and `opus`; efforts are `low`, `medium` and `high`.

A piece that asks a question answers it and no more, and its review writes the answer and
the evidence into the PR description.
