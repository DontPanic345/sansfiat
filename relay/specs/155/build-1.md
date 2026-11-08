# #155 build-1 (easel): the field job

After founding, a clan splits its days between gathering and fields, and the split moves a notch a turn toward
the one that feeds it better. Read `spec.md` items 1–7 and decisions 1–8. `probe.md` is what the spec step ran,
and `probe.diff` is its sketch, against `engine/`. Build it properly in its own module: the sketch's
environment knobs and `PROBE` prints don't go in. Later steps build the judge, the page and the tuning on these
names, so use them as here.

The code: `core/src/actors.rs` (`split`, `recentre`, `middle`, `trim`), `core/src/clans.rs` (`reap`, `at_target`,
`worth`, `value`, `harvest`, `adjust`), `core/src/land.rs` (`domestic`, `tend`, `wear`), `core/src/map.rs`
(`Tile`), `core/src/data.rs` (`Config`, `CLAN`, `TENDING`, `MAP.big_river`), `core/src/events.rs`,
`core/src/world.rs` (`SAVE_VERSION`), `core/src/stage.rs`, `cli/tests/save.rs`.

## What to build

1. **`FIELD` in `data.rs`** (`FieldRules`), each number with its unit and a `note`:
   - `flood: 15.0 * DAY`: days of fields a floodplain takes a turn;
   - `river: 7.5 * DAY`: days of fields a river tile of `MAP.big_river` or more takes, in proportion below it;
   - `grain: 0.8`: food a field day yields, per unit of fertility and of tame past wild (`domestic − 1`);
   - `notch: 0.1`: the share of its days a clan's split moves a turn;
   - `margin: 0.01`: food a person a turn by which the better split must beat the current one;
   - `farmer: 0.1`: the share from which a clan is a farmer, whose home follows its fields;
   - `tame: 1`: steps of tame a turn a field adds to its tile.
   These are the probe's y8 numbers. Build-4 tunes them.
2. **`Config.fields`** (`#[serde(default = "yes")]`, on in `Default`): off is the control, in which no clan farms.
3. **`core/src/fields.rs`**, the field job:
   - `ground(w, i) -> f64`: field days the tile takes. A floodplain gives `FIELD.flood`. A river tile gives
     `FIELD.river × min(river ÷ MAP.big_river, 1)`. Both add. Land only, from no rng.
   - `yield_a_day(w, i) -> f64`: `FIELD.grain × fertility × (domestic(tame) − 1)`, at least 0.
   - `open(w) -> bool`: `w.config.fields` and civs founded (past the pick). Before it nothing here acts, so
     the pre-game is byte for byte main's.
   - `sow(w, tiles, work) -> Vec<f64>`: the field job's days on each field tile, from the even share `split`
     gives each. A tile takes at most its ground. What a tile can't take goes to the job's other field tiles
     below their ground, evenly, and what none can take stays unworked.
   - `notch(w, id)`: the split's move for one clan, as spec item 4 says, with the `ClanFarming` event when the
     share rises from 0. When it falls to 0, a note: "gave up its fields: they fed it less than the hills".
     Called from `clans::adjust` after the territory and home are written.
   - `farmer(c) -> bool`: `c.fields >= FIELD.farmer - 1e-9`.
   - `home_toward(w, c, territory) -> Option<usize>`: for a farmer, its field tile with the most field days on
     this turn's split (ties to the lower tile). `clans::adjust` steps a non-pinned farmer's home one tile
     toward it, as `actors::recentre` steps toward the middle, then trims. Make `recentre` take its target, or
     add a sibling. Don't copy it.
   - `plots(w, c) -> Vec<(usize, f64, f64)>`: a clan's fields this turn, as (tile, days, grain). The page and
     the judge read this.
4. **`Clan.fields: f64`** (`#[serde(default)]`), the share of its days left after routes that go to fields. A
   daughter starts at her mother's share. A clan that moves on starts at 0.
5. **`reap` with two jobs.** `reap` takes the clan's share. `actors::split` gets two jobs: gathering at
   `1 − fields` over every territory tile, and fields at `fields` over the territory's tiles with ground, each
   with its tiles' yearly walks. `sow` caps the field days. The field grain is days × `yield_a_day`, added to
   the tile's take as grain but drawn from no stock. The harvest budget scales gathering and fields by one
   factor. With share 0, or before `open`, `reap` does the same float operations in the same order as now. A
   site's value (`site_value`) is reckoned at share 0. `at_target`, `worth` and `value` use the clan's share.
6. **The harvest step**: subtracts only the gathered take from stock and records only it in `Tile.take`, so
   wear never sees field grain. It sets `Tile.field` (`#[serde(default)]`, days of fields worked on it this
   turn, zeroed with `take`). `tend` adds `FIELD.tame` steps to a tile with `field > 0` that it tames, past the
   hearth too.
7. **`EventKind::ClanFarming`** with its wording, in `events.rs`. The page's feed and sound are build-2's.
8. **`SAVE_VERSION` bumps**, and the pins in `cli/tests/save.rs` move. Fix `engine/CLAUDE.md`'s layout: add
   `fields.rs`, and `clans.rs`'s line names the split.

## Tests: `core/tests/fields.rs`

Build each on a stage (`Stage::flat` or the `map` text): a founded civ (`Stage::civs`), a clan holding a river
tile and a floodplain with tame set (`world_mut`), and enough wild food that gathering alone falls short of the
target.

- `ground`: a floodplain, river tiles of size 1, 4 and 8, and a dry plains tile give 20, 10 × ⅛, 10 × ½, 10 and 0
  days.
- `yield_a_day` is 0 at tame 0 and `1.5 × FIELD.grain` at a hearth of fertility 1.
- The split rises one notch a turn from 0 while fields feed the clan better. It never moves more than a notch.
  `ClanFarming` fires once.
- A starving clan with fields drops one notch a turn.
- No field ground in the territory: the share stays 0.
- Field grain leaves the tile's stock and wear unchanged. `Tile.field` holds the days, never above `ground`.
- A farmer's home steps toward its best field tile, one tile a turn. The civ's own clan stays on the capital.
- Fields off (`Config.fields` false), or before the pick: the share stays 0 and `reap` equals the old one bit
  for bit on several inputs.
- A field tile tames `1 + FIELD.tame` steps a turn while held past `TENDING.after`.

## Proof and record

- `engine/check.sh` green.
- **Fields off is main.** Set `Config::default()`'s `fields` to false locally (don't commit it). Run
  `sansfiat sweep --judge --json` on the branch and on a worktree of the branch's base. Every map's `standing`,
  `motion`, `economy` and `land` blocks are equal.
- **Fields on runs.** `sansfiat sweep --judge --json --floor` with the defaults, then `judge --compare` against
  the base. Aligned Standing at +200 rises past the floor and pre-games by centres stay 7 of 7. Every map has
  clans with fields at +200 (count them with a scratch print, or `stats`). The probe's y8 came out at aligned
  0.253 → 0.312, and the build lacks only tame under fields, so a large miss is a bug. Write the compare and the
  counts to `relay/specs/155/field-job.md`.

## What later steps rely on

`fields::{ground, yield_a_day, open, farmer, plots, notch}`, `Clan.fields`, `Tile.field`, `Config.fields`,
`FIELD`, `EventKind::ClanFarming`. Build-2's judge reads `plots`, `farmer` and `Tile.field`. Build-3's page reads
the same through the view. Build-4 tunes `FIELD` and may change `notch` and `home_toward`.

## Done

`check.sh` green, the tests above, fields off equal to the base, fields on in `field-job.md`.
