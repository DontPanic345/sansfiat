//! The camp: a civ's army. Its warriors spread over its zone and hold it (`hold`), guarding it with
//! labour set aside, so a rival's line can't pass them. At the end of the turn, on the labour its
//! march and guarding left, it forages the tile (pillages a rival's) and resupplies along the line.
//! Its orders are checked here (`check_disband`, `check_move`, `check_take`, `check_camp`, `check_zone`)
//! and applied by `order_move`, `set_zone` and `with_camp`.

use std::collections::BTreeMap;

use super::line::{cut_by, cut_off, cutter, line, path_between, route, walkable};
use super::{camp, disband, leader, name_of, rival_of, room, strip, supply, Unit, EPS};
use crate::actors::{connected, divide, stride, Route, Walks};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{neighbours, passable};
use crate::market::total;
use crate::world::World;

/// A camp's own state: its orders, its zone (`held` and `guard` are the tiles its warriors hold),
/// its switches and what it did last turn.
#[derive(Clone, Debug, PartialEq)]
pub struct CampState {
    /// Where it is going; it moves when the turn ends.
    pub dest: Option<usize>,
    pub auto_forage: bool,
    pub auto_supply: bool,
    /// One-shot orders for this turn's end.
    pub resupply: bool,
    pub forage: bool,
    pub pillage: bool,
    /// What it did last turn, so the feed says when each starts.
    pub cut: bool,
    pub foraging: bool,
    pub supplying: bool,
    /// The tiles of its zone (`territory`) its warriors hold, sorted, and each one's guard share
    /// after its walk, in the same order.
    pub held: Vec<usize>,
    pub guard: Vec<f64>,
    /// Whether the zone was all held last turn, and the civ whose ground cut the line, so the feed
    /// says when each starts.
    pub full: bool,
    pub cut_by: Option<u32>,
}

/// The camp state of unit `id` (the unit is a camp; the callers have checked).
fn cs(w: &World, id: u32) -> &CampState {
    w.units[&id].camp().expect("a camp")
}

fn cs_mut(w: &mut World, id: u32) -> &mut CampState {
    w.units.get_mut(&id).and_then(|u| u.camp_mut()).expect("a camp")
}

/// One camp's march. Returns the labour the march left.
pub(super) fn camp_march(w: &mut World, id: u32) -> f64 {
    let people = w.units[&id].people;
    w.units.get_mut(&id).unwrap().spoil();

    // The march. Every warrior walks, so a step costs one warrior's labour and the camp's labour
    // left is that, times the warriors. A march not done leaves nothing.
    let mut left = people * UNIT.labour;
    let mut arrived = false;
    if let Some((tiles, _)) = route(w, id) {
        let u = &w.units[&id];
        let (taken, rest) = stride(w, u.home, &tiles, u.banked + UNIT.labour, 0.0);
        if taken > 0 {
            let u = w.units.get_mut(&id).unwrap();
            u.actor.home = tiles[taken - 1];
            u.actor.territory = vec![u.actor.home];
            let home = u.actor.home;
            let c = cs_mut(w, id);
            c.held = vec![home];
            c.guard = vec![0.0];
        }
        if taken == tiles.len() {
            cs_mut(w, id).dest = None;
            w.units.get_mut(&id).unwrap().banked = 0.0;
            left = rest * people;
            arrived = true;
        } else {
            w.units.get_mut(&id).unwrap().banked = rest;
            left = 0.0;
        }
    } else {
        cs_mut(w, id).dest = None;
        w.units.get_mut(&id).unwrap().banked = 0.0;
    }
    if arrived {
        let at = w.at(w.units[&id].home);
        w.note(id, format!("arrived at {at}"));
    }
    left
}

/// The labour a camp sets aside to guard: `UNIT.guarding` a warrior. Spent only when it is
/// deployed (its zone is more than its own tile); undeployed, its warriors guard their tile anyway.
fn guarding(u: &Unit) -> f64 {
    u.people * UNIT.guarding
}

/// The spread and the holding, for every camp at once, after the march: each camp's held tiles
/// still in its zone, its own tile and the zone tiles next to them are its candidates; its guarding
/// is divided over them (`divide`, each share paying its walk inside them at `UNIT.inside`), and a
/// tile whose share falls under `UNIT.guard` isn't held (its own tile always is, unless taken). A
/// tile two camps want goes to the larger share; a tie stays with the camp that held it, else the
/// lower civ id. Returns the labour each deployed camp spent guarding.
pub(super) fn hold(w: &mut World) -> BTreeMap<u32, f64> {
    // (tile, unit, civ, share, held last turn)
    let mut want: Vec<(usize, u32, u32, f64, bool)> = Vec::new();
    let mut spent = BTreeMap::new();
    for u in w.units.values() {
        let Some(c) = u.camp() else { continue };
        let zone = &u.actor.territory;
        let mut was: Vec<usize> =
            c.held.iter().copied().filter(|t| zone.binary_search(t).is_ok() || *t == u.home).collect();
        if !was.contains(&u.home) {
            was.push(u.home);
        }
        let mut cand: Vec<usize> = zone
            .iter()
            .copied()
            .filter(|&t| was.contains(&t) || neighbours(w, t).into_iter().any(|j| was.contains(&j)))
            .chain(std::iter::once(u.home))
            .collect();
        cand.sort_unstable();
        cand.dedup();
        let walks = Walks::new(w, u.home, &cand);
        let shares = divide(guarding(u), &[], &walks.yearly(UNIT.inside));
        for (k, &t) in cand.iter().enumerate() {
            if t == u.home || shares[k] + EPS >= UNIT.guard {
                want.push((t, u.id, u.civ, shares[k], c.held.contains(&t)));
            }
        }
        if zone.len() > 1 {
            spent.insert(u.id, guarding(u));
        }
    }
    let mut held: BTreeMap<u32, Vec<(usize, f64)>> = w.units.keys().map(|&id| (id, Vec::new())).collect();
    for &(t, id, civ, share, was) in &want {
        let beaten = want.iter().any(|&(t2, id2, civ2, share2, was2)| {
            t2 == t
                && id2 != id
                && (share2 > share + EPS || (share2 + EPS >= share && ((was2 && !was) || (was2 == was && civ2 < civ))))
        });
        if !beaten {
            held.get_mut(&id).unwrap().push((t, share));
        }
    }
    for (id, tiles) in held {
        let Some(c) = w.units.get_mut(&id).and_then(|u| u.camp_mut()) else { continue };
        c.held = tiles.iter().map(|&(t, _)| t).collect();
        c.guard = tiles.iter().map(|&(_, g)| g).collect();
    }
    spent
}

/// The rest of one camp's turn, after the spread: on the labour left (less its guarding) the
/// one-shot orders, auto-forage, auto-supply (or only the line, if the switches are off), and last
/// eat.
pub(super) fn camp_turn(w: &mut World, id: u32, left: f64) {
    let (civ, people) = (w.units[&id].civ, w.units[&id].people);
    let mut left = left;

    // The zone held, and when it is first all held.
    let tile = w.units[&id].home;
    {
        let (size, held) = (w.units[&id].actor.territory.len(), cs(w, id).held.len());
        let beyond = held.saturating_sub(1) as u32;
        let full = size > 1 && held == size;
        let first = full && !cs(w, id).full;
        cs_mut(w, id).full = full;
        w.civs.get_mut(&civ).unwrap().army.held += beyond;
        if first {
            w.emit(Event::new(EventKind::Held, tile).civ(civ).unit(id).other(size as u32));
            w.note(id, format!("holds its zone of {size} tiles"));
        }
    }

    // The line, and whether it can feed the camp; cut by an army when it could without held ground.
    let line_now = line(w, id);
    let cut = cut_off(w, id);
    let by = if cut && !cut_by(w, id, false) { Some(cutter(w, id)) } else { None };
    {
        let u = w.units.get_mut(&id).unwrap();
        u.actor.routes = match (&line_now, w.civs.get(&civ)) {
            (Some((p, cost, _)), Some(c)) if !p.is_empty() => {
                vec![Route { to: c.capital, path: p.clone(), cost: *cost }]
            }
            _ => Vec::new(),
        };
    }
    if cut {
        w.civs.get_mut(&civ).unwrap().army.cut_off += 1;
    }
    if let Some(by) = by {
        w.civs.get_mut(&civ).unwrap().army.cut_by_army += 1;
        if cs(w, id).cut_by != Some(by) {
            w.emit(Event::new(EventKind::LineCut, tile).civ(civ).unit(id).other(by));
            let name = name_of(w, by);
            w.note(id, format!("its line is cut by {name}'s army"));
        }
    } else if cut && (!cs(w, id).cut || cs(w, id).cut_by.is_some()) {
        w.emit(Event::new(EventKind::CampCutOff, tile).civ(civ).unit(id));
        w.note(id, "its line is cut: it can't carry a turn's food for its warriors".to_string());
    }
    {
        let c = cs_mut(w, id);
        c.cut = cut;
        c.cut_by = by;
    }

    let need = people * UNIT.food;
    let mut foraged = 0.0;
    let mut pillaged = 0.0;
    let mut brought = 0.0;
    let rival = rival_of(w, &w.units[&id]);

    // One-shot orders, on this turn's leftover, with the switches off or on.
    let (once_forage, once_pillage, once_supply) = {
        let c = cs(w, id);
        (c.forage, c.pillage, c.resupply)
    };
    {
        let c = cs_mut(w, id);
        c.forage = false;
        c.pillage = false;
        c.resupply = false;
    }
    if (once_forage && !rival) || (once_pillage && rival) {
        let rate = if rival { UNIT.pillage } else { UNIT.forage };
        let (used, food) = strip(w, id, left, room(&w.units[&id]), rate);
        left -= used;
        if rival {
            pillaged += food;
        } else {
            foraged += food;
        }
    }
    if once_supply {
        let (used, food) = supply(w, id, left, room(&w.units[&id]));
        left -= used;
        brought += food;
    }
    // Auto-forage covers what eating needs this turn that the pack doesn't.
    if cs(w, id).auto_forage {
        let short = (need - total(&w.units[&id].store)).max(0.0);
        let rate = if rival { UNIT.pillage } else { UNIT.forage };
        let (used, food) = strip(w, id, left, short.min(room(&w.units[&id])), rate);
        left -= used;
        if rival {
            pillaged += food;
        } else {
            foraged += food;
        }
    }
    // Auto-supply covers what eating needs that foraging didn't, then fills the pack.
    if cs(w, id).auto_supply {
        let (used, food) = supply(w, id, left, room(&w.units[&id]));
        left -= used;
        brought += food;
    }
    let _ = left;

    {
        let a = &mut w.civs.get_mut(&civ).unwrap().army;
        a.foraged += foraged;
        a.pillaged += pillaged;
        a.resupplied += brought;
        if foraged > 0.0 {
            a.foraged_tiles += 1;
        }
        if pillaged > 0.0 {
            a.pillaged_tiles += 1;
        }
    }
    let (was_foraging, was_supplying) = (cs(w, id).foraging, cs(w, id).supplying);
    let (now_foraging, now_supplying) = (foraged + pillaged > 0.0, brought > 0.0);
    let at = w.at(tile);
    if now_foraging && !was_foraging {
        let kind = if pillaged > 0.0 { EventKind::CampPillaged } else { EventKind::CampForaged };
        w.emit(Event::new(kind, tile).civ(civ).unit(id));
        let verb = if pillaged > 0.0 { "pillages" } else { "forages" };
        w.note(id, format!("{verb} at {at}"));
    }
    if now_supplying && !was_supplying {
        w.emit(Event::new(EventKind::CampResupplied, tile).civ(civ).unit(id));
        w.note(id, "its line brings food to the camp".to_string());
    }
    {
        let c = cs_mut(w, id);
        c.foraging = now_foraging;
        c.supplying = now_supplying;
    }

    // Eating, from the pack in proportion to what it holds.
    let u = w.units.get_mut(&id).unwrap();
    let eaten = u.eat_by_share(need);
    let covered = total(&eaten) / need;
    let fed = total(&eaten) + EPS >= need;
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let hungry = u.hungry;
    if !fed {
        w.civs.get_mut(&civ).unwrap().army.hungry += 1;
    }
    if hungry == 1 {
        w.emit(Event::new(EventKind::CampHungry, tile).civ(civ).unit(id));
        w.note(id, format!("went hungry: its pack covered {:.0}% of its need", covered * 100.0));
    }
    if hungry >= UNIT.hungry_turns {
        w.civs.get_mut(&civ).unwrap().army.starved += 1;
        disband(w, civ, Some(1), true);
    }
}

/// The player's camp, or why there is none.
fn own(w: &World) -> Result<&Unit, String> {
    let civ = leader(w)?;
    camp(w, civ).ok_or_else(|| "you have no camp".to_string())
}

/// Checks `Disband`.
pub fn check_disband(w: &World, n: Option<u32>) -> Result<(), String> {
    own(w)?;
    if n == Some(0) {
        return Err("disband at least one warrior".to_string());
    }
    Ok(())
}

/// Checks `MoveCamp`.
pub fn check_move(w: &World, to: usize) -> Result<(), String> {
    let u = own(w)?;
    if to >= w.tiles.len() {
        return Err(format!("{to} is not a tile"));
    }
    if !(TERRAIN[w.tiles[to].terrain].land && passable(w, to)) {
        return Err("a camp can only go over land".to_string());
    }
    path_between(w, u.home, to).map(|_| ()).ok_or_else(|| "no route there".to_string())
}

/// Checks `Forage` (`pillage` false) and `Pillage`: foraging needs land that isn't a rival's,
/// pillaging a rival's.
pub fn check_take(w: &World, pillage: bool) -> Result<(), String> {
    let u = own(w)?;
    match (pillage, rival_of(w, u)) {
        (true, false) => Err("there is nothing to pillage here: it isn't a rival's land".to_string()),
        (false, true) => Err("this is a rival's land: pillage it".to_string()),
        _ => Ok(()),
    }
}

/// Checks `SetSwitches` and `Resupply`: just that there is a camp.
pub fn check_camp(w: &World) -> Result<(), String> {
    own(w).map(|_| ())
}

/// Applies `MoveCamp`: a new destination and nothing banked; its own tile clears the order. Any
/// move order undeploys: the zone and held ground go back to the camp's own tile.
pub fn order_move(w: &mut World, unit: u32, to: usize) {
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
        let steps = path_between(w, home, to).map_or(0, |(p, _)| p.len());
        let at = w.at(to);
        w.note(unit, format!("marching to {at}, {steps} steps away"));
    }
}

/// The zone `tiles` (the painted ones; the camp's own may be left out) with the camp's tile, sorted.
pub(super) fn zone_of(home: usize, tiles: &[usize]) -> Vec<usize> {
    let mut zone: Vec<usize> = tiles.iter().copied().chain(std::iter::once(home)).collect();
    zone.sort_unstable();
    zone.dedup();
    zone
}

/// The most tiles a camp's zone may have, its own included: one a warrior.
pub fn zone_cap(u: &Unit) -> usize {
    (u.people.floor() as usize).max(1)
}

/// Whether `tiles` may be camp `u`'s zone: walkable land that is no civ's capital, at most
/// `zone_cap` tiles with the camp's own, connected to it.
pub(super) fn zone_ok(w: &World, u: &Unit, tiles: &[usize]) -> Result<Vec<usize>, String> {
    if let Some(&t) = tiles.iter().find(|&&t| t >= w.tiles.len()) {
        return Err(format!("{t} is not a tile"));
    }
    if tiles.iter().any(|&t| !walkable(w, t)) {
        return Err("a zone can only cover land a camp can walk".to_string());
    }
    if tiles.iter().any(|&t| w.civs.values().any(|c| c.capital == t)) {
        return Err("a zone can't cover a capital".to_string());
    }
    let zone = zone_of(u.home, tiles);
    if zone.len() > zone_cap(u) {
        return Err(format!("{} warriors can hold at most {} tiles", u.people, zone_cap(u)));
    }
    if !connected(w, u.home, &zone) {
        return Err("a zone must be connected to the camp".to_string());
    }
    Ok(zone)
}

/// Checks `Zone`.
pub fn check_zone(w: &World, tiles: &[usize]) -> Result<(), String> {
    let u = own(w)?;
    zone_ok(w, u, tiles).map(|_| ())
}

/// Applies `Zone` (and the AI's): the zone is the camp's territory; held tiles outside it stop
/// being held at once. A zone beyond the camp's tile is a deployment.
pub fn set_zone(w: &mut World, unit: u32, tiles: &[usize]) {
    let Some(u) = w.units.get(&unit) else { return };
    let zone = zone_of(u.home, tiles);
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

/// Applies the player's order to the camp's state, through `f`.
pub fn with_camp(w: &mut World, f: impl FnOnce(&mut CampState)) {
    if let Some(civ) = w.player {
        if let Some(id) = camp(w, civ).map(|u| u.id) {
            f(cs_mut(w, id));
        }
    }
}
