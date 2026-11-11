# #157 A rival that plays: progress

One whole session. Started 2026-11-10T23:28Z, cap 300 minutes.

## Plan

1. Baseline: `sweep --judge --json` on main, kept as `judge-main.json`. Done.
2. The planner (`engine/core/src/units/ai.rs`): one pass a civ a turn, giving the player's
   orders through a sink, so the AI (`World::order`) and the automatic led player
   (`script::Auto`, through `World::act`) share it. Done.
3. The judge's led games (`engine/cli/src/sweep_led.rs`): each civ led by the planner beside the
   idle control: place changes, lead changes, early ends, led over idle. Done.
4. Tuning against the judge: `canvas.md`. Done; the build's numbers are `judge.txt`, `judge.json`
   and `compare.txt` (against `judge-main.json`).
5. Tests: `engine/core/tests/planner.rs`; the tests the planner moved are updated. Done.
6. Save version 19 and its pins. Done.
7. Design ledger lines in `war.md`, `founding.md`, `vision.md`. Done.
8. Play: three sonnet players on small 2, medium 8, small 6 through `sansfiat play`, then
   `sansfiat judge` each; `play.md`. Running (games in `play/`).
9. `engine/check.sh`, the PR, CI, the verdict in `relay/runs/result.json`.

## If a second attempt picks this up

- The played games replay only on the rules they were played on: a change to a rule after
  commit b90064e needs the three games played again (`relay/players/player.md`, BINARY built
  from the branch).
