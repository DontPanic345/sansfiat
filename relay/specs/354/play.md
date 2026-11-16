# #354 play note: small 2, seed 2, civ 1 (Red 424), +100 turns

Standing line (`sansfiat judge`): the game ended +100 at **29.4% rank 2**, against **42.3% rank 1** with every civ on the AI and **29.1% rank 2** for the idle led civ. The pick was 22.2%; Blue passed at turn ~290 and held 37.4%. The player beats idle by 0.3 points, inside the floor.

Played: 19 orders (claim 1, disband 1, move 8, raise 4, route 1, setGift 2, signpost 2); 86 of 100 turns with none. One refusal: raise 3 at turn 262 ("the store can't fill their packs", store 10.0).
The play game is `small-2.json`, its lines are `small-2.lines.jsonl`.

## What the rule did
- A signpost at 589 became a settlement at turn 269. `raise ... "at":589` worked. Army 448 moved there and showed `draws on its settlement`, and a "resupplied" feed line.
- Army 491 marched from the capital out of the claim to 346 (turns 305-315): cut off, pack 1.3 of 10, line 16.7 days. No other store was in reach, so nothing to switch to.

## Findings
- **fix** (view wording): army 448 and 490 at 589 say `at the capital` while `draws on its settlement`. Repro: lines turn 280-337, `army 448 at 589 ... at the capital ... draws on its settlement`. Breaks: "The view names the store a unit draws on."
- **fix** (Decided: a line runs to the store that feeds the unit for the least labour): army 490 stands on 505, one tile from the capital, and draws on the settlement at 589 with `line 6.0 days` from turn 283 to 362 (53 turns entrenched). The capital is one tile away. Repro: lines turns 283-362, `why 490` ("its line brings food to the camp" with no store named). The line is not re-chosen to the nearer store.
- **ask**: no `pillage` order was offered at any point, including beside Gold's armies and on tiles Gold had taken from me (0,3), (-2,3). The player could not test pillage.
- **ask**: the settlement at 589 showed `store 22.8 of 30 (levy 7.2)` at turns 287, 302 and 327 and 21.4 at 337. The store doesn't move for 40 turns; the player can't tell if it fills or what the levy is for. A second signpost at 415 got no refusal and no visible result.
- **ask**: gift 6 then 15 posted 50 a turn but `gave 0.0` and `clans joined` rose only with the claim (repeat of the open "'gave 0.0' stays zero" idea; not filed).
- **ask**: the move list repeats 410 (0,2) twice for 491 and 448 lists 410 (0,2) while at 589, 6 tiles away.
- **ask**: a march past the claim is cut off after two tiles with no warning in the order list; the player learns it from the feed after the fact.

## By kind (judge)
- **no choice**: turns 262-362: with one settlement the stores never differ for any unit except 490, which the player did not choose.
- **reason**: `why 490` doesn't name the store it draws on or why.
- **no result**: setGift 6 and 15 (gave 0.0 throughout); signpost 415 (no settlement in 40 turns); disband 491 (no feed line).
- **idle**: turns 292-340, Blue 31% to 38% and Red flat at 28%; the player had no lever that moved the standing (86 idle turns of 100).
