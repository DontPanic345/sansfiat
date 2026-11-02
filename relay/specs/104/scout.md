# #104 build-3: the scout's first errand

`cargo test --release -p engine --test scout_errand -- --ignored --nocapture` (`core/tests/scout_errand.rs`). On each of the seven retro maps the first civ is led and its scout is routed at the pick, with no other order, to the nearest rival capital (by steps), and separately to `Lead`'s target (a forest or hills tile seven steps out). 50 turns. The same file ran on main (`a8d8d3c`, in a worktree) and now.

## The table

| map | to | main: hungry turns / people starved / alive | now (`SCOUT.reach` 1): hungry turns / people starved / alive |
|---|---|---|---|
| small 2 | nearest rival capital | 48 / 2 / no | 33 / 2 / no |
| small 2 | Lead's target | 48 / 2 / no | 45 / 2 / no |
| small 3 | nearest rival capital | 48 / 2 / no | 46 / 2 / no |
| small 3 | Lead's target | 47 / 2 / no | 38 / 2 / no |
| small 6 | nearest rival capital | 48 / 2 / no | 16 / 0 / yes |
| small 6 | Lead's target | 48 / 2 / no | 47 / 2 / no |
| small 9 | nearest rival capital | 29 / 2 / no | 48 / 2 / no |
| small 9 | Lead's target | 47 / 2 / no | 42 / 2 / no |
| medium 8 | nearest rival capital | 48 / 2 / no | 47 / 2 / no |
| medium 8 | Lead's target | 45 / 2 / no | 47 / 2 / no |
| medium 9 | nearest rival capital | 45 / 2 / no | 44 / 2 / no |
| medium 9 | Lead's target | 0 / 0 / yes | 41 / 2 / no |
| large 8 | nearest rival capital | 40 / 2 / no | 40 / 2 / no |
| large 8 | Lead's target | 48 / 2 / no | 48 / 2 / no |

Main loses the scout on 13 of 14 errands. Now it still loses it on 13 of 14: a scout that lives is small 6 to its nearest rival's capital. **Done is not met.** Hungry turns count each turn after death too. One errand is worse than on main: medium 9 to `Lead`'s target kept its scout on main (0 hungry turns) and loses it now; there the scout walks seven steps out, past the line's reach (cost 12 carries 1.75 a turn, under the 2 it eats), and the labour it spends foraging and walking is the labour it no longer has for the line.

## Why, and what `SCOUT.reach` would need

Nothing about reach. Re-run with `SCOUT.reach` 2, 3 and 5 (everything else as above) one scout of 14 is alive each time, so more reach does not feed it. The causes, measured on small 2 (Red, 301):

- **The store is 0 and the camp is first in the queue.** At the pick every store is 0 and the levy brings 3–4 food a turn. The camp (id before the scout's) refills its pack to full from the store every turn, so the store is 0 when the scout's turn comes. The line carries nothing to the scout, whatever its cost.
- **The ground round a capital is bare, and a scout strips it.** The six tiles at reach 1 of small 2's capital hold 9 food at the pick; the clans and the camp take most of it within ten turns, and a scout foraging its need (2 a turn) has left them bare by +14. Past reach 1, a walk there and back costs more labour than the 0.5 food a unit of labour brings (the scout has 7 a turn), so reach 2 and 3 take tiles that cost more than the food they give.
- **On a long errand there is no line to live off.** Seven steps out the line costs 20 and carries a turn's labour of 1 food, under the 2 the scout eats; it must forage, and foraging is limited to what the tile holds and rests (`ceil(UNIT.fallow × taken / held)` turns).
- **Where the route is known, the scout never leaves the capital.** Small 2's route to Blue is plains all the way, so the scout has no tile to reveal and no trail to build: it stands at the capital (where the camp has taken the store) and starves there. Main did the same (it cleared the route and sat at the capital).

What would feed it: the scout's pack filled from the store before the camp's (the store is the only food at the capital), or a scout that walks to rich ground when it has nothing to search. Both are rules this piece did not ask for; they are for build-5 or the owner.

## What the scout is now

- `ScoutState` has `auto_forage`, `auto_supply`, the one-shot `forage`, `pillage` and `resupply` and `built`, written to the flat fields `UnitFlat` carries. `Switches`, `Forage`, `Pillage`, `Resupply` and `Disband` take a scout; `Raise { kind: scout, n }` makes one at the capital with its pack filled from the store (`ScoutRaised`).
- It feeds before it searches: its own tile and the tiles within `SCOUT.reach` (1) by richest, each walk there and back paid from its labour (a tile whose labour a food costs more than the line's is left while the store has food to bring); the line for what is still short; then it searches; then what labour is left refills its pack to full.
- The route stays set once its trails are built (`ScoutRouteFound` is said once when it is found with trails to build and once when they are built); while it is set, the tiles of its known path are `used` each turn.
- `strip` takes only the food it takes, for every unit: each stock drops in proportion, and the tile rests `ceil(UNIT.fallow × taken / held)` turns.
