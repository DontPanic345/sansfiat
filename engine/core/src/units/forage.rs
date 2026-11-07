//! The unit's food: what a camp strips from a tile, what a scout grazes, and what the line brings.
//! And the labour a unit has left in its turn, `Ladder`, spent rung by rung. The ladder lives here,
//! not in `actors`, because its rungs are these functions: an actor's labour spent on food is a unit's.

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

/// The labour a unit has left this turn, spent down rung by rung: each spend takes its labour off
/// `left`, in the order the turn makes them. A rung's food is the function's (`strip` for a camp,
/// `graze_at` for a scout, `supply` along the line), so each keeps its own take rule.
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

    /// Grazes tile `tile` with all the labour left (`graze_at`); returns the food taken.
    pub fn graze(&mut self, w: &mut World, id: u32, tile: usize, room: f64, rate: f64) -> f64 {
        let (used, food) = graze_at(w, id, tile, self.left, room, rate);
        self.left -= used;
        food
    }

    /// Grazes tile `tile` for `food` at `rate`, reached by a walk of `walk` paid with it: the walk
    /// and the labour the grazing used come off `left` together, and nothing if the tile gave none.
    /// Returns the food taken.
    pub fn graze_walked(&mut self, w: &mut World, id: u32, tile: usize, walk: f64, food: f64, rate: f64) -> f64 {
        let (used, taken) = graze_at(w, id, tile, food / rate, food, rate);
        if taken > 0.0 {
            self.left -= walk + used;
        }
        taken
    }

    /// Brings up to `want` of food along the line with all the labour left (`supply`); returns the
    /// food brought.
    pub fn supply(&mut self, w: &mut World, id: u32, want: f64) -> f64 {
        let (used, food) = supply(w, id, self.left, want);
        self.left -= used;
        food
    }
}

/// The room left in the camp's pack.
pub(super) fn room(u: &Unit) -> f64 {
    (pack_cap(u.people) - total(&u.store)).max(0.0)
}
