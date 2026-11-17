# Play note, #355 Goods — small 2, civ 468 Blue (1 seed)

Standing line: at +100 28.7% (rank 2) in the game, 36.6% (rank 2) with every civ on the AI, 27.6% (rank 2) idle led civ. Pick 15.7%.
Land 42 / 61 tiles; motion: clans changed 34 / 70, land 74 / 225. Player orders 8 in 100 turns.

## Judge findings
- **no result**, turns 292-387: `setGift` 6 then 12 — "gave 0.0" stayed while "posted 15-45" (known idea, gift).
- **no result**, turns 292-387: salt stayed 0.0 in the store for all 100 turns; no clan of Blue's held a salt source (12 salt sources listed, none Blue's), so salt never moved the game.
- **no result**, turns 327-387: obsidian sat at 8-10 and refilled after each raise (3, 6, 4 warriors); the view says "100% armed with obsidian" but never what strength it adds, so the raise was not weighed against anything.
- **no choice**, turns 287-387: signpost listed "good sites: none free nearby" the whole game; route targets only the 5 adjacent tiles, so the scout had no errand toward a salt holder.
- **idle**, turns 297-387: 94 of 100 turns with no order; Standing was flat at 28% while Red climbed 32→35%, and no lever offered a way to answer (no pillage order, no free signpost).
- **reason**: view shows store "salt -0.0" at turn 312 (negative zero).
- **reason**: after setGift, line reads "keep 45.0 in force (the default, the camp's full pack)", yet keep was never set.

Obsidian reached the store by levy/market (clans 58, 520 held sources inside Blue's claim) and was used at raising. Salt did not reach it.

## Findings
- ask: Salt never reaches a civ with no salt-holder clan; obsidian does. Repro: relay/specs/355/play/small-2.lines.jsonl, whole game. (Same lever as the open idea on claiming toward sources; not repeated in result.json.)
- ask: raise shows no strength gain from obsidian. Repro: small-2.lines.jsonl turns 292, 327.
- fix: store prints "salt -0.0". Names the Decided line "a store's panel lists every good". Repro: small-2.lines.jsonl turn 312 (view after `end 10`). Fits one build (clamp to 0).
- ask: "keep 45.0 ... (the default...)" label wrong after gift change. Repro: turn 357.
