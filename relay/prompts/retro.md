# The retro

A milestone ends in a retro: the milestone read back from five angles, then the retro and the
next proposal written from what they found. Relay runs it once the milestone is built, as a
`kind:retro` issue whose steps are fixed. Each step is one cold session on the retro's branch, and
its files go in `relay/retro/<m>/`.

| Step | Writes | Reads |
|---|---|---|
| `check` | `play.md` | The milestone play check: `main` played and judged as a play step does, on as many retro maps as fit, small ones first, with its files here rather than in a spec folder. Its findings are all `ask` |
| `ladder` | `ladder.md` | Every step's model, effort, points, list $, minutes and usage from the step blocks; what each whole step did itself and what it handed on, and what a 5-hour window bought; the specs, briefs and `steps.json` against what the builds did. Drafts the ladder's next text |
| `architecture` | `architecture.md` | The code's shape against `docs/architecture.md`: what grew where, and what the next milestone needs first |
| `relay` | `relay.md` | The crew's log, the workflows, the waits and the transcripts: pacing, stops, failures, merges, each prompt line against what it changed, and what could come out |
| `game` | `owner.md`, `game.md`, `players.md` | The owner's notes as they came, written to `owner.md`. Then two angles: each piece's question measured on `main`, and the ledger lines that aren't decisions of the ledger's weight, to strike (`game.md`); the play notes, the check and the judge's sweeps, what a player meets and what the AI does (`players.md`) |
| `retro` | `docs/proposals/<m>-retro.md` | The angles: what the milestone answered in the scores, the questions the owner must answer, and what changes in the process, each pointing at its evidence |
| `proposal` | `docs/proposals/<next>.md`, `<next>-relay.md` | The retro: the game's pieces, and Relay's changes |

The crew waits up to 72 hours after `relay` for the owner's notes, a comment on the retro issue,
before `game` runs. Without them, `game` says so and goes on.

The angles read the run artifacts (the transcripts and the crew's log) with `gh run download`;
they last fourteen days.

## An angle file

It opens with a verdict, measures before it judges, cites its sources (issue, PR, run,
transcript), and ends with "For other angles" and an ideas table: idea, source, value, cost, and
take, try, later or drop. A figure it couldn't measure says so.

## The proposal

`<next>.md` opens with the owner's questions from the retro, answered as the proposal assumes, so
the owner can change one and see what moves. Each piece is a `### X. Title` section, X a letter,
with the lines a piece's issue carries (`relay/README.md`): Goal with its question and control,
Whole (or Spec and Spec-review), Play, Shape, Design, Decided, In scope, Out of scope, and After
as `#X`. When the owner merges the proposal PR, the crew opens each section as a piece.

The owner's notes come first: each one is a piece, a line of a piece, a change in
`<next>-relay.md`, or an answer in the retro, and the proposal says which.

`<next>.md` ends with `## Ideas`: every open idea gets an ending. Read the milestone's ideas
issue and the open `relay:proposed` issues, and write one line for each, or for each cluster
that says the same thing: `- #220, #239: taken in A`, `- #272: dropped, why`, or
`- #181: kept for M7`. When the proposal merges the crew closes each issue a taken or dropped
line names, and the milestone's ideas issue; a kept idea stays open. Keep few.

The design ledgers are decision records (`review.md`, The design ledger). The proposal PR
strikes the ledger lines that aren't, and leaves the prose to the owner.
