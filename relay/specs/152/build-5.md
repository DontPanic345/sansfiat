# #152 build-5 (canvas): the limit, and a story that ends a game

The easels built the end: a civ starts with its scout alone (build-1), the game ends by
conquest, by the led civ's fall or at the limit (`core/src/over.rs`, `END.limit` in `data.rs`,
`Config.limit`, build-2), the judge reads the end on every run (`Scores.end`, the sweep plays
each run to its end, `judge FILE` prints the end beside its controls, build-3), and the page
shows it (build-4). Read `spec.md` (all of it: the question, the unsettled items, the
decisions), `probe.md` (the spec step's runs: no story takes a capital), `start.md` and
`judge.md` (the earlier builds' records), `cli/src/story.rs`, `cli/src/sweep_judge.rs`,
`core/src/units/fight.rs` (militia, entrenchment, the fall), `core/src/units/camp.rs` and
`line.rs` (supply), `docs/design/war.md`.

This step holds the piece's unsettled work. You may change the easels where the painting needs
it (say so in your summary); you may **not** change a fight, supply, economy or founding number
to make a story win (spec decision 12): the question is whether the game as built ends early.

## 1. Whether a story ends a game before the limit

Write a `conquest` story in `story.rs` (or rewrite `stack`: say which and why): the strongest
plan a determined player could script with the levers the player has (keep, gift, claim, the
guard, raise warriors or archers, march, the stage tile, Resupply and the switches, Forage and
Pillage, Zone), to take the nearest rival's capital and then the other's. The spec probe's
lesson: a stack that waits at its stage while the store is empty eats its pack; banking more
before it raises, or pushing as soon as the pack is full, may get more warriors to the capital.
Read `fight.rs` for what a capital's guard and militia need to lose.

Play it on each of the seven retro maps (`stage::RETRO`) with each civ led, to the end
(`--turns 0`), and the same with `--no-fight` as the control; and `stack`, `skirmish` and
`surrender` likewise on small 2 at least. For every save run `judge FILE --json`. Record in
`relay/specs/152/stories.md` a table: map, civ, story, how it ended and on which turn, capitals
fallen, the led civ's rank at the end beside the AI and idle controls', the largest army, the
farthest it got and the warriors left at a rival capital.

**What proves it:** some game ends before the limit (conquest or fell), counted by the
judge's `early`; the `--no-fight` control ends every game at the limit. If none does, the
table shows how near the best came and `stories.md` says why in a paragraph (what ran out:
food, warriors, the fight at the capital), which is the piece's answer: "not yet, on this
economy". Don't spend past an hour of the session on the search.

## 2. The limit per size

Set `END.limit` for small, medium and large. Two constraints, measured, and the method is
yours:

- **In time to matter.** On the seven retro maps all on the AI (`sweep --judge`, or a run of
  your own reading `World.standing`), played well past the limit: the leader at the limit is
  the leader the game settles on, and the spread is no longer moving by more than
  `JUDGE.lead_margin` a sample by then. Define "settles" in `limit.md` (for example: the
  leader after the last lead change, and its turn) and show the turn on each map. A limit far
  past that drags; one before it ends a game still in play.
- **Room for conquest.** The limit on a size is later than the turn any story ends a game or
  takes a capital on that size's maps, with a margin.

Record in `relay/specs/152/limit.md`: per map, the settle turn, the spread's last move, the
story's capital-taking turns, and the limit chosen per size with its reason; and the sweep's
wall time at the chosen limits beside main's 33 s. Put the reason in a `note` beside
`END.limit`.

## 3. Then

- `sansfiat sweep --judge` at the chosen limits: the end group's totals in `limit.md`.
- `cli/tests/story.rs` covers the new story (it saves, replays to its digest, and `judge`
  prints the end line).
- `node www/play/all.mjs` still passes at the chosen limits (a page script that runs to the
  limit uses a staged `limit`, not the size's).

## What the review reads

`stories.md` and `limit.md`, and your summary: the answer to "some games end before the limit
on the stories' maps", with the numbers and the control, and the limits with their reason.

## Done

`engine/check.sh` green; `stories.md` and `limit.md` written; `engine/CLAUDE.md`'s `story`
line names the new story if there is one.
