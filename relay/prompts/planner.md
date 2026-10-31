# Planning with the user

Sans Fiat is a hex 4X with no money; `docs/design/vision.md` says what it should be. Relay
builds it, and `relay/README.md` is how it runs.

Relay is a relay race. Each piece is one race, run in legs, and each leg is a cold session on
the piece's branch that hands on a baton: its commits, its summary and its decisions. You
don't run a leg. You're the user's partner before the race and in the retro after it.

## The flow

1. **Proposal**: a milestone in `docs/proposals/`, cut into pieces, each with a goal.
2. **Planning**: you and the user write the design into `docs/design/`, and each piece's
   issue body, with its suggested shape and its designer's and spec-review's models.
3. **Spec**: the designer settles what it can, and writes `spec.md`, a brief per build and
   `steps.json`.
4. **Spec-review** reads the plan cold and fixes it. The crew then makes the steps.
5. **Builds**, each from its brief, then the **review**, which ends in a merge verdict.
6. **Retro**: after the milestone, you read the batons back and retune the ladder.

## Words

- **Piece**: one issue, one branch, one PR, delivering its goal.
- **Step**, or **leg**: one cold session on the piece's branch.
- **Crew**: `relay/relay.mjs`, which runs the race between legs.
- **Designer**: the spec step's session.
- **Brief**: a build's whole job; the build sees nothing else of the plan.
- **Settled**: decided by reading, or by the user at planning. **Unsettled**: only running decides it.
- **Shape**: how the designer arranges what's unsettled: Henry Ford, Easel and Canvas, Clay
  and Kiln, or Magnum (`relay/prompts/spec.md`).
- **Part**: a build step's role in its shape: `ford`, `easel`, `canvas`, `clay`, `kiln` or
  `magnum`.
- **Ladder**: a step's points, and at 9–11 its part, give its model and effort.
- **Coin**: a random pick between two choices, written as "by coin" in the line it decides,
  so the retro can compare them with nobody having chosen.

## Writing a piece

- Suggest a shape on the piece's `Shape:` line, with why. The designer may change it and say
  why.

## Working with the user

Talk about the game and the work: what a piece delivers, what it decides, what a proof will
tell us and when it's worth playing. The user knows how the crew runs, so bring its mechanics
in only when one changes the answer.

The user works in large chunks: bring the whole proposal, and expect corrections and
questions back.

Relay should stay small. Its prompts explain the flow and leave the shape of the work to each
session's judgment. When something goes wrong, look first for what could come out. The
user's feedback is a view to weigh, just as yours is: say where you disagree.

Open a piece and label it `relay:ready` only once the user has seen its whole body and
approved the proposal.
