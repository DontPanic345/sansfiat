# #102 build-2: the starting numbers, what they did

Measured on build-2's rules, watched from the pick for 200 turns on the seven retro maps, with every civ on
the AI (`claims: all`). Held is `claim::held` (the capital and each claimed tile whose
guard share reaches `CLAIM.floor`); claim is `Civ.claim`; guard is the guard's people; left and arrived
are `ClanLeft` events a civ lost and gained; taken is `ClaimTaken` events it lost.

| Map | Civ | Claim | Held | Guard | Left | Arrived | Taken |
|---|---|---|---|---|---|---|---|
| small 2 | Red | 17 | 2 | 0 | 0 | 0 | 0 |
| small 2 | Blue | 18 | 1 | 0 | 0 | 0 | 0 |
| small 2 | Gold | 27 | 15 | 3 | 0 | 0 | 0 |
| small 3 | Red | 25 | 4 | 0 | 0 | 0 | 0 |
| small 3 | Blue | 9 | 2 | 0 | 0 | 0 | 0 |
| small 3 | Gold | 28 | 1 | 0 | 0 | 0 | 0 |
| small 6 | Red | 34 | 1 | 0 | 0 | 0 | 0 |
| small 6 | Blue | 30 | 3 | 1 | 0 | 0 | 0 |
| small 6 | Gold | 20 | 2 | 0 | 0 | 0 | 0 |
| small 9 | Red | 28 | 1 | 0 | 0 | 0 | 0 |
| small 9 | Blue | 26 | 6 | 2 | 0 | 0 | 0 |
| small 9 | Gold | 16 | 16 | 4 | 0 | 0 | 0 |
| medium 8 | Red | 17 | 2 | 0 | 0 | 0 | 0 |
| medium 8 | Blue | 25 | 1 | 0 | 0 | 0 | 0 |
| medium 8 | Gold | 24 | 2 | 1 | 0 | 0 | 0 |
| medium 9 | Red | 21 | 15 | 4 | 0 | 0 | 0 |
| medium 9 | Blue | 16 | 2 | 0 | 0 | 0 | 0 |
| medium 9 | Gold | 18 | 3 | 1 | 0 | 0 | 0 |
| large 8 | Red | 15 | 15 | 4 | 0 | 0 | 0 |
| large 8 | Blue | 15 | 14 | 3 | 0 | 0 | 0 |
| large 8 | Gold | 40 | 11 | 4 | 0 | 0 | 0 |

Totals at +200 over the 21 civs: 5 civs hold nothing past the capital (held 1), 11 have no guard, and no
civ lost a clan (`ClanLeft` 0). Every map has at least one civ holding land past its capital, so the
brief's "nothing past the capital on most maps" check does not trigger.

Judge report (`build-2-judge.txt`, `sweep --judge`): pre-games 7 of 7 by centres, no panics, no digest
mismatches, people against the `--no-army` control 0.976, camps kept 14 of 21 (build-1: 15 of 21),
warriors starved 16 (build-1: 12), camps lost 2 (build-1: 1). Joins: 59 clans changed civ.

Control (`build-2-noclaims.txt`, `sweep --after 200 --maps retro --no-claims`): it runs; civs join 33 and
no civ holds land beyond its capital.

## What the numbers say

- The guard is the cost, and food is its limit. A founding guard of three eats three food a turn from
  the civ's store. The store at the pick is empty (the camp's and scout's packs take the endowment), and
  after build-1's festivals it takes in about one food a turn at the start (small seed 2: 1.1 at turn 1,
  none at turns 2–3). The guard starves in about five turns, and the claim falls to what the guard held
  at its last full turn. The spec-review's "the store takes in 5–9 food a turn, so the guard is fed" no
  longer holds (see spec item 24).
- Raising `GUARD.guarding` would not feed a guard, so it is not the lever the brief's bar points at. The
  levers are food: the levy, the guard's `keep_turns`, or the guard's size. These are the canvas's.
- Clans leave on none of the seven maps by +200. A civ clan leaves only for a rival's land it is drawn to
  for `LEANING.leave_run` turns at `LEANING.leave_at` or more; the held land of the rivals is small, so
  this is rare. The canvas measures it (`clans_left`).

## Decided in this build

- The guard is raised last at founding, after the camp and the scout, so their draws are unchanged and it
  takes what the clans have left. Test worlds' clans are 14 people, as founding takes eight.
- A claim may not cover another civ's capital. `tile_civ` gives a capital to its own civ before any claim.
- The AI's stand-in drops rival capitals.
