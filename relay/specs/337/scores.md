# #337 Scouts that finish, and can die: the scores

`SCOUT` is pack 36, spare 3, out 0.25 (errands: `errand.md`). `main-sweep.json` is main at 4fea294 (`relay/specs/354/branch-sweep.json`,
the engine has not changed since), `branch-sweep.json` is `sansfiat sweep --judge --json` on this branch, `compare.txt` is
`sansfiat judge --compare main-sweep.json branch-sweep.json`. A change is past the noise floor where `compare.txt` stars it.

## The question: does a scout finish its errand, and can it die

| | main | branch |
|---|---|---|
| routes built by +200, 42 errands between the retro maps' capitals, fight off | 8 | 27 |
| the same, fight on | 8 | 19 |
| scouts starved (people), fight off | 33 | 6 |
| scouts killed by a rival army, fight on | not a rule | 7 |
| led game, small 61288 Red, the one route order, fight off: route built | never; the scout stands at (4, -6) at +200, 6 tiles to search | +52 |

The 6 people starved on the branch are the same three medium 8 scouts every setting loses (`errand.md`).

## The sweep: what the scout's eating moved

In the sweep every civ is on the AI, and no AI scout leaves its ring (`supply.counts.scout_outside` 1,247 on main, 0 on the
branch; `scout_inside` 1,886 to 4,410, as the scouts that starved or foraged away on main now stand at their store and
live). So `SCOUT`'s three numbers do not move the sweep: the same sweep at pack 24, spare 2, out 0.5 gives the same
`standing`, `economy`, `armies`, `decisions` and `supply` lines to the digit. What moved it is that each AI scout eats from
its civ's store, about 2 food a turn, where it foraged for itself.

| score | main | branch | floor |
|---|---|---|---|
| Standing, aligned share at +200 | 0.407 | 0.456 | 0.085 |
| Standing, top at +200 | 0.228 | 0.234 | 0.012 |
| store, mean over the maps | 37.5 | 34.0 | 1.1, past |
| turns at pack | 501 | 482 | 13, past |
| levy a turn | 9.18 | 9.85 | 0.26, past |
| AI orders per 100 turns | 75.7 | 71.5 | 2.7, past |
| of them `raise` | 7.39 | 6.86 | 0.14, past |
| of them `forage` | 8.53 | 7.52 | 0.99, past |
| largest army | 11.9 | 4.0 | 1.5, past |
| camps lost | 2 | 11 | 1, past |
| warriors starved | 198 | 209 | 6, past |
| army-turns outside the ring | 1,531 | 1,278 | 144, past |
| claims taken | 38 | 21 | 10, past |
| land changed | 706 | 481 | 71, past |
| clans changed | 368 | 274 | 45, past |
| led civ's Standing | 0.134 | 0.125 | 0.004, past |
| led wins of 21 | 8 | 5 | 2, past |

By map (`economy.store_mean`, main to branch): small 2 35.1 to 37.4, small 3 36.4 to 28.2, small 6 45.5 to 44.0,
small 9 40.6 to 35.0, medium 8 47.0 to 45.0, medium 9 23.3 to 17.0, large 8 34.6 to 31.3. `raise` per 100 orders: small 2
13.5 to 11.0, small 3 9.0 to 7.7, small 6 9.5 to 7.3, medium 9 5.3 to 6.5. Camps lost: medium 9 0 to 6, small 6 0 to 2, small 9
1 to 2. The largest army's 11.9 on main is one game, small 2; the other six maps' largest is 4.0 on both, and small 2's is 4.0 on
the branch.

- **Stores.** Six of the seven maps end with a store 1.5 to 8 food lower (small 3 by 8.2, medium 9 by 6.3, small 9 by 5.6).
  The scout takes 2 food a turn from a civ whose levy is about 10, and a poor map shows it most (medium 9: store 17 against 23).
- **Armies raised.** The planner raises less on three maps (small 2, 3 and 6; `raise` 7.39 to 6.86 an hundred orders over
  all) and loses 9 more camps (medium 9: 6 of them). The store is the likely cause (an army is raised from what the
  store holds, and the scout has had its food first), but I did not isolate it: no run here switches the scout's eating off.
- **Standing.** The aligned share and the top share both rise inside their floors (+0.049 against 0.085, +0.006 against
  0.012): no Standing moved past the noise. The led civ's own Standing is 0.009 lower, past its floor (0.004), and its wins fall
  from 8 to 5, but the led civs there are idle; the sweep does not say the scout did it.
- This is the piece's main risk, and it is the store's cost, not a pack's: an AI scout that never leaves home still eats
  2 a turn. A scout that waits for its next errand could eat from the store only when it has one; that is a rule, and the
  owner's (an idea below).

## The led game: small 61288, Red, `{"type":"route","unit":232,"from":735,"to":109}`

Lines files: `game/`. The script is `play --game F --seed 61288 --size small --civ 1` and `end 1` a turn.

| game | main | branch |
|---|---|---|
| `--no-fight`, the retro's control | route never built; the scout stands at 528 (4, -6) at +200, 6 tiles to search | built at +52; the scout stands at its capital at +200 and eats from it (pack 2 of 72, its route stays set) |
| fight on, one route order alone | Red's capital falls to Blue at +4 | the same, +4 |
| fight on, raise 2 warriors at +0 and again when none stands on the capital | the capital falls to Blue at +12 | the capital holds to +200; the scout is killed by Blue's army at (4, -7) at +40; no route |

`game/main-no-fight.lines.jsonl`, `game/branch-no-fight.lines.jsonl`, `game/main-fight-army.lines.jsonl`,
`game/branch-fight-army.lines.jsonl`. The fight game on the branch keeps the capital (an army of 1 warrior, entrenched 200
turns) and loses the scout to a rival army: the kill rule is built as the piece says. The scout's route runs through
Blue's land (its capital at (3, -9) and a settlement at (-2, -7)), where a rival army shuts the tiles.

### The route built, an army along it and a signpost past the range (branch, `--no-fight`)

At +52 the route is built; raise 2 warriors at +53 (store 49), `move` to 254 (-5, 1) the far end, 17 tiles out, and post a
signpost at 254. `game/branch-cross-2-warriors.lines.jsonl`; 4 warriors, `game/branch-cross-4-warriors.lines.jsonl`; 2
warriors to 348 (-2, 5), `game/branch-cross-near.lines.jsonl`.

- The army crosses: it is cut off at (6, 6) at +54, hungry at (-2, 8) at +56 and at (-5, 1) at +59, six turns for 17 tiles.
  It is cut off from its capital past (6, 6) ("carries 0.6 food a turn" on a 46-day line) and its pack (4 turns) is spent by +58: it goes hungry
  at (-2, 8), one warrior disbands from hunger at (-2, 1) at +58, the other at (-5, 1) at +59. With 4 warriors the same:
  they reach (-5, 1) and disband from hunger there. The army dies on the far side. The route's trails make the walk, not the
  food.
- No clan settles. The signpost at 254 is worth 0% to a daughter (and 283 and 348 the same), and has been since +0: on 61288
  the tiles along the route feed no one (254 is hills). The nearest clan home is 18.3 days away, past `SETTLE.reach`
  (14 days); 348 is 11.5 days away and still worth 0%. No clan settled at the signpost by +200 in any of the three games.

## Open idea: SETTLE.reach and the draw's walk

Counted at +0 and +50 on the seven maps and 61288, AI civs, fight off: tiles that would feed a daughter
(`settle::worth` at least `SETTLE.fed`) with no clan on them, walk from a civ's clan homes by `cost_map`:

| map | +0 within reach (14 days) | +0 beyond, to 42 days | +50 within | +50 beyond |
|---|---|---|---|---|
| small 2 | 6 | 6 | 0 | 0 |
| small 3 | 0 | 4 | 0 | 0 |
| small 6 | 1 | 13 | 0 | 12 |
| small 9 | 1 | 0 | 0 | 0 |
| medium 8 | 4 | 4 | 0 | 1 |
| medium 9 | 0 | 0 | 0 | 0 |
| large 8 | 0 | 6 | 0 | 5 |
| small 61288 | 1 | 1 | 1 | 2 |

By +50 almost no tile worth a daughter is left within reach (one, on 61288). Beyond it there are some (small 6 12, large 8 5):
on those two maps the reach is the limit, not the land, and a pass would be what a clan has to cross to get there. I did not
change `SETTLE`. A scout's route that opened the far side would not settle it on 61288, whose far tiles feed no one;
that is the land's, not the reach's.
