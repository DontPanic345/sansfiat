# 353 judge: not shown (leans worse)

Sweeps: `branch-sweep.json` against `main-sweep.json`, `compare.txt` (no floor in either report, so nothing is flagged). The engine's rules have not changed since those sweeps: later commits touched only the page, `stage.rs`, the text view and one comment in `data.rs`.

| score | main | branch | floor |
|---|---|---|---|
| Standing: settled civs (5 of 21) at +200 against the same civs with no signposts | 18.2% | 15.7% | not run |
| Standing: all civs at +200 | 15.3% | 14.9% | not run |
| doubt: lead changes (7 maps) | 6 | 10 | not run |
| doubt: spread at +200 | 0.179 | 0.172 | not run |
| decisions: AI orders per 100 | 75.1 | 75.4 | not run |
| motion: people changed | 1548 | 1473 | not run |
| health: panics, digest mismatches | 0, 0 | 0, 0 | in band |

- Question: 7 settlements on 5 of 7 maps; 7 civs up on their control, 7 down, 7 the same. The mechanism fired (daughters walked to signposts, markets held clans, stores filled to the cap).
- Led play (small 2): 4.9% against the 4.9% idle control. The one signpost drew nobody: other clans settled beside it and its tile fed 0%; nothing told the player.
- Not met: a move past the floor on more than one map (the floor was not run, and the settled civs' move is down), and a led player beating idle by the floor.
