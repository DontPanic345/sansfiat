# #356 play — small 2, seed 2, Gold (civ 3), 100 turns from 287

**Standing line:** +100: 5.2% (rank 3) in the game; 9.7% (rank 3) with every civ on the AI; 2.9% (rank 3) idle led civ. Beats idle, loses to the AI control (land 29 vs 60 tiles).

Trade group (judge): fills 13 (AI civs), army-turns fed at a foreign market 5 (0.4 of 13.1 food needed), unit-turns trading 5, trades 14778, largest count share at +200 obsidian 0.0%.

## Played
Claimed chains to clans 275, 280, 162, 212 (Standing 4.1 -> 5.6), one army of 3, scout route, gift 0, one signpost. Rivals Blue 35%, Red 31% at the end.

## Findings
- **no result** (fix? no, ask), turns 287-387: the store never held salt (0.0 all game), so the Decided line "a unit's pack is filled with salt at its store when the player orders it" never offered an order; no fill/trade order appeared in the legal orders and an army walked to clan 275's market (turn 293, 770) showed no trade, so the player could not use the piece's lever. Repro: relay/specs/356/play/small-2.lines.jsonl turns 287-300. Class: ask (the salt supply for a civ with no salt source is a rule question).
- **no result**, turn 340: `{"type":"signpost","tile":524}` — result `ok` but no signpost appears in the next views; the view said "good sites: none free nearby". Repro: lines file turn 340.
- **idle**, turns 330-387: 46-turn run of no orders; nothing offered (reach lines needed 3-tile chains to a 2.7-person clan, store 6).
- **reason**: the reach line names claim tiles by (q,r) while the claim order takes tile numbers; the player had to infer numbers from the map grid; the first claim was refused (turn 287).
- no choice: nothing found. 
