# #157 A rival that plays: progress

One whole session. Started 2026-11-10T23:28Z, cap 300 minutes.

## Plan

1. Baseline: `sweep --judge --json` on main, kept as `judge-main.json`. Done.
2. The planner (`engine/core/src/units/ai.rs`): one pass a civ a turn, giving the player's
   orders through a sink, so the AI (`World::order`) and an automatic led player (`World::act`)
   share it. Needs in order: feed the armies, hold the land, grow Standing, hurt the nearest
   rival, take a capital it outweighs. The stand-in claim goes.
3. The judge: led games (each civ led by the planner through `act`) beside the idle control:
   rank changes, lead changes, early ends, led over idle.
4. Tune the planner and leaving against the judge (`canvas.md`).
5. Play three seeds through `sansfiat play` with a text-only player; `sansfiat judge` each.
6. Tests, design ledger lines, `check.sh`, PR, verdict.

## Done

- 1.
