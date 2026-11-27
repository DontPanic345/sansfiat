# M5 play check

Verdict: one map played (small 2, civ 1, 110 turns past the pick; the 100-turn mark was passed by an extra `end 10`); medium 8 and small 6 were not played. Standing 21.5% at the pick, 36.3% at the end (rank 1), against 32.7% (rank 1) with every civ on the AI and 27.1% (rank 2) idle. Files: `play/small-2.json`, `play/small-2.lines.jsonl`.

## small 2

Judge Standing: game 36.3% / AI control 32.7% / idle 27.1%. Lead changes 0 / 2. Clans changed 60 / 60, claim 58 / 53 tiles. 18 orders in 110 turns, 99 turns with none, longest run 19. 0 refused. Blue's capital fell to Gold at turn 297 (Blue 25.1% to 0.0%); Standing rose mostly by claiming one tile at a time.

## Findings (all `ask`)

- **no choice**, gift, turns 281-391: gift 2, 8 and 20 all "reach no clan at your market"; gave 0.0 for 110 turns. Every unaligned clan traded at another market. The claim alone brought each clan (11 claim orders, 6+ clans joined). Repro: `relay/retro/m5/play/small-2.json.lines.jsonl`, the `setGift` lines.
- **idle**, turns 291-391: the store sat at 49.6-52 and "posted 50.0" each turn while nothing was given; the view named no use for the food. Repro: lines file, turns 291-391.
- **no result**, army: raised 2 warriors, moved, zoned and disbanded army 459; no rival was near and no pillage order ever listed, so no visible gain or loss. Repro: lines file, the raise, move, zone and disband lines.
- **no result**, scout: `route` 166 to 192 (one tile) left "0 tiles to search" for 110 turns, as in M4. Repro: lines file, turn 281.
- **no choice**, guard: no guard order was ever in the legal orders, so the land and guard lever had only claim. Repro: any view in the lines file.
- **idle**, turns 297-391: Blue fell to Gold at 297 (`why 454`) and the player had no order to answer it; the view showed only Blue 0.0%. Repro: lines file, turns 291-301.
- **reason**, turns 288-304: "people leave Red's clans for others" and "clan 475 is starving" appeared with no order that touched them; `why` on the clans was not checked. Repro: lines file, turns 286-304.

## For other angles

| idea | source | value | cost | take |
|---|---|---|---|---|
| Show where a gift can reach, or what unspent food is for | gift, idle findings | high | low | try |
| A visible answer when a rival capital falls | Blue finding | medium | low | later |
| Play medium 8 and small 6 | this check | medium | one session | later |
