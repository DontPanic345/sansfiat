//! The camp: a civ's army is one actor whose people are warriors (`docs/design/supply.md`). It has
//! a pack (its store) it eats from, a tile it stands on and a line to the capital, where the civ's
//! store is. At the end of the turn, on the labour its march left, it forages the tile (pillages a
//! rival's) and resupplies along the line. Warriors are raised from the civ's clans, one person
//! from each, and disbanded to the civ's own clan. The `ai` step keeps each AI camp posted; the
//! `units` step runs every camp.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Actor, Route};
use crate::civs;
use crate::data::*;
use crate::land::{self, build_trail, cost_map, passable, path, path_by, step_cost, trailable};
use crate::market::Basket;
use crate::world::{EventKind, Phase, World};

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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub civ: u32,
    pub kind: UnitKind,
    /// Its warriors (`people`), the tile it stands on (`home`), its pack (`store`) and its line to
    /// the capital (`routes`: one `Route`, or none when cut off or already there).
    #[serde(flatten)]
    pub actor: Actor,
    /// Where it is going; it moves when the turn ends.
    pub dest: Option<usize>,
    /// Labour per warrior put towards the next step and not yet enough to take it.
    pub banked: f64,
    /// Hungry turns in a row.
    pub hungry: u32,
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
    /// A scout's route: the tile it searches from and the tile it searches for.
    #[serde(default)]
    pub route: Option<(usize, usize)>,
    /// A scout has said what its route costs; it is in `heard` once a route is found.
    #[serde(default)]
    pub heard: bool,
    /// A scout has reported that it is blocked, so says it once.
    #[serde(default)]
    pub blocked: bool,
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
    /// The tile the camp stands on.
    pub fn tile(&self) -> usize {
        self.actor.home
    }
}

const EPS: f64 = 1e-9;

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// `civ`'s camp, if it has one.
pub fn camp(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind == UnitKind::Camp)
}

/// `civ`'s scout, if it has one.
pub fn scout(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind == UnitKind::Scout)
}

/// The food `people` warriors' packs hold.
pub fn pack_cap(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

fn total(b: &Basket) -> f64 {
    b.iter().sum()
}

/// Takes up to `amount` of food from `from`, in proportion to what it holds.
pub(crate) fn take(from: &mut Basket, amount: f64) -> Basket {
    let held = total(from);
    if held <= 0.0 || amount <= 0.0 {
        return [0.0; N];
    }
    let share = (amount / held).min(1.0);
    let out = from.map(|v| v * share);
    for f in 0..N {
        from[f] -= out[f];
    }
    out
}

fn add(to: &mut Basket, b: &Basket) {
    for f in 0..N {
        to[f] += b[f];
    }
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
                kind: UnitKind::Camp,
                actor,
                dest: None,
                banked: 0.0,
                hungry: 0,
                auto_forage: true,
                auto_supply: true,
                resupply: false,
                forage: false,
                pillage: false,
                cut: false,
                foraging: false,
                supplying: false,
                route: None,
                heard: false,
                blocked: false,
            };
            w.units.insert(id, unit);
            w.civs.get_mut(&civ).unwrap().army.raised += 1;
            id
        }
    };
    let kind = if existing.is_some() { EventKind::WarriorsRaised } else { EventKind::CampRaised };
    let tile = w.units[&id].home;
    w.unit_event(kind, tile, civ, id, None);
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
        kind: UnitKind::Scout,
        actor,
        dest: None,
        banked: 0.0,
        hungry: 0,
        auto_forage: true,
        auto_supply: true,
        resupply: false,
        forage: false,
        pillage: false,
        cut: false,
        foraging: false,
        supplying: false,
        route: None,
        heard: false,
        blocked: false,
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
    for id in &from {
        w.clans.get_mut(id).unwrap().people -= 1.0;
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
    w.unit_event(kind, tile, civ, id, clan);
}

/// Whether a camp may walk onto tile `i`: passable land, or a built pass over a mountain.
fn walkable(w: &World, i: usize) -> bool {
    passable(w, i) && (TERRAIN[w.tiles[i].terrain].land || w.tiles[i].trail)
}

/// The cheapest route from `from` to `to` over land and built passes, as `land::path`: trails and
/// river crossings count as `step_cost` has them.
pub fn path_between(w: &World, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
    path(w, from, to, |i| walkable(w, i))
}

/// A camp's route from where it stands to its `dest`, with its cost, for the page.
pub fn route(w: &World, unit: u32) -> Option<(Vec<usize>, f64)> {
    let u = w.units.get(&unit)?;
    path_between(w, u.home, u.dest?)
}

/// The camp's line: the cheapest path to the capital over walkable tiles not on a rival civ's
/// land (the camp's own tile never blocks it), its cost (`walk_cost` of the path cost, made there
/// and back each turn) and the food it carries a turn with all the camp's labour (`people ×
/// UNIT.labour`, at `UNIT.carry` a unit of labour over the cost). At the capital the line is empty,
/// costs nothing and carries without limit. `None` when no such path exists.
pub fn line(w: &World, unit: u32) -> Option<(Vec<usize>, f64, f64)> {
    let u = w.units.get(&unit)?;
    let capital = w.civs.get(&u.civ)?.capital;
    let (tiles, cost) =
        path(w, u.home, capital, |i| walkable(w, i) && civs::tile_civ(w, i).is_none_or(|c| c == u.civ))?;
    let cost = walk_cost(cost, 1.0);
    let carry = if cost > 0.0 { u.people * UNIT.labour * UNIT.carry / cost } else { f64::INFINITY };
    Some((tiles, cost, carry))
}

/// Whether the camp is cut off: no line, or one that can't carry a turn's food for its warriors
/// even with all their labour.
pub fn cut_off(w: &World, unit: u32) -> bool {
    match (line(w, unit), w.units.get(&unit)) {
        (Some((_, _, carry)), Some(u)) => carry + EPS < u.people * UNIT.food,
        _ => true,
    }
}

/// One turn's walking along `path` from `from`: spends `banked + UNIT.labour` (one warrior's) on
/// steps while the next one costs no more than what is left. Returns the steps taken and the
/// labour left. Every warrior walks, so the camp moves at one person's pace whatever its size.
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

/// How many turns it takes to walk `path` from `from` with `banked` already banked, as the
/// `units` step does it; 0 for an empty path.
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

/// One camp's turn: spoil, march, then on the labour left the one-shot orders, auto-forage, auto-supply
/// (or only the line, if the switches are off), and last eat.
fn camp_turn(w: &mut World, id: u32) {
    let (civ, people) = (w.units[&id].civ, w.units[&id].people);
    {
        let u = w.units.get_mut(&id).unwrap();
        u.store = u.store.map(|v| v * (1.0 - CLAN.spoilage));
    }

    // The march. Every warrior walks, so a step costs one warrior's labour and the camp's labour
    // left is that, times the warriors. A march not done leaves nothing.
    let mut left = people * UNIT.labour;
    if let Some((tiles, _)) = route(w, id) {
        let u = &w.units[&id];
        let (taken, rest) = walk(w, u.home, &tiles, u.banked);
        let u = w.units.get_mut(&id).unwrap();
        if taken > 0 {
            u.actor.home = tiles[taken - 1];
            u.actor.territory = vec![u.actor.home];
        }
        if taken == tiles.len() {
            u.dest = None;
            u.banked = 0.0;
            left = rest * people;
        } else {
            u.banked = rest;
            left = 0.0;
        }
    } else {
        let u = w.units.get_mut(&id).unwrap();
        u.dest = None;
        u.banked = 0.0;
    }

    // The line, and whether it can feed the camp.
    let tile = w.units[&id].home;
    let line_now = line(w, id);
    let cut = cut_off(w, id);
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
    if cut && !w.units[&id].cut {
        w.unit_event(EventKind::CampCutOff, tile, civ, id, None);
    }
    w.units.get_mut(&id).unwrap().cut = cut;

    let need = people * UNIT.food;
    let mut foraged = 0.0;
    let mut pillaged = 0.0;
    let mut brought = 0.0;
    let rival = rival_of(w, &w.units[&id]);

    // One-shot orders, on this turn's leftover, with the switches off or on.
    let (once_forage, once_pillage, once_supply) = {
        let u = &w.units[&id];
        (u.forage, u.pillage, u.resupply)
    };
    {
        let u = w.units.get_mut(&id).unwrap();
        u.forage = false;
        u.pillage = false;
        u.resupply = false;
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
    if w.units[&id].auto_forage {
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
    if w.units[&id].auto_supply {
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
    let (was_foraging, was_supplying) = (w.units[&id].foraging, w.units[&id].supplying);
    let (now_foraging, now_supplying) = (foraged + pillaged > 0.0, brought > 0.0);
    if now_foraging && !was_foraging {
        let kind = if pillaged > 0.0 { EventKind::CampPillaged } else { EventKind::CampForaged };
        w.unit_event(kind, tile, civ, id, None);
    }
    if now_supplying && !was_supplying {
        w.unit_event(EventKind::CampResupplied, tile, civ, id, None);
    }
    {
        let u = w.units.get_mut(&id).unwrap();
        u.foraging = now_foraging;
        u.supplying = now_supplying;
    }

    // Eating, from the pack in proportion to what it holds.
    let u = w.units.get_mut(&id).unwrap();
    let eaten = take(&mut u.store, need);
    let fed = total(&eaten) + EPS >= need;
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let hungry = u.hungry;
    if !fed {
        w.civs.get_mut(&civ).unwrap().army.hungry += 1;
    }
    if hungry == 1 {
        w.unit_event(EventKind::CampHungry, tile, civ, id, None);
    }
    if hungry >= UNIT.hungry_turns {
        w.civs.get_mut(&civ).unwrap().army.starved += 1;
        disband(w, civ, Some(1), true);
    }
}

// ---- the scout ----------------------------------------------------------------------------------
//
// The scout searches a route for its civ, as `scout/`'s `routePlan` and `scoutWork` did: it prices
// the route as the civ knows the map (`known`) and as the unrevealed tiles could still be (`possible`),
// reveals what could change the answer, and builds the trails the route needs. Its labour is its
// people's, spent on steps, reveals and trails, after it has fed itself.

/// Whether tile `j` is revealed to `civ`.
fn revealed(w: &World, civ: u32, j: usize) -> bool {
    w.civs[&civ].revealed.contains(&j)
}

/// Whether a mountain can be bridged by some span the map can roll, by `CIV.bridge_span`.
fn mountain_may_bridge() -> bool {
    MAP.mountain_span.iter().any(|&(s, _)| s.is_some_and(|s| s <= CIV.bridge_span))
}

/// What entering tile `j` costs `civ`, ignoring any river: as it is known (`possible` false) or as
/// it could be (`possible` true). A tile with a trail, or revealed, is priced with its trail where
/// one could go (`min` of bare and hidden, a mountain only by its trail). An unrevealed tile is its
/// bare cost as known; as possible, its cheapest hidden trail if it could hide one. `None` is
/// impassable.
fn tile_cost(w: &World, civ: u32, j: usize, possible: bool) -> Option<f64> {
    let t = &w.tiles[j];
    let bare = TERRAIN[t.terrain].move_cost;
    if t.trail || revealed(w, civ, j) {
        let hidden = if t.trail || trailable(w, j) { t.trail_cost } else { None };
        return match hidden {
            Some(h) => Some(bare.map_or(h, |c| c.min(h))),
            None => bare,
        };
    }
    if !possible {
        return bare;
    }
    let hidden = match t.terrain {
        FOREST | HILLS => {
            let pairs = if t.terrain == FOREST { MAP.forest_cost } else { MAP.hills_cost };
            Some(pairs.iter().map(|p| f64::from(p.0)).fold(f64::INFINITY, f64::min))
        }
        MOUNTAIN if mountain_may_bridge() => Some(MAP.mountain_cost as f64),
        _ => None,
    };
    match hidden {
        Some(h) => Some(bare.map_or(h, |c| c.min(h))),
        None => bare,
    }
}

/// Whether tile `j` could be trailed as `civ` knows it.
fn may_trail(w: &World, civ: u32, j: usize) -> bool {
    let t = &w.tiles[j];
    if t.trail || revealed(w, civ, j) {
        return t.trail || trailable(w, j);
    }
    match t.terrain {
        FOREST | HILLS => true,
        MOUNTAIN => mountain_may_bridge(),
        _ => false,
    }
}

/// The step into `j` as `civ` prices it (`tile_cost` plus a river crossing, which a trail bridges:
/// known keeps it until a trail is built, possible drops it where one could go).
fn step_as(w: &World, civ: u32, from: usize, to: usize, possible: bool) -> Option<f64> {
    let t = &w.tiles[to];
    let along = w.tiles[from].flow == Some(to) || t.flow == Some(from);
    let bridged = if possible { may_trail(w, civ, to) } else { t.trail };
    let cross = if t.river > 0 && !bridged && !along { RIVER.cross } else { 0.0 };
    Some(tile_cost(w, civ, to, possible)? + cross)
}

/// Whether a trail on revealed tile `j` would cut what entering it costs: it is trailable, not
/// built, and its hidden cost is less than its bare one (or it bridges a river).
fn needs_trail(w: &World, civ: u32, j: usize) -> bool {
    let t = &w.tiles[j];
    if t.trail || !revealed(w, civ, j) || !trailable(w, j) || !TRAIL.work.iter().any(|&(k, _)| k == t.terrain) {
        return false;
    }
    let cut = TERRAIN[t.terrain].move_cost.is_none_or(|c| t.trail_cost.is_some_and(|h| h < c));
    cut || t.river > 0
}

/// The labour a trail on tile `j` still needs, or `None` where none can be built.
fn trail_left(w: &World, j: usize) -> Option<f64> {
    let t = &w.tiles[j];
    TRAIL.work.iter().find(|&&(k, _)| k == t.terrain).map(|&(_, need)| (need - t.trail_work).max(0.0))
}

/// What a route plan says: `known` and `possible` are the cheapest paths from `from` to `to` (tiles
/// after `from`, and cost), and `candidates` are the unrevealed tiles on the possible path whose
/// known and possible costs differ. None when the known path is already as cheap as the possible one.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub known: Option<(Vec<usize>, f64)>,
    pub possible: Option<(Vec<usize>, f64)>,
    pub candidates: Vec<usize>,
}

/// The route plan for `civ` from `from` to `to`: see `Plan`.
pub fn route_plan(w: &World, civ: u32, from: usize, to: usize) -> Plan {
    let known = path_by(w, from, to, |i, j| step_as(w, civ, i, j, false));
    let possible = path_by(w, from, to, |i, j| step_as(w, civ, i, j, true));
    let mut candidates = Vec::new();
    if let Some((pp, pc)) = &possible {
        let kc = known.as_ref().map_or(f64::INFINITY, |k| k.1);
        if kc > pc + EPS {
            let mut prev = from;
            for &j in pp {
                let differ = step_as(w, civ, prev, j, false) != step_as(w, civ, prev, j, true);
                if differ && !revealed(w, civ, j) && !w.tiles[j].trail {
                    candidates.push(j);
                }
                prev = j;
            }
        }
    }
    Plan { known, possible, candidates }
}

/// The labour a scout's walk costs, from `home`: `cost_map` over walkable tiles.
fn walk_costs(w: &World, home: usize) -> BTreeMap<usize, f64> {
    cost_map(w, &[home], f64::INFINITY, |i| walkable(w, i))
}

/// The cheapest tile to stand on to work on tile `j` (`j` itself or a neighbour) that the walk
/// reaches: its walk cost and the tile.
fn beside(w: &World, costs: &BTreeMap<usize, f64>, j: usize) -> Option<(f64, usize)> {
    std::iter::once(j)
        .chain(land::neighbours(w, j))
        .filter_map(|n| costs.get(&n).map(|&c| (c, n)))
        .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
}

/// The walk to a tile beside `j` from `home`: empty when it stands there already, `None` when no
/// walk reaches one.
fn approach(w: &World, costs: &BTreeMap<usize, f64>, home: usize, j: usize) -> Option<Vec<usize>> {
    if steps(w, home, j) <= 1 {
        return Some(Vec::new());
    }
    let (_, n) = beside(w, costs, j)?;
    if n == home {
        return Some(Vec::new());
    }
    path_between(w, home, n).map(|(p, _)| p)
}

/// The tiles a scout works on next, in order: mountains first (the ones that decide whether a
/// route exists), then the nearest to the scout, then the biggest saving. A mountain candidate
/// stands for its range: the unrevealed mountains joined to it whose cheapest possible route
/// through them beats the known one. So the search sweeps along a range from where it stands.
fn ordered(w: &World, civ: u32, plan: &Plan, from: usize, to: usize, costs: &BTreeMap<usize, f64>) -> Vec<usize> {
    let known_cost = plan.known.as_ref().map_or(f64::INFINITY, |k| k.1);
    let mut set: BTreeSet<usize> = plan.candidates.iter().copied().collect();
    let through = |m: usize| {
        let a = path_by(w, from, m, |i, j| step_as(w, civ, i, j, true));
        let b = path_by(w, m, to, |i, j| step_as(w, civ, i, j, true));
        match (a, b) {
            (Some((_, x)), Some((_, y))) => x + y,
            _ => f64::INFINITY,
        }
    };
    for &c in plan.candidates.iter().filter(|&&c| w.tiles[c].terrain == MOUNTAIN) {
        let mut range = BTreeSet::new();
        let mut stack = vec![c];
        while let Some(m) = stack.pop() {
            if !range.insert(m) {
                continue;
            }
            for n in land::neighbours(w, m) {
                if w.tiles[n].terrain == MOUNTAIN && !revealed(w, civ, n) && !range.contains(&n) {
                    stack.push(n);
                }
            }
        }
        set.extend(range.into_iter().filter(|&m| through(m) < known_cost - EPS));
    }
    let mut keyed: Vec<(u8, f64, f64, usize)> = set
        .into_iter()
        .map(|j| {
            let reach = beside(w, costs, j).map_or(f64::INFINITY, |(c, _)| c);
            let tier = u8::from(w.tiles[j].terrain != MOUNTAIN);
            let saving = match tile_cost(w, civ, j, false) {
                Some(k) => k - tile_cost(w, civ, j, true).unwrap_or(k),
                None => f64::INFINITY,
            };
            (tier, reach, saving, j)
        })
        .collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(b.2.total_cmp(&a.2)).then(a.3.cmp(&b.3)));
    keyed.into_iter().map(|k| k.3).collect()
}

/// Puts a scout's walk on `path` into its labour `left`: steps taken while each costs no more than
/// what is left. A step part-paid carries to next turn as `banked`. Returns whether it reached the
/// end of the path.
fn march(w: &mut World, id: u32, path: &[usize], left: &mut f64, banked: &mut f64) -> bool {
    let mut at = w.units[&id].home;
    let mut taken = 0;
    for &next in path {
        let Some(cost) = step_cost(w, at, next) else { break };
        if cost > *left + EPS {
            break;
        }
        *left -= cost;
        at = next;
        taken += 1;
        w.tiles[next].walked = true;
    }
    let u = w.units.get_mut(&id).unwrap();
    if taken > 0 {
        u.actor.home = at;
        u.actor.territory = vec![at];
    }
    let done = taken == path.len();
    if done {
        *banked = 0.0;
    } else {
        *banked = *left;
        *left = 0.0;
    }
    done
}

/// Logs a scout's event about its tile, with a route cost where it has one.
fn scout_event(w: &mut World, kind: EventKind, tile: usize, civ: u32, id: u32, cost: Option<f64>) {
    w.unit_event(kind, tile, civ, id, None);
    if let Some(e) = w.events.last_mut() {
        e.cost = cost;
    }
}

/// Clears a scout's route and what it has reported about it.
fn clear_route(w: &mut World, id: u32) {
    if let Some(u) = w.units.get_mut(&id) {
        u.route = None;
        u.heard = false;
        u.blocked = false;
    }
}

/// Reports a scout is blocked, once until it moves on.
fn blocked(w: &mut World, id: u32, civ: u32) {
    let u = &w.units[&id];
    if u.blocked {
        return;
    }
    let tile = u.home;
    w.units.get_mut(&id).unwrap().blocked = true;
    scout_event(w, EventKind::ScoutBlocked, tile, civ, id, None);
}

/// Spends the scout's labour `left` on its route, as `scout/`'s `scoutWork`: it walks to the first
/// reveal it can reach, reveals it, and builds the trails its route needs; a mountain takes a whole
/// turn's search labour, begun beside it (`start` is what the search had this turn). Returns the
/// labour a part-paid step carries to next turn.
fn search(w: &mut World, id: u32, left: &mut f64, start: f64) -> f64 {
    let civ = w.units[&id].civ;
    let mut banked = 0.0;
    let mut guard = 0;
    while *left > EPS && guard < 100_000 {
        guard += 1;
        let Some((from, to)) = w.units[&id].route else { break };
        let home = w.units[&id].home;
        let plan = route_plan(w, civ, from, to);
        if plan.possible.is_none() {
            let tile = home;
            clear_route(w, id);
            scout_event(w, EventKind::ScoutNoRoute, tile, civ, id, None);
            break;
        }
        let costs = walk_costs(w, home);
        let order = ordered(w, civ, &plan, from, to, &costs);
        let reach = order.iter().find_map(|&j| approach(w, &costs, home, j).map(|p| (j, p)));
        if let Some((j, path)) = reach {
            if !path.is_empty() {
                march(w, id, &path, left, &mut banked);
                w.units.get_mut(&id).unwrap().blocked = false;
                continue;
            }
            if w.tiles[j].terrain == MOUNTAIN {
                // A mountain needs the whole turn's search labour, begun beside it.
                if *left + EPS < start {
                    break;
                }
                reveal(w, id, civ, j);
                *left = 0.0;
            } else {
                if *left + EPS < SCOUT.reveal {
                    break;
                }
                *left -= SCOUT.reveal;
                reveal(w, id, civ, j);
            }
            continue;
        }
        // No reveal can be reached. Once the candidates are none, the known route is settled and
        // its trails are built; while candidates remain unreached, the first reachable trail on the
        // possible path is built instead (a pass that opens the land beyond).
        let known_cost = plan.known.as_ref().map(|k| k.1);
        let no_reveal = !plan.candidates.is_empty();
        let trail_on: &[usize] = match (&plan.known, &plan.possible) {
            (Some((p, _)), _) if !no_reveal => p,
            (_, Some((p, _))) if no_reveal => p,
            _ => &[],
        };
        let needs: Vec<usize> = {
            let mut v: Vec<usize> = trail_on.iter().copied().filter(|&j| needs_trail(w, civ, j)).collect();
            v.sort_by_key(|&j| (steps(w, home, j), j));
            v
        };
        if !no_reveal {
            let pcost = plan.possible.as_ref().map_or(f64::INFINITY, |p| p.1);
            if known_cost.is_some_and(|k| k <= pcost + EPS) && !w.units[&id].heard && !needs.is_empty() {
                let cost = known_cost.unwrap_or(0.0);
                w.units.get_mut(&id).unwrap().heard = true;
                scout_event(w, EventKind::ScoutRouteFound, home, civ, id, Some(cost));
            }
        }
        let trail = needs.iter().find_map(|&j| approach(w, &costs, home, j).map(|p| (j, p)));
        let Some((j, path)) = trail else {
            match known_cost {
                Some(cost) if !no_reveal && needs.is_empty() => {
                    clear_route(w, id);
                    scout_event(w, EventKind::ScoutRouteFound, home, civ, id, Some(cost));
                }
                _ => blocked(w, id, civ),
            }
            break;
        };
        if !path.is_empty() {
            march(w, id, &path, left, &mut banked);
            w.units.get_mut(&id).unwrap().blocked = false;
            continue;
        }
        let Some(need) = trail_left(w, j) else { break };
        let used = need.min(*left);
        if used <= EPS {
            break;
        }
        *left -= used;
        if build_trail(w, j, used) {
            scout_event(w, EventKind::ScoutTrailBuilt, j, civ, id, None);
        }
        w.units.get_mut(&id).unwrap().blocked = false;
    }
    banked
}

/// Reveals tile `j` for `civ`; a mountain that can be bridged is a pass found.
fn reveal(w: &mut World, id: u32, civ: u32, j: usize) {
    w.civs.get_mut(&civ).unwrap().revealed.insert(j);
    w.units.get_mut(&id).unwrap().blocked = false;
    if w.tiles[j].terrain == MOUNTAIN && trailable(w, j) {
        scout_event(w, EventKind::ScoutPassFound, j, civ, id, None);
    }
}

/// One scout's turn: it spoils, feeds itself (forage only on a route and off rival land, then the
/// line for what is still short), searches on the labour left, and eats. Its people go hungry as a
/// camp's do; a scout that starves to nothing is gone.
fn scout_turn(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    {
        let u = w.units.get_mut(&id).unwrap();
        u.store = u.store.map(|v| v * (1.0 - CLAN.spoilage));
    }
    let people = w.units[&id].people;
    let need = people * UNIT.food;
    let mut left = people * UNIT.labour + w.units[&id].banked;
    let mut banked = 0.0;
    w.units.get_mut(&id).unwrap().banked = 0.0;

    // Without a route it walks home to the capital, to be fed there rather than starve out in the
    // country (where it forages nothing).
    if w.units[&id].route.is_none() {
        let (home, capital) = (w.units[&id].home, w.civs[&civ].capital);
        if let Some((p, _)) = path_between(w, home, capital) {
            if !p.is_empty() {
                march(w, id, &p, &mut left, &mut banked);
            }
        }
    }

    // Feeds first: foraging only while it has a route, and never on rival land.
    if w.units[&id].route.is_some() && !rival_of(w, &w.units[&id]) {
        let short = (need - total(&w.units[&id].store)).max(0.0);
        let (used, _) = strip(w, id, left, short.min(room(&w.units[&id])), UNIT.forage);
        left -= used;
    }
    let short = (need - total(&w.units[&id].store)).max(0.0);
    let (used, _) = supply(w, id, left, short.min(room(&w.units[&id])));
    left -= used;

    // Searches on what labour is left.
    let start = left;
    if w.units[&id].route.is_some() {
        banked = search(w, id, &mut left, start);
    }
    w.units.get_mut(&id).unwrap().banked = banked;

    // The line, for the page.
    let routes = match line(w, id) {
        Some((p, cost, _)) if !p.is_empty() => vec![Route { to: w.civs[&civ].capital, path: p, cost }],
        _ => Vec::new(),
    };
    w.units.get_mut(&id).unwrap().actor.routes = routes;

    // Eating, and hunger.
    let u = w.units.get_mut(&id).unwrap();
    let eaten = take(&mut u.store, need);
    let fed = total(&eaten) + EPS >= need;
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let (hungry, tile) = (u.hungry, u.home);
    if hungry == 1 {
        scout_event(w, EventKind::ScoutHungry, tile, civ, id, None);
    }
    if hungry >= UNIT.hungry_turns {
        let u = w.units.get_mut(&id).unwrap();
        u.people -= 1.0;
        scout_event(w, EventKind::ScoutStarved, tile, civ, id, None);
        if w.units[&id].people <= EPS {
            w.units.remove(&id);
        }
    }
}

/// Checks `SetRoute`: the player leads a civ with a scout, and both tiles are different land or
/// mountain tiles on the map.
pub fn check_route(w: &World, from: usize, to: usize) -> Result<(), String> {
    let civ = leader(w)?;
    if scout(w, civ).is_none() {
        return Err("you have no scout".to_string());
    }
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

/// Checks `ClearRoute`: the player leads a civ with a scout.
pub fn check_scout(w: &World) -> Result<(), String> {
    let civ = leader(w)?;
    scout(w, civ).map(|_| ()).ok_or_else(|| "you have no scout".to_string())
}

/// Applies a route order to the player's scout: a new route replaces the old one.
pub fn order_route(w: &mut World, route: Option<(usize, usize)>) {
    let Some(civ) = w.player else { return };
    let Some(id) = scout(w, civ).map(|u| u.id) else { return };
    let u = w.units.get_mut(&id).unwrap();
    u.route = route;
    u.heard = false;
    u.blocked = false;
}

/// The `units` step, after `civs`: fallow tiles count down, then each camp and scout, in id order, takes its turn.
pub fn step(w: &mut World) {
    for t in &mut w.tiles {
        t.fallow = t.fallow.saturating_sub(1);
    }
    let ids: Vec<u32> = w.units.keys().copied().collect();
    for id in ids {
        match w.units.get(&id).map(|u| u.kind) {
            Some(UnitKind::Camp) => camp_turn(w, id),
            Some(UnitKind::Scout) => scout_turn(w, id),
            None => {}
        }
    }
}

/// Where the AI posts `civ`'s camp: up to `UNIT.post` steps along the way from the capital to the
/// nearest rival capital reachable by land, or with none, to its farthest clan, keeping to its
/// own land. The capital when there is nowhere to go.
pub fn post(w: &World, civ: u32) -> usize {
    let c = &w.civs[&civ];
    let to_rival = w
        .civs
        .values()
        .filter(|o| o.id != civ)
        .filter_map(|o| path_between(w, c.capital, o.capital).map(|(p, _)| ((steps(w, c.capital, o.capital), o.id), p)))
        .min_by_key(|(k, _)| *k)
        .map(|(_, p)| p);
    let way = to_rival.or_else(|| {
        let far = w
            .clans
            .values()
            .filter(|k| k.civ == Some(civ))
            .max_by_key(|k| (steps(w, c.capital, k.home), std::cmp::Reverse(k.id)))?;
        path_between(w, c.capital, far.home).map(|(p, _)| p)
    });
    let mut at = c.capital;
    for t in way.unwrap_or_default().into_iter().take(UNIT.post) {
        if civs::tile_civ(w, t) != Some(civ) {
            break;
        }
        at = t;
    }
    at
}

/// The `ai` step, before `units`: each civ the player doesn't lead raises its camp if it has none
/// and can, and sends it to its post if it is idle somewhere else. Draws no random numbers.
pub fn ai(w: &mut World) {
    if w.phase != Phase::Main || !w.config.army {
        return;
    }
    let ids: Vec<u32> = w.civs.keys().copied().filter(|&c| Some(c) != w.player).collect();
    for civ in ids {
        if camp(w, civ).is_none() {
            let _ = raise(w, civ, UNIT.start);
        }
        let Some(u) = camp(w, civ) else { continue };
        if u.dest.is_some() {
            continue;
        }
        let (id, here) = (u.id, u.home);
        let at = post(w, civ);
        if at != here {
            order_move(w, id, at);
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

/// The player's camp, or why there is none.
fn own(w: &World) -> Result<&Unit, String> {
    let civ = leader(w)?;
    camp(w, civ).ok_or_else(|| "you have no camp".to_string())
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

/// Applies `MoveCamp`: a new destination and nothing banked; its own tile clears the order.
pub fn order_move(w: &mut World, unit: u32, to: usize) {
    if let Some(u) = w.units.get_mut(&unit) {
        u.dest = if u.home == to { None } else { Some(to) };
        u.banked = 0.0;
    }
}

/// Applies the player's order to the camp, through `f`.
pub fn with_camp(w: &mut World, f: impl FnOnce(&mut Unit)) {
    if let Some(civ) = w.player {
        if let Some(id) = camp(w, civ).map(|u| u.id) {
            f(w.units.get_mut(&id).unwrap());
        }
    }
}
