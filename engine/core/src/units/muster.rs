//! Muster: how an army and a scout are raised from a civ's clans, and sent back. Raising takes one
//! person from each of the biggest clans able to give (`givers`, `take_people`); a warrior disbanded
//! goes to the civ's own clan (`home_clan`); an army given up, or surrendering to a rival's land, goes
//! whole into a clan (`give_up`, `surrender`). `units/mod.rs` keeps the types, the queries and the
//! `units` step, and re-exports these, so callers name them as before.

use super::camp::CampState;
use super::scout::ScoutState;
use super::{name_of, pack_cap, Unit, UnitKind, UnitState, EPS};
use crate::actors::Actor;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::steps;
use crate::market::{add, take, total, Basket};
use crate::world::World;

impl Unit {
    /// Takes `k` people (at most all) from the unit, an army's from every kind in proportion. Returns
    /// what came off each kind (a scout's as warriors). Every loss of an army's people
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
}
