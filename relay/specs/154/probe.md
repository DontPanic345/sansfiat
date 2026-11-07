# #154 probe: the day's scale and the per-person walk on main

Run by the spec step on the branch's base (270d0d3), `sansfiat sweep --judge --json --floor`, the
seven retro maps, every civ on the AI. Each column is one sweep's totals. The patches are throwaway
(`probe-day.diff` is unit2, `probe-walk.diff` the walk with its `LOAD` and `PERK` knobs); none is
on the branch.

| Column | What it ran |
|---|---|
| main | The base. |
| null, null2 | The base with `CLAN.work` × (1 + 10⁻¹²), and × (1 − 10⁻¹²): a last-bit null. |
| unit | The day's scale (`build-2.md`'s list): costs, hidden costs, crossing, trail work, labour and labour worths × 4/3, forage and pillage × 3/4. |
| unit2 | unit, with the path searches (`cost_map_by`, `path_by`, `Walks::new`) comparing costs in twelfths of a day and `stride` taking a step within 10⁻⁹. |
| pp1 | The per-person walk, naive: a territory tile's walk × people ÷ tiles; a festival route × max(1, brought ÷ 1 food). |
| route, terr | pp1's route half alone, and its territory half alone. |
| route2 | The route half, a load of 2 food. |
| terr2 | The territory half at half weight (× people ÷ tiles ÷ 2, the same as `CLAN.inside` 0.125). |
| both2 | terr2 and route2 together. |
| b23 | terr2 and a load of 3. Identical to terr2: no clan brings 3 food (`MARKET.carry` 0.2 a person), so every route has one walker. |
| b152 | The territory at ÷ 1.5 and a load of 2. |

| metric | main | null | null2 | unit | unit2 | pp1 | route | terr | route2 | terr2 | both2 | b152 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| standing.aligned_200 | 0.2556 | 0.2727 | 0.2441 | 0.2318 | 0.2431 | 0.2469 | 0.2322 | 0.2931 | 0.241 | 0.2517 | 0.2489 | 0.2196 |
| decisions.ai_orders_per_100 | 8.34 | 8.821 | 6.422 | 5.852 | 7.503 | 5.4 | 4.595 | 7.549 | 8.867 | 6.417 | 3.36 | 6.867 |
| motion.clans_joined | 131 | 141 | 106 | 105 | 99 | 78 | 82 | 147 | 95 | 126 | 116 | 99 |
| motion.people_changed | 660.6 | 648.2 | 590.2 | 514.5 | 481.7 | 377.7 | 443.6 | 654.6 | 455.4 | 637.1 | 572.6 | 488.8 |
| motion.land_changed | 1591 | 1694 | 1264 | 1221 | 1252 | 1317 | 806 | 1873 | 1650 | 1432 | 937 | 1451 |
| land.markets_pick | 211 | 204 | 212 | 212 | 211 | 199 | 213 | 195 | 210 | 214 | 210 | 205 |
| land.trading_pick | 0.9292 | 0.9397 | 0.9442 | 0.9421 | 0.9369 | 0.9289 | 0.9024 | 0.9304 | 0.9229 | 0.9325 | 0.9225 | 0.9346 |
| land.trading_200 | 0.9274 | 0.9358 | 0.9474 | 0.9472 | 0.9447 | 0.9305 | 0.903 | 0.9346 | 0.9262 | 0.928 | 0.9214 | 0.9397 |
| land.held_200 | 26.19 | 26.1 | 23.57 | 27.9 | 25.19 | 21.14 | 25.76 | 30.76 | 23.95 | 28.48 | 29.71 | 23.43 |
| economy.levy_per_turn | 7.209 | 7.231 | 6.768 | 8.461 | 6.625 | 6.893 | 7.085 | 8.031 | 6.952 | 7.953 | 7.85 | 7.22 |
| health.pregame_by_centres | 7 | 7 | 7 | 7 | 6 | 6 | 7 | 6 | 7 | 7 | 7 | 6 |

## What it says

- **The floor doesn't hold the pre-game.** The sweep's `--floor` comes from one null that
  differs at the pick. A 10⁻¹² nudge to `CLAN.work` flags 20 of 63 metrics against main
  (`judge --compare main null`), clans joined 131 → 141 and markets at the pick 211 → 204; the
  other nudge, null2, takes clans joined to 106, AI orders to 6.4 and land changed to 1264. The
  nulls alone span clans joined 106–141, AI orders 6.4–8.8, land changed 1264–1694. Any change
  to a float moves the pre-game this much. So the day's scale can't show "no move past
  the floor" (spec decision 1), and every rule here is read against last-bit nulls.
- **The day's scale moves the maps as a null does.** unit and unit2 flag 30 and 25 metrics,
  in both directions; their totals sit in or near the nulls' span (unit2's clans joined 99
  against the nulls' 106–141, its AI orders 7.5 inside it). unit2 ended one pre-game by its backstop; so did terr
  and pp1. Reach is proved on a stage (`build-2.md`, `day.rs`), not here.
- **The naive per-person walk collapses.** pp1: clans joined 78, people changed 378, held land
  21, one pre-game by its backstop. The route half alone at a load of 1 drops trading at the pick
  to 0.902, the lowest of every run, and land changed to 806.
- **Weighted, it holds the pre-game and the festivals**: terr2 and route2 each end 7 of 7 by
  centres with markets and trading at main's and the nulls' level, and pass every bound of
  `build-4.md` taken alone. Together (both2) clans joined is 116 but AI orders fall to 3.4 and
  land changed to 937, under the bounds (half main's orders, 4.2; three quarters of the lowest
  land changed, 948); whether that is chaos or the walk is the canvas's to tell. b152 (a
  heavier territory walk) ends one pre-game by its backstop.
