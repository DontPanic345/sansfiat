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

Each step is a `claude -p` session with a fresh context. It hands on a summary and its
decisions, which every later step reads.

- **The designer plans the work.** The spec step settles open rules, splits the piece into
  builds, and writes a brief for each. Spec-review reads the code and runs the simulation
  to check the plan before anything is built.
- **Prompts are pre-built.** The crew ([`relay/relay.mjs`](relay/relay.mjs)) writes each
  session's whole prompt and runs it on the model and effort the plan chose, from Fable
  down to Haiku.
- **GitHub is the state.** Pieces are issues, steps are sub-issues, and each step's
  attempts, cost, minutes and usage live in a hidden block in its body.
- **It paces itself to the subscription.** A step starts only if the 5-hour usage window
  will stay under 80% after it. If not, Relay sleeps until the window resets and carries on.
- **Every milestone ends in a retro.** Planning sessions read the milestone back from
  several angles (cost, briefs, prompts, the game) and the next proposal starts from it.
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
