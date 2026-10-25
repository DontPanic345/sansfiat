//! Units: people sent out from a civ's clans. The one kind is the warrior, a stand-in that moves,
//! eats and disbands and does nothing else: it holds no land, harvests nothing, blocks no one,
//! and any number share a tile. Made at the capital from a person of a clan near it, fed from the
//! civ's store (the `civs` step), moved by the `units` step, disbanded by order or from hunger.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{passable, path, step_cost};
use crate::market::Basket;
use crate::world::{EventKind, Phase, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitKind {
    Warrior,
}

impl UnitKind {
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Warrior => "warrior",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub civ: u32,
    pub kind: UnitKind,
    pub tile: usize,
    /// Where it is going; it moves when the turn ends.
    pub dest: Option<usize>,
    /// Labour put towards the next step and not yet enough to take it.
    pub banked: f64,
    /// Hungry turns in a row.
    pub hungry: u32,
}

const EPS: f64 = 1e-9;

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// The clan of `civ` with the most people (ties to the lower id) among those with at least
/// `UNIT.min_clan`, from `near` (a round tile within `CIV.territory` of the capital) or from all.
fn giver(w: &World, civ: u32, near: bool) -> Option<u32> {
    let capital = w.civs.get(&civ)?.capital;
    let mut best: Option<(u32, f64)> = None;
    for c in w.clans.values().filter(|c| c.civ == Some(civ) && c.pop >= UNIT.min_clan) {
        if near && !c.round.iter().any(|&t| steps(w, t, capital) <= CIV.territory) {
            continue;
        }
        if best.is_none_or(|(_, p)| c.pop > p) {
            best = Some((c.id, c.pop));
        }
    }
    best.map(|(id, _)| id)
}

/// Puts a new warrior of `civ` on its capital, taking its person from `clan`.
fn spawn(w: &mut World, civ: u32, clan: Option<u32>) -> u32 {
    let capital = w.civs[&civ].capital;
    if let Some(c) = clan.and_then(|id| w.clans.get_mut(&id)) {
        c.pop -= 1.0;
    }
    let id = w.next_id;
    w.next_id += 1;
    let unit = Unit { id, civ, kind: UnitKind::Warrior, tile: capital, dest: None, banked: 0.0, hungry: 0 };
    w.units.insert(id, unit);
    w.unit_event(EventKind::UnitMade, capital, civ, id, clan);
    id
}

/// Makes a warrior for `civ` on its capital from the near clan with the most people (2+).
pub fn make(w: &mut World, civ: u32) -> Result<u32, String> {
    let clan = giver(w, civ, true).ok_or("no clan near the capital has a person to spare")?;
    Ok(spawn(w, civ, Some(clan)))
}

/// The starting warrior of `civ`, at founding: as `make`; else from its most populous clan with
/// `UNIT.min_clan` people; else taking no one. It never fails.
pub fn make_first(w: &mut World, civ: u32) -> u32 {
    let clan = giver(w, civ, true).or_else(|| giver(w, civ, false));
    spawn(w, civ, clan)
}

/// Sends `unit`'s person back to the civ clan whose home is fewest hex steps from it (ties to the
/// lower id), or loses it with no civ clan, and removes the unit. `starved` picks the event.
pub fn disband(w: &mut World, unit: u32, starved: bool) {
    let Some(u) = w.units.remove(&unit) else { return };
    let clan =
        w.clans.values().filter(|c| c.civ.is_some()).min_by_key(|c| (steps(w, c.home, u.tile), c.id)).map(|c| c.id);
    if let Some(c) = clan.and_then(|id| w.clans.get_mut(&id)) {
        c.pop += 1.0;
    }
    let kind = if starved { EventKind::UnitStarved } else { EventKind::UnitDisbanded };
    w.unit_event(kind, u.tile, u.civ, unit, clan);
}

/// Feeds `civ`'s units, in id order, from `store`: each takes `UNIT.food` from every good in
/// proportion to what is held. A unit short of it is hungry this turn; at `UNIT.hungry_turns` in a
/// row it disbands. Called by the `civs` step between the levy and the cap.
pub fn feed(w: &mut World, civ: u32, store: &mut Basket) {
    let ids: Vec<u32> = w.units.values().filter(|u| u.civ == civ).map(|u| u.id).collect();
    let mut starved = Vec::new();
    for id in ids {
        let total: f64 = store.iter().sum();
        let got = if total > 0.0 { UNIT.food.min(total) } else { 0.0 };
        if got > 0.0 {
            let keep = 1.0 - got / total;
            *store = store.map(|v| v * keep);
        }
        let (tile, hungry) = {
            let u = w.units.get_mut(&id).unwrap();
            if got + EPS >= UNIT.food {
                u.hungry = 0;
            } else {
                u.hungry += 1;
            }
            (u.tile, u.hungry)
        };
        if hungry == 1 {
            w.unit_event(EventKind::UnitHungry, tile, civ, id, None);
        }
        if hungry >= UNIT.hungry_turns {
            starved.push(id);
        }
    }
    for id in starved {
        disband(w, id, true);
    }
}

/// Whether a unit may walk onto tile `i`.
fn walkable(w: &World, i: usize) -> bool {
    TERRAIN[w.tiles[i].terrain].land && passable(w, i)
}

/// The cheapest route from `from` to `to` over land, as `land::path`: worn trails and river
/// crossings count; hidden trail costs are never read.
pub fn path_between(w: &World, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
    path(w, from, to, |i| walkable(w, i))
}

/// A unit's route from where it stands to its `dest`, with its cost, for the page.
pub fn route(w: &World, unit: u32) -> Option<(Vec<usize>, f64)> {
    let u = w.units.get(&unit)?;
    path_between(w, u.tile, u.dest?)
}

/// One turn's walking along `path` from `from`: spends `banked + UNIT.labour` on steps while the
/// next one costs no more than what is left. Returns the steps taken and the labour left. The one
/// stepping code, for `march` and `turns`.
fn walk(w: &World, from: usize, path: &[usize], banked: f64) -> (usize, f64) {
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

/// How many `units` steps (turns) it takes to walk `path` from `from` with `banked` already
/// banked, as `march` does it; 0 for an empty path.
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

/// The `units` step, after `civs`: each unit with a `dest`, in id order, recomputes its route
/// from where it stands and walks it with a turn's labour and what it banked.
pub fn march(w: &mut World) {
    let ids: Vec<u32> = w.units.values().filter(|u| u.dest.is_some()).map(|u| u.id).collect();
    for id in ids {
        let (from, banked) = (w.units[&id].tile, w.units[&id].banked);
        let Some((tiles, _)) = route(w, id) else {
            let u = w.units.get_mut(&id).unwrap();
            u.dest = None;
            u.banked = 0.0;
            continue;
        };
        let (taken, left) = walk(w, from, &tiles, banked);
        let u = w.units.get_mut(&id).unwrap();
        if taken > 0 {
            u.tile = tiles[taken - 1];
        }
        if taken == tiles.len() {
            u.dest = None;
            u.banked = 0.0;
        } else {
            u.banked = left;
        }
    }
}

/// The civ the player leads, or why there is none.
fn leader(w: &World) -> Result<u32, String> {
    if w.phase != Phase::Main {
        return Err("the game has not begun".to_string());
    }
    w.player.ok_or_else(|| "you're watching: you lead no civ".to_string())
}

/// Checks `MakeUnit`.
pub fn check_make(w: &World) -> Result<(), String> {
    let civ = leader(w)?;
    giver(w, civ, true).map(|_| ()).ok_or_else(|| "no clan near the capital has a person to spare".to_string())
}

/// The player's unit `unit`, or why an order about it is refused.
fn own(w: &World, unit: u32) -> Result<&Unit, String> {
    let civ = leader(w)?;
    let u = w.units.get(&unit).ok_or_else(|| format!("{unit} is not a unit"))?;
    if u.civ != civ {
        return Err("that unit isn't yours".to_string());
    }
    Ok(u)
}

/// Checks `DisbandUnit`.
pub fn check_disband(w: &World, unit: u32) -> Result<(), String> {
    own(w, unit).map(|_| ())
}

/// Checks `MoveUnit`.
pub fn check_move(w: &World, unit: u32, to: usize) -> Result<(), String> {
    let u = own(w, unit)?;
    if to >= w.tiles.len() {
        return Err(format!("{to} is not a tile"));
    }
    if !walkable(w, to) {
        return Err("a unit can only go over land".to_string());
    }
    path_between(w, u.tile, to).map(|_| ()).ok_or_else(|| "no route there".to_string())
}

/// Applies `MoveUnit`: a new destination and nothing banked; its own tile clears the order.
pub fn order_move(w: &mut World, unit: u32, to: usize) {
    if let Some(u) = w.units.get_mut(&unit) {
        u.dest = if u.tile == to { None } else { Some(to) };
        u.banked = 0.0;
    }
}
