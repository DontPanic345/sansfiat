# #338 play: small 2, Red (civ 1, the control's map and civ), through the page driver, all fixes in

Game `play/small-2.json` (lines: `play/small-2.live.jsonl`, notes `play/notes-small-2.md`, judge
`play/small-2.judge.txt`). Pick t286, played to t389 (+102). Screenshots: `play/pick.png`, `play/small-2-end.png`.

## Standing and decisions

- Standing at +102: **47.4% rank 1** in the game, 45.9% rank 1 with every civ on the AI, **30.1% rank 2 idle led civ**.
  Gap to the idle civ 17 points; to the AI control 1.5 (inside the floor).
- Player orders: **103 in 102 turns (101 per 100), 58 turns with none, longest run 4.** Control (main's check game): 11 orders,
  10.7 per 100, 95 turns with none. 0 refused.
  86 of the 103 are claims: the reach line names the next claim, so a claim was always available; 12 are gift steps; 4 raises; 1 route.
  The one army move I tried did nothing visible (finding 1), pillage was never offered.
- Doubt: lead changes 0 / 2 (game / control); motion: clans changed 81 / 71, claimed tiles taken 7 / 63. Fight: no people fell.

## Findings, worst first

1. **fix** — *Move to… then a click on a tile does nothing for an army.* Army 488 at the capital, `Move to…` then click (-4,-2)
   (Blue land, reachable: tooltip said 5 turns): the army stayed, no refusal, no feed line; the click selected the clan on that
   tile instead. Breaks the page touch point "right-click / Move to moves the selected unit". Repro: `play/small-2.live.jsonl`
   t313-317 (button "Move to…", click -4,-2); the army "goes hungry" at (3,2) on t316.
2. **ask** — *A gift that "reaches no clan" still sees the store fall.* Gift 4 for 20 turns (t357-377): panel said "gave 0.0",
   store went 27 -> 1, with no line saying where it went (home army packs, raises). Gift at 0 let the store rebuild to 27 in 20
   turns. The player cannot tell what spends the store. Repro: live.jsonl t357-377. Already the open idea "The gift reaches no one
   on the small maps" (not repeated).
3. **fix** — *Selecting the civ again after a claim takes up to five clicks on the capital.* Clicking the capital cycles scout,
   army, civ, clan, festival, tile; after each claim the panel is on the claimed clan. There is no one-touch way back to the civ
   panel (Escape clears to nothing; `lead` is a driver word, not a touch point). Breaks the "each claim is one drag" cost the
   reach line promises. Repro: live.jsonl t287 onward.
4. **fix** — *The first claim drag after a selection often does nothing (it selects).* About every other claim needed `P` + drag
   twice (t287-t291). No refusal or hint shows. Repro: live.jsonl t289-t291 (claims at 14,-1 and 12,-2).
5. **ask** — *`end N` stops after 1-2 turns on every rival join alert.* From t287 to t313 `end 5` ended 1-2 turns each call; the
   alert box held about 20 rival joins by t313. Already an open idea (alerts for a rival's clans); not repeated.
6. **ask** — *My home army wasted away: 5 -> 2 warriors by t339 with the store at 0.* "Capital held by your army, 2.0 warriors;
   Blue's army of 4.0 is 8 steps away" is shown, but nothing says that the army starves at home when the gift or claims leave the
   store at 0. A rule question: should a home army eat before the gift?
7. **ask** — *Clans left Red for Gold in bursts of 4 on t319.* 6 "leaves Red for Gold: its home is on Gold's land" in 40 turns; the
   reach line shows what a claim brings but not what Gold's claim will take.
8. Reach line: shown on every turn, named a claim that joined a clan the same turn, 86 times; Standing 21.8 -> 31.5 in 10 turns
   from that alone. It never said "nothing to bring"; no claim was left unnamed on the turns I looked. Works as built.

The camera fix held: selecting tiles one after another (t313) cycled the units on a tile without the view moving.

## Page scripts

`node engine/www/play/all.mjs`: every script passed (army, art, bar, buttons, camera, camp, civ, end, feed, fields, game, goods, layout, pick, ring, save, scout, select, settle, start, towns, views). Screenshots of the played game: `play/pick.png`, `play/small-2-end.png`.
