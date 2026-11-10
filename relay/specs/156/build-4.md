# #156 build-4 (canvas): towns that form and feed

Give a farmer's home the step that takes it to its festival when living there feeds it better. Then make the
towns that follow feed their clans, trade more and walk less, on the retro maps against the no-towns control,
with a large map's turn under its bound. Read `spec.md` in full: its table holds every bound, and its
"Unsettled" says what you may change. `probe.md` is the spec step's sketch of this rule (`probe.diff`) and what
it did on the maps: towns formed, and starving ran at three times main's, part of it from gathering shared
homes twice.

What the easels give you:
- build-1's homes (`core/src/homes.rs`): a home in `territory` that may stand on a tile another clan claims,
  gathering only on one's own claim, `claim_land`, `homes`, shared field ground (`fields::room`,
  `Clan.sown`), and `Config.towns`, which nothing reads yet. `homes.md` holds its proof.
- build-2's scale: the festival costs from one tile (`scale.md` names the function), `markets::by_market`,
  the AI's path cache and the scouts' bound. The large turn sits at or under 0.35 of main's with no towns.
- build-3's town: `towns::towns`, `TOWN`, `TownFormed`, the page's cluster and mark, and the sweep's seventh
  run. `sansfiat sweep --judge --json --floor` prints every bound in the towns block, pass or miss, and
  `total.towns_vs_control` in the JSON. `judge.md` is the starting point.

## What to build first: the home rule

In `homes.rs`, called from `clans::adjust` where the farmer's home step is now, with `Config.towns` on and
`fields::open`:

- A farmer that isn't the civ's own clan may step its home one tile toward its festival's tile, onto any
  passable land tile, whoever claims it. It steps when living there feeds it better: its value
  (`clans::value_at`) from the new home, with the new tile in its territory (trimmed to the radius) and its
  festival route priced from there by build-2's per-tile costs, beats staying by `TOWN.margin` (start 0.01
  food a person, as `FIELD.margin`). The new home tile joins its territory, and `claim_land` claims it only if
  nobody holds it.
- One home step a turn. The sketch ran the field step and then the town step in one turn, and settled farmers
  hopped between them. Make them one choice: the farmer weighs the step toward its busiest field
  (`fields::home_toward`) and the step toward its festival, and takes the one that feeds it better, if either
  beats staying.
- With `Config.towns` off, the farmer's home never steps onto a tile another clan claims: today's game.
- `SAVE_VERSION` bumps and the pins move. Tests in `core/tests/towns.rs` on a stage: a farmer by a festival
  steps onto the festival's tile when its surplus makes the walk dear, and not when the walk is cheap. It never
  steps more than a tile a turn. The civ's own clan stays. With towns off it never lands on a held tile.

## Then the painting

Run the sweep and read the towns block. The sketch's flaws that build-1 removed (gathering a shared home twice)
were part of its starving, so measure first before changing anything else. Then, until every bound holds:

- **No town starves by mistake.** Find who starves: town clans, the clans round a town squeezed for land,
  daughters of town clans, unaligned or a civ's, soon after a town forms or late. Where a rule starves them
  other than as the design means, change it, within the piece's Decided: a home is not a claim, gathering stays
  exclusive, ground is shared by area, a town has no rule of its own. For example, how a crowded home tile's
  ground is shared, or what the farmer weighs before stepping in. Record each change to a rule as a decision.
  Starving from growth and crowding is the world's consequence (`docs/process.md`, Health): don't tune a number
  to lower it.
- **Settled farmers stay.** Settled farmers' home moves, less their town steps, stay under half the gatherers', and
  town clans don't step off their town at more than that rate: no hopping between the fields and the market.
- **They walk less, they trade more.** These should follow from homes at the market. Every clan brings
  `MARKET.carry` a person, so trading more is what the clearing swaps (`World.traded`), which grows with the
  festival's crowd. If town clans don't trade more, find out why and say so.
- **Scale.** `sansfiat bench --size large 8 --after 50` at most 0.45 times main's, measured in the same
  session, two runs each. The home rule prices routes for every farmer each turn, so reuse build-2's per-tile
  costs and price only farmers that have a festival and aren't on its tile.

You may change the easels where a bound needs it. Keep the names the page and the judge read. `TOWN.homes`
stays 3: it is when a tile reads as a town, which the spec settled.

## The bounds

From one `sweep --judge --json --floor` against the no-towns control in the same sweep, and the bench:

1. Towns at +200 on at least 5 of 7 maps, and at least 10 over the seven.
2. At least three quarters of town clans at +200 are farmers, and every town's clans work fields.
3. Over the clan-turns after +100, town clans' festival walk is under half that of farmers outside towns.
4. Town clans trade at least 1.1 times the food per person that farmers outside towns do. Traded per person
   over all clans is no lower than the control's, less the floor.
5. People, land and clans change hands no less than in the control, less the floor.
6. Aligned Standing at +200 no lower than the control's, less the floor.
7. `canvas.md` names who starves round a town and why, and no clan starves for a reason the rules don't
   intend. The starving shares, town clans' beside farmers outside towns, are reported against the control,
   not bound.
8. Settled farmers' home moves per 100 clan-turns, less their town steps (a step a tile nearer the festival's
   tile), under half the gatherers', and town clans' moves off their town under the same half. Town steps are
   reported.
9. The store: maps with the top farmer at a pack half its turns, and civs never at a pack, each no worse than
   the control's by more than 1.
10. `bench --size large 8 --after 50` at most 0.45 times main's.
11. Pre-games by centres 7 of 7, panics 0, `engine/check.sh` green, `www/play/towns.mjs` passing. With towns
    off, every map's scores equal build-3's.

## How

Each sweep takes about two and a half minutes on the runner with the seventh run, and the floor's null
doubles it. Run two settings at a time where you can. Keep a table of every setting you run, with each bound
and the bench, in `relay/specs/156/canvas.md`, and end it with the setting you land, the sweep's towns block,
`judge --compare` against main, and the bench beside main's.

## If a bound can't be met

Stop when the nearest setting misses only on a bound that the piece's Decided stops you meeting (for example,
a town whose clans starve by mistake if only a rule outside the piece would fix it). Land that setting, and send an `ask` that
names the bound, the nearest numbers, and the rule that would meet it.

## The twin

This step runs twice from the same commit, on two rungs, and a pick compares them blind. Both versions
report the same runs: `sansfiat sweep --judge --json --floor` on the seven retro maps against the no-towns
control in the same sweep, `judge --compare` against main's sweep, and `bench --size large 8 --after 50` beside
main's, two runs each. Put the landed setting's towns block first in `canvas.md`, so the two read side by side.

## Done

Every bound holds, or the `ask` above. `canvas.md` written, `check.sh` green, the decisions recorded.
