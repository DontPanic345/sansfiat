# 354 Supply from any store: the scores

`sansfiat sweep --judge` on the seven retro maps, every civ on the AI, 200 turns past the pick.
`main-sweep.json` is main at ee70756 (after #353), the control; `branch-sweep.json` is this branch with
`--floor` (one null sweep); `compare.txt` is `judge --compare` of the two. A change is past the noise
floor where the table says so.

The Goal quotes 363 hungry of 1,243 army-turns outside the capital's ring; main's sweep run here gives
583 of 1,654 by the judge's count, and that is the control below. `army_far` is new in the judge: army-turns
outside the capital's own ring, so it reads as main's `army_outside` does.

## The question: hungry army-turns outside the capital's ring

| | main | branch | floor |
|---|---|---|---|
| army-turns outside the capital's ring | 1,654 | 1,531 | 144 |
| hungry among them | 583 (35%) | 522 (34%) | 71 |
| army-turns outside the ring (every store's) | 1,654 | 1,470 | 126, past it |
| hungry among them | 583 | 513 | 48, past it |
| army-turns inside the ring | 2,733 | 2,835 | 208 |
| hungry among them | 329 | 382 | 98 |
| warriors starved | 221 | 198 | 6, past it |
| armies lost to hunger | 5 | 2 | 1, past it |
| largest army | 4.0 | 11.9 | 1.5, past it |

Only one map had a settlement near a front while armies marched, small 2:

| small 2 | main | branch |
|---|---|---|
| army-turns outside the capital's ring | 357 | 292 |
| hungry among them | 115 (32%) | 77 (26%) |
| of them inside a settlement's ring | 0 | 61, 9 hungry |
| settlements founded, taken | 2, 0 | 3, 18 |

On the other six maps no army stood in a settlement's ring: 4 of 21 civs kept a settlement to +200
(7 on main), so their numbers moved only as a different game moves (raising from a market's clans
changes every game from its first army).

## Games that end before the limit

| | main | branch | floor |
|---|---|---|---|
| capitals fallen | 2 | 3 | 1 |
| games ended early (AI) | 0 of 7 | 0 of 7 | 0 |
| led games ended early | 3 | 4 | 1 |
| mean turns | 228.6 | 228.6 | 0 |

No AI game ends before the limit on either: a game of three goes on after one capital falls.

## Standing and motion

| | main | branch | floor |
|---|---|---|---|
| aligned at +200 | 44.7% | 40.7% | 8.5 |
| top civ at +200 | 23.2% | 22.8% | 1.2 |
| lead changes | 10 | 11 | 0, past it |
| land changed hands | 466 | 706 | 71, past it |
| clans changed civ | 258 | 368 | 45, past it |
| settlements taken | 0 | 18 | 17, past it |
| settled civs' Standing at +200 | 15.7% (5 civs) | 8.4% (3 civs) | 12.0 |
| the same civs with signposts off | 18.2% | 8.9% | 10.9 |

## Read plainly

- The rule works where there is a settlement to draw on: on small 2 hungry turns far from the capital
  fall from 32% to 26%, 61 army-turns are fed from a settlement's ring, and a third capital falls.
- Over the seven maps the question barely moves (35% to 34%, inside the floor): settlements are too few
  and too rarely between rivals. Fewer warriors starve and fewer armies are lost, past the floor.
- A settlement is now worth taking and is taken back and forth (18 times on small 2). The civs that
  settled no longer trail their no-signposts control by 2.5 points (8.4% against 8.9%), but three civs
  is too few to call it.
- No tuning was done: `PLAN.march`, `PLAN.reach` and the store numbers are main's.
