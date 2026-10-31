# The retro

A milestone ends in a retro: the milestone read back from several angles, then the retro and
the next proposal written from what they found. It runs with the user, in planning sessions,
not in Relay.

## The angles

Each angle is one session that reads the milestone from its side and writes one file,
`relay/retro/<m>/<angle>.md`:

| Angle | Reads |
|---|---|
| `cost` | Every step's model, effort, points, list $, minutes and usage, from the step blocks. Did the ladder predict? What a piece takes of a window and a week |
| `briefs` | The specs, briefs and `steps.json` against what the builds did: shapes, parts, cuts, scores, and what spec-review and the review caught. Drafts the ladder's next text |
| `crew` | The crew's log, the workflows and the waits: pacing, stops, failures, merges, and what could come out of `relay.mjs` |
| `prompts` | Each prompt line against the transcripts: what changed behaviour, what did nothing, what misled. Prompt sizes, and `engine/CLAUDE.md` |
| `game` | Each piece's question measured on main, the decisions made by design file, the owner's play notes, and the ledger lines to promote or strike |
| `architecture` | The code's shape against `docs/architecture.md`: what grew where, and what the next milestone needs first |
| `players` | The play notes and the judge's sweeps: what a player meets, what the AI does, and what the scores say |

Add one the milestone needs, named for it.

An angle file opens with a verdict, measures before it judges, cites its sources (issue, PR,
run, transcript), and ends with "For other angles" and an ideas table: idea, source, value,
cost, and take, try, later or drop. A figure it couldn't measure says so.

## The retro and the proposal

One session reads the angles and writes `docs/proposals/<m>-retro.md`: what the milestone
answered in the scores, the questions the owner must answer, and what changes in the process,
each pointing at its evidence. The next proposal follows from it: the game's pieces in
`<next>.md`, Relay's changes in `<next>-relay.md`.

The planning session reads the retro and the proposals, not the angles.
