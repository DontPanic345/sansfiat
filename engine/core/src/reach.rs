//! The festival search: the cheapest routes from one tile to the candidate tiles in reach of it, on flat
//! scratch arrays that a turn reuses. The costs and tie-breaks are `land::cost_map`'s: the cheapest
//! grid cost, then the lowest tile, and on an equal grid cost the first predecessor found. The search
//! stops once every candidate in reach is settled, which can be no earlier than the cheapest step on
//! the map allows: a candidate `n` hex steps off costs at least `n × cheapest`.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use crate::data::*;
use crate::land::{self, grid, passable};
use crate::world::World;

/// Scratch for the search, kept between calls: a grid cost and a predecessor per tile, the tiles
/// touched (reset after each search), and the cheapest step any tile can cost.
pub struct Scratch {
    best: Vec<(i64, f64)>,
    before: Vec<usize>,
    touched: Vec<usize>,
    wanted: Vec<bool>,
    heap: BinaryHeap<Reverse<(i64, usize)>>,
    cheapest: f64,
}

impl Scratch {
    /// Scratch for `w`'s map. The cheapest step is the least a tile can cost to enter: its terrain's
    /// cost, or its hidden trail cost.
    pub fn new(w: &World) -> Self {
        let n = w.tiles.len();
        let cheapest = w
            .tiles
            .iter()
            .flat_map(|t| [TERRAIN[t.terrain].move_cost, t.trail_cost])
            .flatten()
            .fold(f64::INFINITY, f64::min);
        Scratch {
            best: vec![(i64::MAX, 0.0); n],
            before: vec![0; n],
            touched: Vec::new(),
            wanted: vec![false; n],
            heap: BinaryHeap::new(),
            cheapest,
        }
    }
}

/// For each of `candidates` within `most` cost of `from`, in reach as `land::cost_map` would have it
/// (over passable tiles): its cost and its path from `from`, the tiles after `from` ending at it, as
/// `land::path` gives it. The search stops once every candidate that could be in reach is settled.
pub fn settle(
    w: &World,
    s: &mut Scratch,
    from: usize,
    most: f64,
    candidates: &[usize],
) -> BTreeMap<usize, (f64, Vec<usize>)> {
    let limit = grid(most);
    let h = w.tiles[from].hex();
    let mut pending = 0;
    for &t in candidates {
        let steps = f64::from(w.tiles[t].hex().distance(h));
        if steps * s.cheapest <= most + 1e-9 && !s.wanted[t] {
            s.wanted[t] = true;
            pending += 1;
        }
    }
    let mut out = BTreeMap::new();
    if pending > 0 {
        s.best[from] = (0, 0.0);
        s.touched.push(from);
        s.heap.push(Reverse((0, from)));
        while let Some(Reverse((g, i))) = s.heap.pop() {
            let (bg, c) = s.best[i];
            if g > bg {
                continue;
            }
            if s.wanted[i] {
                s.wanted[i] = false;
                pending -= 1;
                out.insert(i, (c, route(s, from, i)));
                if pending == 0 {
                    break;
                }
            }
            for j in land::neighbours(w, i) {
                let Some(step) = land::step_cost(w, i, j) else { continue };
                let nc = c + step;
                let ng = grid(nc);
                if ng > limit || !passable(w, j) || s.best[j].0 <= ng {
                    continue;
                }
                if s.best[j].0 == i64::MAX {
                    s.touched.push(j);
                }
                s.best[j] = (ng, nc);
                s.before[j] = i;
                s.heap.push(Reverse((ng, j)));
            }
        }
    }
    s.heap.clear();
    let touched = std::mem::take(&mut s.touched);
    for &t in &touched {
        s.best[t] = (i64::MAX, 0.0);
    }
    s.touched = touched;
    s.touched.clear();
    for &t in candidates {
        s.wanted[t] = false;
    }
    out
}

/// The tiles of the settled search's path from `from` to `to`, after `from`, ending at `to`.
fn route(s: &Scratch, from: usize, to: usize) -> Vec<usize> {
    let mut tiles = Vec::new();
    let mut at = to;
    while at != from {
        tiles.push(at);
        at = s.before[at];
    }
    tiles.reverse();
    tiles
}
