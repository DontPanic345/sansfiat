# #337 Scouts that finish, and can die: progress

## What the whole step found

- On small 61288 the led scout's route to Gold's capital stood unfinished because the scout spent
  most turns foraging: it left its route for rich ground, came back, and left again, and past
  Blue's land it had no line.
- On today's `main` that game no longer runs 200 turns: led Red, given only the route order, loses
  its capital to Blue at +4 (land without a guard, #336). The retro's control is `--no-fight`, or a
  player who keeps an army on the capital.

## The rule as built (commit "a scout eats from its pack")

- A scout's food is its pack. It holds `SCOUT.pack` (24) turns of its eating and does not spoil.
- Standing on a store of its civ it fills: a full pack when it has an errand, else two turns of food.
- Away from a store, when the pack holds no more than the walk to the nearest store plus
  `SCOUT.spare` (2) turns, it walks there (`homing`), fills, and walks back to the tile it left
  (`back`). It sets out with at least `SCOUT.out` (half) of a pack; an errand further than a full
  pack takes it and back keeps it at the store, and it says so (`Doing::TooFar`, `ScoutBlocked`).
- A built route, or a destination reached, is no errand: the scout stands, and walks home when low.
- No foraging, no line, no switches for a scout: `Switches`, `Resupply`, `Forage`, `Pillage` refuse
  a scout with the reason.
- `units::shut`: every tile a rival army holds or stands on. A scout on one stays (`Doing::Trapped`);
  its walks and its search path round them.
- `scout::kill`, after the fight in the `units` step: a rival army standing on a scout's tile kills
  it (`ScoutKilled`). Off with `Config.fight`.
- `ScoutState.doing` (`Doing`, not saved) and `units::doing_text` are the view's and the panel's words.
- The text view's scout line: what it is doing, route, pack of cap in turns, walk to its store.

## Measured so far (small 61288, Red led, `--no-fight`, one route order 735 to 109)

| pack | route built |
|---|---|
| main (forages) | never by +200 |
| 12 turns, spoiling | never: waits at the capital, too far |
| 16, no spoil | never: too far |
| 20, no spoil | +97 |
| 24, no spoil | +67 |

## Done

- [x] Core rule, view line, `core/tests/scout_feed.rs` (one trip and done by turn 10; killed;
  trapped until the zone lifts; walks round a zone).

## Left: `steps.json`

- build-1: the rest of the suite green, the save version, the pass's stage test, the page.
- build-2: the retro maps: errands, tuning, sweeps, the led game on 61288.
- play (the crew adds it), review.
