# #28 build-1: the map's hidden costs (ford)

Read `relay/specs/28/spec.md`, Settled 1–4. Engine only (`engine/core`).

Deliver:
- `land::step_cost`: a tile with a trail costs `min(bare, trail_cost)` (bare when no
  `trail_cost`, marsh included); a mountain with a trail is entered at its `trail_cost`;
  `passable` true for a trail on a mountain. `TRAIL.cost` removed. River rule unchanged.
- `units::walkable` becomes `passable && (land || trail)`, so the camp's moves and line cross
  a built pass; `check_move` still rejects a mountain target. Territories (`actors::edge`,
  `clans.rs`, `pregame.rs`) keep `land && passable`.
- `CIV.bridge_span = 1` in `data.rs`; a helper `land::trailable(w, i)`: `trail_cost` set and
  (not mountain, or `span <= CIV.bridge_span`).
- `Tile.trail_work: f64` (`serde(default)`) and `land::build_trail(w, i, labour) -> bool`
  (true when finished: sets `trail`, resets work, and sets `rest_run` to 0, or `hold` wears
  the new trail away that same turn, since an unwalked tile's `rest_run` is in the hundreds) with `TRAIL.work` per terrain (forest 2,
  hills 3, mountain 6 times a scale build-2 may change). Worn trails never land on mountains.
- `Civ.revealed: BTreeSet<usize>` (`serde(default)`).
- A generic `land::path_by(w, from, to, cost: impl Fn(usize, usize) -> Option<f64>)` that
  `path` is built on (no behaviour change for `path`).
- Fix the map.rs comment "nothing reads it yet".
- Tests (`core/tests/land.rs`, `wear.rs`): trail costs, mountain pass passable only with a
  trail, a built trail survives the turn's `hold` and then decays unwalked after
  `TRAIL.fade`, a camp's `path_between` crosses a built pass, `path_by` equals `path` on the
  real cost.

Proof: spec-review already ran the pre-game with real trail costs (`relay/specs/28/sweep.md`:
it settles, 21 of 21 civs founded on the retro maps; no fallback cap). Re-run
`sansfiat sweep --size all` and `sansfiat sweep --after 200 --maps retro` (release, under a
minute) after your change and append the summary lines to `sweep.md`. They should match its
"after" lines; if they don't, say why in your summary.

Done: `engine/check.sh` passes. build-2 relies on `trailable`, `build_trail`, `path_by`,
`Civ.revealed`, and units walking over built passes.
