//! Moving over the land, food growing back on it, and what holding it does: trails and tame grain.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

use crate::data::*;
use crate::map::{self, Tile};
use crate::world::{Event, EventKind, World};

/// On-map neighbours of tile `i`, in `DIRS` order.
pub fn neighbours(w: &World, i: usize) -> impl Iterator<Item = usize> + '_ {
    let radius = w.config.radius;
    w.tiles[i].hex().neighbors().filter_map(move |n| map::index(radius, n))
}

/// Whether tile `i` can be entered at all.
pub fn passable(w: &World, i: usize) -> bool {
    TERRAIN[w.tiles[i].terrain].move_cost.is_some()
}

/// Labour to step from tile `from` into tile `to`; `None` is impassable. A trail caps the
/// terrain's cost at `TRAIL.cost`. Entering a river tile costs `RIVER.cross` more, unless the
/// step is along the river's course (either tile flows into the other) or a trail bridges it.
pub fn step_cost(w: &World, from: usize, to: usize) -> Option<f64> {
    let t = &w.tiles[to];
    let along = w.tiles[from].flow == Some(to) || t.flow == Some(from);
    let cross = if t.river > 0 && !t.trail && !along { RIVER.cross } else { 0.0 };
    let terrain = TERRAIN[t.terrain].move_cost?;
    Some(if t.trail { terrain.min(TRAIL.cost) } else { terrain } + cross)
}

struct Entry(f64, usize);

impl PartialEq for Entry {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == Ordering::Equal
    }
}
impl Eq for Entry {}
impl PartialOrd for Entry {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Entry {
    /// Reversed, so the heap pops the cheapest, then the lowest tile.
    fn cmp(&self, o: &Self) -> Ordering {
        o.0.total_cmp(&self.0).then(o.1.cmp(&self.1))
    }
}

/// Path cost from the nearest of `from` to every tile within `max`: sources cost 0, each step
/// costs its `step_cost`, and only `allowed` tiles are entered.
pub fn cost_map(w: &World, from: &[usize], max: f64, allowed: impl Fn(usize) -> bool) -> BTreeMap<usize, f64> {
    let mut best: BTreeMap<usize, f64> = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    for &s in from {
        best.insert(s, 0.0);
        heap.push(Entry(0.0, s));
    }
    while let Some(Entry(c, i)) = heap.pop() {
        if best.get(&i).is_some_and(|&b| c > b) {
            continue;
        }
        for j in neighbours(w, i) {
            let Some(step) = step_cost(w, i, j) else { continue };
            let nc = c + step;
            if nc > max || !allowed(j) || best.get(&j).is_some_and(|&b| b <= nc) {
                continue;
            }
            best.insert(j, nc);
            heap.push(Entry(nc, j));
        }
    }
    best
}

/// The cheapest path from `from` to `to` by `step_cost`, over `allowed` tiles: the tiles after
/// `from`, ending at `to`, and its cost. Dijkstra as `cost_map` is, with the same tie-breaks
/// (cheapest, then lowest tile); on an equal cost a tile keeps the predecessor found first.
/// `from == to` is `([], 0)`; `None` when `to` can't be reached.
pub fn path(w: &World, from: usize, to: usize, allowed: impl Fn(usize) -> bool) -> Option<(Vec<usize>, f64)> {
    if from == to {
        return Some((Vec::new(), 0.0));
    }
    let mut best: BTreeMap<usize, f64> = BTreeMap::new();
    let mut before: BTreeMap<usize, usize> = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    best.insert(from, 0.0);
    heap.push(Entry(0.0, from));
    while let Some(Entry(c, i)) = heap.pop() {
        if best.get(&i).is_some_and(|&b| c > b) {
            continue;
        }
        if i == to {
            break;
        }
        for j in neighbours(w, i) {
            let Some(step) = step_cost(w, i, j) else { continue };
            let nc = c + step;
            if !allowed(j) || best.get(&j).is_some_and(|&b| b <= nc) {
                continue;
            }
            best.insert(j, nc);
            before.insert(j, i);
            heap.push(Entry(nc, j));
        }
    }
    let cost = *best.get(&to)?;
    let mut tiles = vec![to];
    while let Some(&p) = before.get(tiles.last()?) {
        if p == from {
            break;
        }
        tiles.push(p);
    }
    tiles.reverse();
    Some((tiles, cost))
}

/// The `regrow` step: every food on every tile moves towards its cap. Game breeds, so a
/// hunted-out tile recovers slowly.
pub fn regrow(w: &mut World) {
    for t in &mut w.tiles {
        for (f, food) in FOOD.iter().enumerate() {
            let (cap, stock) = (t.cap[f], t.stock[f]);
            if cap <= 0.0 {
                continue;
            }
            let grown = if food.breeds {
                stock + cap * food.restock + food.regen * stock * (1.0 - stock / cap)
            } else {
                stock + cap * food.regen
            };
            t.stock[f] = grown.min(cap);
        }
    }
}

/// A fully tamed tile: its grain cap is `TENDING.domestic` times its wild grain.
pub fn is_hearth(t: &Tile) -> bool {
    t.wild_grain > 0.0 && t.tame >= TENDING.tame_turns
}

/// The `hold` step, after eating: each tile counts the turns it has been on some round, or on
/// none. Held `TRAIL.after` turns it gets a trail, unheld `TRAIL.fade` it loses it. Grain held
/// `TENDING.after` turns tames a step a turn, unheld it goes wild a step a turn, and its cap
/// follows.
pub fn hold(w: &mut World) {
    let round1 = |v: f64| (v * 10.0).round() / 10.0;
    let turn = w.turn;
    for (i, t) in w.tiles.iter_mut().enumerate() {
        let held = t.claim.is_some();
        if held {
            t.held += 1;
            t.unheld = 0;
        } else {
            t.held = 0;
            t.unheld += 1;
        }
        if t.held >= TRAIL.after {
            t.trail = true;
        } else if t.trail && t.unheld >= TRAIL.fade {
            t.trail = false;
        }
        if t.wild_grain <= 0.0 {
            continue;
        }
        if held && t.held >= TENDING.after {
            let before = t.tame;
            t.tame = (t.tame + 1).min(TENDING.tame_turns);
            if before < TENDING.tame_turns && t.tame == TENDING.tame_turns {
                w.events.push(Event {
                    turn,
                    kind: EventKind::HearthTamed,
                    tile: i,
                    clan: None,
                    market: None,
                    other: None,
                    civ: None,
                    unit: None,
                });
            }
        } else if !held {
            t.tame = t.tame.saturating_sub(1);
        }
        let tame = t.tame as f64 / TENDING.tame_turns as f64;
        t.cap[GRAIN] = round1(t.wild_grain * (1.0 + (TENDING.domestic - 1.0) * tame));
    }
}
