# #356 rework: an army's line ends at a market

The earlier builds tuned against starving, which `vision.md` and `process.md` rule out, and
built a unit that trades only where it stands. This rework replaces both with the goal in
`docs/proposals/m5.md`: an army's supply line runs to a market that isn't a rival's.

## What went

- Carry (a clan's second route to another festival), `PASS.cover` and its fed gate. Clans walk
  one route (`clans.md`); trading between markets is a clan's job in M6 (`roadmap.md`).
- `Order::Fill`, `UNIT.salt`, `KEEPING.packs`, `KEEPING.levy`: each named salt as the pay.
- The planner's march to a market (`units/trade.rs`, `PLAN.trade_at`).

## The rule (`core/src/exchange.rs`, `core/src/units/line.rs`, `core/src/units/camp.rs`)

- An army its line home can't feed a turn is paid along that line from its store, each ware at its
  weight. A ware is pay when the friendly market nearest the army (within a line's reach) traded it
  last turn and it buys more food there than its own weight in food. Nothing names salt.
- Pay opens the army's stall at that market. Its pack's wares join that market's clearing, and the
  food it buys waits at the stall, spoiling as a store does.
- The line ends at the stall instead of the store when the food waiting there, up to a turn's
  eating, is more than the store's line brings, or as much on a cheaper path.
- Pay comes first while no stall can feed a turn; otherwise feeding comes first and pay tops up on
  the labour left.
- A store keeps the salt its armies' pay will take back from its own keeping (`exchange::pay_back`).
- A rival's market or a market on the civ's own store never ends a line.

## Proof

- `core/tests/exchange.rs`, on a built map: an army past its line's reach is paid salt from its
  capital, opens a stall at the festival next to it, buys food there, and its line brings it, so it
  eats on the second turn. No pay when the festival didn't trade the ware, when the ware buys less
  than its weight in food, or with exchange off. A rival's market never ends a line. The store keeps
  the pay back from its keeping.
- With exchange off, all 21 digests of `relay/specs/340/digests.sh` equal main's at 0fd7835
  (`digests-off.sh`).
- `engine/check.sh` passes; `save.mjs` timed out once under a parallel load and passes alone.

## On the retro maps (`army_flow`, 200 turns past the pick, every civ on the AI)

| map | army-turns | past reach | paid | fed from a stall | bought (food) | mean store salt |
|---|---|---|---|---|---|---|
| small 2 | 596 | 259 | 0 | 0 | 0 (0.0) | 0.04 |
| small 3 | 452 | 168 | 6 | 0 | 9 (0.7) | 0.01 |
| small 6 | 403 | 226 | 32 | 0 | 31 (4.5) | 0.06 |
| small 9 | 600 | 322 | 2 | 0 | 2 (0.5) | 0.05 |
| medium 8 | 748 | 271 | 6 | 0 | 5 (0.4) | 0.08 |
| medium 9 | 736 | 228 | 0 | 0 | 0 (0.0) | 0.06 |
| large 8 | 704 | 361 | 15 | 0 | 15 (0.5) | 0.00 |

The chain fires on five maps: small 6's army 298, at +16, was paid 0.58 salt by its capital,
bought 0.39 food for it at the festival at (-1, 3), and its line brought it. No army is fed from
a stall: a store holds a mean of under a tenth of a unit of salt, so pay buys crumbs. An army of
three eats 3 food a turn; at the festivals' prices that is several units of salt a turn, and a
capital gets about 0.2.

What stops it is the salt reaching a civ's store, not the line. That is the owner's to rule on
(the PR lists the ways), not a number this rework tunes.
