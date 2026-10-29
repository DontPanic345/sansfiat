//! The camp: a civ's army is one actor whose people are warriors (`docs/design/supply.md`). It has
//! a pack (its store) it eats from, a tile it stands on and a line to the capital, where the civ's
//! store is. At the end of the turn, on the labour its march left, it forages the tile (pillages a
//! rival's) and resupplies along the line. Warriors are raised from the civ's clans, one person
//! from each, and disbanded to the civ's own clan. The `ai` step keeps each AI camp posted; the
//! `units` step runs every camp.

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Actor, Route};
use crate::civs;
use crate::data::*;
use crate::land::{passable, path, step_cost};
use crate::market::Basket;
use crate::world::{EventKind, Phase, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitKind {
    Camp,
}

impl UnitKind {
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Camp => "camp",
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
    w.units.values().find(|u| u.civ == civ)
}

/// The food `people` warriors' packs hold.
pub fn pack_cap(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

fn total(b: &Basket) -> f64 {
    b.iter().sum()
}

/// Takes up to `amount` of food from `from`, in proportion to what it holds.
fn take(from: &mut Basket, amount: f64) -> Basket {
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
    let mut k = 0.0;
    let mut given: Vec<u32> = Vec::new();
    for _ in 0..UNIT.start {
        let able = givers(w, civ);
        // One from each clan in turn, biggest first; round again when there are fewer clans.
        let Some(&clan) = able.iter().find(|c| !given.contains(c)).or(able.first()) else { break };
        if given.len() >= able.len() {
            given.clear();
        }
        given.push(clan);
        w.clans.get_mut(&clan).unwrap().people -= 1.0;
        k += 1.0;
    }
    if k == 0.0 {
        return;
    }
    let pack = take(&mut w.civs.get_mut(&civ).unwrap().store, pack_cap(k));
    join(w, civ, k, pack);
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

/// The `units` step, after `civs`: fallow tiles count down, then each camp, in id order, takes its turn.
pub fn step(w: &mut World) {
    for t in &mut w.tiles {
        t.fallow = t.fallow.saturating_sub(1);
    }
    let ids: Vec<u32> = w.units.keys().copied().collect();
    for id in ids {
        camp_turn(w, id);
    }
}

/// Where the AI posts `civ`'s camp: up to `UNIT.post` steps along the way from the capital to the
/// nearest rival capital reachable by land, or with none, to its farthest clan, keeping to its
/// own land. The capital when there is nowhere to go.
fn post(w: &World, civ: u32) -> usize {
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
fn leader(w: &World) -> Result<u32, String> {
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
