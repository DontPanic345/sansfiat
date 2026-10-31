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

## What proves the piece

The piece's question is asked in the scores (`docs/process.md`, Scores), with a control.
`spec.md` says how each is measured and what the control is. A proof is a run on the retro
maps against that control, read in the scores; a test on a built map shows a rule works, not
that the piece did. Every unsettled item's proof has a real-map run, and so does any step
whose rule the player meets.

What the review must play or measure goes here too: the review reads the spec, not
`steps.json`.

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

Running settles some of it now. A sweep or play script that takes minutes is cheaper here
than in a build: run it, and settle the number or drop the fallback.

What you leave unsettled is what a step's **Open** score measures.

## Choose a shape

The shape is how you arrange what's left unsettled. The piece's `Shape:` line suggests one:
take it, or change it and say why in `spec.md`.

Cut steps by area, not by size. A build may hold several areas: a session has room for far
more than one module, and every new session rereads the code. A brief names the module that
should grow, not only the code to reuse, so no file swells to hold another's work.

Each build step plays a **part** in its shape.

### Henry Ford: the production line

- **When:** everything is settled, or what's left is in features that matter little, and the
  piece splits along seams you can name. A port is settled in its rules, not its numbers: the
  prototype's ran in another economy. Prove them on real maps, or give them a canvas.
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

### Clay, Kiln and Glaze

- **When:** Clay and Kiln, but the kiln's rules are settled while their numbers only work
  together.
- **Parts:** as Clay and Kiln, ending with one `glaze` step that tunes the kiln's numbers on
  real maps and may change the kiln. Give the kiln starting numbers that run.
- **Expect:** Opus clay and glaze, and a cheap kiln.

Where does the unsettled work sit? On top, judged at the end on what the others built:
Easel and Canvas. Underneath, a shape the others build on: Clay and Kiln.

### Magnum

- **When:** the design and the build can't be pulled apart. The piece is one tangled thing,
  and splitting it would cost more seams than it saves.
- **Parts:** one `magnum` step.
- **Expect:** Opus.

## Choose each step's model and effort

Score each build step on four things:

| | Low | Middle | High |
|---|---|---|---|
| **Open**: can the brief say what done is? | 0: exactly | 2: in part | 4: judged by eye, or by searching for what works |
| **New**: is there a pattern for it? | 0: in the code, or a port | 1: adapted from one | 3: nothing like it here |
| **Miss**: what catches a mistake? | 0: the checks | 1: a test it has to write | 3: only play, or a run on real maps |
| **Reach**: what does it change? | 0: a module on its own | 1: a few callers | 2: what every turn runs |

The points add up, and the sum climbs the ladder:

| Points | 0–1 | 2–3 | 4–5 | 6–7 | 8–10 | 11–12 |
|---|---|---|---|---|---|---|
| Step | haiku low | haiku medium | haiku high | sonnet medium | sonnet high; opus low for `canvas`, `glaze` and `clay` | opus medium |

- Haiku takes the lower half, judgement work included: it's more capable than its training
  data shows.
- A `magnum` step runs on Opus: low under 11 points, medium from 11.
- How many files a step touches costs minutes, not a bigger model. Don't go off the ladder
  for it.
- Opus medium is the top.

The ladder picks the model; the shape says what to expect. When the scores and the part
disagree, the cut is usually wrong:

- A `canvas`, `glaze` or `clay` step under 8 isn't holding the hard part. Either the piece is
  Henry Ford, or the hard part is in another step.
- A `ford`, `easel` or `kiln` step at 11 or more holds unsettled work to settle or move.

Re-cut or re-score, and write which in the step's `why`.

Each piece runs one build step a rung below the ladder, and its `why` says what it expects to
see.

The review isn't scored. It runs Sonnet high, or Opus low when `main` has moved under the
piece or the goal is judged by play. Nothing that judges runs at low effort.

## steps.json

Each build step carries its `part`. Its `why` gives its scores, what you expect of the model,
any mismatch with the part and what you did about it, and its size (files, rough lines), so
the retro can read cost against size.

```json
[
  { "id": "build-1", "title": "…", "part": "easel", "model": "…", "effort": "…", "points": 7, "why": "Open 2, New 1, Miss 1, Reach 2; about 4 files, 300 lines: …" },
  { "id": "review", "title": "Review the piece", "model": "sonnet", "effort": "high", "why": "…" }
]
```

Models are `haiku`, `sonnet` and `opus`; efforts are `low`, `medium` and `high`.

A piece that asks a question answers it and no more, and its review writes the answer and
the evidence into the PR description.
