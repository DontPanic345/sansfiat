//! Civs: what founding makes of the pre-game's centres. Each centre's site market becomes a civ
//! with a capital on it, and the centre's clan its own clan; clans in reach of the market join the
//! civ; the civ keeps one store, which its fed clans levy into and which spoils and is capped.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Route};
use crate::data::*;
use crate::land::{cost_map, is_hearth, neighbours, passable, path};
use crate::map;
use crate::market::Basket;
use crate::markets::reach;
use crate::units::{self, Counters};
use crate::world::{EventKind, Phase, World};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    /// Every good the civ holds.
    pub store: Basket,
    /// Goods cut by the cap last turn.
    pub lost: f64,
    /// The civ's own clan: its home is the capital and stays there. If it goes, the civ's biggest
    /// clan takes its place.
    #[serde(default)]
    pub clan: u32,
    /// What its army has done, kept here so it outlives any one camp.
    #[serde(default)]
    pub army: Counters,
    /// The food the player has set it to keep; `None` is the default (see `keep`).
    #[serde(default)]
    pub keep: Option<f64>,
    /// The most food it gives a turn to unaligned clans at its market.
    #[serde(default)]
    pub gift: f64,
    /// Food it posted to its market's clearing last turn.
    #[serde(default)]
    pub posted: f64,
    /// Food it gave last turn.
    #[serde(default)]
    pub given: f64,
    /// Clans that have joined it since founding.
    #[serde(default)]
    pub joined: u32,
    /// Food it has given since founding.
    #[serde(default)]
    pub gifted: f64,
    /// The tiles the civ has revealed: their hidden trail costs are known to it.
    #[serde(default)]
    pub revealed: BTreeSet<usize>,
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

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
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
            store: CIV.endowment,
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
        };
        w.civs.insert(id, civ);
        w.civ_event(EventKind::CivFounded, capital, id, Some(mid));
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
        w.clans.get_mut(&clan).unwrap().civ = Some(civ);
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
            w.clans.get_mut(&clan).unwrap().civ = Some(id);
        }
    }
    // Each civ's own clan: its centre's clan, whose home is the capital; for a site the top-up
    // found without one, the biggest that joined with its home in reach of the capital (ties to
    // the lower id), else its biggest. Nothing moves: the clan's festival stays where it was.
    for (id, own) in owns {
        if let Some(clan) = own {
            w.clans.get_mut(&clan).unwrap().civ = Some(id);
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
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        if w.config.army {
            units::raise_at_founding(w, id);
        }
        units::raise_scout_at_founding(w, id);
    }
    w.phase = Phase::Choose;
}

/// The tiles within `CIV.territory` hex steps of the civ's capital, on the map, sorted: the
/// capital's land.
pub fn territory(w: &World, civ: u32) -> Vec<usize> {
    let Some(c) = w.civs.get(&civ) else { return Vec::new() };
    let mut out: Vec<usize> = w.tiles[c.capital]
        .hex()
        .within(CIV.territory)
        .into_iter()
        .filter_map(|h| map::index(w.config.radius, h))
        .collect();
    out.sort_unstable();
    out
}

/// The civ whose capital is within `CIV.territory` hex steps of tile `i` (capitals are far enough
/// apart that two never claim a tile); else the civ of the clan whose territory holds it; else none.
pub fn tile_civ(w: &World, i: usize) -> Option<u32> {
    w.civs
        .values()
        .find(|c| steps(w, c.capital, i) <= CIV.territory)
        .map(|c| c.id)
        .or_else(|| w.tiles[i].claim.and_then(|clan| w.clans.get(&clan)).and_then(|c| c.civ))
}

/// The civ tile `i` counts for in a camp's line: the civ whose camp holds it, else its land's
/// (`tile_civ`). Clans' claims, forage and pillage, and the AI's post read land without holding.
pub fn holder(w: &World, i: usize) -> Option<u32> {
    w.units.values().find(|u| u.held.contains(&i)).map(|u| u.civ).or_else(|| tile_civ(w, i))
}

/// Every tile whose civ is `civ` (see `tile_civ`), sorted.
pub fn land(w: &World, civ: u32) -> Vec<usize> {
    (0..w.tiles.len()).filter(|&i| tile_civ(w, i) == Some(civ)).collect()
}

/// The tiles the capital works: its own and its on-map neighbours.
pub fn work_tiles(w: &World, civ: u32) -> Vec<usize> {
    let Some(c) = w.civs.get(&civ) else { return Vec::new() };
    std::iter::once(c.capital).chain(neighbours(w, c.capital)).collect()
}

/// Whether clan `id` is the own clan of its civ.
pub fn is_civ_clan(w: &World, id: u32) -> bool {
    w.clans.get(&id).and_then(|c| c.civ).and_then(|civ| w.civs.get(&civ)).is_some_and(|c| c.clan == id)
}

/// A civ whose own clan is gone (merged, died) passes it to its biggest clan (ties to the lower
/// id); the capital stays.
fn repair(w: &mut World) {
    let ids: Vec<u32> = w.civs.keys().copied().collect();
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
/// camp needs: a full pack for its warriors (a new camp's with none), and 0 with `Config.army` off.
pub fn keep(w: &World, civ: u32) -> f64 {
    let Some(c) = w.civs.get(&civ) else { return 0.0 };
    if let Some(k) = c.keep {
        return k;
    }
    if !w.config.army {
        return 0.0;
    }
    units::pack_cap(units::camp(w, civ).map_or(f64::from(UNIT.start), |u| u.people))
}

/// The levy at market `mid`: each clan of the civ whose capital market it is that came there pays
/// `CIV.levy` of what it brought (its basket) into the civ's store, in id order until the store is
/// at `CIV.store_cap`; its harvest drops by the same share, so that growth, which `eat` reckons on
/// the harvest, sees less. (A share of only the harvest beyond need brings next to nothing: clans
/// grow until their harvest just feeds them.) Called by `markets::trade` before the market clears.
/// Nothing with `Config.army` off.
pub fn levy(w: &mut World, mid: u32) {
    if !w.config.army {
        return;
    }
    let Some(civ) = w.civs.values().find(|c| c.market == mid).map(|c| c.id) else { return };
    let ids: Vec<u32> =
        w.clans.values().filter(|c| c.civ == Some(civ) && c.market == Some(mid)).map(|c| c.id).collect();
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
        let store = &mut w.civs.get_mut(&civ).unwrap().store;
        for (f, &p) in paid.iter().enumerate() {
            c.harvest[f] *= 1.0 - share;
            c.basket[f] -= p;
            store[f] += p;
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
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        let mut store = w.civs[&id].store.map(|v| v * (1.0 - CLAN.spoilage));
        let total: f64 = store.iter().sum();
        let mut lost = 0.0;
        if total > CIV.store_cap {
            let keep = CIV.store_cap / total;
            store = store.map(|v| v * keep);
            lost = total - CIV.store_cap;
        }
        let civ = w.civs.get_mut(&id).unwrap();
        civ.store = store;
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
