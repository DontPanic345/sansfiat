# #104 build-4: the page shows armies and their fights

The evidence for `engine/www/play/army.mjs` and the map it plays on. Run it with
`node www/play/army.mjs --out DIR` from `engine/`; it takes about 5 s, and writes
`army-raised.png`, `army-fight.png`, `army-civ.png` and `army-feed.png` (the three in `screens/` are the
laptop-size shots from that run).

## The map: small seed 9, Blue (civ 486)

The brief asked for a retro map where the led civ holds forest. Probing every retro map and civ in node
(`pick` and `chooseCiv` at the pick, then the path from the capital to the nearest rival-held tile):

| map | civ | forest held | path steps to the nearest rival's held tile |
|---|---|---|---|
| small 3 | Red (267) | 5 | 9 (Blue) |
| small 9 | Blue (486) | 9 | 11 (Gold) |
| small 6 | Gold (355) | 6 | 18 |
| medium 9 | Blue (878) | 5 | 13 (Gold) |
| medium 9 | Red (877) | 17 | 43 (Blue): too far |

Small 9 Blue is the one that plays. Its corridor meets Gold's held land from the first turn with both
guards standing: `story skirmish --seed 9 --size small --civ 2` shows two tiles contested from +1, the
store filling (4.9 at +1, 15 by +4), and an army of three raised at +5. Small 3 Red has the shortest walk
but no fight: Blue's guard is at 0 by +17 (the trace shows `rival guard 0.0`), so there is nothing to fight.
Medium 9 Red's nearest rival is 43 steps by path, and its rival guards are gone by +3.

## What the script does, and what it found

- Gift to 0; the civ's Raise… warriors is allowed after 2 turns (the store holds 6.2 food).
- Warriors raised as a new army (`Blue's army 498`) at the capital, through the civ's panel. The civ's
  options: warriors, archers, a guard, a scout.
- Archers raised into it through the army's panel. The army's Raise… was greyed ("the store can't fill
  their packs") until a pack filled. The army then holds 1 warrior and 1 archer; it says "Making bows" the
  turn it is raised.
- The map draws its number as `1.0+1.0` (`game.map.stats.labels`, set by the units layer).
- Move to… onto (379), a tile of Gold's held land that the overlap makes contested: the army arrives on turn
  251 and the feed shows "Gold and Blue fight at (-1, 4)", then "Gold's guard at (-1, 4) falls to Blue's army:
  1 tile(s) lost".
- Clicking the skirmish selects the army: its panel reads "Fighting at (-1, 4) · Hungry 3 turns", and its
  reasons include the fight. The civ's reasons carry "its army took 1 tile(s) of Gold's claim at (-1, 4)".

## A finding the brief should know

No march needs a corridor here, because Blue's and Gold's guards already overlap. Without a rival guard on
the tile, a march does not fight: on small 3 Red the army walked onto Blue's land, Blue's guard there was at
0, and the army took the paper (`claimTaken`) and starved. The AI's stand-in claim skips tiles an army
holds, so an army can fight an AI guard only where that guard still stands on the tile when the army arrives.
