# Play note, #152, small 2 (seed 2), civ 1 (Red 472), +100 turns

Judge: led civ Standing at +100: 5.8% rank 2 in the game, 5.2% rank 3 with every civ on the AI, 4.0% rank 3 idle.
End (game / control / idle): goes on +100 rank 2 / goes on +100 rank 3 / goes on +100 rank 3. Limit is 200, so nothing ended inside the 100 turns played.
Player orders 8 (8 per 100 turns), 94 of 100 turns with none, longest run 39, 1 refusal. Gold led throughout (14.6%); no lead change; no fight.

## no choice
- turns +0..+100: the lone `setGift 6`, with no unaligned clan in reach ("none" in the view), changed nothing; the gift had no one to reach. Standing moved only when the guard and warrior were raised (4.3% to 5.9% at +40).

## reason
- Nothing found: notes match the view.

## no result
- turn +15..+100: one warrior army (490) forage, move, disband: no rival came near, so no fight, pillage or surrender; Standing did not change from them (5.9% flat from +40 to +100).
- the scout route to 331: view still "no route"; no visible outcome.

## idle
- turns +40..+100: Standing flat at 5.9% with Gold at ~14.7%; with every raise refused at turn 292 ("the store can't fill their packs") there was nothing left to decide. 94 of 100 turns with no order.

## Findings
- **ask**, repro `small-2.lines.jsonl` turn 292 and +0..+100: the end screen and limit (200) were never in play in 100 turns; a player with no rival in reach has no lever that moves Standing past ~6%. Whether the start (scout only, store of 10) leaves a first 100 turns with too little to decide is the owner's call. No fix.
- **ask**, turn 292: "the store can't fill their packs" doesn't say how much the store holds or the pack costs; the view shows the store (14.2) but not the pack. Prose only.
