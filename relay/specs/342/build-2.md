# #342 build-2 (kiln): the judge's herds group and the no-herds control

Read `relay/specs/342/spec.md` (The question and how it is measured) and build-1's summary. Keep the
clay's shape: read the herd through `herds::` and `Clan.herds`, never re-derive it. Where the clay
doesn't hold, make the smallest fix that works and record it as a decision. No rule or number
changes here: herds on, every map's scores outside the new group equal build-1's.

## What you build

Follow the fields group exactly: `core/src/judge_fields.rs` (a `Watch` part that reads each turn),
`cli/src/sweep_fields.rs` (the map against its control, the totals, the text) and the run list in
`cli/src/sweep_judge.rs`.

1. **`core/src/judge_herds.rs`**, a `herds` group in `Scores` (`Herds::pick`, `turn`, `scores`, wired
   in `judge.rs` beside `fields`). Per civ and in total, at +100 and +200: herders (`herds::herder`),
   animals held by clans (Σ `store[ANIMALS]`), people and herding people (Σ people × herds), the
   herding share, the animals eaten and the food harvested (Σ `harvest[ANIMALS]` and Σ the
   harvest's food, `FOOD_N` slots, at +200) and their ratio, the **animal share**, herded tiles
   (territory tiles with `graze > 0` of herders); the same animal share for herders alone. Per civ: its **grass
   share** at the pick (its clans' territory tiles with `HERD.graze[terrain] ≥ HERD.graze[HILLS]`
   and `fields::ground` 0, over all its clans' territory tiles), and its store turns at a pack (as
   fields). **Tiles a person** at +200 for herders, gatherers (herds 0 and fields 0) and farmers
   (`fields::farmer`): Σ territory tiles ÷ Σ people. Clan-turns and starving clan-turns for herders
   and gatherers. **Animals' festival-turns**: festivals where animals changed hands each turn, pick
   to +200 (`Market.ware` holds last turn's volume of every good, `markets.rs` 310: count festivals with
   `ware[ANIMALS] > 0` with two or more clans, as `judge_goods.rs` counts salt).
2. **The no-herds control**: `Config { herds: false, ..base }` as one more run a map in
   `sweep_judge.rs`, `no_herds: Scores` and `herds_vs_control: MapHerds` on `MapJudged`, totals on
   `Report`. The floor's nulls run it too (they run the same run list).
3. **`cli/src/sweep_herds.rs`**: per civ `{id, share, animal_share, herds, grass, gain,
   store_turns_at_pack}` with `herds` = animal share ≥ 0.1 (a tenth of its food from its herds, the
   spec's measure, not the days share); per map and in total: herding civs, their mean gain and how many gain,
   the others' mean gain, `herding_over_others`; the grass third's mean gain (the third of civs,
   rounded up, with the highest grass share, over all 21); tiles a person for herders, gatherers and
   farmers and herders over gatherers; maps with any animals' festival-turn; starving share against
   the control's, the ratio and the worst map's; herders' and gatherers' starving shares; clans,
   people and land changed hands against the control. The text prints each beside its control, no
   verdict (`print_herds`, called from `sweep_text.rs`).
4. **`game_judge.rs`**: a herds line for the led civ beside its control, as fields has.
5. **The feed**: `ClanHerding`'s kind in the page's feed and a sound in `www/assets/sounds.json`, as
   `ClanFarming` has (`grep -rn ClanFarming\|clanFarming www web`); `check.mjs` if it lists kinds.

## Tests

`core/tests/herds.rs` gains a short test on a staged world that the group counts a herder, its
animals and its tiles a person. Seconds, not a sweep.

## Done

- `engine/check.sh` green.
- One `sweep --judge --json` (about 4 minutes, now ten runs a map). Write `relay/specs/342/judge.md`:
  the herds group's totals and per-civ table at build-1's start numbers, and the seconds the sweep
  took. Check every score outside `herds` and `herds_vs_control` equals build-1's sweep where
  build-1 left one, else say so.
- `engine/CLAUDE.md` unchanged unless a layout row needs a name.

Commit as `#342 build-2: the herds group`.
