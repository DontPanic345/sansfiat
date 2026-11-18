# #341 build-2 (canvas): capitals that become towns, and towns that hold

**Result.** A rule within the levers reaches the target: capital towns at +200 on **two of the four small maps**
(small 2: 506; small 9: 243), towns at +200 18 on 6 maps, the goods run's starving, all clans' and the farmers'
outside towns better than main past the floor. It is **not committed**, because it turns two real-map tests red
that this step may not loosen: `core/tests/cases.rs`, `settled_scouts_leave_nothing_unrevealed_that_could_beat_them`
and `a_settled_route_has_no_cheaper_trail`, both on "medium 8 to the rival: the scout has not settled in 200 turns".
Every other test passes (the rest of `cargo test`: 46 binaries ok). The rule is `drift.diff` (apply from the repo
root with `git apply relay/specs/341/drift.diff`): `TOWN.drift` and the toward-festival tie in `homes::step`, two tests
in `core/tests/towns.rs`, `SAVE_VERSION` 28 and the pins. The branch holds the easel, as build-1 left it.
Below, "kept" means the drift rule in that diff.

Control: main at 8d1b4ed (`main-floor.json.gz`); the easel is build-1's door step. Reports: `canvas-sweep.txt`
(the kept rule's `sweep --judge`), `canvas-compare.txt` (`judge --compare` main against it),
`canvas-towns-table.txt` (`towns.py`: main, easel, kept). The tries before the kept rule were read from a
seven-map probe (`Stage::retro`, to +200, `towns()` against each alive civ's capital; the same maps as the sweep,
and it reproduced the easel's 18 towns exactly) and, for D, a sweep.

## What I found about the civ's own clan

- Small 6, civ 263: its capital 378 falls to another civ at turn 5 (`CapitalFell`), so it has no own clan to keep
  and a fallen civ's capital is not a capital in the judge (`capital_civ` reads alive civs). The three farmers
  the spec saw on 378 at +30 are a town on a fallen civ's market, not a capital town.
- Small 3, civ 198: its own clan 85 starves with its warriors and merges (`markets::merge`, under `CLAN.min_pop`)
  into its child clan 205 at +58; `repair` then passes the civ to clans at 68, 156, 161, none near the capital
  266. Civ 199's capital 315 falls at +100. Neither capital has a farmer within four tiles or river ground within two.
- Small 9, civ 332 (capital 243) and civ 333 (569) lose their own clans the same way; 332's capital has two clans
  on it at +200 and its civ clan (2) lives one tile off, at 272.
- So the repair is the cause on small 9 only: passing the civ to a clan on the capital first (C) gives 243 its
  third home. It does nothing on a map where the capital has no farmers near. C is not kept: the drift below
  gives 243 its town without it (D + C = D).

## The tries

| try | rule | towns at +200 (maps) | capital towns, small maps (of 10 alive) | capitals over the seven (of 18) | trade a person, town / outside |
|---|---|---|---|---|---|
| main | | 10 (4) | 1 on 1 map (small 9: 472) | 5 | 0.0427 / 0.0803 |
| easel (A) | step onto the festival's tile on a tie | 18 (6) | 1 on 1 map (small 9: 472) | 6 | 0.0476 / 0.1006 |
| C | A, and `repair` passes the civ to a clan on its capital first | 18 (6) | 2 on 1 map (small 9: 472, 243) | 7 | not swept |
| D | A, and a step one tile toward the festival from any distance on a tie | 20 (6) | 3 on 2 maps (small 2: 506, 219; small 9: 243) | 8 | 0.0457 / 0.0901 |
| D + C | D and C | 20 (6) | 3 on 2 maps (same as D; 472 is lost) | 8 | not swept |
| D, within 2 or 3 tiles | the drift only from up to 2 (3) tiles out | 18 (6) / 20 (6) | 1 on 1 map (small 2: 506) | 6 | not swept |
| **kept: D within 4 tiles** (`TOWN.drift`) | the drift from up to 4 tiles out | **18 (6)** | **2 on 2 maps (small 2: 506; small 9: 243)** | **7** | **0.0438 / 0.0926** |

D is the rule that gets the most, and it fails `core/tests/units.rs`
(`on_a_real_map_the_ai_armies_on_their_own_land_are_fed_by_their_line`): at +100 on small 2 civ 469's army stands
on its own land at tile 527, eight steps from its capital, with a line that costs 27 days and carries half
what four warriors eat. The planner put it there; any change to small 2's history moves it. I did not loosen
the test. The drift limited to four tiles keeps two capital towns on two small maps and the test passes; five
tiles and over fail it as D does, two and three lose small 9's capital town. Four is the largest bound
that keeps small 2's army where the test reads it, and it is a search result, not a derived number.

The tie in food needs the clan to carry something (`weight(basket) > 0`): `a_farmer_steps_toward_its_festival_when_its_surplus_makes_the_walk_dear`
holds a clan with nothing to carry at home, and on the maps every clan carries something, so the clause moves no map (D's
sweep and the same rule with the clause have the same digest).

## The three scores, main / easel / kept

| score | main | easel | kept |
|---|---|---|---|
| towns at +200 (maps with one) | 10 (4) | 18 (6) | 18 (6): small 2, 6, 9, medium 8, 9, large 8 |
| capital towns at +200, small maps | 1 on 1 map | 1 on 1 map | **2 on 2 maps** (small 2: 506; small 9: 243) |
| capital towns at +200, all maps | 5 | 6 | 7 (small 2, small 9, medium 8: 1145, medium 9: 547, 780, large 8: 2689, 2156) |
| trade a person, town / outside | 0.0427 / 0.0803 | 0.0476 / 0.1006 | 0.0438 / 0.0926 |

Capital towns hold: under D (probe, every ten turns to +240) small 2's 506 has eight clans from +30 and 219 three from +80,
small 9's 243 three from +40. The easel's 506 did not hold: it had eight clans at +50, seven of them starving small clans
that merged into clan 44 between +89 and +101, and a market that formed at 531 at +95 took the rest (its festival changed,
and the clans' home step follows the festival). The kept rule's capital towns are read at +200 only. Town clans still trade about half as much a person as clans
outside (`MARKET.carry` caps a clan's trade whatever its tile; lifting it at the door is the owner's idea).

## Health, main against the kept rule (`canvas-compare.txt`)

| metric | main | kept | floor |
|---|---|---|---|
| digest mismatches | 0 | 0 | |
| goods run: starving | 0.0265 | 0.0198 * | 0.0019 |
| all clans starving (clan-turns) | 6194 | 4630 * | 220 |
| farmers outside towns, starving | 376 | 211 * | 111 |
| outside a town, starving (a share) | | 4449 of 264,252 = 0.0168 | |
| town clans, starving | 78 of 10,019 = 0.0078 | 181 of 13,589 = 0.0133 * | 43 |
| warriors starved | 329 | 314 | 16 |
| camps kept / lost | 16 / 8 | 16 / 8 | 2 / 0 |
| standing: aligned at +200 | 0.5216 | 0.4999 * | 0.0079 |
| fight: tiles taken | 25 | 16 | 9 |

Better than main past the floor: the goods run's starving, all clans', the farmers' outside towns. The armies and
digests do not move. Two rows are worse than main past the floor, and I could not bring them back:

- **Town clans' starving (0.0078 → 0.0133).** By map (starving clan-turns of town clan-turns): small 6 25 of 63 and
  small 9 8 of 795, medium 8 33 of 2138 (main 2 of 1604), medium 9 48 of 2792 (main 0 of 3105), large 8 65 of 7063
  (main 63 of 4998), small 2 2 of 738. Small 6's town at tile 82 is three clans that starve for most of the 60 turns
  they stand there (the easel's is the same: 29 of 75); medium 8 and 9's are clans on a market's tile where main had
  almost none (I did not trace which clans or why they starve). The all-clans count falls by a quarter, so the rise is
  not the world's; the judge reports this row and binds nothing on it.
- **Aligned standing (0.5216 → 0.4999; easel 0.5035).** By map (main / easel / kept): small 2 0.992 / 0.889 / 0.853, small 3 0.481 /
  0.483 / 0.472, small 6 0.378 / 0.327 / 0.355, small 9 0.801 / 0.838 / 0.802, medium 8 0.263 / 0.261 / 0.253, medium 9 0.575 /
  0.579 / 0.559, large 8 0.161 / 0.149 / 0.206. Small 2 is nearly all of it: main's game there is one civ taking most of the
  map (10 tiles taken), the easel and kept rule stop that (0 and 5 tiles). Large 8 rises by 0.045, the rest move by less than 0.02.
  It is the war on small 2 that differs, not the towns' effect on standing elsewhere.

Warriors starved on large 8 goes 12 → 46 under the kept rule (easel 12) and is hidden in the total, which stays within the floor.

## Why not more

- Small 3: no farmer within four tiles of either alive capital and no river ground within two; its civ 198 loses its own
  clan at +58 and civ 199's capital falls at +100. No home rule makes a town where nobody farms.
- Small 6: civ 263's capital falls at turn 5. Civ 264's capital 68 has six farmers within four tiles and four river tiles
  within two, yet the map's one town forms at 82, not on 68. I did not trace why; the home step takes a clan toward its
  own festival, so it is likely those farmers' festival is not the capital's. Civ 265's capital has no farmer near.
- Small 2 and small 9 are the two that make it. Small 2's other capitals: 219 is a town under a drift of five tiles and over
  (three homes) and not under four; 725 has no farmer near.
