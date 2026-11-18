# #341 build-1 (easel): the door step, the capital's town proved, the judge's towns group, the mark on a capital

Read `relay/specs/341/spec.md` and `measure.md`. Everything here is settled; build-2 (the canvas)
tunes where towns form on top of it, and reads your judge counts and your sweep.

## 1. The door step (`core/src/homes.rs`)

Apply `relay/specs/341/door.diff` (it was cut from main at 8d1b4ed; `git apply` it or make the same
change by hand). In `homes::step`, a step whose home is the clan's festival's tile is taken when it
feeds the clan at least as well as staying (`v >= now`) and its festival walk from there
(`Prices::routes`, summed) is less than from its home now, besides the usual `v > now +
TOWN.margin`. The step chosen is still the best value among those taken.

- Update `homes::step`'s doc comment to say it, in a sentence.
- `TOWN.note` in `core/src/data/clans.rs` gets the reason: walking less to the festival buys a
  clan nothing in food when its fields are capped by their ground and its gathering by its tiles'
  food, so a step onto the festival's tile that feeds it as well is taken (relay/specs/341/measure.md).
  No new number.
- `SAVE_VERSION` 26 → 27 in `core/src/world.rs`, and the pins in `cli/tests/save.rs` (run the
  digests its comment names and paste them).

Tests in `core/tests/towns.rs`, beside the farmer-by-festival tests (`farmer_by_festival`, `send`,
`chosen` are there to reuse): a farmer one tile from its festival, whose land gives it as much from
the festival's tile as from home, steps onto the festival's tile; the same farmer whose step onto it
would feed it less stays; with `Config.towns` off it doesn't step. The file is 472 lines: if it
passes 600, move the home-step tests to `core/tests/town_homes.rs`.

## 2. A capital's festival is a festival for the town rule (`core/tests/towns.rs`)

A test, no rule change: a civ staged on plains (`Stage::flat`, `stage.civs`), and two more clans
living on its capital: `towns()` lists the capital's tile with the civ's market (`Civ.market`)
and three clans, and a turn of the `markets` step (or `towns::check`) emits one `TownFormed` there.

## 3. The judge's towns group (`core/src/judge_towns.rs`, `cli/src/sweep_towns.rs`)

- `TownCiv` gets `capital_towns_100` and `capital_towns_200`: at +100 and +200, 1 under a civ when
  its capital's tile reads as a town (it is in `towns()`), counted under that civ. `add` sums them.
- `TownCiv` gets `outside: TownGroup`: every clan not living in a town, over the clan-turns after
  +100, as `town` and `farmers_outside` are. `share` shares it.
- `TownScores` gets `capitals_200`: the civs alive at +200, so a reader can say "N of M".
- `sweep_towns.rs`: the map line gives capital towns at +200 and traded per person town / outside /
  farmers outside; the totals add a line "capitals become towns: N of M capitals at +200, K of L on
  the small maps" (the small maps: `size == "small"`), and "town clans trade more" reads town
  against outside and farmers outside. `TotalTowns` carries what the lines need. Fix its module
  doc's "Every map reads no towns today": say what it reads.
- `relay/specs/341/towns.py` reads the new fields: a `cap200` column (capital towns at +200) and
  `outside` from `towns.total.outside.traded_per_turn` (it computes it from `all − town` today:
  keep that as a check that both agree).
- A test: `the_judge_counts_town_clans_their_walk_and_the_farmers_outside_towns` in
  `core/tests/towns.rs` is the pattern. A capital town at +200 counts 1 under its civ, and a clan
  outside the town counts in `outside`.

## 4. The page's town mark on a capital (`www/map/layers/towns.js`, `www/play/towns.mjs`)

The capital's house covers the town mark: the house is at least `0.95 × 64/22 ≈ 2.8` world units
wide, centred on the tile; the mark is 1.1 wide at 0.7 above the centre; and `capitals` draws after
`towns` in `LAYERS` (`www/map/view.js`). Check that first with the script below, before you change
it. Then draw the mark on a capital tile where it shows above the house, the same mark as on any
town tile ("The page marks the capital's town as any other"). Moving it up on a capital tile, or
drawing towns after capitals, are both fine; pick what reads best in the screenshot.

`www/play/towns.mjs` stages a town on the capital and turns off `capitals` (and units, centres,
rings) before it checks the mark. Make the mark's check (layer on against layer off) run with the
`capitals` layer **on**, and keep the clans' colour check as it is. The script must fail on main's
layer and pass on yours: run it once before your change and say so in the evidence.

## 5. The sweep (`relay/specs/341/build-1-evidence.md`)

After `cargo build --release`, run `sansfiat sweep --judge --json > /tmp/b1.json` (about 3 minutes)
and `sansfiat sweep --judge` for the text's towns lines. In the evidence file: the `towns.py` table
for your run (main's is in `tables.txt`), the capital towns map by map, and `judge --compare`
against main's report for Standing, doubt, armies and starving totals. Main's report, with its
`--floor` noise, is `relay/specs/341/main-floor.json.gz` (`gunzip -k`). Don't build or run main.

Done: towns at +200 **18 on 6 maps** exactly, as `tables.txt` gives for rule A (the rule is the
same). Every row that was already in the report before your change must match
`relay/specs/341/door-vs-main.txt`'s B column exactly too. The sweep is deterministic, and spec-review
got the spec's numbers to the last digit. For example: aligned standing 0.5035, warriors starved 290,
goods-run starving 0.0294. If they differ, find out why before you go on: the rule isn't the
prototype's. Capital towns: 6 at +200, 1 on the small maps (small 9), per the spec's probe. The
denominator is the civs alive at +200, whatever it reads.

Run `engine/check.sh` once at the end. Commit as `#341 build-1: the door step, capital towns in the
judge, the mark on a capital`.
