# #151 build-3: the cases (core/tests/cases.rs)

Run them with:

    cargo test -p engine --test cases -- --nocapture

Each red case is `#[should_panic(expected = …)]`, so the file is green on main; `--nocapture`
prints the panic text the fix must remove. Measured on main at 21:01 UTC, release-profile test
build (the workspace's `test` profile), all five tests in 17.3 s; the owner's invariant alone
takes 17.5 s (the 14 errands, the seven pre-games cached once, about 5 s of it).

## The scout

Setup for every errand: `Stage::retro(size, seed)`, lead the first civ, route its scout from its
capital to the nearest rival capital ("to the rival") or to `Lead`'s target (the first forest or
hills tile seven steps out, "to Lead's target"). Each turn the civ's store and the scout's pack are
refilled to `pack_cap` before the turn. An errand settles when `built` is true or the route is
cleared with the scout alive; 120 turns without that fail with their own message.

### 1. `settled_scouts_leave_nothing_unrevealed_that_could_beat_them` (green on main)

- Command: the test above.
- Main: passes on all 14 errands. At settle, `route(civ, cap, to).candidates` is empty on each.
- T must do: nothing. It guards the search change: a fix must keep it green.

### 2. `a_scout_searches_a_range_by_best_case` (red on main)

- Command: the test above, `--nocapture`.
- Main, the panic text (small 9 to the rival, turn 2; the probe's first case):

      small 9 turn 2: the scout revealed mountain 304 (best case 28.0) while 306 (best case 26.0) was open

- Medium 8 to the rival (not reached by the test while small 9 fails; run on its own on main):

      medium 8 turn 2: the scout revealed mountain 242 (best case 28.0) while 274 (best case 27.0) was open

- The probe's counts (spec.md, "Measured before cutting"): 12 of 29 reveals out of order on
  small 9 to the rival, 4 of 11 on medium 8.
- T must do: order the search's mountains by best case: the cost through the mountain plus the
  scout's known walk to a tile beside it (0 when it stands beside it). Then remove the
  `#[should_panic]` attribute. The best case is priced in `best_case` in the test; the engine's own
  order is `ordered` in `core/src/units/scout.rs`.

### 2b. `a_scouts_search_on_small_9_to_the_forest_is_in_order` (green on main, the control)

- Passes on main: small 9 to Lead's forest reveals two mountains, both in order.
- T must do: nothing; it must stay green.

### 3. `a_settled_route_has_no_cheaper_trail` (red on main, not T's)

- Command: the test above, `--nocapture`.
- Main, the panic text (small 2 to the rival, turn 2; the first errand):

      small 2 to the rival turn 2: the settled route costs 15.0 known and 14.0 possible; the step into tile 144 costs 2.0 where known it costs 4.0

- Small 3 to Lead's target (run on its own on main; settles on turn 51):

      small 3 to Lead's target turn 51: the settled route costs 19.0 known and 18.0 possible; the step into tile 436 costs 2.0 where known it costs 4.0

- Tiles 144 (hills, river) and 436 (hills, river) are revealed, so the owner's invariant (case 1)
  does not see them. A trail on either would bridge the river and cut the route by one.
- T must do: nothing. The scout builds trails only on its known path, so this waits for a fix piece
  (the spec-review files it as a fix: the scout builds the bridge on a revealed river tile on the
  possible path). Do not change the test to make it pass.

## Disband all

### 4. `disband_all_sends_every_person_home` (green on main, the control)

- Command: the test above.
- Main: passes. On a small 2 stage the led civ's army (4.6 people), scout (1.5) and guard (2.5) are
  each disbanded with `Order::Disband { n: None, fighter: None }`: each unit is gone, the clans gain
  exactly its people, and the world's people are conserved.
- The red case is the page's form (build-4, `www/check-stage.mjs`): the page's `all` option sends
  `n` = floor(people), which removes one person of the army of 4.6 (the stepper opens at 1).
  No Rust test sends the page's count: that would pin an engine change T does not make.
- T must do: nothing in the engine. For the page, make the `all` option send no count.

## Disband all, the page arm

The page's `all` option (`www/panels.js`'s `disband`) sends `n` = floor(people). The stepper opens at
1, so pressing Go as it opens sends `n` = 1. Run by `node www/check.mjs` (after `./build-web.sh`, from
`engine/`), in `www/check-stage.mjs`. Each press is wrapped in `expectBug`, so the file passes on main;
the messages below are the failures the cases raise on main once the wrapper is dropped.

- Camp of 4.6, pressed as the stepper opens (`n` = 1):

      Disband all sends the whole army home: the army is still on the map

- Camp of 4.6, pressed at its cap (`n` = 4): the same message, the army keeps 0.6.
- Scout (2 people, the founding size) pressed as the stepper opens (`n` = 1):

      Disband all sends the whole scout home: the scout is still on the map

- Guard (3 people, its founding size) pressed as the stepper opens (`n` = 1):

      Disband all sends the whole guard home: the guard is still on the map

- The scout's and the guard's caps (2 and 3) are whole, so they send all of it on main: their cap presses
  are plain assertions that pass, not `expectBug`s.
- T must do: make `all` send no count, so the engine disbands the whole unit (as the Rust control does).
  Then drop the four `expectBug` wrappers in `check-stage.mjs`; the messages stay as the case's own.

## Summary

| Case | Test | Main | T |
|---|---|---|---|
| Owner's invariant | `settled_scouts_leave_nothing_unrevealed_that_could_beat_them` | green | keep green |
| Search order | `a_scout_searches_a_range_by_best_case` | red, `should_panic` | order by best case; remove the attribute |
| Search control | `a_scouts_search_on_small_9_to_the_forest_is_in_order` | green | keep green |
| River bridge | `a_settled_route_has_no_cheaper_trail` | red, `should_panic`, not T's | nothing (a fix piece) |
| Disband control | `disband_all_sends_every_person_home` | green | keep green |
| Disband all, the page arm | `expectBug` in `www/check-stage.mjs` | red (four presses) | make `all` send no count |
