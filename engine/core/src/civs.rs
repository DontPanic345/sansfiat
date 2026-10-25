//! Civs: what founding makes of the pre-game's centres. Each centre's market becomes a civ with a
//! capital on it; clans in reach of the market join the civ; the civ keeps one store, which its
//! fed clans levy into and which spoils and is capped.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{cost_map, is_hearth, neighbours, passable};
use crate::map;
use crate::market::Basket;
use crate::markets::reach;
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
}

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// Founding, once, as the pre-game ends: a civ on each centre's market (the k-th takes `CIVS[k]`),
/// clans in reach joining the nearest, a civ with no clan taking the nearest unaligned one, and
/// the phase `Choose`. Draws no random numbers.
pub fn found(w: &mut World) {
    let centres = w.end.as_ref().map(|e| e.centres.clone()).unwrap_or_default();
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
        };
        w.civs.insert(id, civ);
        w.civ_event(EventKind::CivFounded, capital, id, Some(mid));
    }

    let reaches: Vec<(u32, BTreeMap<usize, f64>)> = w.civs.values().map(|c| (c.id, reach(w, c.capital))).collect();
    let joins: Vec<(u32, u32)> = w
        .clans
        .values()
        .filter_map(|c| {
            let mut best: Option<(u32, f64)> = None;
            for (id, r) in &reaches {
                let cost = c.round.iter().filter_map(|t| r.get(t)).copied().min_by(f64::total_cmp);
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
    for (clan, civ) in joins {
        w.clans.get_mut(&clan).unwrap().civ = Some(civ);
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
/// apart that two never claim a tile); else the civ of the clan whose round holds it; else none.
pub fn tile_civ(w: &World, i: usize) -> Option<u32> {
    w.civs
        .values()
        .find(|c| steps(w, c.capital, i) <= CIV.territory)
        .map(|c| c.id)
        .or_else(|| w.tiles[i].claim.and_then(|clan| w.clans.get(&clan)).and_then(|c| c.civ))
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

/// The `civs` step, after eating: each civ's store spoils, its fed clans hand it their share, and
/// what is over the cap is lost.
pub fn upkeep(w: &mut World) {
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        let mut store = w.civs[&id].store.map(|v| v * (1.0 - CLAN.spoilage));
        for c in w.clans.values_mut().filter(|c| c.civ == Some(id) && c.fed) {
            for (have, held) in c.store.iter_mut().zip(store.iter_mut()) {
                let give = CIV.levy * *have;
                *have -= give;
                *held += give;
            }
        }
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
                people: mine_clans.map(|k| k.pop).sum(),
                hearths: mine.iter().filter(|&&i| is_hearth(&w.tiles[i])).count(),
                terrain,
                rival_steps: rivals.iter().map(|&t| steps(w, c.capital, t)).min(),
                rival_by_land: rivals.iter().any(|t| by_land.contains_key(t)),
            }
        })
        .collect()
}
