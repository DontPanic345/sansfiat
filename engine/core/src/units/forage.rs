//! An army's food: what a camp strips from a tile and what the line brings.
//! And the labour a unit has left in its turn, `Ladder`, spent rung by rung. The ladder lives here,
//! not in `actors`, because its rungs are these functions: an actor's labour spent on food is a unit's.

use super::line::line;
use super::{cap_of, Unit, EPS};
use crate::data::*;
use crate::market::{add, take, total};
use crate::settle;
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

/// Puts up to `labour` on the line for up to `want` of food into the pack, from the store the line
/// ends at and no other. Returns
/// the labour used and the food brought.
pub(super) fn supply(w: &mut World, id: u32, labour: f64, want: f64) -> (f64, f64) {
    let (civ, home) = (w.units[&id].civ, w.units[&id].home);
    let Some((path, cost, _)) = line(w, id) else { return (0.0, 0.0) };
    let store = path.last().copied().unwrap_or(home);
    let held = settle::store(w, civ, store).map_or(0.0, total);
    let mut food = want.min(held);
    let mut used = 0.0;
    if cost > 0.0 {
        food = food.min(labour * UNIT.carry / cost);
        used = food * cost / UNIT.carry;
    }
    if food <= EPS {
        return (0.0, 0.0);
    }
    let got = settle::store_mut(w, civ, store).map(|s| take(s, food)).unwrap_or_default();
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (used, food)
}

/// The labour a unit has left this turn, spent down rung by rung: each spend takes its labour off
/// `left`, in the order the turn makes them. A rung's food is the function's (`strip` on a tile,
/// `supply` along the line), so each keeps its own take rule.
pub(super) struct Ladder {
    pub left: f64,
}

impl Ladder {
    pub fn new(left: f64) -> Ladder {
        Ladder { left }
    }

    /// Takes `used` off the labour left: a march, a search, or any labour spent without food.
    pub fn spend(&mut self, used: f64) {
        self.left -= used;
    }

    /// Strips the unit's own tile with all the labour left (`strip`); returns the food taken.
    pub fn strip(&mut self, w: &mut World, id: u32, room: f64, rate: f64) -> f64 {
        let (used, food) = strip(w, id, self.left, room, rate);
        self.left -= used;
        food
    }

    /// Brings up to `want` of food along the line with all the labour left (`supply`); returns the
    /// food brought.
    pub fn supply(&mut self, w: &mut World, id: u32, want: f64) -> f64 {
        let (used, food) = supply(w, id, self.left, want);
        self.left -= used;
        food
    }
}

/// The room left in the unit's pack.
pub(super) fn room(u: &Unit) -> f64 {
    (cap_of(u) - total(&u.store)).max(0.0)
}
