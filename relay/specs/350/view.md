# #350 build-1: a session log, small 2 led as civ 1

Made with `sansfiat play --game g.json --seed 2 --size small --civ 1` and the raise at the pick, then played a line at a time. Excerpts; a line is cut with … where it is long. Turn numbers are the game's.

## The pick: the claim lines, and the reach order sent

```
land: 43 tiles, your claim: 56 (-13, -1) 76 (-12, -2) 77 (-12, -1) 97 (-11, -3) 98 (-11, -2) 99 (-11, -1) 120 (-10, -3) 121 (-10, -2) 122 (-10, -1) 142 (-9, -5) …
  nobody stands on it: a rival army that ends its turn on a tile takes it, with any tiles that cuts off from your capita …
  {"type":"claim","tiles":[56,76,77,97,98,99,120,121,122,142,144,145,146,165,166,167,168,169,170,190,191,192,193,194,195,216,217,218,219,220,221,222,243,244,245,247,248,249,271,272,273,277,301]}  sets your land to exactly the tiles you list, with your capital and settlements: these are your 43 tiles; a tile you leave out is dropped, and a clan whose home is on it is no longer yours
  {"type":"claim","tiles":[56,76,77,97,98,99,120,121,122,142,144,145,146,165,166,167,168,169,170,190,191,192,193,194,195,216,217,218,219,220,221,222,243,244,245,247,248,249,271,272,273,277,301,307]}  the claim in reach: adds 1 tile, 307 (-3, -5), and brings clan 429
```

The reach order sent (the view's last line, its JSON only):

```
{"type":"claim","tiles":[56,76,77,97,98,99,120,121,122,142,144,145,146,165,166,167,168,169,170,190,191,192,193,194,195,216,217,218,219,220,221,222,243,244,245,247,248,249,271,272,273,277,301,307]}
```

Its result, from the play's output (the view after the order is the second `turn 281` view):

```
    281   Clan 429 joins Red: its home is on Red's land
    civ 453 Red 21.5%, rank 1 leader, you
    civ 453 Red 22.3%, rank 1 leader, you
  land: 44 tiles, your claim: 56 (-13, -1) 76 (-12, -2) 77 (-12, -1) 97 (-11, -3) 98 (-11, -2) 99 (-11, -1) 120 (-10, -3) 121 (-10, -2) 122 (-10, -1) 142 (-9, - …
```

The land grows from 43 to 44 tiles (the new one is 307 (-3, -5)), clan 429 joins Red, and Standing goes from 21.5% to 22.3%.

## A signpost posted on turn 311, its line after `end 30`

```
> {"type":"signpost","tile":243}
> end 30
signpost at 243 (-5, -10): posted 30 turn(s) ago, the tile would feed a daughter 0% of her need; your clans that divide or move on may go there, and the first to settle founds a settlement
  it draws nobody: clan 41 gathers this tile, so it feeds a newcomer nothing; a clan goes only where it is fed at least 90%: remove it and post on a free site
```

Tile 243 is one of the led civ's own tiles, four steps from its capital. After 30 turns its line says why it draws nobody: clan 41 gathers the tile.

## An army raised at the capital, its line, and `why`

```
> {"type":"raise","n":1}
  army 600 at 166 (-8, -6): 1.0 warriors, 0.0 archers, pack 5.0 of 5.0 (5.0 turns), entrenched 0 turns, 0% armed with obsidian, dest none, at its capital, no limit, draws on its capital, hungry 0 turns, zone 166 (-8, -6), held 166 (-8, -6)
> why 600
why 600: its reasons, newest first
   341  raised at (-8, -6) with 1 warriors from its clans
```

The army stands at its capital, so its line reads `at its capital, no limit` and it draws on the capital.

