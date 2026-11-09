# #155 play: small seed 2, civ 1 (Red), 100 turns

Judge: led civ Standing at +100 is 16.2% (rank 2) in the game, 26.0% (rank 2) with every civ on the AI, and 18.1% (rank 2) as the idle led civ. Blue ends at 31.1%.
Start 10.8% (rank 1). Store turns at a pack: 100 of 100. The judge's fields line reads "-" for the game's farming share and field tiles because it reads +200 and the game stopped at +100.

Fields were visible from turn 0 on the clan list ("fields 1 tile, 2.9 days, 8.0 grain") and the feed ("Clan 31 takes up fields at (3, 2)"). Clans that took up fields were fed 1.00.

## Findings

- **fix** (health): a claim order replaces the whole claim and the legal-orders line does not say so. One `claim` of 7 tiles at turn 292 cut a 56-tile claim to 8 and the guard's thinnest share to one tile. The result is not a fields problem. Whether to warn is a page matter for the owner, so this is `ask`, not `fix`; see ideas. Repro: relay/specs/155/play/small-2.json, turns 292-293.
- **idle**: 95 of 100 turns with no order, longest run 49. Standing was flat at about 16% from +50 on while Blue went 15% to 31%. The view offered no lever aimed at Blue's growth: no rival in reach, so army, pillage and zone had no target.
- **reason**: the player's turn-292 note ("claim already mine?") misread the claim order as additive; the legal-orders text "paints your claim" does not say it replaces.
- **no result**: `setGift 8` raised clans joined from 0 to 7 by +30, but `why 424` named the gift only as "gave food to 1 clan(s)". The player could not tell how much of the joining the gift caused.
- **no choice**: the scout's route (506 to 535, 4 days) gave nothing the player weighed. The scout has no unexplored tile in reach on this map.
- Fields vs Standing: the game's Standing fell against the AI control (16.2% against 26.0%). The player's claim shrink (guard 2.0 against 15.0, claim 18 against 57 tiles) accounts for part of the gap, so this game does not show fields as the cause.

Screenshots: not a page step (no all.mjs run).
