//! The planner's paths, found once a turn. `ai` keeps one `Paths` for its turn and every civ's
//! planning reads it: nothing the planner's step does changes what `path_between` reads (terrain,
//! trails and rivers change only in the turn's `wear` and `tend`), so a path found once is the path
//! all turn.

use std::collections::BTreeMap;

use super::line::{path_between, walkable};
use crate::data::*;
use crate::land::{cost_map, step_cost};
use crate::world::World;

/// The planner's caches for one turn: the paths it has found by their ends, and the tiles
/// reachable by land from each start.
#[derive(Default)]
pub(super) struct Paths {
    paths: BTreeMap<(usize, usize), Option<Vec<usize>>>,
    reach: BTreeMap<usize, Vec<bool>>,
}

impl Paths {
    pub(super) fn new() -> Self {
        Self::default()
    }
}

/// `path_between`'s tiles from `from` to `to`, found once in `paths`.
pub(super) fn path_cached(w: &World, paths: &mut Paths, from: usize, to: usize) -> Option<Vec<usize>> {
    paths.paths.entry((from, to)).or_insert_with(|| path_between(w, from, to).map(|(p, _)| p)).clone()
}

/// Whether `path_between(from, to)` finds a path: `to` is `from`, or reachable from it by the steps
/// `path_between` takes (a flood fill from `from`, found once in `paths`).
pub(super) fn reaches(w: &World, paths: &mut Paths, from: usize, to: usize) -> bool {
    to == from || paths.reach.entry(from).or_insert_with(|| flood(w, from))[to]
}

/// The tiles a march of `PLAN.march` turns from `from` can reach, with their cost. `turns` walks at
/// most `UNIT.labour` of step cost a turn, so a tile past this bound is past the march. The slack
/// keeps float rounding from dropping one at the edge.
pub(super) fn within_march(w: &World, from: usize) -> BTreeMap<usize, f64> {
    let bound = (PLAN.march as f64 * UNIT.labour) * 1.001 + 1e-6;
    cost_map(w, &[from], bound, |i| walkable(w, i))
}

/// The tiles reachable from `from` over the steps `path_between` takes: from `i` into a neighbour
/// `j` when `step_cost(i, j)` is a step and `j` is walkable. `path_by` has no cost cap, so the flood
/// reaches what the search does.
fn flood(w: &World, from: usize) -> Vec<bool> {
    let mut seen = vec![false; w.tiles.len()];
    let mut todo = vec![from];
    seen[from] = true;
    while let Some(i) = todo.pop() {
        for j in crate::land::neighbours(w, i) {
            if !seen[j] && step_cost(w, i, j).is_some() && walkable(w, j) {
                seen[j] = true;
                todo.push(j);
            }
        }
    }
    seen
}
