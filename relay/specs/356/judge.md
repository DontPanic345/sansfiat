# #356 Exchange: the judge's verdict — not shown

From `sweep-main.json` (main at 40656df) and `sweep-rework.json` (the branch at bcc5791, main merged
in), no `--floor`; full table: `sansfiat judge --compare relay/specs/356/sweep-main.json
relay/specs/356/sweep-rework.json`. Floors are the last `--floor` run's (`sweep-final.json`).

| score | main / control | branch | floor | |
|---|---|---|---|---|
| army-turns fed by a line ending at a market (`line_at_market`) | 0 | 0 | | not moved, on all 7 maps |
| army-turns paid and trading at a market (`unit_turns`) | 0 | 52 | | the lever fires: 6.3 food bought, of 152 needed |
| Standing aligned at +200 | .5216 | .5192 | | drift |
| lead changes / swings | 7 / 14 | 8 / 11 | | drift |
| AI orders per 100 | 85.7 | 86.7 | | drift |
| clans changed / land changed | 410 / 1024 | 444 / 1131 | | up; no mechanism of this piece's moves them |
| starving share | .0328 | .0347 | .0014 | just past; not a goal |
| led beats idle | 16 | 15 | 3 | within |
| panics / digest mismatches | 0 / 0 | 0 / 0 | | health in band |

The question doesn't move: no army-turn on the retro maps is fed from a stall. The chain fires (52
army-turns paid, buying 6.3 food) but a civ's store holds almost no salt (salt in a civ's store at
+200: 0 on main, 1 on the branch), so pay buys crumbs. The motion and doubt moves are what any change to
the world's dice gives; nothing in this piece reaches clans or land.

The play note (`play.md`) predates the rework: it played the Fill order, which is gone. Its finding
stands in kind: the store held no salt.
