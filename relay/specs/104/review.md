# #104 review: the stories re-run, and how far the Standing differences go

I re-ran all three stories (small seed 2, Red 472 against Blue 473), once with fighting and once with `--no-fight`, and
scored each with `sansfiat judge`. The numbers match `glaze.md`.

| story | Red fight / control | Blue fight / control | Gold fight / control (not in any fight) |
|---|---|---|---|
| skirmish +80 | 10.7 / 7.8 (**+2.9**) | 7.9 / 8.9 (−1.0) | 16.6 / 15.7 (+0.9) |
| stack +80 | 5.7 / 5.8 (−0.1) | 9.5 / 8.4 (+1.1) | 14.3 / 15.3 (−1.0) |
| surrender +60 | 5.8 / 5.2 (+0.6) | 8.6 / 8.6 (0.0) | — |

Gold never fights, yet it moves by about a point between the fight run and its control. Clan dynamics drift once a run
differs at all, so differences of about one point are within that drift. Only the skirmish's Red +2.9 is clearly larger.

## Why Red gains in the skirmish (`story skirmish --trace`)

| turn | fight: Red guard, held, Blue guard, Standing | control: Red guard, held, Blue guard, Standing |
|---|---|---|
| +10 | 2.0, 11, 2.7, 7.8 | 3.0, 13, 3.0, 8.2 |
| +30 | 1.8, 11, 0.0, 9.0 | 3.0, 12, 2.0, 8.5 |
| +40 | 2.8, 12, 0.0, 9.2 | 1.0, 7, 0.0, 7.8 |
| +80 | 1.8, 11, 0.0, 10.7 | 1.0, 7, 0.0, 7.8 |

Blue's guard starves to 0 by +30 to +40 in both runs, so its claim goes to paper whether or not anyone fights. Red's
gain comes about indirectly. The fight thins Red's guard early, the thinner guard eats less, and Red's store then keeps a
guard of 2–3 that holds 11–12 tiles. The control's guard of 3 starves to 1 and holds 7. So fighting does move
Standing, but in this story it moves it by feeding the economy, not by taking ground.

## Wording

The feed, the unit list, the move hint and the tile's "Held by" line said "camp" for an army. The spec (item 3) and
build-4 call it an army on the page, so the review changed those strings to "army". Code and the wire keep `camp`.
