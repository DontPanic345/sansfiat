# #336 play — small seed 2, civ 1 (Red, civ 424), 1 seed

I played 115 turns, not 100: my last `end` blocks overshot.

## Standing line
Standing at +115: 28.9% (rank 2) in the played game, 32.8% (rank 2) with every civ on the AI, 30.8% (rank 2) with the led civ idle. The pick was 22.2%. Blue (425) led from +20 and held about 36%.

Judge, game / control: lead changes 1 / 3, spread 0.268 / 0.283; claimed tiles taken 5 / 19; fight tiles taken 1 / 2, people fallen 0.62 / 11.19; no capital fell and no army surrendered in either. The game ended at +115 either way.

## Findings
- **fix** (the claim order's view, a Decided line of #336): the legal-orders line for `claim` reads "paints your claim (now N tiles)". It doesn't say what the order does to the claim or name the tiles. At +3 I sent a 9-tile claim and Standing fell from 20.9% to 10.5% at once, because the clans left my land. The view gave no preview. Repro: `small-2.lines.jsonl`, turn 265, the claim of 9 tiles then the repaint of 56.
- **ask** (the gift row): "last turn posted 40.0, gave 0.0" stayed true for 100 turns with a gift of 6–12 and 25 clans joined. A player can't tell what "gave" counts. Repro: lines file, turns 285–377.
- **ask** (army reach): an army marching off the claim shows a falling pack (20 to 5.9 in 4 turns, line 14 days). The view doesn't say how far it can go. Repro: turns 305–315.
- **ask** (capital falls): no rival army came near my capital in 115 turns, and I couldn't reach Blue's (about 17 tiles, move only onto adjacent legal tiles). On this seed the played game shows no capital fall.

By kind (judge): no choice: after the claim and the first raise, most orders were single-option. Idle: 104 of 115 turns had no order, longest run 34, while Blue went from 22.6% to 36% over turns 272–292 and I only raised the gift. No result: `setGift` 6 and 12 changed no visible number I could trace. Reason: nothing found. 1 refused: a disconnected claim.
