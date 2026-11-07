//! The orders: what a civ's units are told to do. An order names its civ and, for everything but
//! `Raise`, the unit it is for. `World::order` checks an order and applies it without logging it, so
//! the AI's orders go through it and a replay re-derives them; `act` checks the player's, applies
//! them and logs them. The checks run in one order, and each refusal says why: the phase, then (for
//! `act` only) the civ the player leads, then the unit, then the order's own rule.

use serde::{Deserialize, Serialize};

use crate::civs;
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::passable;
use crate::market::total;
use crate::units::{self, Unit, UnitKind};
use crate::world::World;

/// An order to a civ's units. On the wire it is the `order` of `Action::Order`, with `type` in
/// camelCase; each variant's example is that wire form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Order {
    /// Raise `fighter` (warriors by default) into army `into`, which must be the civ's with its line
    /// not cut, or with none into a new army at the capital; or with `kind` `guard` people for its
    /// guard on its claim: one person from each of up to `n` of its clans.
    /// `{"type":"raise","n":2}` (a new army), `{"type":"raise","n":2,"into":12}`,
    /// `{"type":"raise","n":1,"kind":"guard"}`
    Raise {
        n: u32,
        #[serde(default)]
        kind: UnitKind,
        #[serde(default, skip_serializing_if = "is_warriors")]
        fighter: Fighter,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        into: Option<u32>,
    },
    /// Disband `n` of the army's or guard's people, or all with `None`, to the civ's own clan: of
    /// kind `fighter` (all of that kind with `n` none), else of every kind in proportion.
    /// `{"type":"disband","unit":12,"n":1}`, `{"type":"disband","unit":12,"n":1,"fighter":"warriors"}`
    Disband {
        unit: u32,
        n: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fighter: Option<Fighter>,
    },
    /// Replace the civ's claim: the tiles it paints out from its capital, which is added. The guard is
    /// divided over it at once. `{"type":"claim","tiles":[58,59]}`
    Claim { tiles: Vec<usize> },
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

/// Whether `f` is the default kind, which an order's JSON leaves out.
fn is_warriors(f: &Fighter) -> bool {
    *f == Fighter::Warriors
}

impl Order {
    /// The order's wire name, as its `type` reads: what `World::orders` counts it under.
    pub fn kind(&self) -> &'static str {
        match self {
            Order::Raise { .. } => "raise",
            Order::Disband { .. } => "disband",
            Order::Claim { .. } => "claim",
            Order::Move { .. } => "move",
            Order::Zone { .. } => "zone",
            Order::Switches { .. } => "switches",
            Order::Resupply { .. } => "resupply",
            Order::Forage { .. } => "forage",
            Order::Pillage { .. } => "pillage",
            Order::Route { .. } => "route",
            Order::ClearRoute { .. } => "clearRoute",
        }
    }
}

impl World {
    /// Checks `order` for `civ`: `Err` says why it can't happen. Whether `civ` is the one the player
    /// leads is `validate`'s check, not this one: the AI gives orders to every civ it doesn't lead.
    pub fn check_order(&self, civ: u32, order: &Order) -> Result<(), String> {
        if !self.phase.playing() {
            return Err("the game has not begun".to_string());
        }
        if self.civs.get(&civ).is_some_and(|c| c.is_out()) {
            return Err("your civ has fallen".to_string());
        }
        match order {
            Order::Raise { n, kind, fighter, into } => check_raise(self, civ, *n, *kind, *fighter, *into),
            Order::Claim { tiles } => claim::check(self, civ, tiles).map(|_| ()),
            Order::Disband { unit, n, fighter } => {
                let u = unit_among(self, civ, *unit, &[UnitKind::Camp, UnitKind::Scout, UnitKind::Guard])?;
                if let Some(f) = fighter {
                    let Some(c) = u.camp() else {
                        return Err("only an army has kinds: disband a scout or a guard by number".to_string());
                    };
                    if c.stack[f.index()] < 1.0 - units::EPS {
                        return Err(format!("it has no {}", f.rules().name));
                    }
                }
                check_disband(*n)
            }
            Order::Move { unit, to } => {
                check_move(self, unit_among(self, civ, *unit, &[UnitKind::Camp, UnitKind::Scout])?, *to)
            }
            Order::Zone { unit, tiles } => {
                let u = unit_of(self, civ, *unit, UnitKind::Camp)?;
                units::zone_ok(self, u, tiles).map(|_| ())
            }
            Order::Switches { unit, .. } | Order::Resupply { unit } => {
                unit_among(self, civ, *unit, &[UnitKind::Camp, UnitKind::Scout]).map(|_| ())
            }
            Order::Forage { unit } => {
                check_take(self, unit_among(self, civ, *unit, &[UnitKind::Camp, UnitKind::Scout])?, false)
            }
            Order::Pillage { unit } => {
                check_take(self, unit_among(self, civ, *unit, &[UnitKind::Camp, UnitKind::Scout])?, true)
            }
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
        self.count(civ, order.kind());
        apply(self, civ, order);
        Ok(())
    }
}

/// The unit `id`, if it is `civ`'s and of `kind`; else why not. A guard takes no order but raising
/// and disbanding, and says so.
fn unit_of(w: &World, civ: u32, id: u32, kind: UnitKind) -> Result<&Unit, String> {
    unit_among(w, civ, id, &[kind])
}

/// The unit `id`, if it is `civ`'s and of one of `kinds`; else why not.
fn unit_among<'a>(w: &'a World, civ: u32, id: u32, kinds: &[UnitKind]) -> Result<&'a Unit, String> {
    let u = w.units.get(&id).ok_or_else(|| format!("{id} is not a unit"))?;
    if u.civ != civ {
        return Err("that unit is not yours".to_string());
    }
    if !kinds.contains(&u.kind()) {
        return Err(match (u.kind(), kinds) {
            (UnitKind::Guard, _) => "a guard only holds the claim: it takes no order but disband".to_string(),
            (_, [kind]) => format!("that unit is not a {}", kind.name()),
            _ => {
                let names: Vec<String> = kinds.iter().map(|k| format!("a {}", k.name())).collect();
                format!("that unit is not {}", names.join(" or "))
            }
        });
    }
    Ok(u)
}

/// Applies an order that `check_order` has passed.
fn apply(w: &mut World, civ: u32, order: Order) {
    match order {
        Order::Raise { n, kind: UnitKind::Guard, .. } => {
            units::guard::raise(w, civ, n);
        }
        Order::Raise { n, kind: UnitKind::Scout, .. } => {
            units::raise_scout(w, civ, n).expect("checked: a clan can give a person");
        }
        Order::Raise { n, fighter, into, .. } => {
            units::raise(w, civ, n, fighter, into).expect("checked: a clan can give a person");
        }
        Order::Disband { unit, n, fighter } => units::disband_kind(w, unit, n, fighter, false),
        Order::Claim { tiles } => {
            let claim = claim::check(w, civ, &tiles).expect("checked: the claim holds");
            let name = units::name_of(w, civ);
            let n = claim.len();
            claim::set(w, civ, claim);
            w.note(civ, format!("{name} claims {n} tile(s) from its capital"));
        }
        Order::Move { unit, to } => {
            order_move(w, unit, to);
            if let Some(c) = w.units.get_mut(&unit).and_then(|u| u.camp_mut()) {
                c.still = 0;
            }
        }
        Order::Zone { unit, tiles } => set_zone(w, unit, &tiles),
        Order::Switches { unit, forage, supply } => {
            let u = w.units.get_mut(&unit).expect("checked: a unit");
            match &mut u.state {
                units::UnitState::Camp(c) => (c.auto_forage, c.auto_supply) = (forage, supply),
                units::UnitState::Scout(s) => (s.auto_forage, s.auto_supply) = (forage, supply),
                units::UnitState::Guard => {}
            }
        }
        Order::Resupply { unit } => once(w, unit, Once::Resupply),
        Order::Forage { unit } => once(w, unit, Once::Forage),
        Order::Pillage { unit } => once(w, unit, Once::Pillage),
        Order::Route { unit, from, to } => order_route(w, unit, Some((from, to))),
        Order::ClearRoute { unit } => order_route(w, unit, None),
    }
}

/// A one-shot order, for a camp or a scout.
enum Once {
    Resupply,
    Forage,
    Pillage,
}

/// Sets one-shot order `which` on unit `unit`, a camp or a scout.
fn once(w: &mut World, unit: u32, which: Once) {
    let u = w.units.get_mut(&unit).expect("checked: a unit");
    let (resupply, forage, pillage) = match &mut u.state {
        units::UnitState::Camp(c) => (&mut c.resupply, &mut c.forage, &mut c.pillage),
        units::UnitState::Scout(s) => (&mut s.resupply, &mut s.forage, &mut s.pillage),
        units::UnitState::Guard => return,
    };
    match which {
        Once::Resupply => *resupply = true,
        Once::Forage => *forage = true,
        Once::Pillage => *pillage = true,
    }
}

/// Checks `Raise`. A scout's: the army is on, `n` is at least one, a clan can give a person, and the
/// store can fill the packs. A camp's: the army is on, `n` is at least one, warriors (archers come in
/// build-2), `into` (if any) is the civ's army with its line not cut, a clan can give a person, and
/// the store can fill the packs of those that do. A guard's: the army is on, `n` is at least one, the civ
/// may claim land (`Config.claims`), and a clan can give a person.
fn check_raise(w: &World, civ: u32, n: u32, kind: UnitKind, fighter: Fighter, into: Option<u32>) -> Result<(), String> {
    if !w.config.army {
        return Err("armies are off".to_string());
    }
    if n == 0 {
        return Err("raise at least one warrior".to_string());
    }
    match kind {
        UnitKind::Scout => {
            let k = units::givers(w, civ).len().min(n as usize);
            if k == 0 {
                return Err("no clan has a person to spare".to_string());
            }
            return if total(&w.civs[&civ].store) + units::EPS < units::pack_cap(k as f64) {
                Err("the store can't fill their packs".to_string())
            } else {
                Ok(())
            };
        }
        UnitKind::Guard if !claim::may_claim(w, civ) => {
            return Err("this civ may not claim land, so it has no guard".to_string());
        }
        UnitKind::Guard => {
            let k = units::givers(w, civ).len().min(n as usize);
            return if k == 0 { Err("no clan has a person to spare".to_string()) } else { Ok(()) };
        }
        UnitKind::Camp => {}
    }
    if fighter == Fighter::Archers && !civs::land(w, civ).into_iter().any(|t| w.tiles[t].terrain == FOREST) {
        return Err("you hold no forest to make bows on".to_string());
    }
    if let Some(id) = into {
        unit_of(w, civ, id, UnitKind::Camp)?;
        if units::cut_off(w, id) {
            return Err("its line is cut: raise at the capital".to_string());
        }
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

/// Checks `Move` for camp or scout `u`: a tile of the map, land it can stand on, and a walk there.
fn check_move(w: &World, u: &Unit, to: usize) -> Result<(), String> {
    if to >= w.tiles.len() {
        return Err(format!("{to} is not a tile"));
    }
    if !(TERRAIN[w.tiles[to].terrain].land && passable(w, to)) {
        let kind = if u.scout().is_some() { "a scout" } else { "a camp" };
        return Err(format!("{kind} can only go over land"));
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

/// Applies `Move`: a new destination and nothing banked; its own tile clears the order. A camp's
/// move undeploys: the zone and held ground go back to the camp's own tile. A scout's move clears
/// its route, and says how far it is and whether its line reaches there.
fn order_move(w: &mut World, unit: u32, to: usize) {
    let Some(u) = w.units.get_mut(&unit) else { return };
    u.banked = 0.0;
    u.actor.territory = vec![u.actor.home];
    let home = u.actor.home;
    let dest = if home == to { None } else { Some(to) };
    if let Some(c) = u.camp_mut() {
        c.dest = dest;
        let g = c.held.iter().position(|&t| t == home).map(|k| c.guard.get(k).copied().unwrap_or(0.0));
        c.held = g.map(|_| vec![home]).unwrap_or_default();
        c.guard = g.into_iter().collect();
        c.full = false;
    }
    if let Some(s) = u.scout_mut() {
        s.dest = dest;
        s.wander = None;
        s.route = None;
        s.heard = false;
        s.blocked = false;
        s.built = false;
    }
    if dest.is_some() {
        let steps = units::path_between(w, home, to).map_or(0, |(p, _)| p.len());
        let at = w.at(to);
        let scout = w.units[&unit].scout().is_some();
        let text = if scout {
            format!("going to {at}, {steps} steps away")
        } else {
            format!("marching to {at}, {steps} steps away")
        };
        w.note(unit, text);
    }
    if dest.is_some() && w.units[&unit].scout().is_some() {
        reach_note(w, unit, to);
    }
}

/// Says a scout's line from `tile` does not carry what it eats, or reaches nothing there.
fn reach_note(w: &mut World, unit: u32, tile: usize) {
    if !units::past_reach(w, unit, tile) {
        return;
    }
    let at = w.at(tile);
    let text = match units::line_from(w, unit, tile) {
        Some((_, carry)) => {
            let eats = w.units[&unit].people * UNIT.food;
            format!("past its line's reach: its line there would carry {carry:.1} of the {eats:.0} it eats")
        }
        None => format!("past its line's reach: no line reaches {at}"),
    };
    w.note(unit, text);
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
    s.dest = None;
    s.wander = None;
    s.heard = false;
    s.blocked = false;
    s.built = false;
    w.note(unit, text);
    if let Some((_, to)) = route {
        reach_note(w, unit, to);
    }
}
