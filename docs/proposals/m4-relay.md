# M4: Relay and process

What changes in how pieces get built, from the M3 retro ([m3-retro.md](m3-retro.md)). It lands
as one hand-built change from a session here, before any M4 piece opens, except where a line
names the piece it waits on. The game pieces are in [m4.md](m4.md); the process these changes
serve is [`docs/process.md`](../process.md).

## 1. #101's held half

PR 124 closes with a comment pointing at the retro. The change carries over by hand what doesn't
conflict: `relay/players/player.md`, `relay/specs/101/play.md` and `trial/` (less the save,
which no longer replays). It drops `judge.toml`, the band check, the `toml` dependency, the
`check.sh` line, `bands.md` and the CLAUDE.md band lines; `bands.md`'s one durable fact goes in a
line of `play.md`. `vision.md` gains a `## Decided` with "Standing counts a civ's clans and its
units' people" and "the lead changes only past a margin"; "An army mustn't starve its own side"
is struck. `player.md` is updated for land, the guard, armies and archers, pillage, fights and
the scout's orders, and gains three rules: a number that stays at zero after an order gets `why`
on its subject before the next order; try every lever family once; a could-not-tell note per
lever. `judge.md` becomes the review's rubric (§3) and keeps its four problem kinds for the play
note.

## 2. #105

Closes, re-cut into E and R in `m4.md`. Its branch has nothing on it.

## 3. The judge at review time

CI runs no bands. The judge runs inside the review, on Opus medium, with a written rubric:

- It reads `judge --compare` (piece S) on the PR's and main's `sweep --judge --json`, the piece's
  question and control, the stories against their controls, and the play note.
- It calls a change **better** only when the piece's question moves past the noise floor in the
  score it names, against its control, on more than one retro map; the move is in the first four
  scores and health stays in band; the mechanism is the one the piece claims; the lever fired on
  the retro maps; and a led player beats an idle led civ by more than the floor. It refuses to
  call good a move within drift, a total that moved because a count's meaning changed, a single
  built map, or a gain only in AI-against-AI play.
- It writes one block at the top of the PR description, readable on a phone: a verdict (better,
  same, worse, not shown) and one line per score with old, new and floor; the full table goes in
  `relay/specs/<n>/judge.md`. A verdict of worse or not shown doesn't block the merge; the owner
  reads it at the merge.
- `review.md` gains the rubric and says: a band, a test floor or a fail check that encodes a rule
  goes in the verdict as a rule; every new button is pressed once by a script that checks the
  world; screenshots link the commit, not the branch; the review re-runs the earlier pieces'
  questions and reports a fall (L's solo gain fell from 0.017 to 0.012 under A, and no step saw it).

Until S lands, the review compares by hand as M3's did.

## 4. The play step

Relay gains a `play` step after the last build and before the review, so the review's judge
reads the played game:

- A Sonnet player plays the branch through `sansfiat play` for a hundred turns from the pick on
  the retro map the spec names, with `player.md`; a Sonnet judge pass at medium lists problems
  in `relay/specs/<n>/play.md` with turn and evidence, and `judge FILE` puts its Standing beside
  the AI control and the idle control. About $0.20 and a 5-hour point.
- The piece's `**Play:**` line names the model; `none` for a piece the player doesn't meet. A
  page piece's play step also runs `www/play/all.mjs` and attaches its screenshots.
- A bad result is a finding, not a block. Each finding is one line with a class, `fix` or `ask`
  (§5), and a repro: a lines file or a play script that shows it on main.
- A milestone play check runs the same on main across the retro maps when the milestone is
  built, into `relay/retro/<m>/play.md`.

## 5. The loop without the owner

The owner is the bottleneck in three places the crew can take: restarting after a merge, turning
findings into work, and starting the retro.

**Fix or ask.** A finding or an idea is `fix`, and runs without the owner, when all four hold: it
names what it breaks (a line of a design file or its ledger, a piece's Goal or Decided line, a
touch point in `page.md`, or health); it has a repro that fails on main; the fix restores that
line and edits no design prose; and it fits one Ford build, with no new order, kind, module,
score or `data.rs` number a design line doesn't already fix. Everything else is `ask`: a new or
changed rule, prose, a new lever, a band, two lines that conflict, or "less fun" with no line
named. Of your five notes, Disband all and the scout's search are `fix`; the three buttons, the
flat trail and the harness are `ask`, and M4 takes them as pieces.

**The fixes piece.** The crew keeps one open `kind:piece` per milestone, "Fixes for M<n>", and
appends each `fix` finding with its repro. It runs after the milestone's other pieces, Sonnet
medium spec and spec-review, Ford builds; each repro becomes a test on S's stage. `ask` findings
become issues labelled `relay:proposed`, which never run until the owner's `/relay ready` or a
merged proposal makes them pieces. Sessions' `ideas` take the same two classes; the crew skips a
title already open, so none is lost and none repeats.

**A milestone is built** when none of its pieces can run: each merged, closed or paused, and its
fixes piece merged. The crew tags main `m<n>` then and opens a `kind:retro` issue. A paused
piece stays open and the retro says where it goes. M3 is built at ec1e47b: the change tags it
`m3`.

**The retro run**, as a try in M4, on five angles in four sessions. A `kind:retro` issue runs from a fixed
`steps.json` under the same crew and pacing as a piece: `check` (the milestone play check,
Sonnet medium); `ladder`, `architecture` and `relay` on Opus medium, which don't need the
owner's play; a wait of up to 72 hours for the owner's notes as a comment on the retro issue, or
`/relay retro go`; `game` and `players` on Opus medium (one session, two files); `retro`; and
`proposal`, which opens `<next>.md` and `<next>-relay.md` as one PR. When the proposal PR merges,
the crew opens its pieces as the issues Relay reads, labelled `relay:ready`. The Relay change
itself stays a hand-built PR. The angles read the run artifacts with the crew's `gh`; they last
fourteen days, so the retro runs soon after the milestone is built.

**The owner's stops become:** merge the proposal PR; a blocker's reply; a merge the crew wouldn't
make; play main and dictate within 72 hours of the retro issue, or not at all; promote or strike
ledger lines in the proposal PR, where the proposal session drafts them.

## 6. Pacing

An M3-sized milestone with its loop is about 55–78 weekly points: the pieces 33, the play steps
2–3, the fixes piece 4, the play check 1–3, the retro 15–35 (unmeasured; this retro's own cost
is the first figure). The retro isn't new spend: the owner runs those sessions from the same
week today. It runs in the days Relay sat idle.

- A piece's close starts a crew run, so a merge by the owner doesn't wait for `/relay run`.
- One `wait` run at a time: the wait job gets its own concurrency group.
- `EXPECTED_POINTS` goes; a step starts while the 5-hour window is under 75%. The weekly ramp
  stays.
- The schedule goes to once a day, once a close starts a run.

## 7. The ladder

`relay/prompts/spec.md` changes as `ladder.md` §7 drafts:

- 6–8 points runs Sonnet medium; 9–10 Sonnet high; `canvas`, `glaze` and `clay` run Opus low
  from 8 to 10.
- The rung below is the canvas, glaze or clay on Sonnet high in place of Opus low, and its `why`
  says what it expects; a Henry Ford piece runs its largest ford a rung below. From #155 on, the
  twin replaces it: the rung below ran every M4 painter on Sonnet with no Opus run to compare
  against. Each piece twins one build step, which runs on its ladder rung and again from the same
  commit on a rung the spec chooses, and a blind `pick` step measures both and lands one or parts
  of both (`relay/prompts/spec.md`, The twin; `relay/prompts/pick.md`).
- A `ford`, `easel` or `kiln` step with Open 2 and Miss 3, or at 9 or more, holds unsettled work:
  move it to the canvas or glaze, or settle it by running.
- The review runs Sonnet high, or Opus medium when `main` has moved under the piece or the goal
  is judged by play, matching the crew.
- The size line counts tests, pins and `engine/CLAUDE.md`.
- A kiln that has to run stories on real maps to see its own rule work is part of the glaze.
- The designer and spec-review measure on `main` with the piece's earlier settled steps applied
  where a later step relies on the number.
- A Henry Ford spec may run on Sonnet high; S is the probe.
- A brief that adds to a file over 600 lines names the file to split it into.

Spec-review stays at Opus medium: every M3 piece's spec-review changed something that would have
failed a build or the proof.

## 8. Crew fixes

- A review's `result.json` must carry `"merge": true` or `false`; a review without it is
  `pending`. `session.md`'s schema gains the field.
- The crew writes the review's own row of the steps table after the review ends.
- The prompt gives a session its start time and the cap, and says to check the clock before
  giving up on time.
- `relay-events` gets `RELAY_BOARD_TOKEN`, so a close moves the board.
- `/relay ready` labels a proposed issue ready; `/relay retro` starts the retro early.
- The log rounds usage.

## 9. Prompts

- `session.md`: drop "under five is usual"; a build that follows its brief usually has no
  decisions; the summary is what later steps must know, not the diff.
- `engine/CLAUDE.md`: one row for `www/play/`, and a byte budget of 8 KB that `check.sh`
  enforces.
- `review.md`: §3's rubric, the rule-in-a-band line, the buttons script, screenshots at the
  commit, the earlier pieces' questions re-run.
- `spec-review.md`: measure with the earlier settled steps applied.
- `relay/prompts/retro.md`: the angles become five (game, players, ladder, architecture, relay)
  with the owner's notes as a sixth file; the run kind and the 72-hour wait.
- `planner.md`: the flow gains the play step, fix or ask, the fixes piece, the milestone-built
  rule and the retro run.
- `relay/players/player.md`: §1's updates. `relay/players/judge.md`: the review's rubric.

## 10. Before a piece opens

- Close PR 124 and #105; tag `m3`.
- Run the review's compare by hand on main once, so the first M4 review has a baseline.
- The owner reads `labour.md`, `farming.md` and the six answers at the top of `m4.md`.
