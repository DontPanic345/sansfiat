# #356 notes for the kiln and the glaze

## What build-1 leaves

- **`Lots`** (`core/src/lots.rs`): `Lot { good, source, markets, share }`, `Lots(Vec<Lot>)` (serde-transparent),
  `Lots::dug`, `traded_at`, `of`, `mix(good, held, amount_in, &lots_in)`, `arrive(&held_basket, &got_basket, &from)`
  for a basket of wares moving store to store, and `lots::cleared` for a clearing. `LOTS.most` 4, `LOTS.chain` 6
  (`data/goods.rs`). Every holder has one: `Actor.lots` (a clan's, a civ's, a unit's: `c.actor.lots`, and
  `clan.lots` / `unit.lots` through `Deref`), `Settlement.lots`. A clan's lots cover its store and its basket.
  Write one only with `w.config.exchange` on, and only at an arrival: `mix`/`arrive` into the receiver with what
  it held *before* the add. The invariant is `core/tests/lots.rs`'s `check` (a real small map, every turn).
  Build-2's `Order::Fill` must `arrive` the salt it takes from a civ store or settlement into the unit's lots,
  and `give_up` / `disband` already carry a pack's wares with their lots to the clan.
- **`exchange::unit_rows(w, mid)`** takes any unit on a market's tile that isn't a rival's and holds wares; its
  brought wares are scaled together to what buys its pack's room at the last prices. `apply_units` writes the
  pack and `Unit.traded: Option<UnitTrade { market, food, sold }>` (last turn's: `exchange::begin` clears it at
  the start of `trade`; saved when set). The feed: "bought N food at the festival at (q, r) with salt".
  A pack that is full (food room 0) joins no clearing. Scouts trade as armies do.
- **`w.carried`**: derived, per turn, `(clan, market, wares sold)` of the second clearings: the judge's count of
  Carry. `markets::trade` calls `clear_at` once per market, then once more at each market a carrier walked to.
- **`Config.exchange`**, `sansfiat digest --no-exchange`. The sweep's other controls (`no-army`, ...) now run
  with exchange on; build-2's exchange-off control is a `Config { exchange: false, ..base }`.
- `markets.rs` is 579 lines; `bring` lives in `exchange.rs`.

## For the glaze

- A second walk's worth is tiny at any plausible setting (a unit of salt clears at ~0.34 food and a clan's
  unsold pile is a few units): ``PASS.cover` 0.01 is where it runs at all; going down to 0.002 cost health past
  the floor (pass.md). A fed gate (`!c.fed` walks none) is what kept health.
- `food_price` in `exchange.rs` is the price over the mean of the foods traded, the same estimate Carry and the
  unit's pack use.
- Units trade at the first clearing only; a unit's pack salt is sold into whatever the clans there want. The
  clans' worth for salt falls once they hold some (`actors::worths`), so a second army at the same festival
  the same turn gets less.

## After the glaze

- `army_flow` (`core/tests/exchange.rs`, `--ignored --nocapture`, 20 s for the seven maps) reads what an army
  has: army-turns salted, salted and low, foreign army-turns with food, and the civ stores' salt. Quicker than a
  sweep to see what a number does to the flow; only a sweep says what it does to health.
- A pack's salt keeps the pack's food (`spoil_keeping`), about half a unit a person a turn: salt that isn't more
  than a march's keeping is gone before the market. `UNIT.salt` 3 leaves a little to sell.
- The control's trade group is real now (`Market.cleared` is derived and written with exchange off too).
- `KEEPING.levy` is the levy's salt share (with exchange on); `CIV.levy` is still the food's.
