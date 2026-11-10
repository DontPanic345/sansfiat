# #156 build-4 pick: B

Version A is `relay/156-build-4-a` (canvas, Opus low). Version B is `relay/156-build-4-b` (canvas, rung b).
Both built the home rule from build-3 (efa0687) and swept the seven retro maps against the no-towns control
in the same sweep. Each number below comes from that version's `canvas.md` (marked A or B), unless it is
marked "pick". "Pick" is my own sweep of B with A's "no field step away from the festival" rule added.

## The numbers

| Bound / score | A (A's) | B (B's) | B + A's away rule (pick) | Bound |
|---|---|---|---|---|
| 1. Towns at +200 (maps) | 14 (5 of 7) ok | 14 (4 of 7) **MISS** | 14 (6 of 7) ok | ≥ 10, ≥ 5 maps |
| 2. Town clans at +200, farmers | 79, 66 (0.84) ok | 80, 60 (0.75) ok | 71, 53 (0.746) **MISS** | ≥ ¾ |
| 2. Towns working fields | 14 of 14 | 14 of 14 | 14 of 14 | all |
| 3. Walk: town / farmers outside | 0.000 / 0.180 ok | 0.000 / 0.194 ok | 0.000 / 0.175 ok | under half |
| 4. Traded: town / farmers outside | 0.0487 / 0.0378 (1.29×) ok | 0.0451 / 0.0357 (1.26×) ok | 0.0422 / 0.0364 (1.16×) ok | ≥ 1.1× |
| 4. Traded, all clans / control | 0.0600 / 0.0605, floor read 0.0000 **MISS** | 0.0601 / 0.0605, floor 0.0002 **MISS** | 0.0594 / 0.0605, floor 0.0003 **MISS** | ≥ control − floor |
| 5. Clans change hands (control 177) | 201, floor 11 ok | 218, floor 21 ok | 204, floor 7 ok | ≥ control − floor |
| 5. People change hands (control 953.3) | 1021.9, floor 73.3 ok | 1124.0, floor 91.6 ok | 1076.1, floor 28.4 ok | |
| 5. Land changes hands (control 1150) | 1864, floor 348 ok | 1453, floor 205 ok | 1287, floor 69 ok | |
| 6. Aligned Standing +200 (control 0.318) | 0.330 ok | 0.328 ok | 0.326 ok | ≥ control − floor |
| 7. Starving, town clans | 0.033 | 0.0007 | 0.0032 | reported |
| 7. Starving, farmers outside towns | 0.008 | 0.0038 | 0.0049 | reported |
| 7. Starving, all clans (control 0.0132) | 0.0166 | 0.0143 | 0.0136 | reported |
| 7. Who starves named | town clans: crowding on the shared home tile | daughters of town clans and gatherers near towns: crowding; small 9 and small 6 not explained | — | no mistaken starving |
| 8. Settled other moves / 100 | 0.004 ok | 0.027 ok | 0.002 ok | < ½ gatherers' |
| 8. Town steps / 100 | 0.367 | 0.141 | 0.143 | reported |
| 8. Town clans' moves off / 100 | 0.097 **MISS** | 0.000 ok | 0.006 ok | < ½ gatherers' |
| 8. Gatherers' home moves / 100 | 0.123 | 0.111 | 0.128 | |
| 9. Store: pack maps (control 5); never-pack civs (control 1) | 5; 2 ok | 6; 1 ok | 4; 1 ok | each no worse by > 1 |
| 10. Bench large 8, ms a turn (main) | 25.9, 25.2 (65.2, 64.6): 0.39 ok | 25.7, 25.8 (64.4, 64.5): 0.40 ok | not run | ≤ 0.45 |
| 11. Pre-games 7/7, panics 0 | yes | yes | yes | |
| 11. check.sh | green, in parts (A's) | green (B's; and pick: green on the landed branch) | — | |
| 11. Towns off equals build-3 | every no_towns block | 56 blocks equal main (= build-3) | — | |
| Bounds missed | 1 and 4b; 7 is open (see below) | 1 and 4b; 7 is open on two maps | 2 and 4b | |

The pick's sweep: 2 min 6 s on this runner. Towns formed 20, most homes at +200 9. Starving counts: town clans
36 of 11,094 clan-turns, farmers outside 201 of 41,305, all 3,824 of 281,611, control 3,567 of 269,208.

## What the diffs show beyond the numbers

- **Who the towns feed.** The piece asks whether towns feed their clans. In A, town clans starve 4 times as
  often as farmers outside towns (0.033 against 0.008), and A puts this down to crowding. B found a rule
  behind clusters of landless homes and changed it: a farmer that stepped onto a held tile and then divided
  gave the daughter its one claimed tile, so the mother kept a home it didn't hold and gathered nothing
  (`dividing::divide_land`, `clans::eat`). A doesn't have this fix, so some of its town starving is likely
  this mistake rather than crowding. In B, town clans starve at 0.07%, below farmers outside towns.
- **The value weighed.** A leaves out the store but keeps the store's cap: it reaps up to the full target
  with an empty store. B weighs the uncapped harvest (`clans::could_reap`). B traced why: with the cap, a
  farmer at its target valued the step and staying equally to the fourth decimal about a third of the time.
  Both are recorded as decisions. B's is closer to the design's reason for the step: surplus that doesn't
  walk to market.
- **Non-farmers on a shared home.** A gates only a clan on its festival's tile (`homes::keep`). B gates any
  clan that isn't a farmer and lives on a shared home. B's test needs no town and takes town moves off to 0.
  A's still misses that bound (0.097).
- **The judge.** B fixed build-3's floor key for "traded over all clans": it read a path that doesn't exist,
  so the floor was always 0. A reported the 0 floor and guessed at the cause. With the fix, both versions
  still miss bound 4b by about 0.0004–0.0005.
- **Code.** The two have the same shape: one weighed choice in `homes.rs`, called from `adjust`. B caches the
  festival search per tile for the turn (`homes::Prices`), and both stay under the bench bound. B's stage test
  checks one tile nearer the festival, not a step onto its tile; A's checks the same. Neither test proves the
  step lands on the festival's tile itself.
- **B's open questions.** Small 9 starves at 28 times its control with no town there. On small 6, two
  ex-farmers on a shared home starve. B reports both and does not explain them. A's per-map table also has
  small 9's farmers outside starving at 0.020, so this is not B's alone, but it is not traced.

## Adding A's away rule to B

A's "a farmer with a festival never steps toward its fields away from it" is the part of A that might beat
B on bound 1. On B it gave 6 of 7 maps, but farmers among town clans fell to 53 of 71 (just under ¾), and
traded over all clans fell to 0.0594. It swaps one miss for another, from one sweep, so I didn't land it.

## Chosen: B

Both versions miss two bounds, and both miss 4b by the same small amount. A also misses the moves-off bound,
and its town clans starve far more than farmers outside. B misses the town-maps count by one map. B's town
clans starve at 0.07%, and it fixed a mistaken starving rule and the judge's floor. "Towns feed their clans"
is the question this piece answers, so B's version lands, unchanged, as a fast-forward of `relay/156`.
