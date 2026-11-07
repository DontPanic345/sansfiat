//! The unit's food: what a camp strips from a tile, what a scout grazes, and what the line brings.

use super::line::line;
use super::{pack_cap, Unit, EPS};
use crate::data::*;
use crate::land;
use crate::market::{add, take, total};
use crate::world::World;

/// Strips the unit's tile for up to `labour` of work, taking at most `room` of food into the
/// pack at `rate` a unit of labour. Returns the labour used and the food taken; nothing when the
/// tile is fallow or bare.
pub(super) fn strip(w: &mut World, id: u32, labour: f64, room: f64, rate: f64) -> (f64, f64) {
    let tile = w.units[&id].home;
    strip_at(w, id, tile, labour, room, rate)
}

/// As `strip`, of tile `tile`. It takes only the food it takes: each stock drops in proportion to
/// what was taken, and the tile rests `ceil(UNIT.fallow × taken / held)` turns.
pub(super) fn strip_at(w: &mut World, id: u32, tile: usize, labour: f64, room: f64, rate: f64) -> (f64, f64) {
    let t = &mut w.tiles[tile];
    let held = total(&t.stock);
    if t.fallow > 0 || held <= 0.0 || labour <= 0.0 || room <= 0.0 {
        return (0.0, 0.0);
    }
    let food = (labour * rate).min(held).min(room);
    if food <= EPS {
        return (0.0, 0.0);
    }
    let share = food / held;
    let got = t.stock.map(|s| s * share);
    for (s, g) in t.stock.iter_mut().zip(got) {
        *s = (*s - g).max(0.0);
    }
    t.fallow = (f64::from(UNIT.fallow) * share).ceil() as u32;
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (food / rate, food)
}

/// A scout's take of tile `tile`: food at `rate` a unit of labour, up to `room` and no more than the
/// tile's `spare`, each food in proportion to its share of the spare. What it takes counts in the
/// tile's `take` for the turn, and the tile never goes fallow. Returns the labour used and the food
/// taken.
pub(super) fn graze_at(w: &mut World, id: u32, tile: usize, labour: f64, room: f64, rate: f64) -> (f64, f64) {
    let t = &mut w.tiles[tile];
    let spare = land::spare(t);
    let held = total(&spare);
    if held <= EPS || labour <= 0.0 || room <= 0.0 {
        return (0.0, 0.0);
    }
    let food = (labour * rate).min(held).min(room);
    if food <= EPS {
        return (0.0, 0.0);
    }
    let share = food / held;
    let got = spare.map(|s| s * share);
    for ((s, k), g) in t.stock.iter_mut().zip(t.take.iter_mut()).zip(got) {
        *s = (*s - g).max(0.0);
        *k += g;
    }
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (food / rate, food)
}

/// Puts up to `labour` on the line for up to `want` of food from the store into the pack. Returns
/// the labour used and the food brought.
pub(super) fn supply(w: &mut World, id: u32, labour: f64, want: f64) -> (f64, f64) {
    let civ = w.units[&id].civ;
    let Some((_, cost, _)) = line(w, id) else { return (0.0, 0.0) };
    let held = total(&w.civs[&civ].store);
    let mut food = want.min(held);
    let mut used = 0.0;
    if cost > 0.0 {
        food = food.min(labour * UNIT.carry / cost);
        used = food * cost / UNIT.carry;
    }
    if food <= EPS {
        return (0.0, 0.0);
    }
    let got = take(&mut w.civs.get_mut(&civ).unwrap().store, food);
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (used, food)
}

/// The room left in the camp's pack.
pub(super) fn room(u: &Unit) -> f64 {
    (pack_cap(u.people) - total(&u.store)).max(0.0)
}
