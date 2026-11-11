# #157 play: three led games against the planner

Three sonnet players, each knowing only `relay/players/player.md` and the text view, led civ 1 for
100 turns from the pick through `sansfiat play`. The games and their lines are in `play/`; each is
scored with `sansfiat judge`. The rivals are on the planner.

| map | player at +100 | the planner in the same seat | the same civ idle | lead changes (game / all on the AI) | player's orders | longest run with none |
|---|---|---|---|---|---|---|
| small 2, Red | 26.4%, rank 1 | 29.0%, rank 1 | 22.6%, rank 2 | 4 / 0 | 19 | 35 turns |
| small 6, Red | 13.9%, rank 1 | 15.9%, rank 1 | 11.6%, rank 2 | 4 / 2 | 9 | 79 turns |
| medium 8, Red | 10.0%, rank 1 | 10.8%, rank 1 | 7.4%, rank 2 | 1 / 1 | 10 | 74 turns |

## What the games showed

- **The player loses to the planner in its own seat, in all three.** The planner leading the same
  civ ends 0.8 to 2.6 points of Standing above the player. Its claim is fully held behind a guard of
  11 to 26 people where the players kept 4 or 5, and on small 2 it took 3 tiles and drew 4 clans away
  where the player took none.
- **The player beats its idle self in all three**, by 2.3 to 3.8 points, and idle falls to rank 2
  each time.
- **The player never lost first place at the end.** All three led the civ that led at the pick. On
  small 2 Blue passed Red twice (the lead changed 4 times, and Blue ended 0.3 points behind); on
  small 6 Blue led at +10 and +30. On medium 8 the lead changed once, early. No game ended before the
  limit, and no rival army came onto a player's land.
- **So the games were in doubt on two maps of three, and lost on none.** "The play step's player
  loses some" holds against the planner in the same seat, and does not hold as a lost game.

## Findings

- **no result**, small 2 and small 6 and medium 8: a gift of 20 to 30 "gave 0.0" for long runs with
  grain in the store. Each player's last note says they could not tell whom a gift reaches. `why` on
  the civ told one of them (small 2) that it goes only to clans at the market that aren't its own.
- **no result**, all three: the scout's route. "scout still at 506 after route order" (small 2);
  "no route can exist" (small 6); "its route to 1062 never advanced" (medium 8).
- **no choice**, all three: pillage was never among the legal orders, since no army stood on a
  rival's land; an army 15 steps from a rival has nothing to do, and each player disbanded it or
  left it standing.
- **reason**, small 2 and medium 8: the view gives the claim's size and its thinnest tile, not its
  tiles, so no player repainted the claim; the planner widened its own.
- **idle**, small 6 turns +24 to +103 and medium 8 from +30: the player ended ten turns at a time
  with nothing to decide. Standing moved under half a point.

None of these is the planner's; each is in the view or the levers of earlier pieces, and they go to
the owner as ideas.
