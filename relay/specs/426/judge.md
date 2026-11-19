# #426 judge: the review's verdict

From `branch-sweep.json` (d0ec466, the engine as it stands at ed14c2c) and `main-sweep.json` (59cd80f with
`--floor`), compared in `compare.txt`. Verdict: **worse**.

## The piece's question: do farmers come to markets on their own?

| score | main | branch | floor | moved? |
|---|---|---|---|---|
| towns at +200 | 10 | 11 | 1 | no (at the floor) |
| capital towns at +200 | 5 | 3 | 0 | down |
| clans in towns at +200 | 59 | 58 | 8 | no |
| towns formed after the pick | 23 | 34 | 0 | up, but fewer last |
| trade a person a turn, town clans | 0.043 | 0.161 | 0.003 | up |
| trade a person a turn, farmers outside | 0.038 | 0.049 | 0.006 | up |
| markets alive at +200 | 205 | 213 | 7 | up (just past) |
| clans at a market at +200 | 0.924 | 0.951 | 0.005 | up |

The lever fired: a town clan trades 3.3× a farmer outside (main: the same). Festivals hold a little
better. Farmers are not drawn: towns flat, capital towns down. By map, towns at +200 rise on small 3
(0→1) and large 8 (3→7), fall on medium 8 (2→0) and medium 9 (4→2).

## The four scores

| score | main | branch | floor |
|---|---|---|---|
| Standing: aligned share at +200 | 0.522 | 0.560 | 0.008 |
| Standing: top civ at +200 | 0.291 | 0.310 | 0.008 |
| doubt: maps with a lead change | 3 | 6 | 0 |
| doubt: swings | 14 | 9 | 4 |
| doubt: spread at +200 | 0.243 | 0.235 | 0.006 |
| decisions: AI orders per 100 turns | 85.7 | 79.0 | 5.2 |
| motion: clans changed civ | 410 | 343 | 14 |
| motion: land changed | 1024 | 692 | 78 |
| motion: people changed | 2282 | 1846 | 126 |
| led: over idle | 0.069 | 0.066 | 0.002 |
| led: ahead of idle (of 21) | 16 | 16 | 0 |

Motion falls well past the floor, most of it war: tiles taken 25→1, capitals fallen 3→0, fallen
warriors 52→13. Fewer orders, mostly fewer claims, raises and forages.

## Health and its costs

| | main | branch | floor |
|---|---|---|---|
| panics, digest mismatches | 0, 0 | 0, 0 | 0 |
| pre-games ended by centres | 7 | 7 | 0 |
| starving clan-turns (all) | 6194 | 9167 | 220 |
| starving share, gatherers | 3.7% | 5.0% | 0.1% |
| town clans starving (clan-turns) | 78 | 157 | 43 |

Health in band. Starving rises past its floor, most on medium 9's towns (0 → 82 clan-turns) and among
gatherers, who no longer bring food to trade for variety.

## The rule read against the Decided lines

- No cap: built (`MARKET.carry` gone).
- A load goes while its trade is worth more than its walker's days, stopping at the first that is not:
  built, `markets::load`.
- One walker per 2 food, at least one: built (`actors::carriers`, continuous, so each further load adds
  one walker).
- A clan on its festival's tile brings its whole surplus: built, surplus read as the food beyond a food
  a person.
- No tie rule; a clan moves when the new home feeds it better: built, with the load's trade gain added
  to a home's value (`homes::step`).
- **Departure**: a clan with no surplus brings no food, even where its first load (which rides with
  the walker who goes anyway, so it costs no days) would trade for more food. The Decided line would
  have it bring that load. The builder kept it out because "first load + surplus" ran medium 9's
  pre-game to the backstop (6 of 7 by centres, health). That variant otherwise scored best (15 towns,
  5 capital towns, starving 1.6% against main's 2.2%). This is a rule, and the owner's choice.
- `labour.md`'s new ledger line writes the departure in ("one with none brings no food").
- `market::clear` bounds prices within 10^12 of their mean. Main never reached it. It is a guard, not
  a rule.

## Code read cold

No defects found. `trade` reads a route's walk as cost ÷ `Clan.walkers`, so a route set without
`walkers` (only tests do this) reads the whole cost as one walker's walk; `notes.md` says so.
`load` calls `gain` once a load; a bench shows 35.9 ms a turn against 33.8.
