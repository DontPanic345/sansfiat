# Play note, #153 (small 2, civ 1, 100 turns from the pick, 1 seed)

Game: `play/small-2.json`, lines `play/small-2.lines.jsonl`.

## Judge
Standing at +100: 5.4% (rank 2) played, 22.5% (rank 1) with every civ on the AI, 4.7% (rank 3) idle led civ.
Orders 8 (8 per 100 turns), 94 of 100 turns with none, longest run 54. Land held 1 / 43, guard 1.0 / 5.0 people. Motion: clans changed 12 / 35, people 62.4 / 182.8.

## Findings
- **no choice**, turns 260-360: the text view offers the scout only `route`; a Route that ends "no route can exist" (turn 261) leaves it idle at 193 for the rest of the game. No Move, Forage or Disband all in the view, so the piece's three-button scout row was not reachable here. Repro: small-2.lines.jsonl, line `route` at +0. class: ask
- **no result**, turns 265-300: `setGift 6` — "gave 0.0" every turn, clans joined 0; no unaligned clan was in reach of the market, and only the store drained (22.7 to 1.1). class: ask
- **no result**, turn ~296: `raise` into army 551 — the army stayed 1.0 warriors. Repro: lines after the second raise into 551. class: ask
- **idle**, turns 265-360: people left Red's clans every few turns (flagged "!" in the feed) and Standing sat at about 5% against Blue's 20% with 94 turns without an order; the view gave no lever for the leaving, and `why 301` gave only "the AI would post your camp". class: ask
- **reason**: nothing found beyond the above; the scout's hunger stayed "0 turns" through the whole game, so no hunger reason was shown. The scout survived (alive at +100), as the piece wants.
- The page screenshots belong to a page piece's play step; this step plays the text view only.

Standing at +100: 5.4% played against 22.5% (AI control) and 4.7% (idle control).
