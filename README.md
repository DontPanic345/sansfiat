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

The game is just a prototype at the moment, the goal is to explore some ideas like an economy, trade routes and supply lines. With hopefully a different set of abstractions than a typical game in the genre.

It's a Rust engine compiled to WebAssembly, with a CLI that plays, sweeps and judges games
headless. Every run is deterministic: a build that shouldn't change the game must give the
same digest it gave before.

## Relay

I have a few hours a week. I designed this system to squeeze the most out of every token and hour spent.

A piece of the game starts as a proposal that Fable and I write in a planning session. Once
I approve it, it becomes a GitHub issue, and from there Relay runs on its own in GitHub
Actions:

```
proposal ─▶ issue ─▶ spec ─▶ spec-review ─▶ build-1 … build-N ─▶ review ─▶ merge
   me                ╰────────────── cold Claude Code sessions ──────────╯    me / crew
```

Each step is a `claude -p` session with a fresh context. It hands on a summary and its
decisions, which every later step reads.

- **The designer plans the work.** The spec step settles open rules, splits the piece into
  builds, and writes a brief for each. Spec-review reads the code and runs the simulation
  to check the plan before anything is built.
- **Prompts are pre-built.** The crew ([`relay/relay.mjs`](relay/relay.mjs)) writes each
  session's whole prompt and runs it on the model and effort the plan chose: Opus for
  the hard thinking, Sonnet or Haiku where the brief leaves little to decide.
- **GitHub is the state.** Pieces are issues, steps are sub-issues, and each step's
  attempts, cost, minutes and usage live in a hidden block in its body.
- **It paces itself to the subscription.** A step starts only while the 5-hour usage window
  is under 75%. If not, Relay sleeps until the window resets and carries on.
- **Every milestone ends in a retro.** Relay reads the milestone back from five angles (the
  ladder, the architecture, Relay itself, the game and its players), waits for the owner's
  notes, and opens the next proposal as a PR.
  M2 ran 35 steps across six pieces at about $8 a piece at list price.

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
