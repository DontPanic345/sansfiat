//! The army: a civ's camp and its scout are units (`docs/design/supply.md`). A unit holds what every
//! unit has (its warriors, tile, pack and labour) and its kind's own state: a camp's zone, orders
//! and line, or a scout's route. The camp is a pack it eats from, a tile it stands on and a line to
//! the capital, where the civ's store is. Its territory is its zone: the tiles its warriors spread
//! over one ring a turn and hold (`hold`), guarding them with labour set aside, so a rival's line
//! can't pass them. Warriors are raised from the civ's clans, one person from each, and disbanded to
//! the civ's own clan. The `ai` step keeps each AI camp posted; the `units` step runs every unit.
//!
//! The files: `line` (the paths and the line to the capital), `camp` (the camp's turn and orders and
//! its zone), `scout` (the route search), `ai` (the AI's posts and zones). The queries, raising,
//! disbanding and the `units` step are here.

mod ai;
mod camp;
mod line;
mod scout;

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::Actor;
use crate::civs;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::market::{add, take, total, Basket};
use crate::world::{Phase, World};

pub use ai::{ai, post};
pub use camp::{
    check_camp, check_disband, check_move, check_take, check_zone, order_move, set_zone, with_camp, zone_cap, CampState,
};
pub use line::{cut_at, cut_off, line, path_between, route, turns};
pub use scout::{check_route, check_scout, order_route, route_plan, Plan, ScoutState};

use camp::{camp_march, camp_turn, hold};
use scout::scout_turn;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitKind {
    Camp,
    Scout,
}

impl UnitKind {
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Camp => "camp",
            UnitKind::Scout => "scout",
        }
    }
}

/// What a civ's army has done, kept on the civ so it outlives any one camp. The sweep reads it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Counters {
    /// Camps raised (founding, the AI's re-raising, the player's).
    pub raised: u32,
    /// Camps starved out to nothing.
    pub lost: u32,
    /// Turns the camp was hungry.
    pub hungry: u32,
    /// Turns the camp was cut off from its capital.
    pub cut_off: u32,
    /// Food foraged, and the tiles stripped doing it.
    pub foraged: f64,
    pub foraged_tiles: u32,
    /// Food pillaged from rivals, and the tiles stripped.
    pub pillaged: f64,
    pub pillaged_tiles: u32,
    /// Food brought along the line.
    pub resupplied: f64,
    /// Warriors lost to hunger.
    pub starved: u32,
    /// Tile-turns held beyond the camp's own tile.
    #[serde(default)]
    pub held: u32,
    /// Turns the line was cut by a rival's held ground (and would carry the camp's food without it).
    #[serde(default)]
    pub cut_by_army: u32,
}

/// A unit: what every unit has, and its kind's own state. Its JSON is one flat object (see
/// `UnitFlat`), so the page and the digests see the fields they always have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "UnitFlat", into = "UnitFlat")]
pub struct Unit {
    pub id: u32,
    pub civ: u32,
    /// Its warriors (`people`), the tile it stands on (`home`), its pack (`store`) and its line to
    /// the capital (`routes`: one `Route`, or none when cut off or already there).
    pub actor: Actor,
    /// Labour per warrior put towards the next step and not yet enough to take it.
    pub banked: f64,
    /// Hungry turns in a row.
    pub hungry: u32,
    pub state: UnitState,
}

/// What a unit is, and the state only that kind has.
#[derive(Clone, Debug, PartialEq)]
pub enum UnitState {
    Camp(CampState),
    Scout(ScoutState),
}

/// A unit's JSON: the flat object every unit has always written, the fields in their old order.
/// A camp writes the scout's fields at their defaults, and a scout writes the camp's fields as
/// `raise_scout_at_founding` sets them (`scout_camp_fields`), which nothing changes after.
#[derive(Clone, Serialize, Deserialize)]
struct UnitFlat {
    id: u32,
    civ: u32,
    kind: UnitKind,
    #[serde(flatten)]
    actor: Actor,
    dest: Option<usize>,
    banked: f64,
    hungry: u32,
    auto_forage: bool,
    auto_supply: bool,
    resupply: bool,
    forage: bool,
    pillage: bool,
    cut: bool,
    foraging: bool,
    supplying: bool,
    #[serde(default)]
    held: Vec<usize>,
    #[serde(default)]
    guard: Vec<f64>,
    #[serde(default)]
    full: bool,
    #[serde(default)]
    cut_by: Option<u32>,
    #[serde(default)]
    route: Option<(usize, usize)>,
    #[serde(default)]
    heard: bool,
    #[serde(default)]
    blocked: bool,
}

impl From<Unit> for UnitFlat {
    fn from(u: Unit) -> Self {
        let kind = u.kind();
        let (camp, scout) = match u.state {
            UnitState::Camp(c) => (c, ScoutState { route: None, heard: false, blocked: false }),
            UnitState::Scout(s) => (scout_camp_fields(), s),
        };
        UnitFlat {
            id: u.id,
            civ: u.civ,
            kind,
            actor: u.actor,
            dest: camp.dest,
            banked: u.banked,
            hungry: u.hungry,
            auto_forage: camp.auto_forage,
            auto_supply: camp.auto_supply,
            resupply: camp.resupply,
            forage: camp.forage,
            pillage: camp.pillage,
            cut: camp.cut,
            foraging: camp.foraging,
            supplying: camp.supplying,
            held: camp.held,
            guard: camp.guard,
            full: camp.full,
            cut_by: camp.cut_by,
            route: scout.route,
            heard: scout.heard,
            blocked: scout.blocked,
        }
    }
}

impl From<UnitFlat> for Unit {
    /// Reads the state its `kind` names; the other kind's fields are dropped.
    fn from(f: UnitFlat) -> Self {
        let state = match f.kind {
            UnitKind::Camp => UnitState::Camp(CampState {
                dest: f.dest,
                auto_forage: f.auto_forage,
                auto_supply: f.auto_supply,
                resupply: f.resupply,
                forage: f.forage,
                pillage: f.pillage,
                cut: f.cut,
                foraging: f.foraging,
                supplying: f.supplying,
                held: f.held,
                guard: f.guard,
                full: f.full,
                cut_by: f.cut_by,
            }),
            UnitKind::Scout => UnitState::Scout(ScoutState { route: f.route, heard: f.heard, blocked: f.blocked }),
        };
        Unit { id: f.id, civ: f.civ, actor: f.actor, banked: f.banked, hungry: f.hungry, state }
    }
}

/// The camp fields a scout carries: what `raise_scout_at_founding` sets (not `Default`, which would
/// write `auto_forage` false and move every digest).
fn scout_camp_fields() -> CampState {
    CampState {
        dest: None,
        auto_forage: true,
        auto_supply: true,
        resupply: false,
        forage: false,
        pillage: false,
        cut: false,
        foraging: false,
        supplying: false,
        held: Vec::new(),
        guard: Vec::new(),
        full: false,
        cut_by: None,
    }
}

impl Deref for Unit {
    type Target = Actor;
    fn deref(&self) -> &Actor {
        &self.actor
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut Actor {
        &mut self.actor
    }
}

impl Unit {
    /// The tile the unit stands on.
    pub fn tile(&self) -> usize {
        self.actor.home
    }

    pub fn kind(&self) -> UnitKind {
        match self.state {
            UnitState::Camp(_) => UnitKind::Camp,
            UnitState::Scout(_) => UnitKind::Scout,
        }
    }

    pub fn camp(&self) -> Option<&CampState> {
        match &self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) => None,
        }
    }

    pub fn camp_mut(&mut self) -> Option<&mut CampState> {
        match &mut self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) => None,
        }
    }

    pub fn scout(&self) -> Option<&ScoutState> {
        match &self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) => None,
        }
    }

    pub fn scout_mut(&mut self) -> Option<&mut ScoutState> {
        match &mut self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) => None,
        }
    }

    /// The tiles a camp's warriors hold (none for a scout).
    pub fn held(&self) -> &[usize] {
        self.camp().map_or(&[][..], |c| c.held.as_slice())
    }
}

const EPS: f64 = 1e-9;

/// `civ`'s camp, if it has one.
pub fn camp(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Camp)
}

/// `civ`'s scout, if it has one.
pub fn scout(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Scout)
}

/// The food `people` warriors' packs hold.
pub fn pack_cap(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

/// The name of `civ`, for a note.
fn name_of(w: &World, civ: u32) -> String {
    w.civs.get(&civ).map_or_else(|| "a rival".to_string(), |c| c.name.clone())
}

/// The clans of `civ` that can give a person, biggest first (ties to the lower id).
fn givers(w: &World, civ: u32) -> Vec<u32> {
    let mut ids: Vec<u32> =
        w.clans.values().filter(|c| c.civ == Some(civ) && c.people >= UNIT.min_clan).map(|c| c.id).collect();
    ids.sort_by(|a, b| w.clans[b].people.total_cmp(&w.clans[a].people).then(a.cmp(b)));
    ids
}

/// The clan a disbanded warrior goes to: the civ's own, else its biggest.
fn home_clan(w: &World, civ: u32) -> Option<u32> {
    let own = w.civs.get(&civ)?.clan;
    if w.clans.contains_key(&own) {
        return Some(own);
    }
    w.clans
        .values()
        .filter(|c| c.civ == Some(civ))
        .min_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)))
        .map(|c| c.id)
}

/// Adds `k` warriors (each from the pack share taken from the store) to `civ`'s camp, making it
/// at the capital if there is none. The people have already been taken from their clans.
fn join(w: &mut World, civ: u32, k: f64, pack: Basket) -> u32 {
    let capital = w.civs[&civ].capital;
    let existing = camp(w, civ).map(|u| u.id);
    let id = match existing {
        Some(id) => {
            let u = w.units.get_mut(&id).unwrap();
            u.people += k;
            add(&mut u.store, &pack);
            id
        }
        None => {
            let id = w.next_id;
            w.next_id += 1;
            let mut actor = Actor::new(capital, k);
            actor.store = pack;
            let unit = Unit {
                id,
                civ,
                actor,
                banked: 0.0,
                hungry: 0,
                state: UnitState::Camp(CampState {
                    dest: None,
                    auto_forage: true,
                    auto_supply: true,
                    resupply: false,
                    forage: false,
                    pillage: false,
                    cut: false,
                    foraging: false,
                    supplying: false,
                    held: vec![capital],
                    guard: vec![0.0],
                    full: false,
                    cut_by: None,
                }),
            };
            w.units.insert(id, unit);
            w.civs.get_mut(&civ).unwrap().army.raised += 1;
            id
        }
    };
    let kind = if existing.is_some() { EventKind::WarriorsRaised } else { EventKind::CampRaised };
    let tile = w.units[&id].home;
    w.emit(Event::new(kind, tile).civ(civ).unit(id));
    let text = if existing.is_some() {
        format!("raised {k:.0} more warriors: {:.0} in its camp", w.units[&id].people)
    } else {
        format!("raised at {} with {k:.0} warriors from its clans", w.at(tile))
    };
    w.note(id, text);
    id
}

/// Founding's camp: `UNIT.start` warriors from the civ's biggest clans, one each (round again when
/// there are fewer clans), at the capital, their packs from the store. No clan to give, no camp.
pub fn raise_at_founding(w: &mut World, civ: u32) {
    let k = take_people(w, civ, UNIT.start);
    if k == 0.0 {
        return;
    }
    let pack = take(&mut w.civs.get_mut(&civ).unwrap().store, pack_cap(k));
    join(w, civ, k, pack);
}

/// Takes `n` people from the civ's clans, one from each in turn, biggest first; round again when
/// there are fewer clans. Returns how many were taken.
fn take_people(w: &mut World, civ: u32, n: u32) -> f64 {
    let mut k = 0.0;
    let mut given: Vec<u32> = Vec::new();
    for _ in 0..n {
        let able = givers(w, civ);
        let Some(&clan) = able.iter().find(|c| !given.contains(c)).or(able.first()) else { break };
        if given.len() >= able.len() {
            given.clear();
        }
        given.push(clan);
        w.clans.get_mut(&clan).unwrap().people -= 1.0;
        k += 1.0;
    }
    k
}

/// Founding's scout: `SCOUT.people` people from the civ's clans, at the capital, its pack from the
/// store after the camp's. It is made with or without an army.
pub fn raise_scout_at_founding(w: &mut World, civ: u32) {
    let k = take_people(w, civ, SCOUT.people);
    if k == 0.0 {
        return;
    }
    let pack = take(&mut w.civs.get_mut(&civ).unwrap().store, pack_cap(k));
    let capital = w.civs[&civ].capital;
    let mut actor = Actor::new(capital, k);
    actor.store = pack;
    let id = w.next_id;
    w.next_id += 1;
    let unit = Unit {
        id,
        civ,
        actor,
        banked: 0.0,
        hungry: 0,
        state: UnitState::Scout(ScoutState { route: None, heard: false, blocked: false }),
    };
    w.units.insert(id, unit);
}

/// Raises up to `n` warriors for `civ`: one person from each of the `n` biggest clans able to give,
/// wherever they live, each with its share of the pack from the store. Returns how many.
pub fn raise(w: &mut World, civ: u32, n: u32) -> Result<u32, String> {
    let from: Vec<u32> = givers(w, civ).into_iter().take(n as usize).collect();
    if from.is_empty() {
        return Err("no clan has a person to spare".to_string());
    }
    let k = from.len() as f64;
    if total(&w.civs[&civ].store) + EPS < pack_cap(k) {
        return Err("the store can't fill their packs".to_string());
    }
    let name = name_of(w, civ);
    for id in &from {
        w.clans.get_mut(id).unwrap().people -= 1.0;
        w.note(*id, format!("gave a person to {name}'s warriors"));
    }
    let pack = take(&mut w.civs.get_mut(&civ).unwrap().store, pack_cap(k));
    join(w, civ, k, pack);
    Ok(from.len() as u32)
}

/// Sends `n` of `civ`'s warriors (all with `None`) back to the civ's own clan, with their share of the
/// pack; with no clan to take them they are lost. All of them removes the camp. `starved` picks the
/// event and, if it empties the camp, counts it lost.
pub fn disband(w: &mut World, civ: u32, n: Option<u32>, starved: bool) {
    let Some(id) = camp(w, civ).map(|u| u.id) else { return };
    let people = w.units[&id].people;
    let k = n.map_or(people, |n| f64::from(n).min(people));
    if k <= 0.0 {
        return;
    }
    let tile = w.units[&id].home;
    let amount = total(&w.units[&id].store) * k / people;
    let share = take(&mut w.units.get_mut(&id).unwrap().store, amount);
    let clan = home_clan(w, civ);
    if let Some(c) = clan.and_then(|id| w.clans.get_mut(&id)) {
        c.people += k;
        add(&mut c.store, &share);
    }
    if k >= people {
        w.units.remove(&id);
        if starved {
            w.civs.get_mut(&civ).unwrap().army.lost += 1;
        }
    } else {
        w.units.get_mut(&id).unwrap().people -= k;
    }
    let kind = if starved { EventKind::WarriorsStarved } else { EventKind::WarriorsDisbanded };
    w.emit(Event::new(kind, tile).civ(civ).unit(id).clan(clan));
    let home = clan.map_or_else(|| "they are lost".to_string(), |c| format!("they rejoin clan {c}"));
    let text = if starved {
        format!("lost {k:.0} warrior(s) to hunger; {home}")
    } else {
        format!("sent {k:.0} warrior(s) back; {home}")
    };
    w.note(id, text);
}
/// The civ whose land the camp stands on, if it isn't its own.
fn rival_of(w: &World, u: &Unit) -> bool {
    civs::tile_civ(w, u.home).is_some_and(|c| c != u.civ)
}

/// Strips the camp's tile for up to `labour` of work, taking at most `room` of food into the
/// pack at `rate` a unit of labour. Returns the labour used and the food taken; nothing when the
/// tile is fallow or bare.
fn strip(w: &mut World, id: u32, labour: f64, room: f64, rate: f64) -> (f64, f64) {
    let tile = w.units[&id].home;
    let t = &mut w.tiles[tile];
    let held = total(&t.stock);
    if t.fallow > 0 || held <= 0.0 || labour <= 0.0 || room <= 0.0 {
        return (0.0, 0.0);
    }
    let food = (labour * rate).min(held).min(room);
    if food <= EPS {
        return (0.0, 0.0);
    }
    let got = t.stock.map(|s| food * s / held);
    t.stock = [0.0; N];
    t.fallow = UNIT.fallow;
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
    (food / rate, food)
}

/// Puts up to `labour` on the line for up to `want` of food from the store into the pack. Returns
/// the labour used and the food brought.
fn supply(w: &mut World, id: u32, labour: f64, want: f64) -> (f64, f64) {
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
fn room(u: &Unit) -> f64 {
    (pack_cap(u.people) - total(&u.store)).max(0.0)
}

/// The `units` step, after `civs`: fallow tiles count down, each camp in id order spoils and
/// marches, every camp's warriors spread and hold (`hold`), then each camp and scout, in id order,
/// takes the rest of its turn.
pub fn step(w: &mut World) {
    for t in &mut w.tiles {
        t.fallow = t.fallow.saturating_sub(1);
    }
    let ids: Vec<u32> = w.units.keys().copied().collect();
    let camps: Vec<u32> = ids.iter().copied().filter(|id| w.units[id].kind() == UnitKind::Camp).collect();
    let left: BTreeMap<u32, f64> = camps.iter().map(|&id| (id, camp_march(w, id))).collect();
    let spent = hold(w);
    for id in ids {
        match w.units.get(&id).map(|u| u.kind()) {
            Some(UnitKind::Camp) => {
                let left = left.get(&id).copied().unwrap_or(0.0) - spent.get(&id).copied().unwrap_or(0.0);
                camp_turn(w, id, left.max(0.0));
            }
            Some(UnitKind::Scout) => scout_turn(w, id),
            None => {}
        }
    }
}

/// The civ the player leads, or why there is none.
pub fn leader(w: &World) -> Result<u32, String> {
    if w.phase != Phase::Main {
        return Err("the game has not begun".to_string());
    }
    w.player.ok_or_else(|| "you're watching: you lead no civ".to_string())
}

/// Checks `Raise`.
pub fn check_raise(w: &World, n: u32) -> Result<(), String> {
    let civ = leader(w)?;
    if !w.config.army {
        return Err("armies are off".to_string());
    }
    if n == 0 {
        return Err("raise at least one warrior".to_string());
    }
    let k = givers(w, civ).len().min(n as usize);
    if k == 0 {
        return Err("no clan has a person to spare".to_string());
    }
    if total(&w.civs[&civ].store) + EPS < pack_cap(k as f64) {
        return Err("the store can't fill their packs".to_string());
    }
    Ok(())
}
