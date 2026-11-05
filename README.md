# Sans Fiat

A hex 4X with no money, set among the first civilisations. It's built by a pipeline of
unattended Claude Code sessions, called Relay, that turns a written design into merged,
playable pull requests.

[Play the game](https://dontpanic345.github.io/sansfiat/engine/) ·
[Open pull requests](https://dontpanic345.github.io/sansfiat/), each playable

## The game

Most strategy games run on a gold coin in the corner of the screen. Sans Fiat takes it away.
Goods are physical and worth only what people will trade for them, and every unit and action
costs real food and real people's time, taken from somewhere on the map.

The world runs without you. Clans forage, eat, grow, move and trade on their own. You
organise the biggest clan at the market, and you get a few touch points into that world: an
army, your land, and the supply lines that feed both. How to get anything done without a
treasury is the problem you spend the game solving.

It's a Rust engine compiled to WebAssembly, with a CLI that plays, sweeps and judges games
headless. Every run is deterministic: a build that shouldn't change the game must give the
same digest it gave before.

## Relay

I have a few hours a week. Relay spends them on taste and decisions, and spends the rest of
my Claude subscription on everything else.

A piece of the game starts as a proposal I approve. It becomes a GitHub issue, and from
there Relay runs on its own in GitHub Actions:

```
proposal ─▶ issue ─▶ spec ─▶ spec-review ─▶ build-1 … build-N ─▶ review ─▶ merge
   me                ╰────────────── cold Claude Code sessions ──────────╯    me / crew
```

Each step is a fresh `claude -p` session with no memory of the last. The crew
([`relay/relay.mjs`](relay/relay.mjs)) picks the next step, writes the session's whole
prompt, chooses its model and effort, and records how it went. The session hands on what
it did as a summary and its decisions, which every later step reads. The baton is the
branch, the spec, and those notes.

- **The designer plans its own work.** The spec step settles open rules, splits the piece
  into builds, and picks a model, effort and expected cost for each. Spec-review then reads
  the code and runs the simulation to check the plan before anything is built.
- **Every piece answers a question in numbers.** The engine judges itself: a headless sweep
  plays the same maps with and without the change and scores what moved, such as how much
  of the world each civ holds, whether the lead changes hands, and whether anyone starves.
- **The review merges on its own verdict.** It reviews the whole piece against the design,
  fixes what's small, waits for CI, and writes a PR description I can judge from a phone:
  the steps, the answer in the scores, screenshots and a verdict. Every PR deploys a
  playable preview.
- **The design keeps up with the code.** Each review appends what the piece decided to a
  ledger in the design file. I promote or strike each line when I next plan.
- **GitHub is the state.** Pieces are issues, steps are sub-issues, and each step's
  attempts, cost, minutes and usage live in a hidden block in its body. A project board
  shows where everything stands.
- **It paces itself to the subscription.** A step starts only if the 5-hour usage window
  will stay under 80% after it. If not, Relay sleeps until the window resets and carries on.
- **Sessions can say no.** A step ends `done`, `pending` with a note for the next attempt,
  or `blocked` with a question for me. Two failed attempts block it. My reply on the issue
  unblocks it.
- **Driven from the issue.** `/relay run`, `retry`, `skip`, `pause` and `model` let me steer
  a piece from a comment.
- **Sessions never see a long-lived secret.** The crew mints short-lived GitHub App tokens
  that git and `gh` read from a file, and redacts transcripts before saving them.
- **Every milestone ends in a retro.** Planning sessions read the milestone back from
  several angles (cost, briefs, prompts, the game) and the next proposal starts from it.
  M2 ran 35 steps across six pieces, with no crash, timeout or compaction, at about $8 a
  piece at list price.

[`relay/README.md`](relay/README.md) has the detail, and
[`docs/process.md`](docs/process.md) the loop around it.

## The repo

| Where | What |
| --- | --- |
| [`engine/`](engine/) | The game: a Rust engine, a CLI and a wasm page. |
| [`docs/design/`](docs/design/) | What the game should be, starting with [`vision.md`](docs/design/vision.md). |
| [`docs/roadmap.md`](docs/roadmap.md) | The milestones to a finished game. |
| [`docs/proposals/`](docs/proposals/) | Each milestone's pieces, and the retro before it. |
| [`relay/`](relay/) | The crew, the prompts each step gets, and each piece's spec. |
