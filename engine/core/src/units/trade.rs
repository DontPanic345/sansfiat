//! The planner's trading (`exchange.rs`): an army takes salt from a store of its civ before it marches past its
//! ring, and an army with salt whose line can't feed it goes to a market that isn't a rival's to buy food with
//! it, and holds there while it buys. Only with `Config.exchange`; the planner's other orders are unchanged.

use super::ai::{Decision, Give};
use super::line::{line_by, ring, turns};
use super::paths::{path_cached, Paths};
use super::EPS;
use crate::data::*;
use crate::exchange;
use crate::market::total;
use crate::orders::Order;
use crate::world::World;

/// Whether army `id` of `civ` should fill its pack with salt before it marches to `to`: the fill is allowed
/// (`exchange::check_fill`) and `to` lies past the civ's ring, where its line cannot feed it.
pub fn fills_before(w: &World, civ: u32, id: u32, to: usize) -> bool {
    let Some(u) = w.units.get(&id) else { return false };
    if exchange::check_fill(w, civ, u).is_err() {
        return false;
    }
    ring(w, civ).get(&to).is_none_or(|&share| share >= 1.0)
}

/// Plans army `id`'s trade for this turn, if it has one to make: `true` when it gave its order (a march to a
/// market, or a halt there while it buys), `false` when the planner should plan the army as it otherwise
/// would. An army trades when it has salt, its line can't feed it this turn, and its pack holds under
/// `PLAN.trade_at` turns of its food. It goes to the nearest market that isn't a rival's within reach of its
/// pack's turns (at least one), and stays on that market while its pack has room for food.
pub(super) fn plan(w: &mut World, civ: u32, id: u32, paths: &mut Paths, give: Give) -> bool {
    if !w.config.exchange {
        return false;
    }
    let Some(u) = w.units.get(&id) else { return false };
    if u.camp().is_none() || u.store[SALT] <= EPS {
        return false;
    }
    let (here, dest, people) = (u.home, u.dest(), u.people);
    let eats = people * UNIT.food;
    let food = total(&u.store);
    if food + EPS >= PLAN.trade_at * eats {
        return false;
    }
    let fed = line_by(w, id, false).is_some_and(|(_, _, carry)| carry + EPS >= eats);
    if fed {
        return false;
    }
    // On a market that isn't a rival's, it holds while its pack buys food there: it has not traded yet, or
    // its last trade there bought some.
    let here_market = w.markets.values().find(|m| m.tile == here).map(|m| m.id);
    let buying = here_market.is_some_and(|m| {
        !exchange::rival(w, civ, m)
            && u.traded.as_ref().is_none_or(|t| (t.market == m && t.food > EPS) || t.market != m)
    });
    if buying {
        // A march it was on stops here.
        if dest.is_some() {
            w.note(id, "buys food at the festival here with its salt".to_string());
            let _ = give(w, civ, Decision::Order(Order::Move { unit: id, to: here }));
        }
        return true;
    }
    let reach = (food / eats.max(EPS)).max(1.0);
    let mut best: Option<(usize, u32, usize)> = None;
    for m in w.markets.values() {
        if m.tile == here || exchange::rival(w, civ, m.id) {
            continue;
        }
        let Some(p) = path_cached(w, paths, here, m.tile) else { continue };
        if f64::from(turns(w, &p, here, 0.0)) > reach {
            continue;
        }
        if best.is_none_or(|b| (p.len(), m.id) < (b.0, b.1)) {
            best = Some((p.len(), m.id, m.tile));
        }
    }
    let Some((_, _, tile)) = best else { return false };
    if dest != Some(tile) {
        w.note(id, format!("marches to the festival at {} to buy food with its salt", w.at(tile)));
        let _ = give(w, civ, Decision::Order(Order::Move { unit: id, to: tile }));
    }
    true
}
