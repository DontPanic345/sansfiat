# #355 Goods: progress

## Plan

1. Clay: the goods table (`data.rs` `GOOD`: weight, spoil, eaten, use); `market::total`/`take` are
   the eaten goods only; `market::demand`/`clear` take each trader's worth of each good.
2. Kiln: `sources.rs` places salt (coast, marsh) and obsidian (hills) on their own rng stream;
   `fields::jobs` gives a source holder's days a third job (`SOURCE.share`); `Reaped.dug`.
3. Glaze: `actors::spoil` (salt keeps food), `actors::worths` (a clan wants salt by its uncovered
   spoiling; `civ_worths`), `muster::arm` and `fight::might` (obsidian), the levy takes all goods.
4. Scores: `judge_goods.rs` (the `goods` group), `sweep_goods.rs` (the no-sources control,
   `Config.goods` off).
5. Text view: the store lists every good, sources and who holds them, an army's armed share.
6. Page: sources drawn, a store's panel lists every good. **Handed on (build-1).**

## Done

- 1 to 5 are in the engine and build. See `notes.md` for what is left.
- Tests: `core/tests/goods.rs` passes; six old tests fail and are listed in `build-1.md`.
- First sweep: `sweep-1.txt` (32.0% of festivals trade salt or obsidian; control 0).
- PR #375 is open, marked not ready. `steps.json` hands on: build-1 (page, pins, checks, the
  sweep beside main), then play and review.
- Not run: `engine/check.sh`, the sweep on `main`, any played game.

## build-1

- Done: tests green (pins at SAVE_VERSION 26), page draws sources and every store lists its goods (`www/play/goods.mjs`), sweeps on main and branch (`scores.md`, `compare.txt`), `SOURCE.share` 0.05 kept after its one try, `engine/check.sh` run whole.
- Not run: any played game.
