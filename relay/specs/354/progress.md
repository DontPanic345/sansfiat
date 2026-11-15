# 354 Supply from any store: progress

## Plan

1. Core rule (`units/line.rs`): a civ's stores are its capital and its settlements. A unit's line runs
   to the store it reaches for the least labour among those that hold a turn of its food (else the
   cheapest of all). The ring is one search from every store. `settle::store` reads a store by tile.
2. `forage::supply` draws on the line's store alone; `muster` raises at a store (`Order::Raise.at`),
   from the clans at its market, with a pack from it.
3. Planner (`ai.rs`): home is the nearest store; a rival is near once any store is within
   `PLAN.march`; raise at the store nearest the rival.
4. Judge: `army_far`, `army_far_hungry` (outside the capital's ring), comparable with main's
   `army_outside`.
5. Views: text view and unit panel name the store; the ring layer draws the union.
6. Stage tests, save version, sweep on branch and main, play, PR.

## Done

(nothing yet)
