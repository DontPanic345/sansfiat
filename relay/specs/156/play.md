# #156 play note — small 2, civ 1 (Red), 1 seed

Game: `relay/specs/156/play/small-2.json`, lines `small-2.lines.jsonl`, judge output `small-2.judge.txt`. Played 100 turns from the pick (turn 262 to 362).

## small 2

Standing line: led civ 424 at +100 **22.6% (rank 2)** in the game, 28.2% (rank 2) with every civ on the AI, 19.2% (rank 2) idle led civ. Pick was 10.8% (rank 1). Blue (425) led from +10 on and ended at 30.7%.

Motion (game / control): clans changed 30 / 45, people 182 / 253, land tiles 95 / 206. Doubt: lead changes 1 / 1.

Towns: the text view showed a town at the capital tile 506 (3, 2) from about +10 on, growing from 4 clans (2 farming) to 8 clans (5 farming) by +100; a second tile, 475 (2, 2), held two homes at +10. Shared homes were visible in "your clans' homes" (506 and 475 each listed twice). Starvation notes appeared among new daughters at +8 to +9 (clans 414, 450, 449), then nothing in the feed afterwards.

## Findings

**no choice**
- Turns 0-100: the claim lever offered one move, repainting a claim of 56 tiles with 1 held; no clan was on unheld land to bring in. The player left it untouched. Evidence: judge `claim 56 / 53 tiles claimed, 8 / 52 held`.

**reason**
- Turn +10: a note said "gift 6 draws clans", but the view said "unaligned clan 90 ... trades at market 152, so your gift doesn't reach it" and `gave 0.0` for the last turn. The gift was set on a misreading of who it reaches. Evidence: lines file, setGift at +0, view at +10.

**no result**
- Turn +0, `{"type":"raise","n":1,"kind":"guard"}`: guard 1.0 vs 11.0 in the control and held tiles 8 vs 52; no feed line or view change followed (guard "thinnest held share none"). Class ask.
- Turn +0, `{"type":"route","unit":427,"from":506,"to":535}`: "known route 4.0 days, 0 tiles to search" at +10, so the scout built no trail; nothing changed.
- Army move to 535 and `forage`: accepted, no view line or standing change followed.

**idle**
- Turns +12 to +100: 98 of 100 turns with no order, longest run 89, while Red sat at 16-22% and Blue rose to 32% by +50. Nothing in the view said what to do (the gift didn't reach the clans, held land was 8). Class ask.

## Town questions seen in play (class ask unless noted)
- ask: the town on the capital tile (8 clans, 5 farming) is the player's own capital festival, yet Red's Standing trails the control by 5.6 points (22.6 / 28.2) and the land taken is less than half the control's (95 / 206). The play shows no mechanism on its own; the review should read it beside the sweep. Repro: `small-2.lines.jsonl`, turns +0 to +100.
- ask: the view lists a town by text only ("town at 506 (3, 2): 8 clans, 5 farming") and gives the player no order for it; no `why` on a town. Repro: `small-2.json` view at +100.

No fix findings: none was shown on main as well, and none names a design line it breaks.
