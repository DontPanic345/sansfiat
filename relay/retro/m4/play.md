# M4 play check

Verdict: one map played (small 2, civ 1, 103 turns past the pick); medium 8 and small 6 were not played. Standing 10.8% at the pick, 26.6% at the end (rank 1), against 29.4% (rank 1) with every civ on the AI and 22.7% (rank 2) idle. Files: `play/small-2.json`, `play/small-2.lines.jsonl`.

## small 2

Judge Standing: game 26.6% / AI control 29.4% / idle 22.7%. Lead changes 6 / 0. Clans changed 44 / 60. Claim held 48 / 61, guard 3.0 / 25.8 people. 11 orders in 103 turns, 95 turns with none, longest run 29. 0 refused.

## Findings (all `ask`)

- **idle**, turns 282-345: the store sat near 45 and "gave 0.0" for most turns after the unaligned clans in reach were joined. Gift 10 posted ~48 a turn, nothing was given, and the view named no lever to put the food to use. Repro: `relay/retro/m4/play/small-2.json.lines.jsonl`, turns 282-345.
- **no result**, turns 312-335: Standing was flat or falling (Blue passed me at +50) while the player had no order that moved it. No clan was in reach. The AI control reaches 29.4%, so the AI finds something the view did not offer. Repro: lines file, turns 292-335.
- **reason**, turns 326-335: disbanding the guard and moving the army 506 to 410 dropped held tiles from 39 to 2. The view reported it only in the claim line. Nothing said guard people hold tiles or how many tiles one holds. Repro: lines file, the disband and move after turn 312.
- **no result**, scout: `route` to an adjacent tile (turn 267) left "possible route 1.0 days, 0 tiles to search" for 100 turns. No result was ever visible.
- **no result**, pillage: no pillage order was ever listed. No rival was in reach on a small map (rival capitals 11 to 15 tiles away), so the army lever had no use. `could not tell` for army: what an idle army gains.
- **no choice**, gift: after the first 10 turns the gift of 6 to 10 reached only the few clans that trade at my market. Most unaligned clans traded at other markets ("your gift doesn't reach it"), so the gift amount had no choice in it.

## For other angles

| idea | source | value | cost | take |
|---|---|---|---|---|
| Show what a gift reaches and what is left unspent | gift findings | high | low | try |
| Say how many tiles a guard holds | land finding | medium | low | try |
| A reason on the scout's route to a near tile | scout finding | low | low | later |
| Play medium 8 and small 6 | this check | medium | one session | later |
