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
**Shape:** Easel and Canvas: why
**Design:** docs/design/clans.md
**Decided:** …
**In scope:** …
**Out of scope:** …
**After:** #31
```

`Spec` and `Spec-review` are required, each on its own line: a piece without them is held.
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
| `spec` | The designer settles what it can, picks a shape, and writes the spec, a brief per build, and `steps.json` with each build's part and each step's model, effort, points and why (`prompts/spec.md` has the shapes and the ladder). Runs on the model the piece's `Spec` line names |
| `spec-review` | Reads the code each settled rule touches, runs the sweep or a play script where minutes settle a number, and fixes the plan where the run disagrees with the reading. Runs on the model the piece's `Spec-review` line names, never at low effort |
| `build-N` | Builds what its brief says |
| `review` | Merges `main` in, reviews the whole piece against the piece and its design, fixes what is small, appends the piece's decisions to each design file's `## Decided` ledger, writes the PR description for the owner to read on a phone (the steps table, the answer in the scores, the ledger lines, screenshots and a verdict), and waits for CI on its own head commit. On a merge verdict, the crew waits up to ten minutes for CI to go green and merges the PR |

The crew makes spec-review once the spec closes, and the builds and the review once
spec-review closes, from `steps.json` as spec-review left it. Spec-review isn't done until
`steps.json` reads cleanly. Then the crew carries on, with no stop for the owner. Every step's
prompt carries every earlier step's summary and decisions.

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
next session, and starts a run. Anything new a session notices goes under `ideas`, never into
code. The crew lists them in the step's PR comment. They don't become issues: open one yourself
for any worth keeping.

## Commands

On a piece's issue, from the owner only (`relay-events.yml`):

| Command | Effect |
|---|---|
| `/relay run` | Label the piece `relay:ready` if it isn't, and start a crew run now |
| `/relay retry [step]` | Give a blocked step two fresh attempts, and start a run |
| `/relay skip <step>` | Close a step without running it |
| `/relay model <step> <model> [effort]` | Change an open step's model or effort |
| `/relay pause` / `resume` | Hold the piece, or let it go again |
| `/relay status` | The piece's steps, with their state and spend |

## What a session gets

The crew writes each session's whole prompt and saves it beside the transcript in `runs/`.
Spec and spec-review sessions get the piece and its design files; build sessions get only
their brief, which carries all the design it needs; the review gets no brief, but the piece,
the design files, the diff, the whole spec and the steps table. Every session gets the earlier steps' summaries and
decisions, `prompts/session.md` and its kind's prompt.

Sessions never see the App's key. The crew keeps a fresh installation token in a file that
git's credential helper and a `gh` wrapper read, so a 90-minute session never outlives its
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

A step starts only if the 5-hour window plus the points it's expected to take stays under
80%. The expected points are by model and effort, from M2's medians (`EXPECTED_POINTS` in
`relay.mjs`). The weekly window must also be under its share of the week. A 5-hour stop logs
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
| Step | Single select | spec, spec-review, build, review |
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
| `prompts/*.md` | The owner. How the flow works, for the sessions in it. `retro.md` is for the planning sessions after a milestone |
| `specs/<n>/` | Spec and spec-review sessions, on the piece's branch |
| `runs/` | The crew and sessions: prompts, transcripts, `result.json`. Not committed |

`node relay/usage.mjs` prints the subscription's usage windows.

## Running it

A run starts every six hours (`schedule` in `.github/workflows/relay.yml`), from Actions →
relay → Run workflow, from `/relay run`, `/relay retry` or a reply to a blocked step, or
when a `wait` run's window resets. The crew refuses to start a session while the branch
tracks anything under `relay/runs/`. The
`relay` environment holds `RELAY_APP_ID`, `RELAY_APP_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` and, for
the board, `RELAY_BOARD_TOKEN`.

Each run installs the latest Claude Code. The run log and each step's block say which version
and model a session ran on. If a session never sends the init frame the crew
reads (the CLI's stream-json changed, or it failed to start), the run stops without counting
an attempt, and the Relay log issue says why.

When a piece closes the last open issue of a milestone titled `M<n>`, the crew tags main
`m<n>`.

Not built yet: the `play` step and milestone play checks, and releases.
