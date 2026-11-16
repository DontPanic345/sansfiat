# 354 notes for the next builder

- `line_at` runs one `land::path` per store of the civ; fine at a few settlements a civ, worth one
  multi-target search if a civ comes to keep many.
- `judge_ring` computes a second ring (`ring_capital`) a turn for a civ with a settlement and a unit away
  from its capital, for `army_far`.
- `givers_at` is why two older tests set `market` on the clans they place: a clan gives to a unit only
  at the store whose market it goes to.
- `core/tests/stores.rs` stages Red with one settlement between two capitals; `stage()` there is the
  shortest way to a stocked settlement.
- The planner raises a new army at its front store only when that store alone passes `feeds_one_more`;
  a settlement's store (cap half the capital's) seldom does, so most armies are still raised at the capital.
