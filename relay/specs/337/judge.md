# 337 judge: not shown (leans worse)

Sweeps: `branch-sweep.json` against `main-sweep.json`, `compare.txt` (floors in both). The engine has not changed since the
sweeps (`7d0852b` is the last engine commit); the review changed only the design ledgers.

| score | main | branch | floor |
|---|---|---|---|
| Standing: aligned share at +200 | 0.407 | 0.456 | 0.085 (inside) |
| Standing: led civ's own | 0.134 | 0.126 | 0.004 (past, down; the idle led civ fell as far: 0.093 to 0.086) |
| doubt: lead changes / spread at +200 | 11 / 0.182 | 10 / 0.173 | 0 / 0.005 (both past, small) |
| decisions: AI orders per 100 | 75.7 | 71.5 | 2.7 (past, down) |
| motion: land changed / people changed | 706 / 2137 | 481 / 1542 | 71 / 364 (past, down) |
| health: panics, digest mismatches | 0, 0 | 0, 0 | in band |
| led over the idle led civ | 0.041 | 0.040 | 0.011 (holds, 16 of 21 ahead both) |

- Question (does a led scout's errand finish?): yes, in the piece's own count: 27 of 42 routes between the retro capitals built
  by +200 against 8 on main (fight on 19 against 8); no scout starves (main: 33 people); 9 are killed by a rival army on their
  tile. On small 61288 the route is built at +52 (main: never). The mechanism fired. This is the errand evidence
  (`errand.md`), not one of the four scores.
- Not met: no score moved past its floor in a good direction on more than one map. What moved past the floor is a cost: the
  AI's scouts eat from the store (about 3.5 food lower on average), raises fall (7.39 to 6.86 per 100 orders), camps lost go
  from 2 to 11 (6 on medium 9), the largest army from 11.9 to 4.0 (one game on main), the led civ's Standing and wins (8 to 5)
  fall; the idle civ falls with it, so the sweep does not say the scout did it, and no run switches the scout's eating off.
- The crossing: on 61288 an army marches the route 17 tiles and disbands hungry on the far side, and a signpost past the range
  draws no clan (the far tiles feed nobody; the nearest home is past `SETTLE.reach`). The pass is proved on a built map
  (`core/tests/pass.rs`), not on a retro map.
- Play (`play.md`, small 2): the route built in 21 turns, an army along it took two capitals, Standing 18.0 to 29.0 against 35.7
  on the AI and 27.1 idle: the led civ beat idle, not the control.

Rules in this piece, for the owner: the scout never forages, idle or on an errand, and has no switches (the design's
`scouting.md` and `supply.md` prose says it forages and has both); pack 36 turns, not spoiling, spare 3, out 0.25
(`data.rs` `SCOUT`); an errand further than a full pack there and back waits at the store (`Doing::TooFar`); a scout
keeps the passes it built on its way home walked (`scout::keep_way`), so a trail it uses stays open.
