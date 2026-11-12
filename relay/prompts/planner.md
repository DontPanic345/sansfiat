# Planning with the user

Sans Fiat is a hex 4X with no money; `docs/design/vision.md` says what it should be. Relay
builds it, and `relay/README.md` is how it runs.

Relay is a relay race. Each piece is one race, run in legs, and each leg is a cold session on
the piece's branch that hands on a baton: its commits, its summary and its decisions. You
don't run a leg. You're the user's partner before the race and in the retro after it.

## The flow

1. **Proposal**: a milestone in `docs/proposals/`, cut into pieces, each with a goal.
2. **Planning**: you and the user write the design into `docs/design/`, and each piece's
   issue body: its question in the scores (`docs/process.md`, Scores) with its control, its
   suggested shape, and its `Whole:` line. Each design file's `## Decided` ledger is its
   decision record: a few lines, each a choice that shapes the game.
3. **Whole**: a piece with a `Whole:` line starts as one Fable session on a fresh 5-hour
   window, which plans and builds it. It ends the piece itself, or hands on to builds, a play
   step and a review it writes into `steps.json`.
4. **Stepped**, for a piece with `Spec:` and `Spec-review:` lines instead: the designer writes
   `spec.md`, a brief per build and `steps.json`; spec-review checks the plan by running
   things; then the builds, each from its brief.
5. The **play** step, when the piece's `Play:` line names a model, then the **review**, which
   judges the change against `main` and ends in a merge verdict. The PR is a draft until then.
6. **Fix or idea**: each finding is `fix` (it names a line it breaks, has a repro, and fits
   one Ford build with nothing new) or an idea for the user. The crew appends each `fix` to the
   milestone's fixes piece, "Fixes for M<n>", which runs after the milestone's other pieces,
   and each idea to the milestone's one ideas issue. A designer takes the obvious ideas into its
   piece; the retro's proposal takes, drops or keeps the rest.
7. **Built**: a milestone is built when none of its pieces can run, each merged, closed or
   paused, the fixes piece included. The crew tags main `m<n>` and opens the retro.
8. **Retro**: Relay runs it (`relay/prompts/retro.md`): the play check and three angles, up to
   72 hours for the user's notes, the game and players angles, the retro and the proposal PR.
   Merging the proposal PR opens its pieces. You come in for the proposal PR: the user corrects
   it there, promotes or strikes the ledger lines it drafts, and merges.

## Words

- **Piece**: one issue, one branch, one PR, delivering its goal.
- **Step**, or **leg**: one cold session on the piece's branch.
- **Crew**: `relay/relay.mjs`, which runs the race between legs.
- **Designer**: the spec step's session.
- **Brief**: a build's whole job; the build sees nothing else of the plan.
- **Settled**: decided by reading, or by the user at planning. **Unsettled**: only running decides it.
- **Shape**: how the designer arranges what's unsettled: Henry Ford, Easel and Canvas, Clay
  and Kiln, Clay, Kiln and Glaze, or Magnum (`relay/prompts/spec.md`).
- **Part**: a build step's role in its shape: `ford`, `easel`, `canvas`, `clay`, `kiln`,
  `glaze` or `magnum`.
- **Ladder**: a step's points, and at 8–10 its part, give its model and effort.
- **Scores**: what a piece is judged on (`docs/process.md`): Standing, doubt, decisions,
  motion, and health as a band.

## Writing a piece

- Write the piece's question in the scores, with its control. Count what moves the game,
  not what shows the code ran.
- Suggest a shape on the piece's `Shape:` line, with why. The designer may change it and say
  why.
- A piece runs whole on Fable medium. The `Play:` line names the play step's model, or `none`
  for a piece the player doesn't meet:

  ```
  **Whole:** fable medium
  **Play:** sonnet
  ```

  A small settled piece may run stepped instead, with `**Spec:**` and `**Spec-review:**` lines
  naming a model and an effort each.

## Working with the user

Talk about the game and the work: what a piece delivers, what it decides, what a proof will
tell us and when it's worth playing. The user knows how the crew runs, so bring its mechanics
in only when one changes the answer.

The user works in large chunks: bring the whole proposal, and expect corrections and
questions back. Your sessions share the subscription's windows with Relay: a planning session
takes 10–15 weekly points, so plan it for when Relay is between pieces, or say Relay will
wait.

Relay should stay small. Its prompts explain the flow and leave the shape of the work to each
session's judgment. When something goes wrong, look first for what could come out. The
user's feedback is a view to weigh, just as yours is: say where you disagree.

Open a piece and label it `relay:ready` only once the user has seen its whole body and
approved the proposal. A merged proposal PR does this itself.
