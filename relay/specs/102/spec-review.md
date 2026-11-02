# #102 spec-review: what the runs changed in the plan

Taken on `5156ad5` (main's rules), release build.

## The guard's scale

`actors::divide` gives each claimed tile `people × guarding / tiles`, less its walk from the capital,
`2 × path cost × weight`. A tile is held at a share of `CLAIM.floor` 0.1 or more.

Red on small seed 2 (`play --civ 1`, at the pick, turn 283): 12 clans, every home within 3 steps of the
capital. The stand-in claim (item 22: land within 1 step of a home within 4 steps of the capital) is
at most 37 tiles (hex count; any sea in it only shrinks it), at 0 to 4 steps from the capital, taken
as plains at cost 1 a step.

| Guards | Guarding a guard | Walk weight | Share a tile | Held past the capital |
|---|---|---|---|---|
| 3 | 1.0 (the spec's) | 0.2 (`UNIT.inside`, the spec's) | 0.081 | 0 |
| 3 | 3.5 | 0.2 | 0.284 | 1 |
| 3 | 3.5 | 0.05 | 0.284 | 7 |
| 6 | 3.5 | 0.05 | 0.568 | 37 |

With ring 2 (64 tiles) six guards at 3.5 and 0.05 hold 19. At the spec's numbers the AI's land would
be the capital alone everywhere: smaller than main's 2-step ring, nothing for leaning by land, the levy
or Standing to read, and build-2's sweep would show nothing. So the starting numbers are now
`GUARD.guarding` 3.5 (a guard's whole labour, `UNIT.labour`) and a new `GUARD.inside` 0.05 for the
guard's walk; three guards hold the near ring and six the whole stand-in claim, which leaves the AI's
raise-while-paper rule something to do and the canvas a working range.

## The store

Store at the pick and after (small seed 2, `play`, each civ led in turn on the AI's defaults):

| Civ | Pick | +1 | +2 | +3 | +10 | +20 to +50 |
|---|---|---|---|---|---|---|
| Red | 0.0 | 8.9 | 13.9 | 17.8 | 29.8 | 47 (posting 45 a turn) |
| Blue | 0.0 | 12.0 | 17.5 | 21.0 | | |

The levy brings in 5–9 food a turn early on, so `GUARD.start` 3 guards (3 food a turn) are fed from
the first turn; an empty store at the pick doesn't starve them.

## Leaving

Every leaning fades by the same share a turn, so the ratio of two leanings never changes on its own.
A clan whose own leaning (1.0 on joining) has faded for 100 turns holds 0.05; one gift from a rival
leaves more than that, and under the spec's rule as written the clan is then drawn for good and
leaves `leave_run` turns later without going near the rival again. The spec now asks the rival
leaning be at least `LEANING.leave_at` (0.3 to start) too.

## Smaller fixes

- `flips` counts a return within a fixed 10 turns (`JUDGE.flip_window`): build-1 measures main before
  `MARKET.keep_turns` exists.
- `held_trailed` is `worked_trailed`: "held" now means a civ's held claim.
- `Walks::path` leaves home out, so trails by traffic add home explicitly.
- A clan's current festival out of its reach (not among its worths) frees it, as a faded one does.
- The guard is divided at founding so the pick's cards read land.
- `solo_gain` is reported in two parts (clans and units won; unaligned people on the land), since the
  second is Standing by definition and the answer should say how much of the gain it is.
- The "wide claim is thin" play follows from the division at a fixed guard, so build-4 also finds how
  many guards a wide claim takes to hold what the ring holds, and whether the store feeds them.
