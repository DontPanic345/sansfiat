//! Moving over the land, food growing back on it, what taking it wears down, and what walking and
//! holding it do: trails and tame grain.

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

/// Whether tile `i` can be entered at all: its terrain can be entered, or it has a trail that
/// crosses it (a built pass over a mountain).
pub fn passable(w: &World, i: usize) -> bool {
    let t = &w.tiles[i];
    TERRAIN[t.terrain].move_cost.is_some() || (t.trail && t.trail_cost.is_some())
}

/// Whether a trail can be built on tile `i`: it has a hidden trail cost, and a mountain's span is
/// one `CIV.bridge_span` can bridge.
pub fn trailable(w: &World, i: usize) -> bool {
    let t = &w.tiles[i];
    t.trail_cost.is_some() && (t.terrain != MOUNTAIN || t.span.is_some_and(|s| s <= CIV.bridge_span))
}

/// Labour to step from tile `from` into tile `to`; `None` is impassable. A tile with a trail costs
/// the less of its terrain and its hidden `trail_cost`; a mountain with a trail costs its
/// `trail_cost` alone. Entering a river tile costs `RIVER.cross` more, unless the step is along
/// the river's course (either tile flows into the other) or a trail bridges it.
pub fn step_cost(w: &World, from: usize, to: usize) -> Option<f64> {
    let t = &w.tiles[to];
    let along = w.tiles[from].flow == Some(to) || t.flow == Some(from);
    let cross = if t.river > 0 && !t.trail && !along { RIVER.cross } else { 0.0 };
    let terrain = TERRAIN[t.terrain].move_cost;
    let cost = match (t.trail, t.trail_cost) {
        (true, Some(hidden)) => terrain.map_or(hidden, |c| c.min(hidden)),
        _ => terrain?,
    };
    Some(cost + cross)
}

/// Puts `labour` into a trail on tile `i`, which must be `trailable`. It is finished when its work
/// reaches `TRAIL.work` for its terrain: then it has its trail, its work starts again from nothing
/// and its `rest_run` is 0, so `hold` doesn't wear it away on the turn it is built. Returns whether
/// it is finished.
pub fn build_trail(w: &mut World, i: usize, labour: f64) -> bool {
    let terrain = w.tiles[i].terrain;
    let Some(&(_, need)) = TRAIL.work.iter().find(|&&(t, _)| t == terrain) else { return false };
    if !trailable(w, i) {
        return false;
    }
    let t = &mut w.tiles[i];
    t.trail_work += labour;
    if t.trail_work < need {
        return false;
    }
    t.trail = true;
    t.trail_work = 0.0;
    t.rest_run = 0;
    true
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
/// `from`, ending at `to`, and its cost. See `path_by`.
pub fn path(w: &World, from: usize, to: usize, allowed: impl Fn(usize) -> bool) -> Option<(Vec<usize>, f64)> {
    path_by(w, from, to, |i, j| if allowed(j) { step_cost(w, i, j) } else { None })
}

/// The cheapest path from `from` to `to`, where a step from tile `i` into `j` costs `cost(i, j)`
/// (`None` is no step): the tiles after `from`, ending at `to`, and its cost. Dijkstra as `cost_map`
/// is, with the same tie-breaks (cheapest, then lowest tile); on an equal cost a tile keeps the
/// predecessor found first. `from == to` is `([], 0)`; `None` when `to` can't be reached.
pub fn path_by(
    w: &World,
    from: usize,
    to: usize,
    cost: impl Fn(usize, usize) -> Option<f64>,
) -> Option<(Vec<usize>, f64)> {
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
            let Some(step) = cost(i, j) else { continue };
            let nc = c + step;
            if best.get(&j).is_some_and(|&b| b <= nc) {
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
        // A stripped tile regrows nothing while it lies fallow (`units` counts the turns down).
        if t.fallow > 0 {
            continue;
        }
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

/// A tile's grain cap as a multiple of its wild grain after `tame` turns of taming: from 1 up to
/// `TENDING.domestic` at a hearth (`tame_turns`), then up by `TENDING.beyond` for each further
/// `tame_turns`.
pub fn domestic(tame: u32) -> f64 {
    let turns = TENDING.tame_turns as f64;
    let tame = tame as f64;
    if tame <= turns {
        1.0 + (TENDING.domestic - 1.0) * tame / turns
    } else {
        TENDING.domestic + TENDING.beyond * (tame - turns) / turns
    }
}

/// A fully tamed tile: its grain cap is `TENDING.domestic` times its wild grain.
pub fn is_hearth(t: &Tile) -> bool {
    t.wild_grain > 0.0 && t.tame >= TENDING.tame_turns
}

/// A depleted tile: some food's wear is at least `DEPLETION.depleted_at`.
pub fn is_depleted(t: &Tile) -> bool {
    t.wear.iter().any(|&v| v >= DEPLETION.depleted_at)
}

/// Sets each food's cap to its unworn cap less its wear; stock above a fallen cap is cut to it.
fn settle_caps(t: &mut Tile) {
    for f in 0..N {
        t.cap[f] = t.unworn[f] * (1.0 - t.wear[f]);
        t.stock[f] = t.stock[f].min(t.cap[f]);
    }
}

/// The `wear` step, after eating: each food on each tile takes its wear from this turn's take
/// against its sustainable yield at its cap, the yield `regen × cap` (game breeds, so its most,
/// `(regen / 4 + restock) × cap`). A take above the yield wears the food by `DEPLETION.rate` times
/// the excess as a share of the cap; a take at or under it, or none, rests it by
/// `DEPLETION.recover`. Wear stays in `0..=DEPLETION.max`, and the caps follow it.
pub fn wear(w: &mut World) {
    for t in &mut w.tiles {
        for (f, food) in FOOD.iter().enumerate() {
            let cap = t.cap[f];
            let take = t.take[f];
            let yielded = if food.breeds { (food.regen / 4.0 + food.restock) * cap } else { food.regen * cap };
            let wear = if cap > 0.0 && take > yielded {
                t.wear[f] + DEPLETION.rate * (take - yielded) / cap
            } else {
                t.wear[f] - DEPLETION.recover
            };
            t.wear[f] = wear.clamp(0.0, DEPLETION.max);
        }
        settle_caps(t);
    }
}

/// The `hold` step, after eating: each tile counts the turns in a row it was walked, and the turns
/// in a row it wasn't. Walked `TRAIL.after` turns in a row it gets a trail; unwalked `TRAIL.fade`
/// turns in a row the trail wears away. Holding a tile makes no trail. Each tile also counts the
/// turns it has been held, or held by nobody: grain held `TENDING.after` turns tames a step a turn
/// with no ceiling, unheld it goes wild a step a turn; its unworn grain cap follows `domestic`.
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
        if t.walked {
            t.walk_run += 1;
            t.rest_run = 0;
        } else {
            t.rest_run += 1;
            t.walk_run = 0;
        }
        // Worn trails never land on a mountain: nobody walks one bare, so only a built pass is on one.
        if t.walk_run >= TRAIL.after && t.terrain != MOUNTAIN {
            t.trail = true;
        } else if t.trail && t.rest_run >= TRAIL.fade {
            t.trail = false;
        }
        if t.wild_grain <= 0.0 {
            continue;
        }
        if held && t.held >= TENDING.after {
            let before = t.tame;
            t.tame += 1;
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
        t.unworn[GRAIN] = round1(t.wild_grain * domestic(t.tame));
        settle_caps(t);
    }
}
