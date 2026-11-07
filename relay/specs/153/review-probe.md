# #153 spec-review probe: the feeding rule on the retro maps

Run by spec-review on `main` at `feb37ff` in a scratch worktree: a rough version of spec items
14–17, read from environment variables, against `core/tests/scout_errand.rs` (the first civ
led, its scout routed at the pick, 50 turns, no refill). Not code to keep: it checks whether the
rule as the spec words it can pass, and what its starting numbers should be.

What the probe built:

- **Graze**: a scout takes per tile at most `min(stock, yield − take)` (`wear`'s yield, after the
  clans' harvest), adds it to `take`, and never sets `fallow`. Armies unchanged.
- **Wander**: at the start of its turn a scout whose pack holds under `LOW` turns of eating, or
  that is hungry, picks rich ground within `W` steps (nearest by walk cost, else the richest),
  walks there, and forages there for its whole room (not just this turn's eating) until the pack
  holds `FED` turns. It does not search while it wanders.
- **Rich ground**, two ways: **raw**, the round's spare summed (spec item 15 as written); and
  **labour-aware**, what one turn's foraging there would bring with the scout's labour after
  the walk to each tile of the round (`forage_round`'s own costing). The threshold is `RICH ×`
  its eating.
- **FILL**: on its errand too, auto-forage fills the pack, not just this turn's eating.
- **FLAT**: plains and coast get `trail_cost = bare × 0.75` (build-1's rule).

## Routed errands lost, of 14

| Setting | Main's maps | With the flat trail (0.75) |
|---|---|---|
| Graze only, no wander | — | **13** |
| Main (`probe.md`) | 11 | — |
| Raw rich ground, RICH 1, LOW 1, FED 3, W 4 (spec's starting numbers) | 3 | — |
| Raw, RICH 1, W 6 | 3 | — |
| Raw, RICH 1, LOW 2, FED 4, W 6 or 8 | 2 | — |
| Raw, RICH 2, LOW 2, FED 4, re-pick on arrival | 2 | — |
| **Labour-aware, RICH 1.5, LOW 2, FED 4, W 4, re-pick** | **0** | **3** (small 2 to Lead's lost 1 person but lived; small 6 to Lead's, small 9 to the rival and large 8 to the rival lost) |
| Labour-aware, RICH 1.5, LOW 2, FED 4, W 6 | 1 | — |
| Labour-aware, RICH 1.2, LOW 1, FED 3, W 6 | 1 | — |
| Labour-aware, RICH 1.5, LOW 2, FED 4, FILL | — | 2 |

Idle errands (no route), labour-aware RICH 1.5: 0 of 7 lost on main's maps, 1 of 7 (small 3)
with the flat trail. The small 3 loss is the probe's: its idle scout re-checked its ground after
foraging, so it bounced between two grounds a step apart and starved beside a capital whose line
cost 1.5. Spec item 17 reads the ground before foraging and keeps it.

## Turns wandering, of 50, on the routed errands

Labour-aware, RICH 1.5, LOW 2, FED 4, main's maps: 36, 0, 47, 41, 24, 42, 42, 48, 47, 47, 34, 34,
28, 4. That is 474 of 700 scout-turns (68%) off the errand. With FILL: 311 of 700 (44%). With the
flat trail and FILL: 270 of 700 (39%).

Most routed errands find no route in 50 turns either way (the scout sees `heard` within 3
turns on 5 of 14, with the wander or without it), so "found" doesn't tell a working
scout from a fed idle one. The reveals and trails it makes, against a refilled control
(`cases.rs`'s `refill`: the same errand with the pack topped up each turn), do.

## What it settles

1. **Rich ground counts the walk.** The round's raw spare picks hills and coast whose food the
   scout can't reach with its labour (on hills a neighbour costs 4 of its 7 labour there and
   back). Raw loses 2 to 5 of 14; labour-aware loses 0 on main's maps. Spec item 15 now
   measures rich ground as what a turn's foraging there brings.
2. **Rich ground spares more than its eating.** A round that spares only one turn's eating can
   never fill the pack, so `fed` is never reached. `SCOUT.rich` 1.5 (turns of eating a turn's
   foraging there brings) is the starting number.
3. **A wandering scout forages for its pack's room**, not just this turn's eating
   (`forage_round` today wants only `need − store`). Without it the wander never ends.
4. **Starting numbers**: `wander 4`, `low 2.0`, `fed 4.0`, `rich 1.5`. On main's maps they keep
   all 14 routed and all 7 idle scouts alive. On the flat-trail maps 3 routed errands still die,
   so build-3's errand test most likely lands ignored and the canvas has real work.
5. **The canvas needs a bound on wandering**: the starting numbers keep scouts alive by
   spending 68% of their errand off it. FILL (forage to fill on the errand) cuts that to 44%.

## The work the scout does

Tiles the led civ has revealed at 50 turns on each routed errand, flat-trail maps (rival, then
Lead's target, per map in the table's order):

| Setting | Revealed per errand | Total |
|---|---|---|
| Refilled control (the pack topped up each turn, no wander: it never needs to feed) | 11 2 1 25 8 2 19 14 12 9 23 5 18 3 | **152** |
| Labour-aware, RICH 1.5, LOW 2, FED 4 | 1 2 1 1 1 2 1 8 3 3 3 3 0 3 | **32** (21%) |
| The same with FILL | 0 1 1 1 0 2 1 3 4 4 1 1 0 3 | **22** (14%) |

The scouts that live do a fifth of a fed scout's work or less: past the line's reach its labour
goes on its food. Wandering less (FILL) doesn't buy more work, since the foraging that fills
the pack takes the same labour. So survival and work pull against each other at these
numbers. The canvas reports both, and picks among the settings that keep all 28 alive the
one that makes the most reveals and trails on the routed errands. The piece's question is
survival. The work is reported against the control, and an `ask` goes to the owner if it stays
under a quarter.
