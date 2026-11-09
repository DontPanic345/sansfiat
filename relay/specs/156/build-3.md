# #156 build-3 (easel): the town, the judge, the page

A tile with several homes and a festival reads as a town. This step gives the town its reading, its event,
its look on the page and in the text view, and its scores in the judge, with the no-towns control beside it.
No home is shared until build-4, so everything here is proved on a staged town. Read `spec.md` items 6–8 and
10, its table (every bound the judge prints), "What I settled" (when a tile reads as a town, the page's look),
and decision 7.

The code: build-1's `core/src/homes.rs` (`homes`) and `core/tests/towns.rs` (its staged shared home),
`core/src/markets.rs` (`Market`, `form_and_fade`), `core/src/events.rs`, `core/src/data.rs`,
`core/src/judge.rs` and `core/src/judge_fields.rs` (the pattern for a group), `cli/src/sweep_judge.rs`,
`cli/src/sweep_fields.rs` (the pattern for a control), `cli/src/sweep_text.rs`, `cli/src/game_judge.rs`,
`cli/src/view.rs`, `web/src/lib.rs`, `www/map/layers/clans.js`, `www/map/view.js`, `www/panels.js`,
`www/feed.js`, `www/assets/sounds.json`, `www/assets/atlas.json`, `www/check.mjs`, `www/check-stage.mjs`,
`www/play/fields.mjs` (the pattern for a staged touch-point script), `www/play/all.mjs`.

## What to build

1. **`TOWN` in `data.rs`** (`TownRules`): `homes: 3`, the homes on a tile with a festival from which it reads
   as a town, with a `note`: three is the least that reads as many hearths on one tile, and two clans side by
   side are neighbours. Build-4 adds its rule's numbers here.
2. **`core/src/towns.rs`**, the town reading. It reads the world and changes only `Market.town`.
   - `Town { tile, market, clans: Vec<u32> }` and `towns(w) -> Vec<Town>`: each festival whose tile has
     `TOWN.homes` or more homes (`homes::homes`), in market id order.
   - `Market.town: bool` (`#[serde(default, skip_serializing_if = "is_false")]`, so the save and the digest
     don't see it while no tile is a town).
   - `check(w)`, called at the end of the `markets` step (`form_and_fade`): for each market, when its tile
     first reads as a town, `Market.town` turns true and `EventKind::TownFormed` goes to the feed
     ("A town forms at (3, −2): 4 clans live by the festival", with the market and the count). When it stops
     reading as one, `Market.town` turns false and the market gets a note ("is no longer a town: 2 clans live
     here"). No other rule reads `Market.town`.
3. **The judge's `towns` group** (`core/src/judge_towns.rs`). `judge.rs` is 641 lines, so first move its score
   structs (`Scores` and the `*Scores` below it) into `core/src/judge_scores.rs`, re-exported from `judge`.
   `Watch` reads the group after each turn, as it reads `fields`. The counts, per civ (a clan-turn counts under
   the clan's civ at the turn's end) and in total:
   - towns at +100 and +200, town clans at +200, and of them the farmers, and towns whose clans work fields
     (Σ `plots` days > 0); the most homes on one tile at +200; the first turn past the pick a town formed;
   - over the clan-turns after +100, for **town clans**, for **farmers outside towns** and for **all clans**:
     clan-turns, the festival walk (Σ route cost ÷ `people × CLAN.work`), food brought per person, and
     starving turns. Brought is what `trade` would carry: the clan's store at the end of the last turn plus
     this turn's harvest, capped at `MARKET.carry × people`, for a clan with a festival. `Watch` keeps each
     clan's last store for it.
4. **The no-towns control in the sweep.** `sweep_judge.rs` is 599 lines, so first move the `Total*` structs and
   the `total_*` functions into `cli/src/sweep_total.rs`. Then `sweep --judge` runs `Config { towns: false,
   ..base }` as a seventh run a map (`judge_map`), and `cli/src/sweep_towns.rs` reads the run against it, as
   `sweep_fields.rs` reads the fields: per map and over the seven, every bound in `spec.md`'s table, each
   printed as pass or miss in the text, and `total.towns_vs_control` in the JSON. The floor's null gets its
   own no-towns control, as it does for fields, so `--floor` gives every towns score a floor. The scale bound
   isn't the sweep's: `bench` reports it.
5. **The page.**
   - The view (`web/src/lib.rs`) gives `towns: [{tile, market, clans}]`.
   - `clans.js`: clans sharing a home are drawn as small circles in a ring round the tile's centre, each in
     its own colour, with its starving ring and civ ring, sized by people but capped below a lone clan's, so
     each stays visible. A lone clan draws as now.
   - `map/layers/towns.js`: a town mark (`icon.town`, a new placeholder cell on `www/assets/atlas.json`) at the
     tile's top, on by default, listed under View with the other layers.
   - The tile tooltip and the festival panel say "Town: 5 clans at the festival". The tile panel lists the
     clans living on the tile. A click already cycles every clan whose home is the tile (`candidates`).
     `panels.js` is 593 lines, so first move `festivalShape` and `tileShape` with their helpers into
     `www/panels-land.js`.
   - The feed shows `TownFormed`, and `sounds.json` maps `townFormed` to `town-formed.ogg` (silent, as every
     sound is).
6. **The text view** (`cli/src/view.rs`): one line per town of the led civ's land and in sight: tile, clans,
   farmers. `view.rs` is 399 lines; its own text is held under 80 lines, so put the town lines beside the
   fields lines.
7. **`www/play/towns.mjs`**: on a staged game (`Sim.stage`) with three clans living on a festival's tile, it
   checks that the cluster draws three hearths on the tile, the town mark is there, the tooltip reads as a
   town, the tile panel lists the three clans, and a turn's feed has the town line. It saves a screenshot to
   `relay/specs/156/towns.png`. `all.mjs` runs it. `check-stage.mjs` checks the staged town's view data.
8. **The design edits.** In `docs/design/clans.md`, Clans:
   - Territory: "A clan holds a connected patch of tiles around its home tile. Its home is where it lives, not
     land it holds: many clans may live on one tile (`farming.md`, Towns)."
   - Holding land, its last sentence: "Home follows the middle of the territory; a farmer's follows its fields
     and its festival (`farming.md`)."
   In `docs/architecture.md`: the Tiles shared row's On main reads "A home is not a claim: many clans may live
   on one tile, each gathering its own claim and sharing the tile's field ground (`homes.rs`)" and Built by
   "Done in M4 W". Leave the Scale row to the review, which writes the bench's ms.
9. `engine/CLAUDE.md`: `towns.rs`, `judge_towns.rs`, `judge_scores.rs`, `sweep_towns.rs`, `sweep_total.rs`,
   `panels-land.js`, `towns.js` and `towns.mjs` in the layout, under 8 KB.

## Tests

In `core/tests/towns.rs`: on build-1's staged shared home with a festival raised on it (`markets::raise`),
`towns` finds the town at 3 homes and not at 2. `TownFormed` fires once as the third clan arrives, and the
note comes when one leaves. `Market.town` serialises only when true. The judge's `towns` group counts town
clans, their festival walk (0 at the market), and the farmers outside towns on a stage with both.

## Proof and record

- `engine/check.sh` green, the pins in `cli/tests/save.rs` unchanged from build-2's: with no town, nothing
  saved changes.
- `sansfiat sweep --judge --json --floor` runs with the seventh run. The towns block reads 0 towns on every map
  (no home is shared until build-4), with run and control equal. Write the text block and the sweep's time,
  before and after, to `relay/specs/156/judge.md`.
- `node www/play/towns.mjs` passes. Look at `towns.png`: the three hearths read as three clans on one tile,
  and the mark reads as a town at the default zoom and zoomed out.

## What later steps rely on

`towns::towns`, `TOWN`, `EventKind::TownFormed`, the `towns` group's counts, `total.towns_vs_control` in the
JSON with every bound marked pass or miss, and the page's cluster and mark. Build-4 tunes on the sweep's towns
block, and the review reads it.

## Done

`check.sh` green with build-2's pins, the tests above, `towns.mjs` passing, the design edits, and `judge.md`
written.
