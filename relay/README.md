# Relay
_You and a team of runners are going a long way to a known finish, further than any of you
could go alone. Pick up the baton from the runner before you, run your leg and hand it on;
each leg starts right where the last one finished. The support crew marks out the course and
keeps the race going between legs._

Relay builds Sans Fiat one piece at a time, in cold `claude -p` sessions, with no one
watching. The crew, `relay/relay.mjs`, runs in GitHub Actions (`relay.yml`). It picks the
next step, checks the budget, builds the session's prompt, starts the session on the
step's model and effort, and records how it went. It's ported from Tortoise Spelling's
Jogging, with GitHub instead of `state.json` holding the state.

## Pieces and steps

A **piece** starts as a proposal Claude brings to the owner. Once the owner is happy with it, Claude
opens it as an issue labelled `kind:piece` and `relay:ready`, and starts a run. Relay reads
these lines from its body, in any order:

```
**Goal:** what the piece delivers, and its question in the scores with its control
**Spec:** opus medium
**Spec-review:** opus medium
**Play:** sonnet
**Shape:** Easel and Canvas: why
**Design:** docs/design/clans.md
**Decided:** …
**In scope:** …
**Out of scope:** …
**After:** #31
```

`Spec` and `Spec-review` are required, each on its own line: a piece without them is held.
`Play` names the play step's model (medium unless it names an effort, and "three seeds" plays
three maps), or `none`.
`Shape` is a suggestion the designer may change. A
field's value carries on over hard-wrapped lines to a blank line or the next field. A piece is
also held until every piece its `After` line names is merged. Every `docs/design/*.md` path in
the body goes to the spec, spec-review and review sessions in full.

Its **steps** are sub-issues. A step's body is its status line, its model, effort, points, part
and why, links to the piece, the PR, `spec.md` and its brief, and the brief as it was when the
step was made; the brief on the branch is the one builds keep up. A hidden
`<!-- relay {...} -->` block carries its attempts, cost, minutes, usage, summary and
decisions. Step issues get no comments: the status line says how the step stands. The spec,
the briefs and `steps.json` live on the piece's branch, `relay/<number>`, in
`relay/specs/<number>/`. The twinned step's two runs each carry their `branch`, and the pick
records what it `chose` and each run how much of it was `picked`.

| Step | What it does |
|---|---|
| `spec` | The designer settles what it can, picks a shape, and writes the spec, a brief per build, and `steps.json` with each build's part and each step's model, effort, points and why (`prompts/spec.md` has the shapes and the ladder). Runs on the model the piece's `Spec` line names |
| `spec-review` | Reads the code each settled rule touches, runs the sweep or a play script where minutes settle a number, and fixes the plan where the run disagrees with the reading. Runs on the model the piece's `Spec-review` line names, never at low effort |
| `build-N` | Builds what its brief says |
| `build-N-twin` | The one step `steps.json` gives a `twin` runs twice from the same commit: `build-N` on its ladder rung and `build-N-twin` on the rung the spec chose, on branches `relay/<number>-build-N-a` and `-b`, a coin deciding which run takes which. Neither sees the other's summary, and the twin is told it is `build-N`. A blocker on either blocks both, and the owner's reply goes to both |
| `build-N-pick` | Runs the brief's proof on both branches, writes `pick.md` with every run side by side, and merges A, B or parts of both into the piece's branch. It sees the versions only as A and B. Its `result.json` must say `"chose": "A"`, `"B"` or `"both"`. The crew then records which run won and how much of each landed, posts both runs' comments on the PR, and files the ideas of each run that landed. Runs Opus medium |
| `play` | Plays the branch through `sansfiat play` for 100 turns from the pick (`relay/players/player.md`), judges the game (`relay/players/judge.md`), and writes `play.md` with each finding classed `fix` or `ask`. The crew adds it before the review when the piece's `Play` line names a model |
| `review` | Merges `main` in, judges the change against `main` with the rubric in `relay/players/judge.md`, reviews the whole piece against the piece and its design, fixes what is small, appends the piece's decisions to each design file's `## Decided` ledger, writes the PR description for the owner to read on a phone (the judge's verdict, the steps table, the answer in the scores, the ledger lines, screenshots and a merge verdict), and waits for CI on its own head commit. Its `result.json` must say `"merge": true` or `false`. The crew then rewrites the steps table with the review's row, and on a merge verdict waits up to ten minutes for CI to go green and merges the PR |

The crew makes spec-review once the spec closes, and the builds and the review once
spec-review closes, from `steps.json` as spec-review left it. Spec-review isn't done until
`steps.json` reads cleanly. Then the crew carries on, with no stop for the owner. Every step's
prompt carries every earlier step's summary and decisions, with the pick's in place of the
twinned step's two runs.

Otherwise it's the owner's turn: merge the PR, which closes the piece, or close the piece,
which tags the branch head `proof/<number>` and closes the PR. Closing a piece closes its open
steps. The owner plays the game on main or on a PR's preview, and what plays badly becomes a
new piece.

The crew works on the oldest piece that has a step to run, so a later piece goes ahead only
while an earlier one waits on the owner, and never ahead of a piece its `After` line names.

## How a step counts

Steps mark their own homework. A session commits and pushes to the piece's branch and
writes `relay/runs/result.json`:

- `done`, with a `summary` and any `decisions`: the crew closes the step and posts both on
  the PR, with what the step took. The PR is made as soon as the branch has commits, and the
  crew links it and the spec folder on the piece.
- `pending`, with a `note`: the next attempt picks up from the note. Two unfinished attempts
  (a crash or timeout counts) block the step.
- `blocked`, with a `blocker`: the step can't go on as it stands.

A blocked step labels the piece `waiting`. The owner's reply on the piece unblocks it, goes to the
next session, and starts a run.

## Fix or ask

Anything new a session notices goes under `ideas`, never into code, and the play step's problems go
under `findings`. Each has a class. A `fix` names the line it breaks (a design line or its ledger,
a piece's Goal or Decided line, a touch point, or health), has a repro, restores that line without
editing design prose, and fits one Ford build with nothing new. Everything else is an `ask`.

The crew lists both in the step's PR comment, then files them:

- A `fix` goes on its milestone's **fixes piece**, "Fixes for M<n>", which the crew opens the first
  time and keeps (`relay:fixes`). It runs after the milestone's other pieces, on Sonnet medium,
  each repro becoming a test. A `fix` that comes after the fixes piece has started is an `ask`.
- An `ask` becomes an issue labelled `kind:piece` and `relay:proposed`, which never runs until the
  owner's `/relay ready` or a merged proposal.

A title already open is skipped, so none is filed twice.

## The milestone and its retro

A milestone is built when none of its pieces can run: each merged, closed or paused, the fixes
piece included. The crew then tags main `m<n>` and opens a `kind:retro` issue, "M<n> retro". The
retro runs as a piece does, under the same pacing, from fixed steps (`prompts/retro.md`): the
play check and the ladder, architecture and relay angles; a wait of up to 72 hours for the owner's
notes, a comment on the retro issue, or `/relay retro go`; the game and players angles; the retro;
and the proposal, which ends with a PR for the owner. Merging it closes the retro, and the crew
opens each `### X.` section of `docs/proposals/m<n+1>.md` as a piece, `relay:ready`, in milestone
`M<n+1>`, with `#X` in an After line made the issue's number.

## Commands

On a piece's or a retro's issue, from the owner only (`relay-events.yml`):

| Command | Effect |
|---|---|
| `/relay run` | Label the piece `relay:ready` if it isn't, and start a crew run now |
| `/relay retry [step]` | Give a blocked step two fresh attempts, and start a run |
| `/relay skip <step>` | Close a step without running it |
| `/relay model <step> <model> [effort]` | Change an open step's model or effort |
| `/relay pause` / `resume` | Hold the piece, or let it go again |
| `/relay status` | The piece's steps, with their state and spend |
| `/relay ready` | Make a proposed issue a piece: label it `relay:ready` |
| `/relay retro` | On a piece: open its milestone's retro now. On a retro: `/relay retro go` ends the wait for notes |

## What a session gets

The crew writes each session's whole prompt and saves it beside the transcript in `runs/`.
Spec and spec-review sessions get the piece and its design files; build sessions get only
their brief, which carries all the design it needs; the review gets no brief, but the piece,
the design files, the diff, the whole spec and the steps table. The pick gets the brief and
each version's summary, decisions, ideas and diff stat, as A and B. The play step gets the piece,
`prompts/play.md` and both of `players/`. A retro step gets the retro issue and `prompts/retro.md`,
and the game step the owner's notes. Every session gets when it started and its cap, the earlier
steps' summaries and decisions, `prompts/session.md` and its kind's prompt.

Sessions never see the App's key. The crew keeps a fresh installation token in a file that
git's credential helper and a `gh` wrapper read, so a 90-minute session never outlives its
token. Transcripts are redacted of the crew's secrets and of anything shaped like a GitHub
token before they're written.

## Spend

Each step records its cost at list price (the CLI's `total_cost_usd`, summed across
attempts), its minutes, the 5-hour and weekly usage before and after each attempt and the
points it rose by, the context its last turn held as a share of the window, and how many
times it compacted. On a subscription the cost isn't money spent; it makes steps, models and
pieces comparable. Both runs of a twinned step record their own spend, so the steps table
puts their costs side by side, with how much of each landed. The usage probe reads the whole account, so the owner's own sessions in
the same hours land on Relay's steps. A crashed attempt has no result frame and records no
cost. Usage is read once at each step boundary and rounded.

A step starts only while the 5-hour window is under 75%, and the weekly window under its share
of the week. A 5-hour stop logs
when the window resets and starts a `wait` run, which sleeps until then and starts a crew
run, so a stop costs the window, not the schedule's lateness. The weekly window is the real
limit on how much Relay gets done. The owner's own sessions share both windows.

## The board

The crew updates "Sans Fiat Work", a project on the owner's account, when it can. A GitHub App
can't reach a personal account's project, so the board needs its own token: a classic
personal access token with the `project` scope, stored as `RELAY_BOARD_TOKEN` in the `relay`
environment. Without it, or without the project, the run log says so and everything else
carries on. Sessions never get the token.

The crew sets only the fields that exist, and only to options that exist, each on its own
with Status first, so one that fails doesn't stop the rest:

| Field | Type | Options |
|---|---|---|
| Status | Single select | Ready, Spec, Building, Waiting, Review, Done |
| Step | Single select | spec, spec-review, build, play, review, retro |
| Model | Single select | haiku, sonnet, opus |
| Effort | Single select | low, medium, high |
| Attempts (sessions run), List $, Minutes, Points, 5-hour points, Weekly points | Number | |
| Waiting on | Text | Set on a piece: "5-hour usage, until 14:20", "weekly usage, until Thu 09:00", "your answer", "your merge" |

The board is for watching. The retro reads the step blocks, which carry the same figures.
Two views are worth making: **Pieces**, a board by Status over `kind:piece`; and **Steps**, a
table grouped by parent issue, summing List $ and the points.

## Files

| File | Written by |
|---|---|
| `relay.mjs`, `token.mjs`, `usage.mjs`, tests | Claude Code sessions, by hand |
| `prompts/*.md` | The owner. How the flow works, for the sessions in it |
| `players/*.md` | The owner. The player and the judge, for the play step and the review |
| `retro/<m>/` | The retro's sessions, on the retro's branch |
| `specs/<n>/` | Spec and spec-review sessions, on the piece's branch |
| `runs/` | The crew and sessions: prompts, transcripts, `result.json`. Not committed |

`node relay/usage.mjs` prints the subscription's usage windows.

## Running it

A run starts once a day (`schedule` in `.github/workflows/relay.yml`), whenever a piece or a
retro closes, from Actions → relay → Run workflow, from `/relay run`, `/relay retry`, a reply to
a blocked step or a retro's notes, or when a `wait` run's window resets. One `wait` run waits at a
time. The crew refuses to start a session while the branch
tracks anything under `relay/runs/`. The
`relay` environment holds `RELAY_APP_ID`, `RELAY_APP_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` and, for
the board, `RELAY_BOARD_TOKEN`.

Each run installs the latest Claude Code. The run log and each step's block say which version
and model a session ran on. If a session never sends the init frame the crew
reads (the CLI's stream-json changed, or it failed to start), the run stops without counting
an attempt, and the Relay log issue says why.

Not built yet: releases.
