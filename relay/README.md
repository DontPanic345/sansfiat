# Relay

Relay builds Sans Fiat one piece at a time, in cold `claude -p` sessions, with no one
watching. The crew, `relay/relay.mjs`, runs in GitHub Actions (`relay.yml`). It picks the
next step, checks the budget, builds the session's prompt, starts the session on the
step's model and effort, and records how it went. It's ported from Tortoise Spelling's
Jogging, with GitHub instead of `state.json` holding the state.

## Pieces and steps

A **piece** is an issue opened by Alan from a piece form (proof, merge or polish). Relay
works on it once Alan adds `relay:ready`. Its **steps** are sub-issues, each carrying its
model, effort, attempts, tokens, context and compactions in a hidden `<!-- relay {...} -->`
block. The spec, the briefs and `steps.json` live on the piece's branch, `relay/<number>`,
in `relay/specs/<number>/`.

| Step | What it does |
|---|---|
| `spec` | Writes the spec (with its scope map and design questions), a brief per step, and `steps.json` |
| `spec-review` | Reviews them as their sessions will read them, and cuts work outside the scope |
| *sign-off* | The crew makes the build and review steps and labels the piece `waiting`. Alan answers the design questions in `docs/design/`, checks the models, and comments `/relay go` |
| `build-N` | Builds what its brief says. At most four |
| `review` | The crew marks the PR ready, so CI runs. The session reviews the whole piece, fixes what's substantive, and waits for CI to go green |

Then it's Alan's turn: a proof's PR stays open with its findings and Alan closes the piece;
a merge's or polish's PR is Alan's to approve and merge.

Merges never overlap: a merge waits while an older merge piece is open, and while an
earlier milestone (by its number, "M1", "M2" …) is still open. Proofs can run ahead.

## How a step counts

Steps mark their own homework. A session commits and pushes to the piece's branch and
writes `relay/runs/result.json`:

- `done`, with a `summary`: the crew closes the step and posts the summary on the step and the PR.
- `pending`, with a `note`: the next attempt picks up from the note. Two unfinished attempts
  (a crash or timeout counts) block the step.
- `blocked`, with a `blocker`: only for infrastructure or an account Alan must supply.

A blocked step labels the piece `waiting`. Alan's reply on the piece unblocks it and goes
to the next session. Anything new a session notices goes under `ideas` and becomes a
`kind:idea` issue, never code.

## Commands

On a piece's issue, from Alan only (`relay-events.yml`):

| Command | Effect |
|---|---|
| `/relay go` | Sign off the spec and start building |
| `/relay model <step> <model> [effort]` | Change an open step's model or effort |
| `/relay retry [step]` | Give a blocked step two fresh attempts |
| `/relay skip <step>` | Close a step without running it |
| `/relay pause` / `resume` | Hold the piece, or let it go again |
| `/relay status` | The piece's steps, with their state and spend |
| `/relay scope` | The spec's scope map and the ideas raised so far |
| `/relay run` | Label the piece `relay:ready` if it isn't, and start a crew run now |

## What a session gets

The crew writes each session's whole prompt and saves it beside the transcript in
`runs/`. Spec and spec-review sessions get the piece as Alan wrote it and its design file;
build sessions get only their brief, which carries all the design it needs; the review
gets its brief, the diff and the whole spec. Every session gets `prompts/session.md`, its
kind's prompt and its piece type's (`type-proof.md` and so on).

Sessions never see the App's key. The crew keeps a fresh installation token in a file
that git's credential helper and a `gh` wrapper read, so a 90-minute session never
outlives its token.

## Spend

Each step records its tokens (every model, input, cache and output), the context its last
turn held as a share of the window, and how many times it compacted. Pieces are sized so
sessions shouldn't need to compact: a count above 0 says the step was too big. The board
gets these as Tokens, Context and Compactions if those fields exist. Usage is paced as in
Jogging: no step starts once the 5-hour window passes 50% or the weekly window passes its
share of the week.

## The board

The crew updates "Sans Fiat Work", a project on Alan's account, when it can. A GitHub App
can't reach a personal account's project, so the board needs its own token: a classic
personal access token with the `project` scope, stored as `RELAY_BOARD_TOKEN` in the `relay`
environment. Without it, or without the project, the run log says so and everything else
carries on. Sessions never get the token.

The crew sets only the fields that exist, and only to options that exist:

| Field | Type | Options |
|---|---|---|
| Status | Single select | Ready, Spec, Sign-off, Building, Waiting, Review, Done |
| Piece type | Single select | proof, merge, polish |
| Step | Single select | spec, spec-review, build, review |
| Model | Single select | haiku, sonnet, opus |
| Effort | Single select | low, medium, high |
| Attempts, Tokens, Context, Compactions | Number | |

## Files

| File | Written by |
|---|---|
| `relay.mjs`, `token.mjs`, tests | Claude Code sessions, by hand |
| `prompts/*.md` | Alan owns them (CODEOWNERS). The rules sessions follow |
| `usage.py` | Copied from Jogging: reads the subscription's usage |
| `specs/<n>/` | Spec and spec-review sessions, on the piece's branch |
| `runs/` | The crew and sessions: prompts, transcripts, `result.json`. Not committed |

## Running it

The schedule is off: start a run from Actions → relay → Run workflow, or `/relay run` on a
piece. To run every six hours, uncomment `schedule` in `.github/workflows/relay.yml`. The
`relay` environment holds `RELAY_APP_ID`, `RELAY_APP_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` and,
for the board, `RELAY_BOARD_TOKEN`.

Each run installs the latest Claude Code. The run log and each finished step's comment say
which version and model a session ran on. If a session never sends the init frame the crew
reads (the CLI's stream-json changed, or it failed to start), the run stops without counting
an attempt, and the Relay log issue says why.

Not built yet: milestone play checks, tags and releases.
