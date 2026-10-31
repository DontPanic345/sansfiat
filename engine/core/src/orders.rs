//! The orders: what a civ's units are told to do. An order names its civ and, for everything but
//! `Raise`, the unit it is for. `World::order` checks an order and applies it without logging it, so
//! the AI's orders go through it and a replay re-derives them; `act` checks the player's, applies
//! them and logs them. The checks run in one order, and each refusal says why: the phase, then (for
//! `act` only) the civ the player leads, then the unit, then the order's own rule.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::passable;
use crate::market::total;
use crate::units::{self, Unit, UnitKind};
use crate::world::{Phase, World};

/// An order to a civ's units. On the wire it is the `order` of `Action::Order`, with `type` in
/// camelCase; each variant's example is that wire form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Order {
    /// Raise warriors for the civ's camp (making it at the capital if there is none): one person
    /// from each of up to `n` of its clans. `{"type":"raise","n":2}`
    Raise { n: u32 },
    /// Disband `n` of the camp's warriors, or all with `None`, to the civ's own clan.
    /// `{"type":"disband","unit":12,"n":1}`
    Disband { unit: u32, n: Option<u32> },
    /// Send the camp to a tile; it moves when the turn ends. `{"type":"move","unit":12,"to":5}`
    Move { unit: u32, to: usize },
    /// Spread the camp over a zone: the painted tiles (the camp's own may be left out), which replace
    /// the zone; none undeploys. `{"type":"zone","unit":12,"tiles":[5,6]}`
    Zone { unit: u32, tiles: Vec<usize> },
    /// Set the camp's auto-forage and auto-supply.
    /// `{"type":"switches","unit":12,"forage":true,"supply":false}`
    Switches { unit: u32, forage: bool, supply: bool },
    /// This turn's end: put the labour left on the line. `{"type":"resupply","unit":12}`
    Resupply { unit: u32 },
    /// This turn's end: forage the tile on the labour left. `{"type":"forage","unit":12}`
    Forage { unit: u32 },
    /// This turn's end: pillage the rival's tile on the labour left. `{"type":"pillage","unit":12}`
    Pillage { unit: u32 },
    /// Set the scout's route: search from `from` for `to`, building trails as it goes.
    /// `{"type":"route","unit":13,"from":58,"to":4}`
    Route { unit: u32, from: usize, to: usize },
    /// Stop the scout's search. `{"type":"clearRoute","unit":13}`
    ClearRoute { unit: u32 },
}

impl World {
    /// Checks `order` for `civ`: `Err` says why it can't happen. Whether `civ` is the one the player
    /// leads is `validate`'s check, not this one: the AI gives orders to every civ it doesn't lead.
    pub fn check_order(&self, civ: u32, order: &Order) -> Result<(), String> {
        if self.phase != Phase::Main {
            return Err("the game has not begun".to_string());
        }
        match order {
            Order::Raise { n } => check_raise(self, civ, *n),
            Order::Disband { unit, n } => {
                unit_of(self, civ, *unit, UnitKind::Camp)?;
                check_disband(*n)
            }
            Order::Move { unit, to } => check_move(self, unit_of(self, civ, *unit, UnitKind::Camp)?, *to),
            Order::Zone { unit, tiles } => {
                let u = unit_of(self, civ, *unit, UnitKind::Camp)?;
                units::zone_ok(self, u, tiles).map(|_| ())
            }
            Order::Switches { unit, .. } | Order::Resupply { unit } => {
                unit_of(self, civ, *unit, UnitKind::Camp).map(|_| ())
            }
            Order::Forage { unit } => check_take(self, unit_of(self, civ, *unit, UnitKind::Camp)?, false),
            Order::Pillage { unit } => check_take(self, unit_of(self, civ, *unit, UnitKind::Camp)?, true),
            Order::Route { unit, from, to } => {
                unit_of(self, civ, *unit, UnitKind::Scout)?;
                check_route(self, *from, *to)
            }
            Order::ClearRoute { unit } => unit_of(self, civ, *unit, UnitKind::Scout).map(|_| ()),
        }
    }

    /// Checks `order` for `civ` and applies it. Nothing is logged: `act` logs the player's orders,
    /// and the AI's are re-derived on a replay. `Err` leaves the world as it was.
    pub fn order(&mut self, civ: u32, order: Order) -> Result<(), String> {
        self.check_order(civ, &order)?;
        apply(self, civ, order);
        Ok(())
    }
}

/// The unit `id`, if it is `civ`'s and of `kind`; else why not.
fn unit_of(w: &World, civ: u32, id: u32, kind: UnitKind) -> Result<&Unit, String> {
    let u = w.units.get(&id).ok_or_else(|| format!("{id} is not a unit"))?;
    if u.civ != civ {
        return Err("that unit is not yours".to_string());
    }
    if u.kind() != kind {
        return Err(format!("that unit is not a {}", kind.name()));
    }
    Ok(u)
}

/// Applies an order that `check_order` has passed.
fn apply(w: &mut World, civ: u32, order: Order) {
    match order {
        Order::Raise { n } => {
            units::raise(w, civ, n).expect("checked: a clan can give a person");
        }
        Order::Disband { unit, n } => units::disband(w, unit, n, false),
        Order::Move { unit, to } => order_move(w, unit, to),
        Order::Zone { unit, tiles } => set_zone(w, unit, &tiles),
        Order::Switches { unit, forage, supply } => {
            let c = units::cs_mut(w, unit);
            c.auto_forage = forage;
            c.auto_supply = supply;
        }
        Order::Resupply { unit } => units::cs_mut(w, unit).resupply = true,
        Order::Forage { unit } => units::cs_mut(w, unit).forage = true,
        Order::Pillage { unit } => units::cs_mut(w, unit).pillage = true,
        Order::Route { unit, from, to } => order_route(w, unit, Some((from, to))),
        Order::ClearRoute { unit } => order_route(w, unit, None),
    }
}

/// Checks `Raise`: the army is on, `n` is at least one, a clan can give a person, and the store can
/// fill the packs of those that do.
fn check_raise(w: &World, civ: u32, n: u32) -> Result<(), String> {
    if !w.config.army {
        return Err("armies are off".to_string());
    }
    if n == 0 {
        return Err("raise at least one warrior".to_string());
    }
    let k = units::givers(w, civ).len().min(n as usize);
    if k == 0 {
        return Err("no clan has a person to spare".to_string());
    }
    if total(&w.civs[&civ].store) + units::EPS < units::pack_cap(k as f64) {
        return Err("the store can't fill their packs".to_string());
    }
    Ok(())
}

/// Checks `Disband`: at least one warrior.
fn check_disband(n: Option<u32>) -> Result<(), String> {
    if n == Some(0) {
        return Err("disband at least one warrior".to_string());
    }
    Ok(())
}

/// Checks `Move` for camp `u`: a tile of the map, land a camp can stand on, and a walk there.
fn check_move(w: &World, u: &Unit, to: usize) -> Result<(), String> {
    if to >= w.tiles.len() {
        return Err(format!("{to} is not a tile"));
    }
    if !(TERRAIN[w.tiles[to].terrain].land && passable(w, to)) {
        return Err("a camp can only go over land".to_string());
    }
    units::path_between(w, u.home, to).map(|_| ()).ok_or_else(|| "no route there".to_string())
}

/// Checks `Forage` (`pillage` false) and `Pillage` for camp `u`: foraging needs land that isn't a
/// rival's, pillaging a rival's.
fn check_take(w: &World, u: &Unit, pillage: bool) -> Result<(), String> {
    match (pillage, units::rival_of(w, u)) {
        (true, false) => Err("there is nothing to pillage here: it isn't a rival's land".to_string()),
        (false, true) => Err("this is a rival's land: pillage it".to_string()),
        _ => Ok(()),
    }
}

/// Checks `Route`: both tiles are different land or mountain tiles on the map.
fn check_route(w: &World, from: usize, to: usize) -> Result<(), String> {
    for t in [from, to] {
        if t >= w.tiles.len() {
            return Err(format!("{t} is not a tile"));
        }
        let terrain = w.tiles[t].terrain;
        if !(TERRAIN[terrain].land || terrain == MOUNTAIN) {
            return Err("a scout can only search land and mountains".to_string());
        }
    }
    if from == to {
        return Err("a route needs two different tiles".to_string());
    }
    Ok(())
}

/// Applies `Move`: a new destination and nothing banked; its own tile clears the order. Any move
/// order undeploys: the zone and held ground go back to the camp's own tile.
fn order_move(w: &mut World, unit: u32, to: usize) {
    if let Some(u) = w.units.get_mut(&unit) {
        u.banked = 0.0;
        u.actor.territory = vec![u.actor.home];
        let home = u.actor.home;
        if let Some(c) = u.camp_mut() {
            c.dest = if home == to { None } else { Some(to) };
            let g = c.held.iter().position(|&t| t == home).map(|k| c.guard.get(k).copied().unwrap_or(0.0));
            c.held = g.map(|_| vec![home]).unwrap_or_default();
            c.guard = g.into_iter().collect();
            c.full = false;
        }
    }
    if let Some(home) = w.units.get(&unit).filter(|u| u.camp().is_some_and(|c| c.dest.is_some())).map(|u| u.home) {
        let steps = units::path_between(w, home, to).map_or(0, |(p, _)| p.len());
        let at = w.at(to);
        w.note(unit, format!("marching to {at}, {steps} steps away"));
    }
}

/// Applies `Zone`: the zone is the camp's territory; held tiles outside it stop being held at once.
/// A zone beyond the camp's tile is a deployment.
fn set_zone(w: &mut World, unit: u32, tiles: &[usize]) {
    let Some(u) = w.units.get(&unit) else { return };
    let zone = units::zone_of(u.home, tiles);
    let (tile, civ) = (u.home, u.civ);
    let Some(c) = w.units.get_mut(&unit).and_then(|u| u.camp_mut()) else { return };
    let keep: Vec<(usize, f64)> = c
        .held
        .iter()
        .copied()
        .zip(c.guard.iter().copied().chain(std::iter::repeat(0.0)))
        .filter(|(t, _)| zone.binary_search(t).is_ok())
        .collect();
    c.held = keep.iter().map(|&(t, _)| t).collect();
    c.guard = keep.iter().map(|&(_, g)| g).collect();
    c.full = false;
    let size = zone.len();
    w.units.get_mut(&unit).unwrap().actor.territory = zone;
    if size > 1 {
        w.emit(Event::new(EventKind::Deployed, tile).civ(civ).unit(unit).other(size as u32));
        w.note(unit, format!("deployed over {size} tiles, its warriors spreading to hold them"));
    }
}

/// Applies `Route` (or clears the scout's route with `None`): a new route replaces the old one.
fn order_route(w: &mut World, unit: u32, route: Option<(usize, usize)>) {
    let text = match route {
        Some((from, to)) => format!("route set from {} to {}", w.at(from), w.at(to)),
        None => "route cleared".to_string(),
    };
    let Some(s) = w.units.get_mut(&unit).and_then(|u| u.scout_mut()) else { return };
    s.route = route;
    s.heard = false;
    s.blocked = false;
    w.note(unit, text);
}
