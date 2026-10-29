# #28 build-1: the map's hidden costs (ford)

Read `relay/specs/28/spec.md`, Settled 1–4. Engine only (`engine/core`).

Deliver:
- `land::step_cost`: a tile with a trail costs `min(bare, trail_cost)` (bare when no
  `trail_cost`); a mountain with a trail is entered at its `trail_cost`; `passable` true for a
  trail on a mountain. `TRAIL.cost` removed. River rule unchanged.
- `CIV.bridge_span = 1` in `data.rs`; a helper `land::trailable(w, i)`: `trail_cost` set and
  (not mountain, or `span <= CIV.bridge_span`).
- `Tile.trail_work: f64` (`serde(default)`) and `land::build_trail(w, i, labour) -> bool`
  (true when finished: sets `trail`, resets work) with `TRAIL.work` per terrain (forest 2,
  hills 3, mountain 6 times a scale build-2 may change). Worn trails never land on mountains.
- `Civ.revealed: BTreeSet<usize>` (`serde(default)`).
- A generic `land::path_by(w, from, to, cost: impl Fn(usize, usize) -> Option<f64>)` that
  `path` is built on (no behaviour change for `path`).
- Fix the map.rs comment "nothing reads it yet".
- Tests (`core/tests/land.rs`, `wear.rs`): trail costs, mountain pass passable only with a
  trail, built trail decays unwalked after `TRAIL.fade`, `path_by` equals `path` on the real cost.

Proof, written to `relay/specs/28/sweep.md`: `sansfiat sweep --size all` levels off and the
retro maps (small 2, 3, 6, 9; medium 8, 9; large 8) still found three civs, before and after.
If not, keep the old 1.0 cap for worn trails only, and record it as a decision.

Done: `engine/check.sh` passes. build-2 relies on `trailable`, `build_trail`, `path_by`,
`Civ.revealed`.
