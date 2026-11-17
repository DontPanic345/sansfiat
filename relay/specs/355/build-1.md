# build-1: the page, the pins, and the sweep (#355 Goods)

The whole step built the rules in the engine: read `relay/specs/355/progress.md` and `notes.md`
first, and `docs/design/goods.md` (a draft the whole step wrote). You finish what it left. Do them
in this order; each is small.

## 1. Green checks

- Run `cargo test --no-fail-fast` in `engine/`. The whole step fixed the tests it saw fail and ran
  out of window before reading the full run. Expect failures of three kinds, and fix each the same way:
  - a 5-wide basket literal or a sum over all goods in a test: baskets are `[f64; 7]` now (foods,
    then salt, obsidian), and `market::total` is the food;
  - a digest pin: bump the pins in `cli/tests/save.rs` (and any other pinned digest) to
    `SAVE_VERSION` 26;
  - a test that reads a real map's clan or score: the map has sources now. Where the test's subject is
    not goods, give its `Config` `goods: false` only if the old number is the point; else re-pin.
- `www/check.mjs`, `check-stage.mjs` and the page read baskets by `goods()` order: make them pass
  with seven goods.
- Run `engine/check.sh` whole once at the end; it must pass. Keep `engine/CLAUDE.md` under 8 KB and
  add to its Layout: `core/src/sources.rs` (placing sources, `of`, `steps_from`),
  `core/src/judge_goods.rs` (the `goods` group), `cli/src/sweep_goods.rs` (goods against the
  no-sources control). Add the Goods row's "On main" in `docs/architecture.md`.

## 2. The page (the piece's Decided line: "The page draws sources, and a store's panel lists every good")

- `web/src/lib.rs`: `layout` (or `view`) gives each tile's `source` (the good's name, or null);
  a camp row gives `armed`. `goods()` already follows `GOODS`.
- `www/map/`: a new layer file `sources.js`, drawn like `towns.js`/`settle.js`: a small mark on each
  source tile, salt and obsidian told apart at a glance (a white crystal, a black shard; placeholder
  art is fine, name the cells in `www/assets/atlas.json` if the atlas is used). The tile tooltip and
  the tile panel (`panels-land.js`) name the source.
- `www/panels.js`, `panels-land.js`: the civ's store, a settlement's store and a clan's store list
  every good that is held (food first, then salt and obsidian), and the army panel says its armed share.
- A touch-point script `www/play/goods.mjs` in `all.mjs`: it exits non-zero unless a source is drawn
  on small 2 and the civ panel lists salt and obsidian. Add a `StageSpec` way to put a source on a
  tile (`sources: [{tile, good}]`) if `check-stage.mjs` needs one.

## 3. The sweep, old and new

- `relay/specs/355/sweep-1.txt` is the branch's sweep before two late changes (a civ store's fill,
  `ARMS.hold`; a starving clan leaves its source). Run `sansfiat sweep --judge` on the branch as you
  leave it (about 4 minutes) into `relay/specs/355/sweep-branch.txt`, and `--json` into
  `sweep-branch.json`; build `main` in a worktree and run the same into `sweep-main.txt/json`; run
  `sansfiat judge --compare sweep-main.json sweep-branch.json` into `compare.txt`.
- Write `relay/specs/355/scores.md`: the goods section's totals (the share of festivals where salt or
  obsidian trades against the control's 0; mean and furthest steps from a source at +200 for each
  good), then every score that moved past its noise floor, old and new. In `sweep-1.txt` starving
  clan-turns were 0.034 against the control's 0.022 and people 1.039 of the control's: say what the
  final run gives, and if starving is still more than a quarter above the control, say so plainly as
  the piece's cost. Do not tune numbers to hide it; one try at `SOURCE.share` 0.05 is allowed, kept
  only if it lowers starving without dropping the festival share under 20%.

## Not yours

The rules (`data.rs` `GOOD`, `SOURCE`, `KEEPING`, `ARMS`; `actors::spoil`, `worths`; `muster::arm`)
are settled: change one only for a bug, and say so in your summary. `data.rs` and `map.rs` are past
800 lines and were before this piece; splitting them is a note for the retro, not this build.
