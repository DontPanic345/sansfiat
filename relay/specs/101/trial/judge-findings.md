Findings for the game: led civ 716 (Red), turns 283–391. I ran the judge once and resumed a copy of the game. `why 321` printed only the standard view and no per-clan reasons. I did not get at clan 321's reasons directly.

**no choice**
- Turn 283, `{"type":"forage","unit":719}`. The camp has one unit, and the note ("start feeding clans … surprised: store is empty at the pick") shows no alternative was weighed. The only other legal orders were move, zone, disband and raise. The judge counts 6 orders in 108 turns, 3 of them `setGift`.
- Turns 283–323, the `end 5` and `end 10` calls. 5 of the 6 orders were issued in the first 40 turns. Afterwards the player's whole policy was "keep feeding and let clans join". The AI control reaches a similar Standing (8.9% vs 10.0%) without it.

**reason**
- Turn 283, `setGift 4`. The note says "gift food to nearby clans", but the final view lists only one unaligned clan in reach: clan 321, lean 0.02 for civ 716 (join at 1.0). The player had no reason to expect gifts to turn into Standing.
- Turn 288, `setGift 12`. Note: "store grows but gave 0.0". The feed shows "291 gave food to 1 unaligned clan(s)" and "289 clan 707 joined it". So gifts did land (since founding: gave 22.7, clans joined 1), and "gave 0" was a misreading. The player then raised the gift to 30 on turn 323 for the same reason, with no better information.
- Turn 323, `setGift 30`. The note says "store is idle at 47, spend on clans". The view says "last turn posted 45.0, gave 0.0". The reach list has one clan at lean 0.02, so more gift cannot help. The player's own "could not tell" note says they never found out which clans a gift reaches.
- Turn 333, `move 719 to 194`. The player took the "suggested post" from the view ("the AI would post your camp at 194"). Their note ("gifts give nothing from the capital") was a guess. At turn 341 they moved the camp back, saying "pack falls and gifts still 0". The 8-turn detour changed nothing visible.
- Turns 351–381, notes "reach +100 while watching Standing". This is a goal about the turn counter, not a reason. Standing was 10.3% falling to 9.8% while Blue rose from 14.6% to 14.4–15.0% (judge, turns 353–383), and no decision followed.
- Turn 391, "could not tell" note. The player could not say why gifts went to 0, which clans a gift reaches, why their clans starved, or what raises a clan's lean. The view gives no per-clan reason for a gift being skipped. The leans (0.02, 0.00, 0.00) don't say what raises them.

**no result**
- Turn 283, `{"type":"setGift","food":4}`. Clans joined stayed at 0 until clan 707 joined at turn 289. No clan's lean is shown moving because of this gift.
- Turn 323, `{"type":"setGift","food":30}`. After it, "last turn gave 0.0" and the clan count stayed at 1 joined. The feed shows only "323 the player set its gift to 30.0 food a turn", with no effect.
- Turn 333, `{"type":"move","unit":719,"to":194}`. Neither the player's note nor the view shows any change in gifts or lean.
- Turn 341, `{"type":"move","unit":719,"to":166}`. Same: no visible result.
- Turn 333, `why 321`. This is the player's one information request. The final view shows the lean only as 0.02 for civ 716 and gives no cause.

**idle**
- Turns 341–391: no orders for 50 turns (the judge reports the longest run without orders as 49, and 103 of 108 turns without any). In that run Red's Standing fell from 10.8% (turn 363) to 10.0%. Blue led throughout (15.0% at 363). The feed shows "380 Clan 61 is starving" and "367, 368, 371, 372 people leave Red's clans for others", and the player responded to none of it.
- Turns 293–323: three `end 10` calls while Blue passed Red between turns 293 and 303 (judge: Red 9.4%, Blue 8.1% became Red 9.6%, Blue 10.4%). No decision followed.
- The game gave the player almost nothing to decide. The legal orders at the end are raise, disband, move, zone, switches, resupply, forage, route, setKeep and setGift. Only setGift pointed at Standing, and only one unaligned clan was in reach. The turn-391 view includes "raise n:1", which the player never tried, and the player's own notes never mention it.

Led civ 716's Standing at the last turn (+108): 10.0% (rank 2) in the game against 8.9% (rank 2) with every civ on the AI. Blue (717) leads on 14.9%.
