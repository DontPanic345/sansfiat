# 354 Supply from any store: progress

## Done

1. The rule (`units/line.rs`): a line runs to the cheapest store of the civ (capital or settlement) that
   holds a turn of the unit's food, else the cheapest of all; `ring` is one search from every store;
   `ring_capital` is the capital's alone; `store_of` names a unit's store. `settle::store`, `store_mut`,
   `store_market`, `store_name` read a store by its tile.
2. `forage::supply` and the scout's `line_price` draw on the line's store alone. `muster`: `Order::Raise`
   has `at` (a store's tile); people come from `givers_at` (the clans at that store's market), the pack
   from that store; into an army, from the store its line ends at.
3. The planner (`ai.rs`): home is the store the line ends at; `front` is the store nearest a rival's
   capital, where an idle army is posted and a new one raised when that store can; a rival is near once
   any store is within `PLAN.march`.
4. The judge: `army_far`, `army_far_hungry` (outside the capital's own ring), in the sweep's text too.
5. Views: the text view and the unit panel say what a unit draws on (`draws_on` in the web view); the
   text view lists raises at each settlement; the settlement panel has Raise…; the ring layer already
   draws `units::ring`, now the union.
6. Tests: `core/tests/stores.rs` (8 stages, one per Goal clause and Decided line); save version 23.
7. Scores: `scores.md`, `main-sweep.json`, `branch-sweep.json` (with floor), `compare.txt`.
8. Ledger lines in `supply.md`, `founding.md`, `war.md`.
9. `engine/check.sh` green on the last commit.

## Open, plainly

- Not played: no game was played through `sansfiat play`. The crew's play step does it.
- Not tuned: one sweep and its floor were run; `PLAN.march`, `PLAN.reach` and the store numbers are
  main's. Over seven maps the question barely moves, because settlements are few (scores.md).
- `www/play/army.mjs` keeps 5 people at the capital, not 3: with 3 the led capital fell before the
  army's skirmish on this branch's small 3.
- The settlement panel's Raise… is asserted as data in `check-stage.mjs`; no play script presses it on a
  settlement and reads the world after.
