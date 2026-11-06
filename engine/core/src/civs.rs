//! Civs: what founding makes of the pre-game's centres. Each centre's site market becomes a civ
//! with a capital on it, and the centre's clan its own clan; clans in reach of the market join the
//! civ; the civ keeps one store, which its fed clans levy into and which spoils and is capped.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Actor, Route};
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{cost_map, is_hearth, passable, path, steps};
use crate::market::{add, take, total, Basket};
use crate::markets::reach;
use crate::units::{self, Counters};
use crate::world::{Phase, World};

pub use crate::claim::tile_civ;

/// A civ. Its store is its actor's (`Deref` to `Actor`), so it spoils as every actor does; its
/// JSON is one flat object (see `CivFlat`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "CivFlat", into = "CivFlat")]
pub struct Civ {
    pub id: u32,
    pub name: String,
    pub color: String,
    /// The founding market; it never fades.
    pub market: u32,
    /// The market's tile: the capital city.
    pub capital: usize,
    /// The turn it was founded.
    pub founded: u32,
    /// The civ as an actor: its store, with home the capital.
    pub actor: Actor,
    /// Goods cut by the cap last turn.
    pub lost: f64,
    /// The civ's own clan: its home is the capital and stays there. If it goes, the civ's biggest
    /// clan takes its place.
    pub clan: u32,
    /// What its army has done, kept here so it outlives any one camp.
    pub army: Counters,
    /// The food the player has set it to keep; `None` is the default (see `keep`).
    pub keep: Option<f64>,
    /// The most food it gives a turn to unaligned clans at its market.
    pub gift: f64,
    /// Food it posted to its market's clearing last turn.
    pub posted: f64,
    /// Food it gave last turn.
    pub given: f64,
    /// Clans that have joined it since founding.
    pub joined: u32,
    /// Food it has given since founding.
    pub gifted: f64,
    /// The tiles the civ has revealed: their hidden trail costs are known to it.
    pub revealed: BTreeSet<usize>,
    /// The land the civ claims, sorted, with its capital (`claim.rs`); a claim may be paper.
    pub claim: Vec<usize>,
    /// Each claimed tile's guard share, aligned with `claim`: what its guard and its camps' hold on it.
    pub guard: Vec<f64>,
    /// A rival army stands on its capital this turn: its guard is drawn back there (`fight.rs`).
    pub pressed: bool,
    /// Every walkable neighbour of its capital is held or stood on by a rival army: its own clans
    /// there harvest nothing and no levy reaches its store (`fight.rs`).
    pub besieged: bool,
    /// The turn its capital fell, and the civ that took it. A fallen civ has no land, no Standing,
    /// no turn and no orders (`fall`).
    pub fallen: Option<(u32, u32)>,
    /// The food the market levy has paid into its store this turn: `trade` zeroes it at the turn's
    /// start and `levy` adds to it. Derived, not saved: `CivFlat` leaves it out and a loaded civ
    /// has 0. A replay recomputes it, so a live world equals its replay, but a live world does not
    /// equal a loaded snapshot of it. Read by the judge's economy group.
    pub levied: f64,
}

impl Civ {
    /// Whether the civ has fallen: its capital was taken with no part of it left there.
    pub fn is_out(&self) -> bool {
        self.fallen.is_some()
    }
}

/// The civs that have not fallen, in id order.
pub fn alive(w: &World) -> impl Iterator<Item = &Civ> {
    w.civs.values().filter(|c| !c.is_out())
}

impl Deref for Civ {
    type Target = Actor;
    fn deref(&self) -> &Actor {
        &self.actor
    }
}

impl DerefMut for Civ {
    fn deref_mut(&mut self) -> &mut Actor {
        &mut self.actor
    }
}

/// A civ's JSON: the fields a civ has always written, in that order. `store` is the actor's, and
/// the civ's home is its `capital`, so the actor is built from those two when the civ is read.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CivFlat {
    id: u32,
    name: String,
    color: String,
    market: u32,
    capital: usize,
    founded: u32,
    store: Basket,
    lost: f64,
    #[serde(default)]
    clan: u32,
    #[serde(default)]
    army: Counters,
    #[serde(default)]
    keep: Option<f64>,
    #[serde(default)]
    gift: f64,
    #[serde(default)]
    posted: f64,
    #[serde(default)]
    given: f64,
    #[serde(default)]
    joined: u32,
    #[serde(default)]
    gifted: f64,
    #[serde(default)]
    revealed: BTreeSet<usize>,
    #[serde(default)]
    claim: Vec<usize>,
    #[serde(default)]
    guard: Vec<f64>,
    #[serde(default)]
    pressed: bool,
    #[serde(default)]
    besieged: bool,
    #[serde(default)]
    fallen: Option<(u32, u32)>,
}

impl From<Civ> for CivFlat {
    fn from(c: Civ) -> Self {
        CivFlat {
            id: c.id,
            name: c.name,
            color: c.color,
            market: c.market,
            capital: c.capital,
            founded: c.founded,
            store: c.actor.store,
            lost: c.lost,
            clan: c.clan,
            army: c.army,
            keep: c.keep,
            gift: c.gift,
            posted: c.posted,
            given: c.given,
            joined: c.joined,
            gifted: c.gifted,
            revealed: c.revealed,
            claim: c.claim,
            guard: c.guard,
            pressed: c.pressed,
            besieged: c.besieged,
            fallen: c.fallen,
        }
    }
}

impl From<CivFlat> for Civ {
    fn from(f: CivFlat) -> Self {
        Civ {
            id: f.id,
            name: f.name,
            color: f.color,
            market: f.market,
            capital: f.capital,
            founded: f.founded,
            actor: Actor { store: f.store, ..Actor::new(f.capital, 0.0) },
            lost: f.lost,
            clan: f.clan,
            army: f.army,
            keep: f.keep,
            gift: f.gift,
            posted: f.posted,
            given: f.given,
            joined: f.joined,
            gifted: f.gifted,
            revealed: f.revealed,
            claim: f.claim,
            guard: f.guard,
            pressed: f.pressed,
            besieged: f.besieged,
            fallen: f.fallen,
            levied: 0.0,
        }
    }
}

/// At founding, each clan going to a festival that isn't a capital's switches to the capital
/// market whose path from its home costs no more than its festival's (the cheapest, ties to the
/// lower id), and takes that path as its route, so the new markets start with the clans around
/// them; each market's traders are counted again for it. After that the clans choose as ever.
fn switch_to_capitals(w: &mut World) {
    let capitals: Vec<(u32, usize)> = w.civs.values().map(|c| (c.market, c.capital)).collect();
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = &w.clans[&id];
        let Some(festival) = c.market.filter(|m| !capitals.iter().any(|&(cm, _)| cm == *m)) else { continue };
        let near = cost_map(w, &[c.home], MARKET.reach_walk / 2.0, |i| passable(w, i));
        let Some(&cost) = w.markets.get(&festival).and_then(|m| near.get(&m.tile)) else { continue };
        let best = capitals
            .iter()
            .filter_map(|&(mid, tile)| near.get(&tile).filter(|&&k| k <= cost).map(|&k| (k, mid)))
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let Some((_, mid)) = best else { continue };
        let to = w.markets[&mid].tile;
        let Some((path, cost)) = path(w, c.home, to, |i| passable(w, i)) else { continue };
        let c = w.clans.get_mut(&id).unwrap();
        c.market = Some(mid);
        c.routes = vec![Route { to, path, cost: walk_cost(cost, 1.0) }];
    }
    // Last turn's traders, as the clans now stand, so the next choice sees who switched.
    for m in w.markets.values_mut() {
        m.traders = w.clans.values().filter(|c| c.market == Some(m.id)).count();
    }
}

/// Founding, once, as the pre-game ends: a civ on each centre's site market (the k-th takes
/// `CIVS[k]`), clans in reach joining the nearest, a civ with no clan taking the nearest unaligned
/// one, each civ's own clan, and the phase `Choose`. Draws no random numbers.
pub fn found(w: &mut World) {
    let (centres, centre_clans) = w.end.as_ref().map(|e| (e.centres.clone(), e.clans.clone())).unwrap_or_default();
    // Each civ and its centre's clan, if the site had one.
    let mut owns: Vec<(u32, Option<u32>)> = Vec::new();
    for (k, mid) in centres.into_iter().enumerate() {
        let (Some(&(name, color)), Some(m)) = (CIVS.get(k), w.markets.get(&mid)) else { continue };
        let capital = m.tile;
        let id = w.next_id;
        w.next_id += 1;
        let civ = Civ {
            id,
            name: name.to_string(),
            color: color.to_string(),
            market: mid,
            capital,
            founded: w.turn,
            actor: Actor { store: CIV.endowment, ..Actor::new(capital, 0.0) },
            lost: 0.0,
            clan: 0,
            army: Counters::default(),
            revealed: BTreeSet::new(),
            keep: None,
            gift: LEANING.gift_default,
            posted: 0.0,
            given: 0.0,
            joined: 0,
            gifted: 0.0,
            claim: vec![capital],
            guard: vec![0.0],
            pressed: false,
            besieged: false,
            fallen: None,
            levied: 0.0,
        };
        w.civs.insert(id, civ);
        w.emit(Event::new(EventKind::CivFounded, capital).civ(id).market(mid));
        owns.push((id, centre_clans.get(k).copied().flatten().filter(|c| w.clans.contains_key(c))));
    }

    let reaches: Vec<(u32, BTreeMap<usize, f64>)> = w.civs.values().map(|c| (c.id, reach(w, c.capital))).collect();
    let joins: Vec<(u32, u32)> = w
        .clans
        .values()
        .filter_map(|c| {
            let mut best: Option<(u32, f64)> = None;
            for (id, r) in &reaches {
                let cost = c.territory.iter().filter_map(|t| r.get(t)).copied().min_by(f64::total_cmp);
                // Civs come in id order, so only a strictly cheaper one replaces.
                if let Some(cost) = cost {
                    if best.is_none_or(|(_, b)| cost < b) {
                        best = Some((*id, cost));
                    }
                }
            }
            best.map(|(id, _)| (c.id, id))
        })
        .collect();
    for &(clan, civ) in &joins {
        let k = w.clans.get_mut(&clan).unwrap();
        k.civ = Some(civ);
        k.leaning = BTreeMap::from([(civ, LEANING.join)]);
        let name = w.civs[&civ].name.clone();
        w.note(clan, format!("joined {name} at founding: in reach of its capital"));
    }
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        let n = joins.iter().filter(|j| j.1 == id).count();
        let at = w.at(w.civs[&id].capital);
        w.note(id, format!("founded at {at} with {n} clan(s) in reach"));
    }

    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        if w.clans.values().any(|c| c.civ == Some(id)) {
            continue;
        }
        let capital = w.civs[&id].capital;
        let nearest = w
            .clans
            .values()
            .filter(|c| c.civ.is_none())
            .min_by_key(|c| (steps(w, c.home, capital), c.id))
            .map(|c| c.id);
        if let Some(clan) = nearest {
            let k = w.clans.get_mut(&clan).unwrap();
            k.civ = Some(id);
            k.leaning = BTreeMap::from([(id, LEANING.join)]);
        }
    }
    // Each civ's own clan: its centre's clan, whose home is the capital; for a site the top-up
    // found without one, the biggest that joined with its home in reach of the capital (ties to
    // the lower id), else its biggest. Nothing moves: the clan's festival stays where it was.
    for (id, own) in owns {
        if let Some(clan) = own {
            let k = w.clans.get_mut(&clan).unwrap();
            k.civ = Some(id);
            k.leaning = BTreeMap::from([(id, LEANING.join)]);
            w.civs.get_mut(&id).unwrap().clan = clan;
            w.note(clan, "is its civ's own clan: its home is the capital".to_string());
            continue;
        }
        let near = reach(w, w.civs[&id].capital);
        let joined = |near_only: bool| {
            w.clans
                .values()
                .filter(|c| c.civ == Some(id) && (!near_only || near.contains_key(&c.home)))
                .min_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)))
                .map(|c| c.id)
        };
        let Some(clan) = joined(true).or_else(|| joined(false)) else { continue };
        w.civs.get_mut(&id).unwrap().clan = clan;
    }
    // Civ clans are drawn to their capitals now, so routes are laid again; then the clans at least
    // as close to a capital switch to it.
    crate::markets::go(w);
    switch_to_capitals(w);
    // Each civ claims the stand-in (or its capital alone), then raises its camp, its scout and last its
    // guard on the claim, which takes what the clans have left; its guard is divided at once, so the
    // pick's cards read land.
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for &id in &ids {
        let claim = if claim::may_claim(w, id) { claim::ai_claim(w, id) } else { vec![w.civs[&id].capital] };
        w.civs.get_mut(&id).unwrap().claim = claim;
    }
    for id in ids {
        if w.config.army {
            units::raise_at_founding(w, id);
        }
        units::raise_scout_at_founding(w, id);
        if w.config.army && claim::may_claim(w, id) {
            units::guard::raise_at_founding(w, id);
        }
        claim::divide(w, id);
    }
    w.phase = Phase::Choose;
}

/// `civ`'s capital falls to `by`'s army (`fight.rs` finds it: a rival army on the capital and no
/// part of `civ` left there). Its claim is cleared and its guard disbands to its own clan; its clans
/// go unaligned with their leanings cleared; its armies and scouts disband to the nearest clan by
/// steps; its store fills the packs of `by`'s armies on the tile in id order, and the rest is lost.
/// Its capital's market is an ordinary festival from then on (`markets::fade`). Notes both civs and
/// emits `CapitalFell`.
pub fn fall(w: &mut World, civ: u32, by: u32) {
    let capital = w.civs[&civ].capital;
    let turn = w.turn;
    {
        let c = w.civs.get_mut(&civ).unwrap();
        c.fallen = Some((turn, by));
        c.claim.clear();
        c.guard.clear();
        c.pressed = false;
        c.besieged = false;
    }
    if let Some(g) = units::guard_of(w, civ).map(|u| u.id) {
        units::disband(w, g, None, false);
    }
    let clans: Vec<u32> = w.clans.values().filter(|k| k.civ == Some(civ)).map(|k| k.id).collect();
    for id in clans {
        let k = w.clans.get_mut(&id).unwrap();
        k.civ = None;
        k.leaning.clear();
        w.note(id, "its civ has fallen: it is unaligned".to_string());
    }
    let units: Vec<(u32, usize)> = w.units.values().filter(|u| u.civ == civ).map(|u| (u.id, u.home)).collect();
    for (id, home) in units {
        match units::nearest_clan(w, home) {
            Some(clan) => units::give_up(w, id, clan),
            None => {
                w.units.remove(&id);
            }
        }
    }
    // The store: the taking civ's armies on the capital fill their packs in id order.
    let armies: Vec<u32> = w
        .units
        .values()
        .filter(|u| u.civ == by && u.kind() == units::UnitKind::Camp && u.home == capital)
        .map(|u| u.id)
        .collect();
    for id in armies {
        let room = (units::pack_cap(w.units[&id].people) - total(&w.units[&id].store)).max(0.0);
        let amount = room.min(total(&w.civs[&civ].store));
        let got = take(&mut w.civs.get_mut(&civ).unwrap().store, amount);
        add(&mut w.units.get_mut(&id).unwrap().store, &got);
    }
    w.civs.get_mut(&civ).unwrap().store = [0.0; N];
    w.emit(Event::new(EventKind::CapitalFell, capital).civ(civ).other(by));
    let (name, taker) = (w.civs[&civ].name.clone(), w.civs[&by].name.clone());
    let at = w.at(capital);
    w.note(civ, format!("its capital at {at} fell to {taker}'s army: it has no guard, army or militia left there"));
    w.note(by, format!("took {name}'s capital at {at}"));
}

/// The civ tile `i` counts for in a camp's line: the civ whose camp holds it, else its land's
/// (`tile_civ`). Forage and pillage, and the AI's post, read land without holding.
pub fn holder(w: &World, i: usize) -> Option<u32> {
    w.units.values().find(|u| u.held().contains(&i)).map(|u| u.civ).or_else(|| tile_civ(w, i))
}

/// Every tile whose civ is `civ` (see `tile_civ`), sorted.
pub fn land(w: &World, civ: u32) -> Vec<usize> {
    (0..w.tiles.len()).filter(|&i| tile_civ(w, i) == Some(civ)).collect()
}

/// Whether clan `id` is the own clan of its civ.
pub fn is_civ_clan(w: &World, id: u32) -> bool {
    w.clans.get(&id).and_then(|c| c.civ).and_then(|civ| w.civs.get(&civ)).is_some_and(|c| c.clan == id)
}

/// A civ whose own clan is gone (merged, died) passes it to its biggest clan (ties to the lower
/// id); the capital stays.
fn repair(w: &mut World) {
    let ids: Vec<u32> = alive(w).map(|c| c.id).collect();
    for id in ids {
        if w.clans.contains_key(&w.civs[&id].clan) {
            continue;
        }
        let best = w
            .clans
            .values()
            .filter(|c| c.civ == Some(id))
            .min_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)))
            .map(|c| c.id);
        if let Some(clan) = best {
            w.civs.get_mut(&id).unwrap().clan = clan;
        }
    }
}

/// The food `civ` keeps in its store out of posting and gifts: what the player set, else what its
/// camp needs: a full pack for its warriors (a new camp's with none), and 0 with `Config.army` off;
/// and `GUARD.keep_turns` turns of its guard's food.
pub fn keep(w: &World, civ: u32) -> f64 {
    let Some(c) = w.civs.get(&civ).filter(|c| !c.is_out()) else { return 0.0 };
    let guard = GUARD.keep_turns * UNIT.food * claim::guard_people(w, civ);
    let base = match c.keep {
        Some(k) => k,
        None if !w.config.army => 0.0,
        None => units::pack_cap(units::camp(w, civ).map_or(f64::from(UNIT.start), |u| u.people)),
    };
    base + guard
}

/// The levy at market `mid`: each clan there that is the civ's, or whose home is on the civ's land, pays
/// `CIV.levy` of what it brought (its basket) into the civ's store, in id order until the store is
/// at `CIV.store_cap`; its harvest drops by the same share, so that growth, which `eat` reckons on
/// the harvest, sees less. (A share of only the harvest beyond need brings next to nothing: clans
/// grow until their harvest just feeds them.) Called by `markets::trade` before the market clears.
/// Nothing with `Config.army` off.
pub fn levy(w: &mut World, mid: u32) {
    if !w.config.army {
        return;
    }
    let Some(civ) = alive(w).find(|c| c.market == mid).map(|c| c.id) else { return };
    if w.civs[&civ].besieged {
        return;
    }
    let ids: Vec<u32> = w
        .clans
        .values()
        .filter(|c| c.market == Some(mid) && (c.civ == Some(civ) || tile_civ(w, c.home) == Some(civ)))
        .map(|c| c.id)
        .collect();
    let mut room = (CIV.store_cap - w.civs[&civ].store.iter().sum::<f64>()).max(0.0);
    let mut paid_by = Vec::new();
    for id in ids {
        let c = w.clans.get_mut(&id).unwrap();
        let total: f64 = c.basket.iter().sum();
        if total <= 0.0 || room <= 0.0 {
            continue;
        }
        let share = CIV.levy.min(room / total);
        room -= share * total;
        let paid = c.basket.map(|v| v * share);
        let civ_state = w.civs.get_mut(&civ).unwrap();
        for (f, &p) in paid.iter().enumerate() {
            c.harvest[f] *= 1.0 - share;
            c.basket[f] -= p;
            civ_state.store[f] += p;
            civ_state.levied += p;
        }
        paid_by.push((id, share < CIV.levy));
    }
    if !paid_by.is_empty() {
        w.note(civ, "collected its levy at its market".to_string());
    }
    let name = w.civs[&civ].name.clone();
    for (id, cut) in paid_by {
        let text = if cut {
            format!("paid a cut levy to {name}: its store is at the cap")
        } else {
            format!("paid a levy to {name}")
        };
        w.note(id, text);
    }
}

/// The `civs` step, after eating: each civ's own clan is checked, its store spoils and what is
/// over the cap is lost.
pub fn upkeep(w: &mut World) {
    repair(w);
    let ids: Vec<u32> = alive(w).map(|c| c.id).collect();
    for id in ids {
        let civ = w.civs.get_mut(&id).unwrap();
        civ.spoil();
        let total: f64 = civ.store.iter().sum();
        let mut lost = 0.0;
        if total > CIV.store_cap {
            let keep = CIV.store_cap / total;
            civ.store = civ.store.map(|v| v * keep);
            lost = total - CIV.store_cap;
        }
        civ.lost = lost;
        if lost > 0.0 {
            w.note(id, "its store is over its cap: food is lost".to_string());
        }
    }
}

/// What the pick shows of one civ.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Card {
    pub civ: u32,
    pub name: String,
    pub color: String,
    /// The capital's tile and hex.
    pub capital: usize,
    pub q: i32,
    pub r: i32,
    /// Its clans and their people.
    pub clans: usize,
    pub people: f64,
    /// Hearths on its land.
    pub hearths: usize,
    /// The three commonest terrains on its land and their share, ties by terrain index.
    pub terrain: Vec<(String, f64)>,
    /// Hex steps from its capital to the nearest other capital; none with no other civ.
    pub rival_steps: Option<i32>,
    /// Whether any other capital can be reached over passable land.
    pub rival_by_land: bool,
}

/// One card per civ, in id order.
pub fn cards(w: &World) -> Vec<Card> {
    let owner: Vec<Option<u32>> = (0..w.tiles.len()).map(|i| tile_civ(w, i)).collect();
    w.civs
        .values()
        .map(|c| {
            let mine: Vec<usize> = (0..w.tiles.len()).filter(|&i| owner[i] == Some(c.id)).collect();
            let mut counts = [0usize; TERRAIN.len()];
            for &i in &mine {
                counts[w.tiles[i].terrain] += 1;
            }
            let mut order: Vec<usize> = (0..TERRAIN.len()).filter(|&t| counts[t] > 0).collect();
            order.sort_by_key(|&t| (std::cmp::Reverse(counts[t]), t));
            let terrain = order
                .into_iter()
                .take(3)
                .map(|t| (TERRAIN[t].name.to_string(), counts[t] as f64 / mine.len() as f64))
                .collect();
            let rivals: Vec<usize> = w.civs.values().filter(|o| o.id != c.id).map(|o| o.capital).collect();
            let by_land = cost_map(w, &[c.capital], f64::INFINITY, |i| passable(w, i));
            let h = w.tiles[c.capital].hex();
            let mine_clans = w.clans.values().filter(|k| k.civ == Some(c.id));
            Card {
                civ: c.id,
                name: c.name.clone(),
                color: c.color.clone(),
                capital: c.capital,
                q: h.q,
                r: h.r,
                clans: mine_clans.clone().count(),
                people: mine_clans.map(|k| k.people).sum(),
                hearths: mine.iter().filter(|&&i| is_hearth(&w.tiles[i])).count(),
                terrain,
                rival_steps: rivals.iter().map(|&t| steps(w, c.capital, t)).min(),
                rival_by_land: rivals.iter().any(|t| by_land.contains_key(t)),
            }
        })
        .collect()
}
