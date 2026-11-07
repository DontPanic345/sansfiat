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
pub mod fight;
pub mod guard;
mod line;
mod scout;

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::Actor;
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::steps;
use crate::market::{add, take, total, Basket};
use crate::world::World;

pub use ai::{ai, guard_order, post};
pub use camp::{zone_cap, CampState};
pub(crate) use camp::{zone_of, zone_ok};
pub use line::{cut_at, cut_by_rival, cut_off, line, path_between, route, turns};
pub use scout::{route_plan, Plan, ScoutState};

use camp::{camp_march, camp_turn, hold};
use scout::scout_turn;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitKind {
    #[default]
    Camp,
    Scout,
    Guard,
}

impl UnitKind {
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Camp => "camp",
            UnitKind::Scout => "scout",
            UnitKind::Guard => "guard",
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
    /// People lost in fights: its armies', guard's and (build-2) militia's.
    #[serde(default)]
    pub fallen: f64,
    /// Tiles it won by fighting.
    #[serde(default)]
    pub taken: u32,
    /// Its armies that surrendered (build-2).
    #[serde(default)]
    pub surrendered: u32,
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

/// What a unit is, and the state only that kind has. A guard has none: its people and hunger are the
/// unit's (`claim.rs` divides them over the claim).
#[derive(Clone, Debug, PartialEq)]
pub enum UnitState {
    Camp(CampState),
    Scout(ScoutState),
    Guard,
}

/// A unit's JSON: the flat object every unit has always written, the fields in their old order.
/// A camp writes the scout's fields at their defaults, and a scout writes the switches and one-shot
/// orders it shares with the camp, and the camp's other fields as `scout_camp_fields` has them.
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
    #[serde(default)]
    built: bool,
    #[serde(default)]
    stack: [f64; 2],
    #[serde(default)]
    still: u32,
    #[serde(default)]
    bows: bool,
}

impl From<Unit> for UnitFlat {
    fn from(u: Unit) -> Self {
        let kind = u.kind();
        let (camp, scout) = match u.state {
            UnitState::Camp(c) => (c, ScoutState::new()),
            UnitState::Scout(s) => (
                CampState {
                    auto_forage: s.auto_forage,
                    auto_supply: s.auto_supply,
                    resupply: s.resupply,
                    forage: s.forage,
                    pillage: s.pillage,
                    ..scout_camp_fields()
                },
                s,
            ),
            UnitState::Guard => (scout_camp_fields(), ScoutState::new()),
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
            built: scout.built,
            stack: camp.stack,
            still: camp.still,
            bows: camp.bows,
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
                stack: f.stack,
                still: f.still,
                bows: f.bows,
            }),
            UnitKind::Scout => UnitState::Scout(ScoutState {
                route: f.route,
                heard: f.heard,
                blocked: f.blocked,
                built: f.built,
                auto_forage: f.auto_forage,
                auto_supply: f.auto_supply,
                resupply: f.resupply,
                forage: f.forage,
                pillage: f.pillage,
            }),
            UnitKind::Guard => UnitState::Guard,
        };
        Unit { id: f.id, civ: f.civ, actor: f.actor, banked: f.banked, hungry: f.hungry, state }
    }
}

/// The camp fields a scout carries beyond the switches and one-shot orders it has of its own (not
/// `Default`, which would write `auto_forage` false).
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
        stack: [0.0; 2],
        still: 0,
        bows: false,
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
            UnitState::Guard => UnitKind::Guard,
        }
    }

    pub fn camp(&self) -> Option<&CampState> {
        match &self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) | UnitState::Guard => None,
        }
    }

    pub fn camp_mut(&mut self) -> Option<&mut CampState> {
        match &mut self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) | UnitState::Guard => None,
        }
    }

    pub fn scout(&self) -> Option<&ScoutState> {
        match &self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) | UnitState::Guard => None,
        }
    }

    pub fn scout_mut(&mut self) -> Option<&mut ScoutState> {
        match &mut self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) | UnitState::Guard => None,
        }
    }

    /// Takes `k` people (at most all) from the unit, an army's from every kind in proportion. Returns
    /// what came off each kind (a scout's or guard's as warriors). Every loss of an army's people
    /// goes through here or `take_kind`, so its stack sums to its people.
    pub fn take_people(&mut self, k: f64) -> [f64; 2] {
        let people = self.actor.people;
        let k = k.clamp(0.0, people);
        let out = match &mut self.state {
            UnitState::Camp(c) if people > EPS => {
                let out = c.stack.map(|s| s * k / people);
                for (s, o) in c.stack.iter_mut().zip(out) {
                    *s = (*s - o).max(0.0);
                }
                out
            }
            _ => [k, 0.0],
        };
        self.actor.people = (people - k).max(0.0);
        out
    }

    /// Takes `k` people of kind `f` (at most all of it) from an army. Returns how many.
    pub fn take_kind(&mut self, f: Fighter, k: f64) -> f64 {
        let Some(c) = self.camp_mut() else { return self.take_people(k)[0] };
        let k = k.clamp(0.0, c.stack[f.index()]);
        c.stack[f.index()] -= k;
        self.actor.people = (self.actor.people - k).max(0.0);
        k
    }

    /// Adds `k` people of kind `f` (an army's stack; other units' people alone).
    pub fn add_people(&mut self, f: Fighter, k: f64) {
        self.actor.people += k;
        if let Some(c) = self.camp_mut() {
            c.stack[f.index()] += k;
        }
    }

    /// The tiles a camp's warriors hold (none for a scout).
    pub fn held(&self) -> &[usize] {
        self.camp().map_or(&[][..], |c| c.held.as_slice())
    }
}

pub(crate) const EPS: f64 = 1e-9;

/// `civ`'s first army by id, if it has one.
pub fn camp(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Camp)
}

/// `civ`'s scout, if it has one.
pub fn scout(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Scout)
}

/// `civ`'s guard, if it has one.
pub fn guard_of(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Guard)
}

/// `civ`'s guard, if it has one, for changing.
pub(crate) fn guard_of_mut(w: &mut World, civ: u32) -> Option<&mut Unit> {
    w.units.values_mut().find(|u| u.civ == civ && u.kind() == UnitKind::Guard)
}

/// The food `people` warriors' packs hold.
pub fn pack_cap(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

/// The name of `civ`, for a note.
pub(crate) fn name_of(w: &World, civ: u32) -> String {
    w.civs.get(&civ).map_or_else(|| "a rival".to_string(), |c| c.name.clone())
}

/// The clans of `civ` that can give a person, biggest first (ties to the lower id).
pub fn givers(w: &World, civ: u32) -> Vec<u32> {
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

/// Adds `k` people of `fighter` (each with the pack share taken from the store) to `civ`'s army
/// `into`, or to a new army at the capital with none. The people have already been taken from
/// their clans.
fn join(w: &mut World, civ: u32, k: f64, pack: Basket, fighter: Fighter, into: Option<u32>) -> u32 {
    let capital = w.civs[&civ].capital;
    let existing = into.filter(|id| w.units.get(id).is_some_and(|u| u.civ == civ && u.camp().is_some()));
    let id = match existing {
        Some(id) => {
            let u = w.units.get_mut(&id).unwrap();
            u.add_people(fighter, k);
            add(&mut u.store, &pack);
            id
        }
        None => {
            let id = w.next_id;
            w.next_id += 1;
            let mut actor = Actor::new(capital, 0.0);
            actor.store = pack;
            let mut unit = Unit {
                id,
                civ,
                actor,
                banked: 0.0,
                hungry: 0,
                state: UnitState::Camp(CampState::new(capital, [0.0; 2])),
            };
            unit.add_people(fighter, k);
            w.units.insert(id, unit);
            w.civs.get_mut(&civ).unwrap().army.raised += 1;
            id
        }
    };
    let kind = if existing.is_some() { EventKind::WarriorsRaised } else { EventKind::CampRaised };
    let tile = w.units[&id].home;
    w.emit(Event::new(kind, tile).civ(civ).unit(id));
    let name = fighter.rules().name;
    let text = if existing.is_some() {
        format!("raised {k:.0} more {name}: {:.0} in its army", w.units[&id].people)
    } else {
        format!("raised at {} with {k:.0} {name} from its clans", w.at(tile))
    };
    w.note(id, text);
    id
}

/// Makes an empty camp at `civ`'s capital, with no people and no pack, for a staged army to be
/// placed in (`Stage::army`): as `join` makes one, but takes nobody from the clans and emits nothing.
pub(crate) fn make_camp(w: &mut World, civ: u32) -> u32 {
    let capital = w.civs[&civ].capital;
    let id = w.next_id;
    w.next_id += 1;
    let unit = Unit {
        id,
        civ,
        actor: Actor::new(capital, 0.0),
        banked: 0.0,
        hungry: 0,
        state: UnitState::Camp(CampState::new(capital, [0.0; 2])),
    };
    w.units.insert(id, unit);
    w.civs.get_mut(&civ).unwrap().army.raised += 1;
    id
}

/// Takes `n` people from the civ's clans, one from each in turn, biggest first; round again when
/// there are fewer clans. Returns how many were taken.
pub(crate) fn take_people(w: &mut World, civ: u32, n: u32) -> f64 {
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
    make_scout(w, civ, k);
}

/// Makes a scout of `k` people (already taken from their clans) at `civ`'s capital, its pack from
/// the store. Returns its id.
fn make_scout(w: &mut World, civ: u32, k: f64) -> u32 {
    let pack = take(&mut w.civs.get_mut(&civ).unwrap().store, pack_cap(k));
    let capital = w.civs[&civ].capital;
    let mut actor = Actor::new(capital, k);
    actor.store = pack;
    let id = w.next_id;
    w.next_id += 1;
    let unit = Unit { id, civ, actor, banked: 0.0, hungry: 0, state: UnitState::Scout(ScoutState::new()) };
    w.units.insert(id, unit);
    id
}

/// Raises a scout of up to `n` people for `civ`: one person from each of the `n` biggest clans able
/// to give, at the capital, their pack from the store. A civ may have any number of scouts. Returns
/// how many people it has.
pub fn raise_scout(w: &mut World, civ: u32, n: u32) -> Result<u32, String> {
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
        w.note(*id, format!("gave a person to {name}'s scout"));
    }
    let id = make_scout(w, civ, k);
    let tile = w.units[&id].home;
    w.emit(Event::new(EventKind::ScoutRaised, tile).civ(civ).unit(id));
    w.note(id, format!("raised at {} with {k:.0} people from its clans", w.at(tile)));
    Ok(from.len() as u32)
}

/// The clan nearest tile `tile` by steps, of any civ (ties to the lower id).
pub fn nearest_clan(w: &World, tile: usize) -> Option<u32> {
    w.clans.values().min_by_key(|k| (steps(w, tile, k.home), k.id)).map(|k| k.id)
}

/// Moves all of unit `id`'s people and its pack into clan `clan`, and removes the unit. It is no
/// fall: the people are the clan's again.
pub fn give_up(w: &mut World, id: u32, clan: u32) {
    let Some(u) = w.units.remove(&id) else { return };
    if let Some(k) = w.clans.get_mut(&clan) {
        k.people += u.people;
        add(&mut k.store, &u.store);
    }
}

/// Army `id` surrenders to civ `by`, whose land cuts its line (`line::cut_by_rival`): its people and
/// pack join `by`'s clan nearest its tile by steps (ties to the lower id). Counted in its civ's
/// `surrendered`, and noted on both civs and the clan. False, and nothing changes, when `by` has no
/// clan to take them.
pub fn surrender(w: &mut World, id: u32, by: u32) -> bool {
    let Some(u) = w.units.get(&id) else { return false };
    let (civ, tile, people) = (u.civ, u.home, u.people);
    let Some(clan) =
        w.clans.values().filter(|k| k.civ == Some(by)).min_by_key(|k| (steps(w, tile, k.home), k.id)).map(|k| k.id)
    else {
        return false;
    };
    give_up(w, id, clan);
    w.civs.get_mut(&civ).unwrap().army.surrendered += 1;
    claim::divide(w, civ);
    w.emit(Event::new(EventKind::ArmySurrendered, tile).civ(civ).other(by).unit(id).clan(clan).amount(people));
    let (name, taker) = (name_of(w, civ), name_of(w, by));
    let at = w.at(tile);
    w.note(civ, format!("its army at {at} surrendered to {taker}: {people:.1} people went to its clan {clan}"));
    w.note(by, format!("{name}'s army at {at} surrendered: {people:.1} people joined clan {clan}"));
    w.note(clan, format!("took in {people:.1} people of {name}'s army at {at}"));
    true
}

/// Raises up to `n` of `fighter` for `civ`, into army `into` or a new one at the capital: one person
/// from each of the `n` biggest clans able to give, wherever they live, each with its share of the
/// pack from the store. Archers make their bows on the army they join or make: it marches nothing
/// the turn they are raised (`CampState.bows`). Returns how many.
pub fn raise(w: &mut World, civ: u32, n: u32, fighter: Fighter, into: Option<u32>) -> Result<u32, String> {
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
    let id = join(w, civ, k, pack, fighter, into);
    if fighter == Fighter::Archers {
        w.units.get_mut(&id).unwrap().camp_mut().expect("an army").bows = true;
        let tile = w.units[&id].home;
        w.emit(Event::new(EventKind::ArchersRaised, tile).civ(civ).unit(id));
        w.note(id, format!("{k:.0} archers raised: its bows take this turn's labour"));
    }
    Ok(from.len() as u32)
}

/// Sends `n` of camp `id`'s people (all with `None`) back to its civ's own clan, with their share of
/// the pack: of kind `fighter` (all of it with `n` none), else of every kind in proportion. With no
/// clan to take them they are lost. All of them removes the camp. `starved` picks the event and, if
/// it empties the camp, counts it lost.
pub fn disband(w: &mut World, id: u32, n: Option<u32>, starved: bool) {
    disband_kind(w, id, n, None, starved);
}

/// As `disband`, of kind `fighter` when it names one.
pub fn disband_kind(w: &mut World, id: u32, n: Option<u32>, fighter: Option<Fighter>, starved: bool) {
    let Some((civ, was)) = w.units.get(&id).map(|u| (u.civ, u.kind())) else { return };
    let people = w.units[&id].people;
    let of = match (fighter, w.units[&id].camp()) {
        (Some(f), Some(c)) => c.stack[f.index()],
        _ => people,
    };
    let k = n.map_or(of, |n| f64::from(n).min(of));
    if k <= 0.0 {
        return;
    }
    let tile = w.units[&id].home;
    let amount = total(&w.units[&id].store) * k / people;
    let share = take(&mut w.units.get_mut(&id).unwrap().store, amount);
    {
        let u = w.units.get_mut(&id).unwrap();
        match fighter {
            Some(f) => {
                u.take_kind(f, k);
            }
            None => {
                u.take_people(k);
            }
        }
    }
    let clan = home_clan(w, civ);
    if let Some(c) = clan.and_then(|id| w.clans.get_mut(&id)) {
        c.people += k;
        add(&mut c.store, &share);
    }
    if k + EPS >= people {
        w.units.remove(&id);
        if starved && was == UnitKind::Camp {
            w.civs.get_mut(&civ).unwrap().army.lost += 1;
        }
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
    if was == UnitKind::Guard {
        claim::divide(w, civ);
    }
}

/// The civ whose land the camp stands on, if it isn't its own.
pub(crate) fn rival_of(w: &World, u: &Unit) -> bool {
    claim::tile_civ(w, u.home).is_some_and(|c| c != u.civ)
}

/// Strips the unit's tile for up to `labour` of work, taking at most `room` of food into the
/// pack at `rate` a unit of labour. Returns the labour used and the food taken; nothing when the
/// tile is fallow or bare.
fn strip(w: &mut World, id: u32, labour: f64, room: f64, rate: f64) -> (f64, f64) {
    let tile = w.units[&id].home;
    strip_at(w, id, tile, labour, room, rate)
}

/// As `strip`, of tile `tile`. It takes only the food it takes: each stock drops in proportion to
/// what was taken, and the tile rests `ceil(UNIT.fallow × taken / held)` turns.
fn strip_at(w: &mut World, id: u32, tile: usize, labour: f64, room: f64, rate: f64) -> (f64, f64) {
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
/// marches (stopping where a rival has people, as they stood before the marches), every camp's
/// warriors spread and hold (`hold`), the fight resolves every contested tile (`fight::step`), then
/// each unit, in id order, takes the rest of its turn.
pub fn step(w: &mut World) {
    for t in &mut w.tiles {
        t.fallow = t.fallow.saturating_sub(1);
    }
    let ids: Vec<u32> = w.units.keys().copied().collect();
    let camps: Vec<u32> = ids.iter().copied().filter(|id| w.units[id].kind() == UnitKind::Camp).collect();
    let holders = fight::army_holders(w);
    let rivals: BTreeMap<usize, Vec<u32>> = fight::presence(w)
        .into_iter()
        .map(|(t, parts)| (t, parts.iter().filter(|p| p.people > FIGHT.floor).map(|p| p.civ).collect()))
        .collect();
    let left: BTreeMap<u32, f64> = camps.iter().map(|&id| (id, camp_march(w, id, &rivals))).collect();
    let spent = hold(w);
    fight::step(w, &holders);
    for id in ids {
        match w.units.get(&id).map(|u| u.kind()) {
            Some(UnitKind::Camp) => {
                let left = left.get(&id).copied().unwrap_or(0.0) - spent.get(&id).copied().unwrap_or(0.0);
                camp_turn(w, id, left.max(0.0));
            }
            Some(UnitKind::Scout) => scout_turn(w, id),
            Some(UnitKind::Guard) => guard::turn(w, id),
            None => {}
        }
    }
}

/// The civ the player leads, or why there is none.
pub fn leader(w: &World) -> Result<u32, String> {
    if !w.phase.playing() {
        return Err("the game has not begun".to_string());
    }
    let civ = w.player.ok_or_else(|| "you're watching: you lead no civ".to_string())?;
    if w.civs.get(&civ).is_some_and(|c| c.is_out()) {
        return Err("your civ has fallen".to_string());
    }
    Ok(civ)
}
