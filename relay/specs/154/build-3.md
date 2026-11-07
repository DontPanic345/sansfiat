# #154 build-3 (easel): the ring

The ring round a civ: the tiles from which an army's line feeds it with days to spare, in core,
on the page and in the judge. Read `spec.md` item 8, the ring's rows of the measures table, and
"What the ring's layer shows at a glance" under What I settled. `labour.md`, Supply as a flow,
is the rule.

The code: `core/src/units/line.rs` (`line_at`, `line_from`, `past_reach`, `walkable`;
`civs::holder`), `core/src/land.rs` (`cost_map_by`, `step_cost`, `grid`), `core/src/judge.rs`
(`Watch`, `Scores`; over 600 lines, so the new counts go in a new `core/src/judge_ring.rs`),
`cli/src/sweep_judge.rs` and `sweep_text.rs` (the report and its text), `web/src/lib.rs`
(`line_from`, `route`), `www/map/view.js` and `www/map/layers/lines.js` (a layer's shape),
`www/screens/play.js` (selection, the tooltip, `homeLine`), `www/play.mjs`, `www/play/camp.mjs`,
`www/play/all.mjs`, `core/src/stage.rs`.

## What to build

1. **`units::line::ring(w, civ) -> BTreeMap<usize, f64>`**: for every tile from which a line may
   reach `civ`'s capital (the tiles `line_at` would accept, reading held ground), the share of an
   army's days its line takes to feed it there:
   `UNIT.food × walk_cost(path cost, 1) / (UNIT.labour × UNIT.carry)`. The capital is 0. One
   Dijkstra from the capital over the reversed steps (a line's cost is the steps into each tile
   on the way home, so price the step from `j` into `i` when you reach `j` from `i`), on `grid`
   as the path searches compare. Doc: inside the ring the share is under one; it has no army
   size in it, so one map serves every army and scout of the civ. A fallen civ has no ring.
   Test in `core/tests/ring.rs`: on a retro map at the pick and on a stage, for every tile in
   the ring, `ring` equals `eats / carry` from `line_from` for a 3-person camp to 10⁻⁹, and a
   tile `line_from` can't reach is absent.
2. **The staged walk-out** (`core/tests/ring.rs`): a straight plains stage, the capital at one
   end; for each tile along it, a 3-person army staged there with a full pack, auto-supply and
   auto-forage on (forage off the tile: give the tiles no food), run 20 turns. Where the share is
   under one it never goes hungry; on the first tile past the ring it is hungry within
   `UNIT.pack + 2` turns.
3. **`Sim.ring(civ)`** in `web/src/lib.rs`: the ring as JSON `[[tile, share], …]`, empty for no
   civ or a fallen one.
4. **`www/map/layers/ring.js`**, registered in `view.js` under the unit layers. Drawn when an
   army or a civ is selected (the civ of the selection): each tile in the ring with share under
   one filled in the civ's colour at alpha `0.08 + 0.32 × (1 − share)`, and a solid line in the
   civ's colour, edged dark as the line home is, along every hex edge between a tile inside
   (share under one) and one outside it or past the ring. Nothing past the ring. Not drawn for a
   clan, a festival, a scout alone or nothing selected. The tile tooltip, with an army or civ
   selected, adds "Ring: its line takes 40% of an army's days here" inside, or "Past the ring:
   its line can't feed an army here" past it.
5. **`www/play/ring.mjs`**, in `all.mjs`: on small seed 2, raise an army (or stage one), select
   it: the ring layer is drawn and covers the capital's tile; select the civ: still drawn;
   select a clan: gone. Check the tooltip's ring line on the capital and on a tile past the ring.
   Screenshot `relay/specs/154/ring.png`.
6. **The judge's counts**, in `core/src/judge_ring.rs`, read by `Watch` each turn and reported as
   a `supply` group in `Scores` (and the sweep's report and text): per civ and in total, the
   tiles in the ring and the held tiles in it at the pick and +200; army-turns and scout-turns
   with the unit inside the ring and outside it (share at its tile under one, else outside), and
   the hungry turns among each; and the mean territory tiles a clan at the pick and +200. A unit's
   share is read from its civ's ring (compute the ring once a civ a turn, or only on turns a civ
   has a unit away from its capital).
7. **`docs/design/supply.md`**, The pack: after "A full pack holds several turns of food." add
   "The pack is the army's train: what it carries with it, measured in turns of its eating."
   Change nothing else.

## Proof and record

- `engine/check.sh` green (it runs `ring.mjs` through `all.mjs`).
- `sansfiat sweep --judge --json` on the branch: the `supply` group's totals and per map in
  `relay/specs/154/ring.md`: tiles in the ring and held in it, army- and scout-turns inside and
  outside, and the hungry rate in each (hungry turns over unit-turns). The spec's bound (outside
  at least four times inside; inside under 0.05) is judged by build-4 on the finished rules;
  report where it stands now, and if it fails, which units starve inside the ring and why.

## What later steps rely on

`ring` and its agreement with `line_from`; the `supply` group in the sweep's JSON (build-4 reads
the hungry rates and the territory tiles from it); `ring.mjs`.

## Done

`check.sh` green; `ring.md` and `ring.png` written; `engine/CLAUDE.md` names `judge_ring.rs`,
`ring.js` and `ring.mjs`, and stays under 8 KB.
