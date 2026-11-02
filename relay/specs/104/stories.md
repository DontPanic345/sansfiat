# #104 build-3: the three stories

All three on **small seed 2, Red (`--civ 1`, civ 472) against Blue (civ 473, 13 steps away)**, each with fighting on and with `--no-fight`, then `judge FILE`. I tried medium seed 9 Gold (`--civ 3`, against Blue, 10 steps) first. There Blue's claim does not reach toward Gold: the first path tile in Blue's stand-in claim is the ring round its capital (step 9 of 10), so the corridor cannot overlap before the capital and a skirmish cannot be made short of marching there. Small seed 2 is the one map where the corridor overlaps Blue's held land (two tiles contested from +1), so the skirmish plays there; the other two run on it for one map and one comparison.

Commands (Build-5 re-runs them):

```
sansfiat story skirmish  --save /tmp/skirmish.json      # defaults: small seed 2, civ 1, 80 turns
sansfiat story stack     --save /tmp/stack.json         # 80 turns
sansfiat story surrender --save /tmp/surrender.json     # 60 turns
sansfiat story NAME --no-fight --save /tmp/NAME-nofight.json
sansfiat judge FILE                                     # the fight group beside the AI control
sansfiat story NAME --trace --save FILE                 # one line a turn: store, guard, claim, contested, fallen, taken, Standing, the army
```

## The measures (fight run / control)

Standing at the end (+80, surrender +60); the led civ is Red (472), the rival Blue (473). Difference is fight less control, in points.

| story | Red Standing | Blue Standing | Red diff | Blue diff | people fallen (Red / Blue) | tiles taken | armies surrendered | turns with a tile contested |
|---|---|---|---|---|---|---|---|---|
| skirmish | 8.5% / 7.8% | 8.7% / 8.9% | +0.7 | −0.2 | 1.16 / 0.32 (control 0) | 0 | 0 | 19 (control also reads 30: the tiles are contested, nobody fights) |
| stack | 5.3% / 5.8% | 8.7% / 8.4% | −0.5 | +0.3 | 0.76 / 0.41 | 0 | 0 | 6 |
| surrender | 3.9% / 5.2% | 9.2% / 8.6% | −1.3 | +0.6 | 0.53 / 0.44 | 0 | 0 | 2 |

`JUDGE.lead_margin` is 0.005 (half a point): fighting moves Red's Standing by more than that in all three (skirmish +0.7, stack −0.5 on the edge, surrender −1.3) and Blue's in two (stack +0.3 and surrender +0.6 are over; skirmish −0.2 is not). Fighting costs the attacking civ Standing in stack and surrender, and gives Red some in the skirmish, where it is the one that keeps a corridor of held land (the control's guard, free of fighting, loses nothing either, but Blue's guard falls to 0.8 and gives up its tiles toward Red).

## What happened

**skirmish.** Red disbands its starting army at +0 and claims the capital and a 13-tile corridor ending two tiles inside Blue's stand-in claim, kept every turn. Its guard (3, then 2 and 1.8 as the AI's rule trims it) holds 11–13 of the 13 tiles, and two of them are contested from +1: both guards lose a little each turn (Red 1.16, Blue 0.32 in all, over 19 turns of contest, then Blue's guard falls to 0.8 and holds 4 tiles, and it stops). No tile changes hands (guards never retreat and a guard's tile changes only when its people there are all lost). The store never fills three packs, so the army the script would raise and stage is not raised in 80 turns. It is the guards' skirmish.

**stack.** The army is disbanded at +0; the store banks to 14 at +16 (the guard is 2), and the story raises what it fills (three warriors) at +20 and stages them at 191, four steps out, where the line still carries. Its pack never fills (the store is 0 again: the guard and the pack's refill eat it), so it pushes after `WAIT` (10) turns, at +31, with the pack at 1.3. It is eleven steps out at +33, hungry three turns with its line cut, fights on the way (Red 0.46 fallen, Blue 0.28), and retreats to the stage with one warrior. It never reaches Blue's capital (13 steps), takes nothing and does not make the capital fall. What stops it: the packs are five turns and the way is thirteen steps, so the push starves on the way whatever the stage.

**surrender.** The starting army (3, both switches on) goes at +0 to the second tile of Blue's stand-in claim on the way. It is cut off at +2, out of pack at +4, then **pillages Blue's land and feeds itself**: its pack climbs from 0.6 to 2.4 and it is never hungry three turns in a row. It fights twice at the edge of Blue's land (Red 0.53 fallen, Blue 0.44), loses a warrior and falls back four steps to where Blue's land stops. No surrender. The finding: on rival land pillage feeds an army (`UNIT.pillage` 1.0 a unit of labour against 0.5 for forage), so an army past the guard on a rival's richer land does not go hungry, and `FIGHT.surrender` (hungry three turns) never fires there. A surrender needs an army that cannot pillage: on bare land, or with its labour spent on guarding a zone. That is for build-5's tuning (or a second script with the army deployed over a zone), not for this step to bend.

## Notes for the review

- The led civ's guard: every story gives `units::guard_order` through `act`, so the save replays. The guard still falls to 1–2 people, since the store is 0 at the pick and the levy brings 3–4 food a turn.
- Every store is 0 at the pick and stays under 14 for the first 40 turns on this map, so "every warrior the store fills" is 2–3, not nine.

## Final runs (build-5, glaze)

With `FIGHT.rate` 0.2, the one-step retreat and the widened `cut_by_rival`, the table is in `glaze.md`. Skirmish: Red +2.9, Blue −1.0. Stack: the push surrenders to Blue at +32 (Blue +2.1). Surrender: it fires at +20 (Red +0.6). No capital falls: every push runs out of food 9–10 of 13 steps out.
