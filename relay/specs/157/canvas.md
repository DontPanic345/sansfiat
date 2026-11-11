# #157 canvas: the planner's tuning

Every row is `sansfiat sweep --judge` on the seven retro maps (small 2, 3, 6, 9; medium 8, 9;
large 8). Watched is every civ on the planner; led is each of the 21 civs led by the planner
through `act` (`script::Auto`), beside the same civ led and given no order (idle).

| build | lead changes (watched) | AI orders /100 | clans left | people fallen | tiles taken | warriors sent home hungry | led place changed | idle games fell | led over idle |
|---|---|---|---|---|---|---|---|---|---|
| main (stand-in claim, guard only) | 4 | 12.7 | 2 | 0 | 0 | 0 | - | - | - |
| planner, home only when hungry, march 3 | 7 | 52.5 | 4 | 41.5 | 13 | 136 | 18 of 21 | 2 | +0.060 |
| the same, march 4 | 7 | 53.2 | 4 | 44.7 | 13 | 131 | 18 of 21 | 2 | +0.060 |
| home at a pack under 2 turns on bare land; leaving 3 turns at 0.05 | 5 | 58.2 | 2 | 20.3 | 5 | 12 | 16 of 21 | 0 | +0.039 |
| home at a pack under 1 turn (the build) | 8 | 60.1 | 8 | 36.0 | 17 | 90 | 16 of 21 | 1 | +0.050 |

The last row's full report is `judge.txt` and `judge.json`; main's is `judge-main.json`.

## What the runs showed

- **The levy is not the income.** A store near its cap takes only the room left, so `Civ.levied`
  reads 14 where the clans would pay 60. The planner counts a store three quarters full as fed
  (`PLAN.full`), else its army never grows past what a full store's trickle shows.
- **Feed 1.25 never fills a pack.** Stores and packs spoil a tenth a turn, so a levy a quarter over
  the mouths settles at two turns of food in all: an army waited for its pack for ever. At 1.5 the
  pack reaches four fifths (`PLAN.packed`).
- **A stack can't cross the country.** On small 2 the capitals are 15 to 19 steps apart. Red sent 14
  warriors at Gold's capital, which it outweighed, six times in 200 turns: each time the pack ran
  out a step short and the army walked home losing a warrior a turn. Marching leaves no labour to
  forage, and a wild tile feeds about four people. So the army is a stack only where a rival's
  capital is within `PLAN.march` turns' walk (or a rival army is on its claim), and a party of
  `PLAN.party` elsewhere, which lives off the land a tile at a time.
- **A pressed guard is not a thin guard.** A rival army on a capital draws the whole guard back, so
  the rest of the claim reads as paper; raising a guard for it took Red's guard from 10 to 30 in 20
  turns and left nothing for an army. The planner raises no guard while pressed, and sends a guard
  home when the levy feeds no more and every tile has two floors of guard (`PLAN.thick`).
- **Coming home.** Home only when hungry: parties reach rivals 15 steps off, take 13 tiles and two
  idle capitals, and lose 136 warriors to hunger on the walk back (they rejoin their clans; nobody
  dies). Home at two turns of pack: 12 lost, but 5 tiles and no capital. One turn is between.
- **Leaving.** With the planner playing, leaving at 5 turns drawn and 0.1 leaning left 2 to 4 clans
  over the seven maps (main: 2). At 3 turns and 0.05 it is 8. A civ's clan leaves where it lives on
  a rival's held land or comes to a rival's festival; at the end of small 2 and small 9 no civ's
  clan had any leaning to a rival, so the run and the line are not what holds leaving down: contact is.

## What "in doubt" measures on one map

A led game is in doubt when the led civ's place in Standing changes after the pick, a rival
passing it or being passed only when ahead by more than half a point (`JUDGE.lead_margin`, the
lead's own margin), or when the game ends before the limit. The judge counts both for the planner
as the player and for the idle player, per civ, and the Standing the planner ends with over its
idle self.

## Games that end

No game the planner leads ends before the limit (0 of 21), and no watched game does; one idle led
civ falls (small 3, Gold, at +156). A longer march (7 turns), a longer siege (12) and a thinner
remnant (0.2) together move nothing toward it: 0 of 21 still, and the idle fall goes too, with the
gain over idle at +0.045. A capital behind the planner's guard (8 to 26 people, always entrenched)
and its militia outweighs a quarter of a rival's people after a walk of 15 steps or more, so the
planner never marches on one. Ending games needs something the planner's numbers don't hold: a
capital that is easier to take, or an army that eats on the march.
