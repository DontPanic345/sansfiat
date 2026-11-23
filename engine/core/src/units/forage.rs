//! An army's food: what a camp strips from a tile and what the line brings.
//! And the labour a unit has left in its turn, `Ladder`, spent rung by rung. The ladder lives here,
//! not in `actors`, because its rungs are these functions: an actor's labour spent on food is a unit's.

use super::line::line;
use super::{cap_of, Unit, EPS};
use crate::actors::{Actor, Carrier};
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

/// Puts up to `labour` on the line for up to `want` of food into the pack, from the end the line
/// ends at and no other: a store of its civ, or its stall at a market (`exchange::Stall`). Returns
/// the labour used and the food brought.
pub(super) fn supply(w: &mut World, id: u32, labour: f64, want: f64) -> (f64, f64) {
    let (civ, home) = (w.units[&id].civ, w.units[&id].home);
    let Some((path, cost, _)) = line(w, id) else { return (0.0, 0.0) };
    let end = path.last().copied().unwrap_or(home);
    let stall = settle::store(w, civ, end).is_none();
    let held = if stall { stall_food(w, id, end) } else { settle::store(w, civ, end).map_or(0.0, total) };
    let mut food = want.min(held);
    let mut used = 0.0;
    if cost > 0.0 {
        let load = Actor::carry(Carrier::Warrior).load;
        food = food.min(labour * load / cost);
        used = food * cost / load;
    }
    if food <= EPS {
        return (0.0, 0.0);
    }
    let got = if stall {
        w.units.get_mut(&id).unwrap().stall.as_mut().map(|s| take(&mut s.food, food)).unwrap_or_default()
    } else {
        settle::store_mut(w, civ, end).map(|s| take(s, food)).unwrap_or_default()
    };
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (used, food)
}

/// The food in unit `id`'s stall, when its stall is at the market on `tile`.
fn stall_food(w: &World, id: u32, tile: usize) -> f64 {
    let u = &w.units[&id];
    u.stall
        .as_ref()
        .filter(|s| w.markets.get(&s.market).is_some_and(|m| m.tile == tile))
        .map_or(0.0, |s| total(&s.food))
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
