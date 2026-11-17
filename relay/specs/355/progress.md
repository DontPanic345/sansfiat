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
