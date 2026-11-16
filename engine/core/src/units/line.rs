//! The army's paths: what a camp may walk, the cheapest path, and the line to a store of its civ
//! (the capital or a settlement), which is what the camp's food is carried along and whether rival
//! ground cuts it. `walk` and `turns` are
//! the walking the camp's labour pays for.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use super::{UnitKind, EPS};
use crate::actors::{stride, walk_cost};
use crate::civs;
use crate::claim;
use crate::data::*;
use crate::land::{grid, neighbours, passable, path, step_cost};
use crate::market::total;
use crate::settle;
use crate::world::World;

/// Whether a camp may walk onto tile `i`: passable land, or a built pass over a mountain.
pub(super) fn walkable(w: &World, i: usize) -> bool {
    passable(w, i) && (TERRAIN[w.tiles[i].terrain].land || w.tiles[i].trail)
}

/// The cheapest route from `from` to `to` over land and built passes, as `land::path`: trails and
/// river crossings count as `step_cost` has them.
pub fn path_between(w: &World, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
    path(w, from, to, |i| walkable(w, i))
}

/// A unit's march from where it stands to its `dest` (a camp's, or a scout's with no route), with
/// its cost, for the page.
pub fn route(w: &World, unit: u32) -> Option<(Vec<usize>, f64)> {
    let u = w.units.get(&unit)?;
    path_between(w, u.home, u.dest().filter(|&d| d != u.home)?)
}

/// The camp's line: the cheapest path to a store of its civ (`line_at` says which) over walkable tiles that don't count for a
/// rival civ (`civs::holder`: a rival's held tile blocks it, a rival's land unless this civ holds
/// it; the line's ends never block it), its cost (`walk_cost` of the path cost, made there
/// and back each turn) and the food it carries a turn with all the camp's labour (`people ×
/// UNIT.labour`, at `UNIT.carry` a unit of labour over the cost). At the store the line is empty,
/// costs nothing and carries without limit. `None` when no such path exists.
pub fn line(w: &World, unit: u32) -> Option<(Vec<usize>, f64, f64)> {
    line_by(w, unit, true)
}

/// The line as `line` would be from `tile`: its cost and the food it carries a turn, as if the unit
/// stood there. `None` when no line reaches a store from `tile`.
pub fn line_from(w: &World, unit: u32, tile: usize) -> Option<(f64, f64)> {
    line_at(w, unit, tile, true).map(|(_, cost, carry)| (cost, carry))
}

/// Whether a unit standing on `tile` has no line, or one that carries less than what its people eat
/// in a turn: the line's reach from there.
pub fn past_reach(w: &World, unit: u32, tile: usize) -> bool {
    let eats = w.units.get(&unit).map_or(0.0, |u| u.people * UNIT.food);
    line_from(w, unit, tile).is_none_or(|(_, carry)| carry < eats)
}

/// The line as `line`, reading held ground (`hold`) or only land.
pub(super) fn line_by(w: &World, unit: u32, hold: bool) -> Option<(Vec<usize>, f64, f64)> {
    line_at(w, unit, w.units.get(&unit)?.home, hold)
}

/// The line from `from` (`line_by`'s body, for any tile the unit could stand on): to the store of
/// its civ, capital or settlement, that the cheapest path reaches, among those holding a turn of the
/// unit's food; with none so stocked, among them all. Ties go to the capital, then the older
/// settlement.
fn line_at(w: &World, unit: u32, from: usize, hold: bool) -> Option<(Vec<usize>, f64, f64)> {
    let u = w.units.get(&unit)?;
    w.civs.get(&u.civ).filter(|c| !c.is_out())?;
    let civ_of = |i| if hold { civs::holder(w, i) } else { civs::tile_civ(w, i) };
    let eats = u.people * UNIT.food;
    let mut best: Option<(bool, f64, Vec<usize>)> = None;
    for store in settle::roots(w, u.civ) {
        let stocked = settle::store(w, u.civ, store).is_some_and(|s| total(s) + EPS >= eats);
        if best.as_ref().is_some_and(|b| b.0 && !stocked) {
            continue;
        }
        let Some((tiles, cost)) =
            path(w, from, store, |i| walkable(w, i) && (i == store || civ_of(i).is_none_or(|c| c == u.civ)))
        else {
            continue;
        };
        if best.as_ref().is_none_or(|b| (stocked && !b.0) || (stocked == b.0 && cost < b.1)) {
            best = Some((stocked, cost, tiles));
        }
    }
    let (_, cost, tiles) = best?;
    let cost = walk_cost(cost, 1.0);
    let carry = if cost > 0.0 { u.people * UNIT.labour * UNIT.carry / cost } else { f64::INFINITY };
    Some((tiles, cost, carry))
}

/// The store unit `unit`'s line ends at, by its tile: where it stands when the line is empty.
pub fn store_of(w: &World, unit: u32) -> Option<usize> {
    let (tiles, _, _) = line(w, unit)?;
    tiles.last().copied().or_else(|| w.units.get(&unit).map(|u| u.home))
}

/// The ring of civ `civ`: every walkable tile (land, or a built pass) from which a line may reach one
/// of its stores (the tiles `line_at` accepts, reading held ground), with the share of an army's days
/// its line takes to feed it there: `UNIT.food × walk_cost(path cost, 1) / (UNIT.labour × UNIT.carry)`,
/// the cost of the cheapest line to any store, priced as `line_at` prices it. A store is 0. Inside the
/// ring the share is under one; it has no army size in it, so one map serves every army and scout of
/// the civ. A fallen civ has none.
pub fn ring(w: &World, civ: u32) -> BTreeMap<usize, f64> {
    ring_from(w, civ, &settle::roots(w, civ))
}

/// The ring of `civ`'s capital alone: what `ring` is for a civ with no settlement.
pub fn ring_capital(w: &World, civ: u32) -> BTreeMap<usize, f64> {
    ring_from(w, civ, &w.civs.get(&civ).map(|c| c.capital).into_iter().collect::<Vec<_>>())
}

/// `ring`, from the stores on `stores`.
fn ring_from(w: &World, civ: u32, stores: &[usize]) -> BTreeMap<usize, f64> {
    let mut out = BTreeMap::new();
    if w.civs.get(&civ).is_none_or(|c| c.is_out()) {
        return out;
    }
    // Who holds each tile, as `civs::holder` reads it: the first army holding it, else the land's civ.
    let mut holder: Vec<Option<u32>> = vec![None; w.tiles.len()];
    for u in w.units.values() {
        for &t in u.held() {
            holder[t].get_or_insert(u.civ);
        }
    }
    // A tile a line may pass through: walkable and not a rival's. A store is always its own end.
    let open: Vec<bool> = (0..w.tiles.len())
        .map(|i| {
            let held = holder[i].or_else(|| civs::tile_civ(w, i));
            walkable(w, i) && (stores.contains(&i) || held.is_none_or(|h| h == civ))
        })
        .collect();
    // Dijkstra from every store over the reversed steps: a tile's cost is the path cost of its line
    // to the nearest, so the step from `i` into `j` (`j` nearer a store) is priced when `j` is settled.
    let mut best: Vec<Option<(i64, f64)>> = vec![None; w.tiles.len()];
    let mut heap = BinaryHeap::new();
    for &s in stores {
        best[s] = Some((0, 0.0));
        if open[s] {
            heap.push(Reverse((0, s)));
        }
    }
    while let Some(Reverse((g, j))) = heap.pop() {
        let Some((bg, cj)) = best[j] else { continue };
        if g > bg {
            continue;
        }
        for i in neighbours(w, j) {
            let Some(step) = step_cost(w, i, j) else { continue };
            let nc = cj + step;
            let ng = grid(nc);
            if best[i].is_some_and(|(b, _)| b <= ng) {
                continue;
            }
            best[i] = Some((ng, nc));
            // A tile's line starts there whatever it is; only an open tile carries a line on.
            if open[i] {
                heap.push(Reverse((ng, i)));
            }
        }
    }
    for (i, b) in best.iter().enumerate() {
        if let Some((_, cost)) = b {
            if walkable(w, i) {
                let cost = walk_cost(*cost, 1.0);
                out.insert(i, UNIT.food * cost / (UNIT.labour * UNIT.carry));
            }
        }
    }
    out
}

/// Whether the camp is cut off: no line, or one that can't carry a turn's food for its warriors
/// even with all their labour.
pub fn cut_off(w: &World, unit: u32) -> bool {
    cut_by(w, unit, true)
}

pub(super) fn cut_by(w: &World, unit: u32, hold: bool) -> bool {
    match (line_by(w, unit, hold), w.units.get(&unit)) {
        (Some((_, _, carry)), Some(u)) => carry + EPS < u.people * UNIT.food,
        _ => true,
    }
}

/// How many turns it takes to walk `path` from `from` with `banked` already banked, as the
/// `units` step does it; 0 for an empty path.
pub fn turns(w: &World, path: &[usize], from: usize, banked: f64) -> u32 {
    let (mut rest, mut at, mut banked, mut n) = (path, from, banked, 0);
    while !rest.is_empty() {
        // Every warrior walks, so the camp moves at one person's pace whatever its size.
        let (taken, left) = stride(w, at, rest, banked + UNIT.labour, 0.0);
        if taken > 0 {
            at = rest[taken - 1];
        }
        rest = &rest[taken..];
        banked = left;
        n += 1;
    }
    n
}
/// Where a rival's held ground stops camp `unit`'s line, for the page: the line it would have
/// without held ground, the first tile of it a rival holds and that rival. `None` when the line
/// isn't cut by an army.
pub fn cut_at(w: &World, unit: u32) -> Option<(Vec<usize>, usize, u32)> {
    let u = w.units.get(&unit)?;
    if !cut_off(w, unit) || cut_by(w, unit, false) {
        return None;
    }
    let (p, _, _) = line_by(w, unit, false)?;
    let store = p.last().copied();
    let rival_at = |i: usize| w.units.values().find(|o| o.civ != u.civ && o.held().contains(&i)).map(|o| o.civ);
    let (at, by) = p.iter().filter(|&&i| Some(i) != store).find_map(|&i| rival_at(i).map(|c| (i, c)))?;
    Some((p, at, by))
}

/// The rival whose land or held ground cuts army `unit`'s line, when its line doesn't carry a turn's
/// food: the first rival on its tile or the open path to its nearest store (`tile_civ` or a rival army's held tile),
/// else the first rival whose land borders its tile. `None` when the line is open, or when no rival's
/// land is on the way or beside it (cut by length alone), so it starves as it does on main.
pub fn cut_by_rival(w: &World, unit: u32) -> Option<u32> {
    let u = w.units.get(&unit)?;
    w.civs.get(&u.civ).filter(|c| !c.is_out())?;
    let (open, _) = settle::roots(w, u.civ)
        .into_iter()
        .filter_map(|s| path(w, u.home, s, |i| walkable(w, i)))
        .min_by(|a, b| a.1.total_cmp(&b.1))?;
    if !cut_off(w, unit) {
        return None;
    }
    std::iter::once(u.home)
        .chain(open.iter().copied())
        .find_map(|i| {
            let army = w.units.values().find(|o| o.civ != u.civ && o.held().contains(&i)).map(|o| o.civ);
            army.or_else(|| claim::tile_civ(w, i).filter(|&c| c != u.civ))
        })
        .or_else(|| {
            // Its tile and the walk home are no rival's land, but rival land round its tile cuts it off.
            crate::land::neighbours(w, u.home).find_map(|i| claim::tile_civ(w, i).filter(|&c| c != u.civ))
        })
}

/// The civ whose held ground cuts camp `unit`'s line: the first rival holding a tile of the line
/// it would have without held ground (the lowest rival civ if none is on it, as when the ground
/// blocks a way round rather than the way itself).
pub(super) fn cutter(w: &World, unit: u32) -> u32 {
    let civ = w.units[&unit].civ;
    cut_at(w, unit)
        .map(|(_, _, by)| by)
        .or_else(|| w.units.values().filter(|u| u.civ != civ && u.held().len() > 1).map(|u| u.civ).min())
        .or_else(|| w.units.values().filter(|u| u.kind() == UnitKind::Camp).map(|u| u.civ).filter(|&c| c != civ).min())
        .unwrap_or(civ)
}
