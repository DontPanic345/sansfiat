# M4 retro: the players

Verdict: a player on a small map gives a handful of orders in the first twenty turns and then
ends turns. The check's player gave 11 orders in 103 turns and went 95 turns with none. Idle on
the owner's map, a civ is second at the pick and second at the limit, 200 turns on. The AI
finds more to do than the view offers a player: led by the planner, a civ ends 4.9 points of
Standing ahead of its idle self. The gap is levers the player isn't shown, not levers that are
missing.

This session played in the text view (`sansfiat play`), not in a browser. The page's own
faults are the owner's notes (`owner.md`), and piece P plays the page first.

Sources: `play.md` and `play/` (the check, small 2 as civ 1); `game/idle-61288.txt` and
`game/scout-61288.txt`; `game/sweep.txt`, led games; the five play steps' findings (#199, #219,
#220, #239, #286, #296, #297, #299) and the check's (#316 to #320).

## 1. What a player meets

| Turns | What there is to do | Source |
|---|---|---|
| The pick | Choose a civ from three cards: clans, people, terrain, the nearest rival's distance | the text view |
| Early | Set the gift, send the scout, raise a guard so the claim holds | check: 11 orders in 103 turns |
| While clans are in reach | Watch them join. On small 61288 nine join an idle civ by themselves | the idle game's last view |
| After | End turn. The store sits at the keep, the gift gives 0.0, and no clan is in reach | check, turns 282 to 345 |

Rival capitals are 11 to 18 steps away on a small map, and the check's player found no use for
an army. The check's player never saw a pillage order listed: the view lists
only orders that would be accepted now, and pillage needs an army standing on a rival's tile.

## 2. The same four findings, every piece

Five play steps and the check played small 2 as civ 1, and each filed these again:

| Finding | Times | Cause |
|---|---|---|
| A gift shows no effect | 4 | A gift reaches only clans that trade at the civ's market. The view says so in one clause and never says how many that is or what was left unspent |
| Nothing to decide for most of 100 turns | 4 | §1 |
| A scout's route shows no result | 4 | A near route is found the turn it's ordered and the view keeps printing the order; a far one stalls (`game.md` §2) |
| Held land drops with no word | 1 | Nothing says a guard's people hold tiles or how many |

The fourth goes with the guard (piece L). The first three are a readout each.

## 3. What the AI does that a player doesn't

Per 100 turns the planner gives 60 orders: 30 moves, 10 raises, 9 forages, 6 claims, 3
disbands, 2 gift changes, 0.5 zones and 0.07 pillages. Its gain over idle comes from widening
the claim a tile a turn toward clans' homes and keeping the guard fed, which the text view
offers a player as one line (`claim`, with the tiles to type). On the page it is painting.

## 4. The play step

Each play step took 1.1 to 1.7 minutes and about $0.20, on the same map and civ, and its
findings repeat. From M5 the play step's prompt carries the open ideas and no build summaries,
and its findings go to one issue (`relay/README.md`). What is still missing is the page: the
play step plays the text view, and the owner plays the page. The owner's notes this milestone
are all page notes (the camera, the scout's buttons) or rules met on the page.

## For other angles

- **retro and proposal**: the page needs a played pass of its own, and the three readouts above
  are small.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| The gift line says how many clans it reaches and what was left | §2; #220, #239, #296, #316, #320 | high | small | take, piece P |
| A scout's line says what it is doing this turn and why it waits | §2; #219, #297, #319 | medium | small | take, piece C |
| The view names the next thing worth doing when nothing is in reach | §1; #199, #286, #299, #317 | high | the owner's call on what that is | take, piece P plays for it |
| The play step plays the page | §4 | high | a browser in the play step | try, in piece P |
| Vary the play step's map and civ | `relay.md` §5 | medium | one line in `play.md` | take |
