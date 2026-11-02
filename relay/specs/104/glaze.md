# #104 build-5 (glaze): the fight tuned against the stories

All three stories on small seed 2, Red (472) against Blue (473): `sansfiat story NAME --save F`, the same with
`--no-fight`, then `sansfiat judge F`.

## What changed, and the run that moved it

| what | from | to | the run |
|---|---|---|---|
| `FIGHT.rate` | 0.1 | 0.2 | skirmish: at 0.1 Red +0.7 / Blue −0.2 against the control, at 0.2 Red +2.9 / Blue −1.0; guards of 2–3 still last the contest |
| retreat's tile (`fight::retreat`) | the nearest held tile anywhere, at once (6 steps on surrender) | one step back: the uncontested neighbour, own-held first, nearest the capital | surrender: the beaten army jumped 6 steps home, refilled, and was never hungry three turns running |
| `line::cut_by_rival` | none when the line was cut by length too, and only rivals on the open path home | any cut line, with the army's own tile and its neighbours' rival land counted | surrender: hungry 3+ turns, cut, surrounded on 5 of 6 sides by Blue's land, and `cut_by_rival` gave none, so it starved instead |
| surrender while marching | allowed | only an army with no march (`dest` none) surrenders | camp.mjs's hold: the camp marching onto a rival post surrendered on the way |
| surrender while fighting | allowed | an army whose tile is contested this turn stands and doesn't surrender | army.mjs: Blue's army arrived hungry 2 turns, fought Gold's guard and surrendered the same turn, so the skirmish had no army to select. The stories' numbers didn't move |
| `entrench`, `entrench_cap`, `retreat`, `hungry`, `militia`, `surrender`, `floor`, the kinds, `SCOUT.reach` | — | unchanged | none of the stories reached a capital, militia or a held entrenchment that would read them; scout.md shows reach doesn't move the scout |

## The stories (fight / control, Standing at the end)

| story | Red | Blue | Red diff | Blue diff | fallen Red / Blue | taken | surrendered |
|---|---|---|---|---|---|---|---|
| skirmish (+80) | 10.7% / 7.8% | 7.9% / 8.9% | **+2.9** | **−1.0** | 1.22 / 0.48 | 0 | 0 |
| stack (+80) | 5.7% / 5.8% | 9.5% / 8.4% | −0.1 | **+1.1** | 1.29 / 0.74 | 0 | 1 |
| surrender (+60) | 5.8% / 5.2% | 8.6% / 8.6% | **+0.6** | 0.0 | 2.06 / 1.19 | 0 | 1 |

`JUDGE.lead_margin` is 0.5 points: skirmish moves both civs past it, stack moves Blue, and surrender moves Red.

- **skirmish**: 1.70 fallen over about 19 turns on two contested tiles, about 0.045 a contested tile-turn. That is the band for guards of 2–3 people: a few people in all, decisive for neither, and no tile changes hands through the fight (0 of 2). (The "Red's claim taken by Blue's unit" events at +12 to +21 are Blue's AI camp on Red's paper, not the fight.)
- **stack**: three warriors are raised at +20, staged 4 steps out, and pushed at +31. They take Blue's claim at (−10, −2) on the way, then hunger and Blue's ground stop them: "Red's army at (−13, 0) surrenders to Blue: 2.8 people join its clans" (+32). Later armies of one person retreat and surrender the same way, and Blue gains the people. Blue's capital never falls.
- **a capital can fall**: no, on nine map/civ pairs (small 2 with each civ, small 3 with each, small 9 Red and Blue, small 4 Red and Blue), each over 120 turns. Small 2's Blue guard is down to 1 from +24 (as the brief said), but no Red army gets nearer than 9–10 of the 13 steps. The stage sits 4 steps out and a pack is five turns, so every push runs out of food short of the capital. That is the economy: the store never fills more than three packs (14 food at most in 80 turns), so it is part of the piece's answer, not a fight number.
- **surrender**: the starting army is cut at +2 and its pack is empty at +4. It fights Blue's guard at (−12, 0), retreats a step at +4, goes hungry, and "Red's army at (−11, −1) surrenders to Blue: 0.9 people join its clans" at +20. That is 16 turns after the pack ran out, not the brief's ten: forage at its tile breaks the hungry run at times, and hunger and the fight take it from 3 people to 0.9 first. Red loses fewer people to hunger than its control's starving army does, so Red is +0.6.

## Health

`sweep --judge` (glaze-judge.txt) matches build-3-judge.txt line for line apart from the timing (32.0 s against 35.5 s). Nothing fights on the AI maps, so the rate and the retreat's form reach no AI score.

## The page

`play/army.mjs` passes inside check.sh with these numbers (small 9, Blue against Gold's guard). The army arrives
hungry, fights at (−1, 4) on turn 251, and takes the tile. Its panel reads "Fighting at (−1, 4) · Hungry 3 turns:
loses a warrior at 3", its reasons "lost 1 warrior(s) to hunger", and the civ's "its army took 1 tile(s) of Gold's
claim at (−1, 4)". So the Why says what happened. It says why only in part: hunger and the tile taken are there, and
the losses to the guard come in the reasons on the turns they fall.
