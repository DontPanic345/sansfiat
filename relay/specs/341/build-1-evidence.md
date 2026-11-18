# #341 build-1 evidence: the door step, capital towns in the judge, the mark on a capital

Measured on the branch after build-1's edits, on the spec's runner. The sweep is deterministic:
`sweep --judge --json` and `sweep --judge` ran once each (about 2 m 50 s each).

## Towns at +200 (`towns.py` on the branch's report)

`python3 -I towns.py b1.json`, the table in `build-1-towns-table.txt`. Rule A as `tables.txt` gives it:

| map | towns at +200 | capital towns (of civs alive) | clans | formed | trade town | farmers outside | outside | town clan-turns |
|---|---|---|---|---|---|---|---|---|
| small 2 | 1 | 0 of 3 | 3 | 6 | 0.0339 | 0.0379 | 0.0618 | 410 |
| small 3 | 0 | 0 of 2 | 0 | 0 | 0.0000 | 0.0501 | 0.0610 | 0 |
| small 6 | 1 | 0 of 2 | 3 | 4 | 0.0963 | 0.0400 | 0.0629 | 75 |
| small 9 | 2 | 1 of 3 | 7 | 5 | 0.0605 | 0.0359 | 0.0591 | 830 |
| medium 8 | 3 | 1 of 3 | 16 | 3 | 0.0685 | 0.0375 | 0.0707 | 2439 |
| medium 9 | 4 | 2 of 3 | 21 | 5 | 0.0349 | 0.0429 | 0.0890 | 3120 |
| large 8 | 7 | 2 of 2 | 28 | 26 | 0.0441 | 0.0354 | 0.1270 | 5863 |
| **total** | **18 on 6 maps** | **6 of 18** | 78 | 49 | **0.0476** | 0.0375 | **0.1006** | 12,737 |

- Towns at +200: 18 on 6 maps, exactly as `tables.txt` gives for rule A. The town clans' trade
  (0.0476) and the outside trade (0.1006) match A's row too.
- `outside` is read from `towns.total.outside.traded_per_turn`; the reader asserts it equals
  all − town, and it does on every map.
- Capital towns: 6 at +200 over the seven maps, 1 on the small maps (small 9), which is the spec's
  probe. The denominator is the civs alive at +200: 18 over the seven, 10 on the small maps. So
  "6 of 18 capitals; 1 of 10 on the small maps". `spec.md`'s 21 and 12 were a fixed count; the
  alive civs replace it.
- Main's own report has no `outside` or capital fields; `towns.py` reads it as before (`outside` as
  all − town, capitals as 0) and prints main's table unchanged.

## Every row already in the report matches `door-vs-main.txt`

`judge --compare` for main (`main-floor.json.gz`, gunzipped) against the branch: `build-1-compare.txt`.
Diffing its values against `door-vs-main.txt`'s B column: every row that was already in that report
matches to the last digit (aligned standing 0.5035, warriors starved 290, goods-run starving 0.0294, and
every other row). The only additions are the new towns rows: `capitals_200`, the capital town counts,
the `outside` group and `small_capital_towns_200` / `small_capitals_200`. The "changed" and "flagged"
counts are 152 and 78, as before.

## Health, against main's floor

| metric | main | branch | change | floor |
|---|---|---|---|---|
| standing: aligned at +200 | 0.5216 | 0.5035 | −0.0181 | 0.0079 * |
| standing: top at +200 | 0.2912 | 0.2799 | −0.0113 | 0.0076 * |
| doubt: spread at +100 / +200 | 0.2299 / 0.2434 | 0.2107 / 0.2351 | | 0.0009 * / 0.0055 * |
| doubt: lead changes (maps with one) | 7 (3) | 10 (4) | +3 (+1) | 3 (0) |
| armies: warriors starved | 329 | 290 | −39 | 16 |
| armies: camps kept / lost | 16 / 8 | 16 / 8 | 0 | 2 / 0 |
| goods run: starving | 0.0265 | 0.0294 | +0.0029 | 0.0019 * |
| goods run: control starving | 0.0216 | 0.0198 | −0.0018 | 0.0010 * |
| fight: taken / claim taken | 25 / 220 | 13 / 108 | | 9 / 79 * |

The rows past the floor are the ones spec-review named (`door-vs-main.txt`), and they are the same
values. Aligned standing and the goods run's starving sit past main's floor; the canvas reads them
against main and the easel (`spec.md`, "For the review"). Starving on the towns rows: town clans
0.0088 (112 turn-starves), farmers outside 0.0067, all clans 0.0200 vs the control's 0.0205 (within
floor).

## The page's mark on a capital (`engine/www/play/towns.mjs`)

- Before any change, the script as it stood (capitals off for the mark check) passed on main's layer:
  the mark was checked with no house on the tile. With the capitals layer on, main's layer order gives
  **14 pixels** differing in the 48 px square (the house covers the mark, only its white edge shows).
  The check was `differing > 0`, which 14 pixels pass, so it could not tell. The script now runs the
  mark check with the capitals layer on (the clans' colour check still runs with capitals off, as
  before), and the threshold is `>= 100` pixels.
- With the new check on main's layer order: **fails** (`towns: the town mark shows above the capital's
  house (14 pixels differ with the layer off)`, exit 1).
- The change: `towns` draws after `capitals` in `LAYERS` (`www/map/view.js`), and the towns layer's
  header says why. The mark is the same as any town's, same size and offset.
- After the change: **passes** (`405 pixels differ with the layer off`), the feed's town line also
  passes. The screenshot shows the white mark over the capital's roof (`towns.png`, not committed).
- Why this over moving the mark up on capital tiles: one reorder in `LAYERS` does it, and the mark's
  code is unchanged. The screenshot reads clearly with the mark over the roof.

## Checks

- `cargo fmt --check`: clean.
- `core/tests/towns.rs`: 23 tests, all pass. The door step's tests and the capital's two tests are in it;
  the file is under the brief's 600-line limit, so the home-step tests stay there.
- `cli/tests/save.rs`: `VERSION` 26 → 27. The seed 3 pin changed (`cdf9fef2ef13ed47` → `d8a90a872d75f75d`),
  seeds 1 and 2 and the led game's two digests did not move. Both save tests pass.
- `core/tests/save.rs` (three version strings) and `www/check.mjs` (the page's save version) also pin 26;
  they are bumped to 27 with `SAVE_VERSION`. The brief named only `cli/tests/save.rs`.
- `engine/check.sh` whole, after the last edit: exit 0.

## Decisions taken here

- The capital's count takes the civs alive at +200 as its denominator.
- The page draws the towns layer after the capitals layer, not higher on the capital tile.
- The page's mark check is 100 px at the 48 px window, the threshold between the house-covered 14 px
  and the mark's 405 px.
