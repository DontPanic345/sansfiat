# The live driver, a session (build-1)

`node www/play/live.mjs start "?size=small&seed=61288&run=1" --dir /tmp/sl` in the background, then one
command per call (`node www/play/live.mjs CMD ARGS --dir /tmp/sl`). Outputs trimmed to the panel's first
lines and the feed. The save is read by `sansfiat judge`:

```
$ lead Red
turn 258 · Main game · Standing: Red 8.9%you▲; Blue 8.5%; Gold 7.4%
panel: Red — You lead Red: Standing 8.9%, 1st of 3
$ click 3,-9
panel: Clan 94 — Walks to festival 77
$ click 3,-9
panel: Festival 77 (centre) — 9 clans come
$ click 3,-9
panel: hills — Red's capital
$ click 3,-9
panel: Red's scout 203 — Idle
$ click 3,-9
panel: Red — You lead Red: Standing 8.9%, 1st of 3
  Standing 8.9%
  People 47.38
  Living gathering 47 · farming 0 · in the army 2
  Clans 9
  Store 10 of 60
  Salt 0
  Obsidian 0
  Last turn kept 15 · posted 0
  Joined 0
  Claimed 27
  Gift 2.0 a turn · reaches no clan · gave 0.0 · left 2.0
  Keep 15 (not set: the default, a full pack for its camps, 15 now)
  Leaning clans: none
  Units (1): Scout 203
  because 257 · founded at (3, -9) with 8 clan(s) in reach
  buttons: [Raise…] [Gift −1] [Gift +1] [Keep −1] [Keep +1] [Claim (P)] [Unclaim (U)] [Signpost…]
$ stepper "Raise…" 1 warriors go
stepper Raise…: {"max":2,"go":true,"title":"","options":["warriors","archers","a scout"]}
turn 258 · Main game · Standing: Red 8.9%you▲; Blue 8.5%; Gold 7.4%
panel: Red — You lead Red: Standing 8.9%, 1st of 3
  Standing 8.9%
  People 46.38
  Living gathering 46 · farming 0 · in the army 3
$ end 10
ended 6 of 10: stopped for a new alert
turn 264 · Main game · Standing: Red 8.3%you; Blue 7.9%; Gold 8.8%▲
panel: Red — You lead Red: Standing 8.3%, 2nd of 3
feed (42 new): 261 ·✂Gold's army at (-2, 5) is cut off from its capital | 262 ·≡Clan 212 takes up fields at (14, -11) | 262 ·■A hearth is tamed at (-1, 12) | 262 ·■A hearth is tamed at (11, -10) | 263 ·!Clan 212 is starving | 263 ·÷Clan 107 divides; clan 213 s
ALERT: 263 ·✚! Clan 182 joins Gold: its home is on Gold's land
$ shot t10
shot: /tmp/sl/t10.png (0.5 scale; @x,y reads this)
$ save g.json
saved /tmp/sl/g.json
$ stop
stopped
```

`sansfiat judge /tmp/sl/g.json` on that save:
`player orders: 1 (raise 1), 16.67 per 100 turns; turns with none 5 of 6, longest run 5`

The click cycle on the capital is clan, festival, tile, scout, then the civ: five clicks from the pick.
`end 10` stopped at turn 264 for a new alert (another civ's clan joined Gold).

## The gift line on a small 2 game

`relay/specs/336/play/small-2.json` is save version 20 and this build reads 26, so neither front end can
open it. A fresh small 2 game as civ 1 stands in (`sansfiat play --game G --seed 2 --size small --civ 1`,
`end 40`, turn 327): the same civ and map, with the gift at its default 2.0 and reaching no clan.

Text view (`sansfiat play --game G`, `view`):

```
keep 15.0 in force (not set: the default, a full pack for its camps, 15.0 now); gift 2.0 a turn, the most 60, reaches no clan at your market: gave 0.0, left 2.0; last turn posted 45.0; since founding gave 2.0, clans joined 11
```

The page, the same file loaded with `live.mjs load G`, the civ's panel:

```
  Last turn kept 15 · posted 45
  Gift 2.0 a turn · reaches no clan · gave 0.0 · left 2.0
  Keep 15 (not set: the default, a full pack for its camps, 15 now)
```

The staged case with a clan reached (`check-stage.mjs`): `reaches 1 clan · gave 0.0 · left N.N`.
