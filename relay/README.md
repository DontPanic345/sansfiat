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
**Goal:** what the piece delivers, or the one question a proof answers
**Ends:** main, or its branch
**Spec:** opus high, because …
**Design:** docs/design/clans.md
**Decided:** …
**In scope:** …
**Out of scope:** …
```

`Ends` and `Spec` are required: a piece without them is held. `Ends: main` changes the game;
anything else is a proof, which stays on its branch. Every `docs/design/*.md` path in the
body goes to the spec and review sessions in full.

Its **steps** are sub-issues, each carrying its model, effort, attempts, tokens, cost,
context, compactions, summary and decisions in a hidden `<!-- relay {...} -->` block. The
spec, the briefs and `steps.json` live on the piece's branch, `relay/<number>`, in
`relay/specs/<number>/`.

| Step | What it does |
|---|---|
| `spec` | Writes the spec, a brief per later step, and `steps.json` with each step's model, effort and why. Runs on the model the piece's `Spec` line names |
| `spec-review` | Reads the plan the way the later sessions will, and fixes it |
| `build-N` | Builds what its brief says |
| `review` | The crew marks the PR ready, so CI runs. The session reviews the whole piece, fixes what matters, and waits for CI to go green |

The spec step isn't done until `steps.json` reads cleanly; then the crew makes every later
step and carries on, with no stop for the owner. Every step's prompt carries every earlier step's
summary and decisions.

Then it's the owner's turn: a proof's PR stays open with its findings and the owner closes the piece,
which tags the branch head `proof/<number>`; a piece that ends on main has a PR for the owner to
approve and merge. Closing a piece closes its open steps.

Pieces that end on main never overlap: one waits while an older one is open. Proofs never
wait.

## How a step counts

Steps mark their own homework. A session commits and pushes to the piece's branch and
writes `relay/runs/result.json`:

- `done`, with a `summary` and any `decisions`: the crew closes the step and posts both on
  the step and the PR.
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
their brief, which carries all the design it needs; the review gets its brief, the design
files, the diff and the whole spec. Every session gets the earlier steps' summaries and
decisions, `prompts/session.md` and its kind's prompt.

Sessions never see the App's key. The crew keeps a fresh installation token in a file that
git's credential helper and a `gh` wrapper read, so a 90-minute session never outlives its
token. Transcripts are redacted of the crew's secrets and of anything shaped like a GitHub
token before they're written.

## Spend

Each step records its tokens (every model, input, cache and output), its cost at list price
(the CLI's `total_cost_usd`, summed across attempts), the context its last turn held as a
share of the window, and how many times it compacted. On a subscription the cost isn't
money spent; it makes steps, models and pieces comparable. A crashed attempt has no result
frame and records nothing. Usage is paced as in Jogging: no step starts once the 5-hour
window passes 50% or the weekly window passes its share of the week.

## The board

The crew updates "Sans Fiat Work", a project on the owner's account, when it can. A GitHub App
can't reach a personal account's project, so the board needs its own token: a classic
personal access token with the `project` scope, stored as `RELAY_BOARD_TOKEN` in the `relay`
environment. Without it, or without the project, the run log says so and everything else
carries on. Sessions never get the token.

The crew sets only the fields that exist, and only to options that exist:

| Field | Type | Options |
|---|---|---|
| Status | Single select | Ready, Spec, Building, Waiting, Review, Done |
| Ends | Single select | main, branch |
| Step | Single select | spec, spec-review, build, review |
| Model | Single select | haiku, sonnet, opus |
| Effort | Single select | low, medium, high |
| Attempts, Tokens, List $, Context, Compactions | Number | |

## Files

| File | Written by |
|---|---|
| `relay.mjs`, `token.mjs`, `usage.mjs`, tests | Claude Code sessions, by hand |
| `prompts/*.md` | The owner. How the flow works, for the sessions in it |
| `specs/<n>/` | Spec and spec-review sessions, on the piece's branch |
| `runs/` | The crew and sessions: prompts, transcripts, `result.json`. Not committed |

`node relay/usage.mjs` prints the subscription's usage windows.

## Running it

A run starts every six hours (`schedule` in `.github/workflows/relay.yml`), from Actions →
relay → Run workflow, or from `/relay run`, `/relay retry` or a reply to a blocked step. The
`relay` environment holds `RELAY_APP_ID`, `RELAY_APP_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` and, for
the board, `RELAY_BOARD_TOKEN`.

Each run installs the latest Claude Code. The run log and each finished step's comment say
which version and model a session ran on. If a session never sends the init frame the crew
reads (the CLI's stream-json changed, or it failed to start), the run stops without counting
an attempt, and the Relay log issue says why.

Not built yet: the retro after a milestone, milestone play checks, tags and releases.
