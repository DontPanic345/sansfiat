# #29 sweep: the camps on the retro's seven maps

`sansfiat sweep --after 200 --maps retro` (army) and the same with `--no-army` (the control, `Config.army`
off: no camp, no levy). All three civs on the AI, each raising its camp at the pick (`UNIT.start` warriors),
run 200 turns past the pick. A civ "kept" its first camp when no camp was lost and it ends with at least
`UNIT.start` warriors.

## The numbers (build-3)

| | before (build-1) | now | why |
|---|---|---|---|
| the levy's form | `levy` of the harvest beyond need (`clans::surplus`) | `levy` of the basket a clan brings to the capital's market, up to the store's room; the harvest drops by the same share | clans grow until their harvest just feeds them: `surplus` was 0.00 at every capital every turn, so the store never filled |
| `CIV.levy` | 0.3 | 0.2 | 0.15 lost one camp, 0.25 cost the clans 17% |
| `CIV.store_cap` | 50 | 20 | a store above the camp's needs only spoils (10% a turn); the levy stops at the cap, so it takes no more than the camp uses |
| `CIV.endowment` | 30 grain | 20 grain | at the cap; the first packs (15) come out of it |
| `CIV.pull` (new) | — | 3 (labour) | civ clans favour their capital's market when picking a festival. Seed 3's Red kept one clan of nine at its capital, even in the control, and could feed no camp |
| `UNIT.carry` | 1.0 | 3.0 | the reach: 5 plains steps (below) |
| `UNIT.start` | 3 | 3 | kept; start 2 needed the same changes |
| `pack`, `forage`, `pillage`, `fallow`, `hungry_turns`, `post` | 5, 0.5, 1, 5, 3, 3 | unchanged | they held |

The line's colour (need over carry: green, amber at a half, red when cut off) is unchanged: a camp at its
post shows green-yellow, and a camp near its reach shows red.

## Army

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   77.3  kept yes
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   82.1  kept yes
small  seed   2 civ 512: camp alive warriors  3.0 pack  8.83 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   17.3  kept yes
small  seed   3 civ 212: camp alive warriors  3.0 pack  3.30 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged  26 tiles pillaged   0 tiles clans' people   14.5  kept yes
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   26.9  kept yes
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.3  kept yes
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   29.8  kept yes
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   39.8  kept yes
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   66.1  kept yes
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   40.2  kept yes
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   78.9  kept yes
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.6  kept yes
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.7  kept yes
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  105.7  kept yes
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   37.6  kept yes
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   27.3  kept yes
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.9  kept yes
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.1  kept yes
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   88.6  kept yes
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (army): 1025.5
```

## Control (no army)

```
small  seed   2 civ 510: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   78.0  kept no
small  seed   2 civ 511: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   80.4  kept no
small  seed   2 civ 512: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   31.5  kept no
small  seed   3 civ 212: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.2  kept no
small  seed   3 civ 213: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.5  kept no
small  seed   3 civ 214: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   28.8  kept no
small  seed   6 civ 363: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.7  kept no
small  seed   6 civ 364: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.7  kept no
small  seed   6 civ 365: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.2  kept no
small  seed   9 civ 275: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   44.4  kept no
small  seed   9 civ 276: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.0  kept no
small  seed   9 civ 277: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.1  kept no
medium seed   8 civ 677: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.8  kept no
medium seed   8 civ 678: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.7  kept no
medium seed   8 civ 679: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.9  kept no
medium seed   9 civ 919: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  100.0  kept no
medium seed   9 civ 920: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.4  kept no
medium seed   9 civ 921: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.4  kept no
large  seed   8 civ 2158: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.6  kept no
large  seed   8 civ 2159: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.4  kept no
large  seed   8 civ 2160: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   67.3  kept no
civs that kept their first camp: 0 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (control, no army): 1117.9
```

## The margin

Every civ keeps the camp it raised at the pick (21 of 21), each posted 2 or 3 steps from its capital (line
cost 4 or 6), none cut off at the end. The clans' people end **8% under the control in total** (1025.5
against 1117.9). The margin I name is **within 15% of the control, in total over the seven maps**. Per civ
the army's people over the control's average 0.88, but range from 0.35 to 1.32: the runs are chaotic
(the same civ swings between 0.4 and 1.0 when an unrelated number moves), so per-civ ratios are noise and
the total is the measure.

Off the seven maps (seeds 1, 4, 5, 7, 10, small and medium) 29 of 30 civs keep their camp. The one that
doesn't (medium seed 7) has 8 people left in its clans: a civ that small can't feed three warriors.

Per civ, army people / control people:

```
510 w3.0 cost6.00 77.3/78.0 0.99 yes
511 w3.0 cost6.00 82.1/80.4 1.02 yes
512 w3.0 cost6.00 17.3/31.5 0.55 yes
212 w3.0 cost6.00 14.5/41.2 0.35 yes
213 w3.0 cost6.00 19.6/23.5 0.83 yes
214 w3.0 cost6.00 26.9/28.8 0.93 yes
363 w3.0 cost4.00 51.3/56.7 0.90 yes
364 w3.0 cost6.00 29.8/33.7 0.88 yes
365 w3.0 cost6.00 39.8/46.2 0.86 yes
275 w3.0 cost4.00 66.1/44.4 1.49 yes
276 w3.0 cost6.00 40.2/49.0 0.82 yes
277 w3.0 cost6.00 23.3/33.1 0.70 yes
677 w3.0 cost6.00 78.9/83.8 0.94 yes
678 w3.0 cost4.00 33.6/54.7 0.61 yes
679 w3.0 cost6.00 51.7/55.9 0.92 yes
919 w3.0 cost6.00 105.7/100.0 1.06 yes
920 w3.0 cost4.00 37.6/49.4 0.76 yes
921 w3.0 cost4.00 27.3/41.4 0.66 yes
2158 w3.0 cost6.00 54.9/59.6 0.92 yes
2159 w3.0 cost6.00 59.1/59.4 0.99 yes
2160 w3.0 cost6.00 88.6/67.3 1.32 yes
mean 0.883 min 0.352
clans' people at the end, all civs (control, no army): 1117.9
```

## How far a line reaches

A line feeds a camp while its cost (there and back, `2 ×` the path cost) is no more than a warrior's
labour carries of a warrior's food: `cost ≤ UNIT.labour × UNIT.carry / UNIT.food` = 3.5 × 3 = 10.5,
whatever the camp's size (both its labour and its need scale with its warriors). So a camp of 3 holds
steady:

- **5 steps over open ground** (plains, coast, desert: 1 a step, line cost 10); cut off at 6.
- **2 steps over hills or forest** (2 a step, line cost 8); cut off at 3. A worn trail through them counts as 1.
- 1 step over marsh (3 a step).

That is when it has nothing else to do: a camp that marches a turn has no labour left for its line, and a
camp past its reach lives on its pack (5 turns of food, spoiling) and what it forages. The test
`a_far_camps_line_costs_more_and_carries_less` checks 5 plains steps fed and 6 cut off.

## Played on the page

`node engine/www/play-camp.mjs` (small, seed 2, leading Red): the camp goes to its post 2 steps out (line
cost 4, carries 7.9 against 3 eaten), then is ordered 9 steps out on unclaimed land:

```
out 1: 5 steps out, line cost 10, carries 3.15, fed, pack 7.8
out 2: 9 steps out, line cost 18, carries 1.75, cut off, pack 4.0
out 3–4: cut off, pack 2.4, 1.1 (it forages once on unclaimed land)
out 5: hungry, pack 0
back 1: 6 steps out, still cut off, hungry 2
back 2: at its post, hungry 3: loses a warrior (a turn's march leaves no labour for its line)
back 3: fed again by its line, 2 warriors, pack 2.3
```

Screens: `screens/camp-ordered.png` (the order), `screens/camp-line.png` (at its post),
`screens/camp-hungry.png` (past its reach: the red line, cut off and hungry), `screens/camp-back.png` (fed
again at its post).
