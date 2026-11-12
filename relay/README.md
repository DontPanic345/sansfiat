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
**Whole:** fable medium
**Play:** sonnet
**Shape:** Easel and Canvas: why
**Design:** docs/design/clans.md
**Decided:** …
**In scope:** …
**Out of scope:** …
**After:** #31
```

`Whole` names the model and effort of the piece's first session, which plans and builds it
(below). A piece without it runs stepped and needs `Spec` and `Spec-review`, each on its own
line naming a model and an effort: a piece with neither is held.
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
`relay/specs/<number>/`.

| Step | What it does |
|---|---|
| `whole` | One session on the model the piece's `Whole` line names, up to five hours, that plans and builds the piece however it chooses (`prompts/whole.md`). It ends the piece itself, with `"merge": true` or `false` as a review does, or hands on: it writes `steps.json` with any builds and their briefs, then `review`, and finishes without `merge` |
| `spec` | A stepped piece's first step. The designer settles what it can, picks a shape, and writes the spec, a brief per build, and `steps.json` with each build's part and each step's model, effort, points and why (`prompts/spec.md` has the shapes and the ladder). Runs on the model the piece's `Spec` line names |
| `spec-review` | Reads the code each settled rule touches, runs the sweep or a play script where minutes settle a number, and fixes the plan where the run disagrees with the reading. Runs on the model the piece's `Spec-review` line names, never at low effort |
| `build-N` | Builds what its brief says |
| `play` | Plays the branch through `sansfiat play` for 100 turns from the pick (`relay/players/player.md`), judges the game (`relay/players/judge.md`), and writes `play.md` with each finding classed `fix` or `ask`. The crew adds it before the review when the piece's `Play` line names a model |
| `review` | Merges `main` in, judges the change against `main` with the rubric in `relay/players/judge.md`, reviews the whole piece against the piece and its design on the axes the builds couldn't check, fixes what is small, adds a decision of the ledger's weight to a design file's `## Decided` ledger, writes the PR description for the owner to read on a phone (the judge's verdict, the steps table, the answer in the scores, the ledger lines, screenshots and a merge verdict), and waits for CI on its own head commit. Its `result.json` must say `"merge": true` or `false`. The crew then rewrites the steps table with the review's row, marks the PR ready, and on a merge verdict waits up to ten minutes for CI to go green and merges it |

A whole step starts only while the 5-hour window is under 10%, so the session gets a whole
window, and only in a job with five hours left, so a run that has already spent its time starts
a fresh one. The prompt gives the session both usage windows as it starts, and a hook
(`usage-hook.mjs`) repeats them every fifteen minutes, on every session. A session the usage
limit ends costs its step no attempt: the next attempt waits for the reset and is told why the
last one ended and the branch's last commit.

The crew makes the steps after a whole step that hands on, or after spec-review, from
`steps.json` as that step left it, with the play step before the review. Neither is done until
`steps.json` reads cleanly. Then the crew carries on, with no stop for the owner. Every step's
prompt carries every earlier step's summary and decisions; the play step's carries the piece
alone.

Otherwise it's the owner's turn: merge the PR, which closes the piece, or close the piece,
which tags the branch head `proof/<number>` and closes the PR. Closing a piece closes its open
steps. The owner plays the game on main or on a PR's preview, and what plays badly becomes a
new piece.

The crew works on the oldest piece that has a step to run, so a later piece goes ahead only
while an earlier one waits on the owner, and never ahead of a piece its `After` line names.

## How a step counts

Steps mark their own homework. A session commits and pushes to the piece's branch and
writes `relay/runs/result.json`:

- `done`, with a `summary` and any `decisions`: the crew closes the step and keeps both in its
  block for the sessions after it. The PR is made as a draft as soon as the branch has commits,
  and the crew links it and the spec folder on the piece. It is marked ready at its verdict.
  Nothing a step writes for other sessions is posted on the PR: its description is the owner's.
- `pending`, with a `note`: the next attempt picks up from the note. Two unfinished attempts
  (a crash or timeout counts) block the step.
- `blocked`, with a `blocker`: the step can't go on as it stands.

A blocked step labels the piece `waiting`. The owner's reply on the piece unblocks it, goes to the
next session, and starts a run.

## Fix or idea

What a session notices that the owner would have to decide goes under `ideas`, never into code,
and the play step's problems go under `findings`. A builder's note goes in the piece's
`notes.md`, and what a designer could settle is settled. Each idea has a class. A `fix` names
the line it breaks (a design line or its ledger, a piece's Goal or Decided line, a touch point,
or health), has a repro, restores that line without editing design prose, and fits one Ford
build with nothing new. Everything else is an `ask`.

The crew files them:

- A `fix` goes on its milestone's **fixes piece**, "Fixes for M<n>", which the crew opens the first
  time and keeps (`relay:fixes`). It runs after the milestone's other pieces, on Sonnet medium,
  each repro becoming a test. A `fix` that comes after the fixes piece has started is an `ask`.
- An `ask` becomes a line on the milestone's one ideas issue, "Ideas from M<n>" (`kind:idea`).
  A spec, whole or play session's prompt lists the open lines; a designer takes the obvious
  ones into its piece and names them under `took`, and the crew ticks them. A title already
  there is skipped.
- A retro's ideas stay in its angle files. Its proposal ends with `## Ideas`, a line for every
  open idea and `relay:proposed` issue: taken, dropped or kept. When the proposal merges the
  crew closes each issue a taken or dropped line names, the ideas issue and the milestone's
  log.

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
the design files, the diff, the whole spec and the steps table. The play step gets the piece,
`prompts/play.md` and both of `players/`. A whole session gets the piece, its design files and
`prompts/whole.md`. A spec, whole or play session also gets the milestone's open ideas. A retro
step gets the retro issue and `prompts/retro.md`, and the game step the owner's notes. Every
session gets when it started, its cap and the usage windows, `prompts/session.md` and its
kind's prompt, and every session but the play step the earlier steps' summaries and decisions.

Sessions never see the App's key. The crew keeps a fresh installation token in a file that
git's credential helper and a `gh` wrapper read, so a session never outlives its
token. Transcripts are redacted of the crew's secrets and of anything shaped like a GitHub
token before they're written.

## Spend

Each step records its cost at list price (the CLI's `total_cost_usd`, summed across
attempts), its minutes, the 5-hour and weekly usage before and after each attempt and the
points it rose by, the context its last turn held as a share of the window, and how many
times it compacted. On a subscription the cost isn't money spent; it makes steps, models and
pieces comparable. The usage probe reads the whole account, so the owner's own sessions in
the same hours land on Relay's steps. A crashed attempt has no result frame and records no
cost. Usage is read once at each step boundary and rounded.

A step starts only while the 5-hour window is under 75% (10% for a whole step), and the weekly
window under its share of the week. A stop on either window starts a `wait` run, which sleeps
until the window lifts, five hours at most, and starts a crew run, which waits again if the
lift is further off. So a stop costs the window, not the schedule's lateness. The weekly window
is the real limit on how much Relay gets done. The owner's own sessions share both windows.

Each milestone has one log issue, "Relay log: M<n>". A run that did anything reports there: why
it stopped, each step it ran with a link to the step's issue, and both windows, the week beside
the share the day allows.

Before each piece of work the crew fetches `main`, and once the crew's code there differs from
the code it is running, it starts a fresh run and stops.

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
| Step | Single select | spec, spec-review, build, play, review, whole, retro |
| Model | Single select | haiku, sonnet, opus, fable |
| Effort | Single select | low, medium, high |
| Attempts (sessions run), List $, Minutes, Points, 5-hour points, Weekly points | Number | |
| Waiting on | Text | Set on a piece: "5-hour usage, until 14:20", "weekly usage, until Thu 09:00", "your answer", "your merge" |

The board is for watching. The retro reads the step blocks, which carry the same figures.
Two views are worth making: **Pieces**, a board by Status over `kind:piece`; and **Steps**, a
table grouped by parent issue, summing List $ and the points.

## Files

| File | Written by |
|---|---|
| `relay.mjs`, `token.mjs`, `usage.mjs`, `usage-hook.mjs`, tests | Claude Code sessions, by hand |
| `prompts/*.md` | The owner. How the flow works, for the sessions in it |
| `players/*.md` | The owner. The player and the judge, for the play step and the review |
| `retro/<m>/` | The retro's sessions, on the retro's branch |
| `specs/<n>/` | The piece's sessions, on its branch: the spec, the briefs, `steps.json`, evidence and `notes.md` |
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
