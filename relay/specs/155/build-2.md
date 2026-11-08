# #155 build-2 (easel): the judge, the control, the feed and the design edits

The judge learns to read fields, and the sweep runs a no-fields control beside every map, so build-4 can tune
on these numbers and the review can read the piece's question in them. Read `spec.md`: "The question and how it
is measured", whose table these counts serve, and item 8. Build-1 left `fields::{ground, yield_a_day, farmer,
plots}`, `Clan.fields`, `Tile.field`, `Config.fields` and `EventKind::ClanFarming`.

The code: `core/src/judge.rs` (`Watch`, `Economy`, `Scores`; over 600 lines, so the new counts go in a module of
their own), `core/src/judge_ring.rs` (the pattern: a tally `Watch` feeds each turn and a scores struct per civ
and in total), `cli/src/sweep_judge.rs` (`judge_map` and its runs; 588 lines), `cli/src/sweep_text.rs`,
`cli/src/game_judge.rs`, `www/feed.js`, `www/check.mjs` (`KINDS`), `www/assets/sounds.json`.

## What to build

1. **`core/src/judge_fields.rs`**: the `fields` group. `Watch` feeds it after each turn (`Fields::turn(w)`), and
   `Scores` gains `fields: FieldScores`. Per civ (`FieldCiv`) and in total:
   - `field_tiles_100`, `field_tiles_200`: tiles with `Tile.field > 0`, and `field_days_200`: Σ `Tile.field`;
   - `farming_share_200`: Σ people × fields ÷ Σ people over its clans (for the total, over every clan, aligned or
     not);
   - `farmers_200`: clans with `fields::farmer`;
   - `store_turns_at_pack`: turns its store held `units::pack_cap(UNIT.first)` or more (a per-civ split of
     `economy.turns_at_pack`), and `turns`: its turns alive;
   - clan-turns, home moves and moves-on (`ClanMoved` events) for **settled farmers**, clans that have been
     farmers ten turns in a row before the turn (`FIELD_SETTLED: u32 = 10` in `judge_fields.rs`, a judge's line
     and not a rule), and for **gatherers**, clans with share 0 before the turn. A clan counts under its civ at
     the turn's end;
   - starving clan-turns for settled farmers, for gatherers, and for every clan, with every clan-turn.
   `total` also counts `civs_never_at_pack`, civs whose store never held a pack.
2. **The no-fields control in the sweep.** `judge_map` runs `Config { fields: false, ..base }` as a sixth run.
   `MapJudged` gains `no_fields: Scores`. Add the comparisons in **`cli/src/sweep_fields.rs`** (new; keep
   `sweep_judge.rs` under 600 lines). For each map:
   - per civ, Standing at +200 less the control's, and whether the civ farms (`farming_share_200 ≥ 0.1`);
   - farming civs' mean gain, how many of them gain, and the non-farming civs' mean;
   - the civ that farms most, and whether it held a pack on at least half its turns;
   - starving share against the control's.
   Then the totals over the maps, in the JSON as `total.fields_vs_control`, so `judge --compare` lists every
   number. `sweep_text.rs` prints a `fields` block: a line per map with each civ's share, gain and store turns,
   then the totals with the spec's bounds beside them, marked pass or miss.
3. **`judge FILE`** (`game_judge.rs`) prints the led civ's fields line: farming share, field tiles, store turns
   at a pack.
4. **The feed.** `www/feed.js` `KINDS.clanFarming` ({ icon: '≡', color: '#d9c45a', label: 'Fields' }).
   `check.mjs`'s `KINDS` gains it. `sounds.json` gains `"clanFarming": "clan-farming.ogg"`, silent until the
   file exists. A click on it selects the clan, as `clanMoved` does.
5. **The design edits**, word for word:
   - `docs/design/clans.md`, Taming: "- Farming and irrigation tame it further, later." becomes "- Fields tame it
     further (`farming.md`); irrigation later."
   - `docs/design/clans.md`, Out of scope: "- Work besides harvesting: traders, crafters, farmers." becomes
     "- Work besides harvesting in the pre-game: traders, crafters, farmers. Clans farm from founding
     (`farming.md`)."
   - `docs/design/actors.md`, Later actors: the farmer's bullet, in place, becomes "- **The farmer** is a clan with a
     second job: fields on the river-side and floodplain ground of its tiles, worked from home and traded
     from home. Its days split between gathering and fields before each job divides over its tiles, and
     farmed land doesn't deplete as gathered land does (`farming.md`)."
6. `engine/CLAUDE.md`: `judge_fields.rs` and `sweep_fields.rs` in the layout, the sweep's control list in its
   commands line. Keep it under 8 KB.

## Tests

- `core/tests/fields.rs` (build-1's file) gains: on a stage with a farmer and a gatherer, `Watch` over 15 turns
  counts the farmer as settled from its eleventh farming turn, counts a home move once, and counts its
  starving turns apart from the gatherer's. `store_turns_at_pack` per civ sums to `economy.turns_at_pack`.
- `www/check.mjs` passes with the new kind.

## Proof and record

- `engine/check.sh` green.
- `sansfiat sweep --judge --json --floor` on the branch, with the text. Write the `fields` block and the JSON's
  `total.fields_vs_control` to `relay/specs/155/judge.md`. These are build-4's starting numbers. With build-1's
  rules and numbers they should sit near `probe.md`'s y8 table: farming civs gain about +0.03 to +0.04, 2 of 21
  civs never hold a pack. A count far from it is a bug in the count. Say which.
- The sweep takes a sixth more time than before. Give both times.

## What later steps rely on

The `fields` group's names in the sweep's JSON and text. Build-4 reads every bound from them, and the review
reads them into the PR.

## Done

`check.sh` green, `judge.md` written, the design edits in.
