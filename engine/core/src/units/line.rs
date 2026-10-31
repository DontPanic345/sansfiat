//! The army's paths: what a camp may walk, the cheapest path, and the line to the capital, which is
//! what the camp's food is carried along and whether rival ground cuts it. `walk` and `turns` are
//! the walking the camp's labour pays for.

use super::{UnitKind, EPS};
use crate::actors::walk_cost;
use crate::civs;
use crate::data::*;
use crate::land::{passable, path, step_cost};
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

/// A camp's route from where it stands to its `dest`, with its cost, for the page.
pub fn route(w: &World, unit: u32) -> Option<(Vec<usize>, f64)> {
    let u = w.units.get(&unit)?;
    path_between(w, u.home, u.camp()?.dest?)
}

/// The camp's line: the cheapest path to the capital over walkable tiles that don't count for a
/// rival civ (`civs::holder`: a rival's held tile blocks it, a rival's land unless this civ holds
/// it; the line's ends never block it), its cost (`walk_cost` of the path cost, made there
/// and back each turn) and the food it carries a turn with all the camp's labour (`people ×
/// UNIT.labour`, at `UNIT.carry` a unit of labour over the cost). At the capital the line is empty,
/// costs nothing and carries without limit. `None` when no such path exists.
pub fn line(w: &World, unit: u32) -> Option<(Vec<usize>, f64, f64)> {
    line_by(w, unit, true)
}

/// The line as `line`, reading held ground (`hold`) or only land.
pub(super) fn line_by(w: &World, unit: u32, hold: bool) -> Option<(Vec<usize>, f64, f64)> {
    let u = w.units.get(&unit)?;
    let capital = w.civs.get(&u.civ)?.capital;
    let civ_of = |i| if hold { civs::holder(w, i) } else { civs::tile_civ(w, i) };
    let (tiles, cost) =
        path(w, u.home, capital, |i| walkable(w, i) && (i == capital || civ_of(i).is_none_or(|c| c == u.civ)))?;
    let cost = walk_cost(cost, 1.0);
    let carry = if cost > 0.0 { u.people * UNIT.labour * UNIT.carry / cost } else { f64::INFINITY };
    Some((tiles, cost, carry))
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

/// One turn's walking along `path` from `from`: spends `banked + UNIT.labour` (one warrior's) on
/// steps while the next one costs no more than what is left. Returns the steps taken and the
/// labour left. Every warrior walks, so the camp moves at one person's pace whatever its size.
pub(super) fn walk(w: &World, from: usize, path: &[usize], banked: f64) -> (usize, f64) {
    let mut left = banked + UNIT.labour;
    let mut at = from;
    let mut taken = 0;
    for &next in path {
        let Some(cost) = step_cost(w, at, next) else { break };
        if cost > left {
            break;
        }
        left -= cost;
        at = next;
        taken += 1;
    }
    (taken, left)
}

/// How many turns it takes to walk `path` from `from` with `banked` already banked, as the
/// `units` step does it; 0 for an empty path.
pub fn turns(w: &World, path: &[usize], from: usize, banked: f64) -> u32 {
    let (mut rest, mut at, mut banked, mut n) = (path, from, banked, 0);
    while !rest.is_empty() {
        let (taken, left) = walk(w, at, rest, banked);
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
    let capital = w.civs.get(&u.civ)?.capital;
    if !cut_off(w, unit) || cut_by(w, unit, false) {
        return None;
    }
    let (p, _, _) = line_by(w, unit, false)?;
    let rival_at = |i: usize| w.units.values().find(|o| o.civ != u.civ && o.held().contains(&i)).map(|o| o.civ);
    let (at, by) = p.iter().filter(|&&i| i != capital).find_map(|&i| rival_at(i).map(|c| (i, c)))?;
    Some((p, at, by))
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
