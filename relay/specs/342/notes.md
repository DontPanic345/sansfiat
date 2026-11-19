# #342 notes for the next builders

- **build-3's `herds.mjs`** caps its play at `HERDING_BY` (10 turns), named in `core/tests/herds.rs`: on a flat
  plains stage with the terrain table's food on every tile, a civ's own clan has a herding share from turn 2,
  animals at once, and harvests animals every turn after. Stage it as that test does (`Stage::flat`,
  `s.civs(..)`, fertility 1, `cap`/`unworn`/`stock` from `TERRAIN[t].food`).
- **Judge**: `FieldCiv` (judge_fields.rs) is untouched. The clay's readings used a temporary print of
  `herding_people_200`, `herders_200`, `animals_200` and `Σ harvest[ANIMALS]` against `Σ total(harvest)`
  per civ at +200 and summed from the pick, in that group; build-2 builds the herds group properly.
- **`retro.rs`** takes `herds` from `Config::default()`: the no-herds control is a copy of the run's `Config`
  with `herds` false, as the no-fields control is.
- **Tests that paint `[x; N]` stocks** now paint a wild-herd slot too: a tile holds no animals, so zero it
  (`towns.rs` does).
- **`jobs::jobs`** returns a `Jobs` struct (`gather`, `field`, `fk`, `dug`, `herd`, `hk`); `fields::home_toward`
  passes herds 0.
- **Tile food/gather of the animals slot** is 0 everywhere: nothing regrows or is gathered of it.
- **`StageSpec.herds`** (`Option<bool>`, `None` leaves herds on) turns herds off in a stage; `www/play/fields.mjs`
  uses it. A staged clan that starves drops its fields and, with herds open, takes up herds; build-3's
  `herds.mjs` stages as `core/tests/herds.rs` does and needs no flag.
- **Fields still pay** (from the sweep): 7 farming civs gain +0.046 against no fields (main: 10 civs, +0.049).
