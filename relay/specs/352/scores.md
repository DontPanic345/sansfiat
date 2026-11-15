# #352 scores

`sansfiat sweep --judge --json` on the seven retro maps, every civ on the AI, 200 turns past the
pick (1,400 turns in all). Control: `main-sweep.json`, main at 22ed7a4 with only the judge's two
new measures added (`motion.land_joins`, `motion.land_wait`). Branch: `branch-sweep.json`.
`compare.txt` is `sansfiat judge --compare main-sweep.json branch-sweep.json --all`.

## The question: how fast is a clan on a civ's land that civ's, and how many change civ?

| | main | branch |
|---|---|---|
| clans that came to a civ with their home on its land, joining or leaving a rival | 405 | 239 |
| turns their home had been on that land first, mean | 10.0 | 0 |
| clans joined, all (`ClanJoined`) | 405 | 294 |
| clans that left a civ for another (`ClanLeft`) | 10 | 40 |
| clans changing civ per hundred turns | 29.6 | 23.9 |

On main a joining was a clan already on a civ's land, ten turns after its home came onto it.
On the branch a clan is its civ's the turn its home is on the land. The branch counts fewer
joinings because the clans a civ's first claim covers are its own from the pick and make no event:
on medium 8 the led civ starts with 24 clans and 109 people where main gives it 11 and 55, at the
same Standing, since main already counted the people on its land.

Clans that leave are four times as many. A tile an army takes carries its clans with it (small 2:
25 against 3; small 9: 14 against 6), where on main such a clan waited to lean.

## What else moved

| | main | branch |
|---|---|---|
| lead changes | 13 on 6 maps | 6 on 6 maps |
| swings | 16 | 9 |
| spread of Standing at 200 | 0.177 | 0.179 |
| aligned Standing at 200 | 0.468 | 0.459 |
| top civ's Standing at 200 | 0.234 | 0.238 |
| levy a turn | 9.5 | 9.1 |
| store, mean | 39.0 | 36.9 |
| claim tiles taken by an army | 36 | 30 |
| capitals fallen | 2 | 2 |
| fallen in fights | 26.5 | 10.1 |
| warriors starved | 170 | 191 |
| led games ahead of idle | 18 of 21 | 18 of 21 |
| panics, digest mismatches | 0, 0 | 0, 0 |

The lead changes less: 13 to 6, with small 3 going from 6 to 1 and small 2 from 3 to 1. On main the
ten-turn wait staggered each civ's joinings, and the staggering traded the lead while the civs were
close; with joining at once Standing follows the claim, and the claims grow at the same pace. No
noise floor was run (`--floor`), so how much of the 7 is chance is not known.

## The scout errand test

`core/tests/scout_errand.rs` plays each scout's first errand with fields and the fight off.

| | main | branch |
|---|---|---|
| scouts lost of 28 | 3, all on medium 9 | 2, both on medium 8 |
| people lost | 6 | 7 |
| hungry scout-turns | 172 | 105 |

The test held the lost to at most 4 and to medium 9. The count holds; the map moved, because who
pays a civ's levy at the pick moved. The test now holds the count alone.
