# #356 build-4 (glaze): armies fed at others' markets, tuned

Every row is `sansfiat sweep --judge --json` on the seven retro maps, every civ on the AI, with the same map's
`exchange`-off control in the report. main = `sweep-main.json` (8d1b4ed); kiln = `sweep-kiln.json` (build-2's
numbers). `sweep-final.json` is the kept row, run with `--floor` (its metrics are the plain run's; the floor is the
noise of one extra roll at the pick).

**Headline.** With the kept numbers armies are fed at a market that isn't their civ's on 5 of 7 maps (35 army-turns,
control 0), on crumbs: 8.4 food bought where the armies ate 120.6 (7%). Health is **not** within the floor of main:
starving clan-turns 6,194 → 7,773 (floor 335) and starving share .0328 → .0359 (floor .0014). No setting tried
fed armies and held health; the one that held it (c) fed none. The review should weigh this; the one-number way back is
`KEEPING.packs` 0 (below).

## What stops an army (measured before tuning, `army_flow` in `core/tests/exchange.rs`, `--ignored --nocapture`)

- A pack's salt keeps the pack's food (`actors::spoil_keeping`): a pack of 2.4 people holding 11 food burns about
  1.1 salt a turn. An army filled with 1 salt a person (the kiln) has none left by the third day of a march; on the
  trace of one army 2.4 salt at the capital was 0.39 at the market and bought 0.2 food. So `UNIT.salt` must be well
  above what the march keeps.
- Armies carry salt on 5% of army-turns (3–60 of 400–750 a map at the kiln's numbers) because Fill happens only
  before a march past the ring; the rest are fed by their line. Foreign trades are therefore few however the
  numbers sit.
- A unit of salt sells for 0.07–0.5 food (the market's price falls as the clans there fill), so a full pack of 7 salt
  buys about 0.3–0.6 food against 2.4 an army eats a turn: the count is a count over crumbs, on every try.
- **Small 3**: no salt reaches the capital's store (mean 0.19 at any setting; its clans bring no salt), so no army
  has any to sell. **Large 8**: armies seldom march past the ring with salt (3 of 722 army-turns salted); it got
  1–8 army-turns on some settings and 0 on the kept one. **Medium 9**: 1 army-turn (0.00 food).
- `PLAN.trade_at` 3 or 4 fills more armies but fails `core/tests/units.rs`'s
  `on_a_real_map_the_ai_armies_on_their_own_land_are_fed_by_their_line` (an army on its own land is cut off, or none
  stands on it, at turn 387 of small 2). That test is a bound, so `trade_at` stays 2 (the kiln's).

## The tries

| try | `UNIT.salt` | `trade_at` | `KEEPING.packs` | `KEEPING.levy` | foreign army-turns (s2, s3, s6, s9, m8, m9, l8) | food bought / eaten | starving clan-turns (main 6,194) | starving share (main .0328) | goods people over control (main 1.0127) | obsidian mean/max steps | salt mean/max steps |
|---|---|---|---|---|---|---|---|---|---|---|---|
| main | | | | | 0 | | 6,194 | .0328 | 1.0127 | 5.35 / 18 | 3.52 / 11 |
| kiln (build-2) | 1 | 2 | 10 | 0.2 | 20 (1, 0, 5, 4, 5, 5, 0) | 2.15 / 80.0 | 7,428 | .0340 | 1.0350 | 5.28 / 18 | 3.73 / 14 |
| a | 3 | 4 | 10 | 0.2 | 18 (2, 0, 6, 4, 2, 4, 0) | 3.86 / 72.5 | 7,155 | .0354 | 1.0319 | 5.03 / 18 | 4.08 / 14 |
| b | 3 | 4 | 10 | 0.5 | 22 (3, 0, 10, 6, 2, 1, 0) | 7.24 / 68.9 | 7,269 | .0347 | 1.0181 | 5.50 / 18 | 3.40 / 11 |
| c | 3 | 4 | 0 | 0.2 | 1 (0, 0, 0, 1, 0, 0, 0) | 0.32 / 3.0 | 5,399 | .0292 | 0.9931 | 5.31 / 18 | 3.83 / 14 |
| f | 3 | 4 | 3 | 0.2 | 10 (0, 0, 8, 1, 0, 1, 0) | 2.17 / 29.6 | 7,568 | .0353 | 1.0149 | 5.41 / 18 | 3.96 / 14 |
| **final** | **3** | **2** | **10** | **0.5** | **35 (7, 0, 19, 5, 3, 1, 0)** | **8.38 / 120.6** | **7,773** | **.0359** | **1.0236** | **5.40 / 18** | **3.39 / 11** |

(Steps are the totals' `goods_vs_control.goods` means.) Five sweeps in all: a, b, c, f, and the final, run with
`--floor`. b had the best health of the fed rows but `trade_at` 4 fails the unit-land test above, so the final
takes b's numbers with `trade_at` back at 2. A diagnostic that runs the seven maps for 200 turns in 20 s
(`army_flow`) chose the candidates: foreign counts sit at 15–40 on every setting that fills and are noise-sized
between them, so health chose among them, until the unit-land test took `trade_at`.

- **c (no reserve) is the healthiest, and has no armies at a market.** The reserve costs health: the store's food
  spoils faster when its salt is spared from keeping, the cost of any fill. f (a reserve of 3) costs as much as 10
  and fills less. The kiln guessed this; c is the measure.
- Trajectories move by about ±1,000–1,500 starving clan-turns with any change in the second routes (build-1 saw
  it; the sweep's own `control_all` runs with exchange on, 5,509 → 7,058), and the one-roll floor of a plain
  sweep is 335 here (1,372 on b's run): it is an estimate with large spread.
- `KEEPING.levy` 0.5 is the lever on the reserve; it makes no salt, it takes a larger share of the salt the clans
  bring. On the diagnostic it is what put salt into the stores of more civs.

## The two numbers, the kept row beside the control and main (at +200, per map)

Army-turns fed at a market that isn't the civ's, counted from the pick to +200: **35 over the seven maps (7, 0, 19, 5,
3, 1, 0); main and the control 0.** Above 0 on 5 of 7 maps; 8.38 food bought where the armies needed 120.6 (7%):
a count over crumbs. 14 more army-turns were fed at the civ's own market (a settlement); 389 fills.

Largest share any one good has of trades at +200, by count and by worth (the trade group counts every row of a
clearing, food in nearly every one, so the largest is a food's by construction):

| map | run, by count | control, by count | run, by worth | control, by worth |
|---|---|---|---|---|
| small 2 | grain .193 | grain .194 | grain .373 | grain .364 |
| small 3 | shellfish .191 | shellfish .189 | grain .342 | grain .338 |
| small 6 | shellfish .193 | shellfish .189 | grain .358 | grain .363 |
| small 9 | fish .196 | shellfish .194 | grain .426 | grain .352 |
| medium 8 | shellfish .195 | shellfish .194 | grain .361 | grain .371 |
| medium 9 | fish .209 | fish .211 | none: 0 at +200 (see below) | grain .350 |
| large 8 | fish .199 | fish .198 | grain .368 | grain .361 |

Nothing stands on one side of most trades: the largest count share is about a fifth (five foods share the rest
evenly), the largest worth about a third to two-fifths, grain's. Salt's share by count at +200 is 1.3–6.2% (control
1.6–7.3%), obsidian's 0 on six maps and 0.3% on small 6 (control 0): obsidian almost never changes hands at a market,
since it sells only at a capital with room (`probe.md`). Salt's share by worth at +200 falls against the control
(small 2: 0.8% against 7.3%): salt sells at lower prices where armies also bring it (not tested). On medium 9 the +200
turn had no clearing that moved a good at a price in the run (the control has one): its worth shares at +200 read 0 and
its largest worth reads as obsidian by tie. A single turn's reading.

## Steps from source at +200 (mean / furthest, `goods_vs_control.goods`)

| good | main | build-1 | kiln | final |
|---|---|---|---|---|
| obsidian | 5.35 / 18 | 5.35 / 18 | 5.28 / 18 | 5.40 / 18 |
| salt | 3.52 / 11 | 3.68 / 11 | 3.73 / 14 | 3.39 / 11 |

Obsidian holds at or above build-1's. Salt's mean falls 0.13 against main (floor 0.09, flagged) and its furthest holds
at 11. Per map (main → final, mean/furthest) obsidian: small 2 8.32/18 → 8.81/18, small 3 3.80/13 → 3.95/13,
small 6 4.82/11 → 4.18/11, small 9 2.96/8 → 3.24/8, medium 8 3.90/11 → 4.06/12, medium 9 3.98/12 → 4.14/11, large 8
5.91/14 → 5.97/14.

## Health against main

`sansfiat judge --compare sweep-main.json sweep-final.json` (floors from sweep-final.json's `--floor` run):

| metric | main | final | floor | |
|---|---|---|---|---|
| starving clan-turns (`towns_vs_control.run.all.starving`) | 6,194 | 7,773 | 335 | **past** |
| starving share | .0328 | .0359 | .0014 | **past** |
| gatherer starving share | .0365 | .0410 | .0024 | **past** |
| settled starving share | .0040 | .0044 | .0002 | past |
| starving ratio (fields) | .8476 | 1.0363 | .1294 | past |
| goods people over control | 1.0127 | 1.0236 | .0131 | within |
| goods starving | .0265 | .0291 | .0026 | at the floor |
| armies people over control | 0.9991 | 0.9934 | .0274 | within |
| settle people over control | 0.9892 | 0.9900 | .0103 | within |
| worst map's starving ratio | 1.397 | 1.232 | .1874 | within (better) |
| armies' warriors starved | 329 | 282 | 47 | within (better) |

b's run (the same but `trade_at` 4) read 7,269 against a floor of 1,372 and .0347 against .0041: within. The floor
moves fourfold between runs, so read the health as +1,000 to +1,600 starving clan-turns on any setting that fills
(2.2% → 2.8% of clan-turns for the final), none on c.

The compare flags 107 rows past their floor (`PASS.cover` is untouched at 0.01); the groups:

- decisions: `claim` 16.5 → 10.5, `raise` 10.3 → 8.5, `disband`, `unpost`, `zone`, `levers_repeated` 4.6 → 5.4; `fill`
  is new.
- motion and land: `claim_taken` 220 → 71, `land_changed` 1,024 → 714, `claimed_200`, `festival_changes_per_100`,
  `flips_per_100`, `markets_200` 205 → 213.
- doubt, standing and led: `aligned_200` .52 → .48, `maps_with_lead_change` 3 → 4, led `won` 7 → 8, `over_idle`,
  `idle_early` / `idle_fell` 6 → 8.
- economy and armies: `largest_army` 14.4 → 4.0, `store_max`, `turns_at_pack`, `camps_kept` 16 → 14, `camps_lost`
  8 → 12, `fight.fallen` 52 → 26, `taken` 25 → 5, `capitals_fallen` 3 → 4, `armies_armed` 33 → 30,
  `army_inside_hungry` 506 → 416.
- goods: `festivals` +856, salt `held_200` 628 → 776, `civ_stores_200` 0 → 9 (the reserve, in civ stores), salt and
  obsidian `stores_200`; obsidian `festivals` 524 → 491.
- settle and towns: `civs_settled_200` 8 → 11, settlements founded 10 → 15, `towns_200` 10 → 13, the run's `traded` and
  `walk`, `town_clans_200`; `control_all.traded`.

The armies are smaller (`largest_army` 14.4 → 4.0) and fight less (`fallen` 52 → 26), and the civs lose more camps
(8 → 12). That is a bigger move than 35 foreign army-turns can explain; the likelier cause is the trajectory itself
(any change in the second routes did the same to the control runs), but it was not separated.

## The choice for the review

- **Kept: final (reserve 10, levy 0.5).** Armies are fed abroad on 5 of 7 maps; starving clan-turns +1,579 (+25%),
  starving share +.0031, both past their floors.
- **Fallback: `KEEPING.packs` 0** (c). Health at or better than main (5,399 starving clan-turns, share .0292), and
  an army carries salt only from a store that has some beyond its keeping: 1 army-turn on the seven maps. Fill and the
  planner's trade remain, so the rest of the piece stands.

## What the glaze changed

- `UNIT.salt` 1 → 3, `PLAN.trade_at` 2 (unchanged), `KEEPING.packs` 10 (unchanged), `KEEPING.levy` new, 0.5.
- `KEEPING.levy`: with exchange on, a store's levy takes 0.5 of a clan's salt (food keeps `CIV.levy`'s 0.2), in
  `civs::levy`. With exchange off the salt pays the food's share, as on main (all 21 digests equal main's:
  `digests-off.sh`).
- `Market.cleared` is derived (`#[serde(skip)]`) and written with exchange off too, so the control has a trade group
  (the kiln's control rows were all 0). The saved world and the digests are unchanged.
- `core/tests/exchange.rs`: `the_levy_takes_salt_at_its_own_share_…`; `army_flow` (ignored): the diagnostic above; the
  Fill test seeds 100 salt, since a pack now takes more than 5.
