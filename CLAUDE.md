# Sans Fiat

A hex 4X with no money, set among the first civilisations. `docs/design/vision.md` says what
it should be, and everything else serves that.

| Where | What |
|---|---|
| `docs/design/` | What the game should be: the vision and one file per system. The owner's. |
| `docs/process.md` | How the game gets built, and the scores a piece is judged on. |
| `docs/roadmap.md` | The milestones to a finished game. |
| `docs/architecture.md` | The shape the code is growing into, and what each system needs first. |
| `docs/proposals/` | Each milestone's pieces, and the retro before it. |
| `engine/` | The game: a Rust engine, a CLI and a wasm page. `engine/CLAUDE.md` is its map. |
| `relay/` | Relay builds pieces in cold sessions. `relay/README.md` is how it runs; `relay/prompts/` is what each step is told. |

## Whoever you are

- The design is the owner's. Build what `docs/design/` says. Where it's silent, decide the
  way it points and record the decision; the review adds it to the design file's Decided
  ledger, and the owner promotes it into the prose or strikes it.
- A piece answers one question, in the scores that matter (`docs/process.md`, Scores). Count
  what moves the game, not what shows the code ran.
- Prove on the maps the game is played on. A built test map shows a rule works; the retro
  maps show the piece does.
- Every number lives in `engine/core/src/data.rs`. A rule's reason is one `note` beside it.
- Evidence goes in a file under `relay/specs/<n>/`, which the summary names. Never commit
  `relay/runs/`.
- Write plainly, in the present tense. A comment says what is, not how it came to be: the
  commit carries the history.

## Checks

`engine/check.sh` runs everything CI runs on the engine.

The judge: `check.sh` runs `sansfiat sweep --judge`, which plays the seven retro maps and checks
its totals against the bands in `engine/judge.toml` (health fails, the four scores warn). A piece
that moves a band says so in its PR, with the old and new numbers, and moves the band with it.

Play: `sansfiat play --game FILE` plays a game a line at a time through the text view; the
players' and the judge's prompts are in `relay/players/`, and `sansfiat judge FILE` scores a
played game beside the same map on the AI.
