# #337 play: small 2, seed 2, civ Blue (425)

The game ended at +63 (turn 325): Blue won as the last capital standing, so 100 turns were not reached.

**Standing** (judge): Blue 29.0% rank 1 in the played game, 35.7% rank 1 with every civ on the AI (control), 27.1% rank 2 idle led civ. Start 18.0% rank 2.

What happened: the scout built the route 166→506 (Red capital) with a trip home to fill its pack at +13 (errand finished by +21: "walking to its capital to fill its pack" showed in the view as the Decided line says). One warrior-army marched the route in 5 turns, took Red's capital (Red 0.0% at +50), and a second reached Gold's capital via a second route; the game ended in conquest at +63.

## Findings (by kind)

**no choice**
- turns 304-325: once the route stood, the only live choices were move/route; no guard, no pillage order was ever offered. (ask)

**reason**
- turn 304: the army walked the route in 5 turns when the scout view says "known route 28.0 days"; the view never says why an army is faster. (ask) repro: play/small-2.lines.jsonl, turn 299-304.
- turn 304-309: two warriors "disband from hunger at (2,2); rejoin clan 37" and the capital still fell to the one that remained; the view does not say what held the capital. (ask)

**no result**
- turns 283-325: after the errand finishes the scout sits at its capital with "pack 2.0 of 72" and does not refill (an open idea already: idle scout and store). Repeats an open idea, not reported.
- turns 309-325: "gave 0.0" with gift 6 and store 24-45 (open idea, not reported).

**idle**
- 57 of 63 turns had no order (judge); nothing called for one after Red fell.

## Findings with class
- `ask`: the route list in "legal orders" offers 5 adjacent tiles but a far tile (506, 90) is accepted; a player cannot learn this from the view. repro: play/small-2.lines.jsonl, turn 267.
- `ask`: an army's days on a scout's route (28) differ from its real march (5 turns). repro: turns 299-304.
