//! Moving over the land, food growing back on it, what taking it wears down, and what walking and
//! holding it do: trails and tame grain.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

use crate::data::*;
use crate::events::{Event, EventKind};
use crate::map::{self, Tile};
use crate::market::Basket;
use crate::world::World;

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

/// The steps between tiles `a` and `b` as the crow flies: the hex distance.
pub fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// Whether a trail can be built on tile `i`: it is rough ground or a mountain (`TRAIL.work`) with a
/// hidden trail cost, and a mountain's span is one `CIV.bridge_span` can bridge. A flat tile's
/// trail is never built by labour (it comes of traffic alone).
pub fn trailable(w: &World, i: usize) -> bool {
    let t = &w.tiles[i];
    t.trail_cost.is_some()
        && TRAIL.work.iter().any(|&(k, _)| k == t.terrain)
        && (t.terrain != MOUNTAIN || t.span.is_some_and(|s| s <= CIV.bridge_span))
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

/// The hidden trail cost of a flat tile of `terrain`: `TRAIL.flat` of its bare cost on plains and
/// coast, and `None` on any other terrain (rough ground and mountains have their own).
pub fn flat_cost(terrain: usize) -> Option<f64> {
    match terrain {
        PLAINS | COAST => TERRAIN[terrain].move_cost.map(|c| c * TRAIL.flat),
        _ => None,
    }
}

/// Puts `labour` into a trail on tile `i`, which must be `trailable`. It is finished when its work
/// reaches `TRAIL.work` for its terrain: then it has its trail, its work starts again from nothing
/// and its traffic is `TRAIL.on`, so `tend` doesn't wear it away on the turn it is built. Returns
/// whether it is finished.
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
    t.traffic = t.traffic.max(TRAIL.on);
    true
}

/// A cost on the grid of a twelfth of a day, in whole twelfths: every step's cost lands on it
/// exactly, so costs that tie by the rules tie in the searches though a third isn't exact in binary.
pub fn grid(c: f64) -> i64 {
    (c * 12.0 / DAY).round() as i64
}

struct Entry(i64, usize);

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
        o.0.cmp(&self.0).then(o.1.cmp(&self.1))
    }
}

/// Path cost from the nearest of `from` to every tile within `max`: sources cost 0, each step
/// costs its `step_cost`, and only `allowed` tiles are entered.
pub fn cost_map(w: &World, from: &[usize], max: f64, allowed: impl Fn(usize) -> bool) -> BTreeMap<usize, f64> {
    cost_map_by(w, from, max, allowed, |i, j| step_cost(w, i, j))
}

/// `cost_map` with each step priced by `cost(i, j)` (`None` is no step), as `path_by` prices a path.
pub fn cost_map_by(
    w: &World,
    from: &[usize],
    max: f64,
    allowed: impl Fn(usize) -> bool,
    cost: impl Fn(usize, usize) -> Option<f64>,
) -> BTreeMap<usize, f64> {
    let limit = grid(max);
    let mut best: BTreeMap<usize, (i64, f64)> = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    for &s in from {
        best.insert(s, (0, 0.0));
        heap.push(Entry(0, s));
    }
    while let Some(Entry(g, i)) = heap.pop() {
        let (bg, c) = best[&i];
        if g > bg {
            continue;
        }
        for j in neighbours(w, i) {
            let Some(step) = cost(i, j) else { continue };
            let nc = c + step;
            let ng = grid(nc);
            if ng > limit || !allowed(j) || best.get(&j).is_some_and(|&(b, _)| b <= ng) {
                continue;
            }
            best.insert(j, (ng, nc));
            heap.push(Entry(ng, j));
        }
    }
    best.into_iter().map(|(i, (_, c))| (i, c)).collect()
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
    let mut best: BTreeMap<usize, (i64, f64)> = BTreeMap::new();
    let mut before: BTreeMap<usize, usize> = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    best.insert(from, (0, 0.0));
    heap.push(Entry(0, from));
    while let Some(Entry(g, i)) = heap.pop() {
        let (bg, c) = best[&i];
        if g > bg {
            continue;
        }
        if i == to {
            break;
        }
        for j in neighbours(w, i) {
            let Some(step) = cost(i, j) else { continue };
            let nc = c + step;
            let ng = grid(nc);
            if best.get(&j).is_some_and(|&(b, _)| b <= ng) {
                continue;
            }
            best.insert(j, (ng, nc));
            before.insert(j, i);
            heap.push(Entry(ng, j));
        }
    }
    let cost = best.get(&to)?.1;
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

/// The yield a tile's food `f` sustains a turn: what it regrows, whatever its stock. A food that
/// breeds regrows a quarter of its rate and restocks.
pub fn yield_of(t: &Tile, f: usize) -> f64 {
    let food = &FOOD[f];
    let cap = t.cap[f];
    if food.breeds {
        (food.regen / 4.0 + food.restock) * cap
    } else {
        food.regen * cap
    }
}

/// What a scout may take from a tile this turn: per food, the stock up to what the yield leaves after
/// this turn's take. Nothing on a fallow tile.
pub fn spare(t: &Tile) -> Basket {
    let mut b = [0.0; N];
    if t.fallow > 0 {
        return b;
    }
    for (f, s) in b.iter_mut().enumerate() {
        *s = t.stock[f].min((yield_of(t, f) - t.take[f]).max(0.0));
    }
    b
}

/// The `wear` step, after eating: each food on each tile takes its wear from this turn's take
/// against its sustainable yield at its cap, the yield `regen × cap` (game breeds, so its most,
/// `(regen / 4 + restock) × cap`). A take above the yield wears the food by `DEPLETION.rate` times
/// the excess as a share of the cap; a take at or under it, or none, rests it by
/// `DEPLETION.recover`. Wear stays in `0..=DEPLETION.max`, and the caps follow it.
pub fn wear(w: &mut World) {
    for t in &mut w.tiles {
        for f in 0..FOOD.len() {
            let cap = t.cap[f];
            let take = t.take[f];
            let yielded = yield_of(t, f);
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

/// The `tend` step, after eating: a tile whose traffic has reached `TRAIL.on` gets a trail, and a
/// trail whose traffic has fallen under `TRAIL.off` wears away, unless a camp's line or a scout's
/// route was walked on it this turn. Each tile also counts the
/// turns it has been held, or held by nobody: grain held `TENDING.after` turns tames a step a turn
/// with no ceiling, unheld it goes wild a step a turn; its unworn grain cap follows `domestic`.
pub fn tend(w: &mut World) {
    let round1 = |v: f64| (v * 10.0).round() / 10.0;
    let mut tamed = Vec::new();
    for (i, t) in w.tiles.iter_mut().enumerate() {
        let held = t.claim.is_some();
        if held {
            t.claim_run += 1;
            t.free_run = 0;
        } else {
            t.claim_run = 0;
            t.free_run += 1;
        }
        // Worn trails never land on a mountain: nobody walks one bare, so only a built pass is on one.
        if t.traffic >= TRAIL.on && t.terrain != MOUNTAIN {
            t.trail = true;
        } else if t.trail && t.traffic < TRAIL.off && !t.used {
            t.trail = false;
        }
        if t.wild_grain <= 0.0 {
            continue;
        }
        if held && t.claim_run >= TENDING.after {
            let before = t.tame;
            t.tame += 1;
            if before < TENDING.tame_turns && t.tame == TENDING.tame_turns {
                tamed.push(i);
            }
        } else if !held {
            t.tame = t.tame.saturating_sub(1);
        }
        t.unworn[GRAIN] = round1(t.wild_grain * domestic(t.tame));
        settle_caps(t);
    }
    for i in tamed {
        w.emit(Event::new(EventKind::HearthTamed, i));
    }
}
