# #356 Exchange

**Question:** decisions and motion. Two numbers at +200, with every civ on the AI, on the seven
retro maps (`sansfiat sweep --judge`):

- **Army-turns fed at a market that isn't the civ's**: turns in which an army took food in the
  clearing of a market that is neither its civ's capital market nor one of its settlements'.
  Main's is 0: no unit trades on main.
- **The largest share any one good has of trades**, by count and by worth (the trade group,
  below). Reported, not targeted: it shows whether one good comes to stand on one side of most
  trades.

**The control** is main's sweep after G (#355), which the branch reproduces with
`Config.exchange` off: off, no unit trades, no good passes between markets, no lot is
recorded and the planner fills no pack, so all 21 digests of `relay/specs/340/digests.sh` are
main's at 8d1b4ed. The sweep runs that control beside each map, as it runs the no-sources one,
so the trade group has a control row (its shares, and its 0 army-turns). Main's own sweep at
8d1b4ed is in `sweep-main.txt` / `sweep-main.json` (this step).

Health is a band: starving clan-turns and people over control stay within `judge --compare`'s
floor of the control, or the review says why not.

## What the piece delivers

1. **Lots: where a good in a store came from** (build-1, `core/src/lots.rs`, new). Every store
   that holds salt or obsidian (a clan's store and basket, the civ's store, a settlement's, a
   unit's pack) carries `lots: Lots` beside its `Basket`. A lot is a share of one good in that
   store with its chain: the source tile it was dug at and the markets it changed hands at,
   oldest first. The last of them is "the market it last traded at" (the Decided line); the
   chain is kept whole so the page draws it without a search.

   ```rust
   pub struct Lot { pub good: usize, pub source: usize, pub markets: Vec<u32>, pub share: f64 }
   #[derive(Default)] pub struct Lots(pub Vec<Lot>);
   impl Lots {
       /// Mixes `amount_in` of `good` arriving with `lots_in` into this store, which held `held`
       /// of it: each lot's share is reweighted by amount; equal chains merge.
       pub fn mix(&mut self, good: usize, held: f64, amount_in: f64, lots_in: &Lots);
       pub fn dug(good: usize, source: usize) -> Lots;           // one lot, share 1, no markets
       pub fn traded_at(&self, good: usize, market: u32) -> Lots; // a copy with `market` appended
       pub fn of(&self, good: usize) -> impl Iterator<Item = &Lot>;
       pub fn is_empty(&self) -> bool;
   }
   ```

   Shares, not amounts: spoiling, eating salt, arming with obsidian, a levy's share and a
   seller's goods leaving all take a good in proportion, so they leave the shares as they are.
   Only an arrival writes (`mix`): a dig, a clearing's goods coming back, the levy into a store,
   a merge, a pack filled, a store taken. A clearing pools each good's sellers' lots by what each
   sold and hands every buyer the pool with the market appended. A store keeps at most
   `LOTS.most` lots a good (dropping the smallest and renormalising) and a chain at most
   `LOTS.chain` markets (dropping the oldest market, never the source). Both are data, in
   `data/goods.rs`. Lots are `#[serde(default, skip_serializing_if = "Lots::is_empty")]`, and
   with `exchange` off none is written, so the control's JSON, and its digest, are main's.

2. **A unit is a row in the clearing** (build-1, `core/src/exchange.rs`, new; `markets::trade`
   calls it). At the `trade` step a unit whose tile is a market's tile, where that market isn't a
   rival civ's (any other living civ's capital market or settlement market), joins that market's
   clearing as a clan does. It brings what its pack holds that isn't eaten, worth 0 to it, and
   wants food, worth 1. It brings only the salt that buys its pack's room at the market's last
   prices (all of it when the market has no prices yet), so the food it takes fits its pack.
   Its own civ's markets count: an army at a settlement trades there too. A unit's trade gives
   no clan a leaning. The feed says "bought N food at the festival at (q, r) with salt".

3. **A good passes between markets: Carry** (build-1). A clan holding a ware its own festival
   didn't take this turn (nobody there wanted it, so it went home unsold) walks a second route,
   the next turn, to the nearest other festival in its reach where that good last traded,
   carrying only its wares, and trades them there. Its walk is costed as every route, on the
   wares' weight.

   The piece asked the designer to settle Carry against Lines (a civ's stores posting wares and
   sending them along lines) by which moves obsidian further. Spec-review settled it by probe
   (`probe.md`): Lines can move only what settlement stores hold, ~0 salt and at most 2.6
   obsidian on average on the one map with the most, within one civ, to a capital that mostly
   wants none; Carry has 12–455 clan-turns a map of obsidian that a capital in reach would take
   and 68–3,950 of salt that another festival would. Lines isn't built.

   Obsidian is bought only where a civ posts and wants it (a capital's market under
   `ARMS.hold`), so Carry takes obsidian only to capitals: expect its steps from source to move
   little. What Carry leaves open is what the second walk costs a clan and when it is worth it,
   judged by health on the sweep (build-1).

4. **The pack filled with salt** (build-2). `Order::Fill { unit }`: the unit stands on a store
   of its civ (capital or settlement) and takes salt from it up to its pack's salt room,
   `UNIT.salt` a person (start 1 unit; at the festivals' median price, 0.34 food a unit of salt
   (`probe.md`), that is a third of a warrior-turn of food; the glaze tunes it). It takes what
   there is: refused, with the reason, only off a store, with no salt there or with its room
   full. A civ's store, capital or settlement, keeps salt for packs back from its keeping: its
   spoil uses only the salt beyond `KEEPING.packs` (start 10), and `civ_holds`' salt is
   `cap / KEEPING.keeps + KEEPING.packs`, so its levy and its market still want salt while the
   reserve is short. Both only with `exchange` on. Today every unit of salt that reaches a store
   is used by its keeping the turn it arrives (`probe.md`), so a cap alone would leave nothing to
   fill a pack with. Disbanding a unit gives its pack's wares, with their lots, to the clan it
   joins.

5. **The planner trades** (build-2, `core/src/units/trade.rs`, new: `ai.rs` is 631 lines). An
   army the planner marches past its ring fills its pack with salt first, at the store it
   stands on. An army with salt whose line can't feed it this turn and whose pack falls under
   `PLAN.trade_at` turns (start 2) steps to the nearest market that isn't a rival's within its
   pack's turns, and stands there a turn. Starting numbers that run; the glaze tunes them.

6. **The judge's trade group** (build-2, `core/src/judge_trade.rs`, new; `cli/src/sweep_trade.rs`
   prints it). Each turn from the pick to +200, over every clearing that cleared: a *trade* is
   one row's part in a clearing in which any good moved for it; a good is *in* a trade if that
   row gave or took it. Each good's share by count is the trades it is in over the sum, over
   goods, of the trades each is in; by worth, the volume times the price it cleared at over the
   sum of every good's. The group reports both shares, each good's, at +200 (that turn) and over
   the watch; the largest share of each at +200; and army-turns fed at a market not the civ's
   (and at its own, and units' trades), with the food those trades bought over the eating of
   the armies that made them, so the count's size reads: an army-turn counts with any food
   bought, and at the probe's salt prices a full pack buys a fraction of a turn. The sweep runs the `exchange`-off control as a config,
   prints a line a map and the totals, and keeps them in the JSON for `judge --compare`.
   `judge FILE` shows the group for a played game.

7. **The text view and the page** (build-2 the view, build-3 the page). The view lists a
   unit's pack wares, the Fill order, and, for a store, each good's lots as chains:
   `obsidian 4.2: 60% from (3, 9) via festival (5, 7), capital market`. The page: a store's
   panel (capital or settlement) lists every good, and clicking a ware selects it; the flow
   layer (`www/map/layers/flow.js`, new) then draws each lot's chain from its source through
   each market's tile to the store, its width by share. The unit's row gets Fill salt, greyed
   with its reason off a store. `www/play/exchange.mjs` presses Fill and checks the pack's salt
   rose and the store's fell, and selects a settlement's obsidian and checks the layer draws a
   chain that starts on a source.

## Shape: Clay, Kiln and Glaze

The issue said Clay and Kiln, with the passing as the clay's unsettled core. Spec-review
settled the passing by probe (Carry); the clay still holds the lots model, which every arrival
of a ware must write, the unit as a row, and what Carry's second walk costs, judged on the
sweep. I add a glaze: the planner's trading and the pack's salt room
are rules settled here, but whether an army ever stands on a foreign market fed, the
question's first number, rests on three numbers (`UNIT.salt`, `KEEPING.packs`,
`PLAN.trade_at`) that only work together on the retro maps. A kiln that tuned them would hold
Open 2 and Miss 3, which the ladder says is unsettled work.

- **build-1, clay:** lots, a unit as a clearing row, and Carry, measured beside main. It leaves `Lots` and its invariant, `exchange::unit_rows`, the switch, and the tests.
- **build-2, kiln (engine):** the order, the store's pack salt, the planner, the judge's trade
  group, the sweep's control, the text view.
- **build-3, kiln (page):** the panel's goods, the flow layer, the Fill button, the play script.
- **build-4, glaze:** tunes `UNIT.salt`, `KEEPING.packs`, `PLAN.trade_at` (and Carry's
  numbers if the clay left any), and may raise the salt a store gets on the sweep, and writes `scores.md`: both numbers and health
  beside the control.

## Settled here

- G is #355, merged; main is 8d1b4ed. Main's sweep is `sweep-main.txt`/`.json` (run in this
  step). Civ stores hold no salt at +200 on main ("civ stores holding it 0" on every map): the
  store's keeping uses all its salt the turn it arrives (`probe.md`: 0–0.63 salt a turn reaches
  a capital, mean about 0.18; half the civs get under 0.1). The levy's cap never binds, so the
  reserve is salt the spoil doesn't use, not a higher cap.
- Carry, not Lines (`probe.md`, item 3).
- `relay/specs/340/digests-main.txt` is this branch's base: spec-review ran `digests.sh` at
  943eac6 and all 21 matched.
- **Expect the first number small.** A median civ's store gets about 20 salt over 200 turns,
  which buys about 7 food at the median price: half a turn of one army of 14. The count can
  pass 0 on most maps (any food bought counts) while the food is a rounding error. The glaze
  reports both, and may raise the salt reaching a store if numbers alone can't (below).
- Obsidian on main at +200 (`sweep-main.json`): mean 5.35 steps from a source, furthest 18;
  salt 3.52 and 11. That is the baseline both passings are read against.
- There is food to buy for: on main 1,643 army-turns stand outside the ring and 710 of them go
  hungry (`sweep-main.json`, supply counts), so an army with salt has a reason to seek a market.
- `markets.rs` is 531 lines and `ai.rs` 631: the unit rows and the passing go in
  `exchange.rs`, the planner's trading in `units/trade.rs`.
- The digest is the world's JSON (`stats::digest`), so lots in the state move it; skipped when
  empty, they leave the control's untouched.

## Unsettled, and who holds it

- Carry's second walk: when it is worth a clan's days and what it costs (build-1), by health
  on the sweep beside main; obsidian and salt steps reported.
- `UNIT.salt`, `KEEPING.packs`, `PLAN.trade_at` (build-4): army-turns fed at a market not the
  civ's and the food they bought, read beside health. If the reserve can't fill on most civs,
  the glaze may give the levy its own share for salt (`KEEPING.levy`, today `CIV.levy`'s 0.2)
  and records it as a decision.

## Decisions I made

- **Design says "One route. Each clan walks one route, to one festival"** and lists "More than
  one festival per clan" out of scope (`clans.md`). The piece's Decided names a clan carrying
  light goods to a second festival as one way to pass goods, so a clan with unsold wares walks
  a second route that carries only them; its own festival stays one.
- **Design says "Every unit of a good in a store remembers the market it last traded at."** A
  lot holds its whole chain, not only the last market: chasing the last market back through
  every store each time the page asks would need history the world doesn't keep. Chains are
  capped at `LOTS.chain` markets, lots at `LOTS.most` a good a store.
- "A rival's" market is any other living civ's capital or settlement market; a fallen civ's is
  an ordinary festival, as `fade` treats it.
- A unit trades at its own civ's markets too ("any market", the issue).
- A unit brings only the salt that buys its pack's room, so trade never overfills a pack.
- A unit's trade gives no leaning: leaning by trade is what a civ's posting does.
- Scouts trade as armies do; the army-turns score counts armies only.
- A civ's store keeps `KEEPING.packs` salt for packs back from its keeping: its spoil uses
  only the salt beyond it. (Spec-review: a higher cap alone holds nothing, `probe.md`.)
- **Design says** the designer settles Carry or Lines "by which moves obsidian further on the
  retro maps". Spec-review settled it by probing what each could move (`probe.md`) rather than
  building both: Lines has almost nothing to move. Carry's own sweep row is in `pass.md`.
- The trade group counts a trade per row per clearing, not per pair: barter clears all rows at
  once, so there are no pairs to count.
- The exchange-off control is the branch with `Config.exchange` off, proved equal to main by
  the 21 digests, so the sweep shows the control beside each map as it does for goods.

## Ideas taken

None. Every open idea is outside this goal: signposts, scouts, gifts, alerts, claims, town
clans, and obsidian's strength in the raise view (G's view, not exchange).

## For the review

- `relay/specs/340/digests.sh` with `exchange` off (the default flipped by a one-line local
  change, or the env/flag build-1 adds): all 21 equal `relay/specs/340/digests-main.txt`.
- The sweep's trade group: army-turns fed at a market not the civ's (control 0) and the largest
  share by count and by worth at +200, beside the control, in the PR. Health within the floor
  of `judge --compare sweep-main.json <branch>.json`.
- `pass.md`: Carry's obsidian and salt rows beside main's, and health.
- Run `node www/play/exchange.mjs`: Fill moves salt store to pack; the flow layer draws a chain
  from a source.
- Play a small map as one civ: fill an army's pack, march it past the ring to a festival, and
  see it fed there (the feed line), then click the capital's obsidian and read the chain.
