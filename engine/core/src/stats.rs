//! Numbers read off a world: a summary line, and the evidence that centres form around fertile
//! land. Nothing here changes the world.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::data::*;
use crate::land::{is_depleted, is_hearth};
use crate::markets::reach;
use crate::pregame::{centres, homes, land_cover, sites, End};
use crate::world::World;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Summary {
    pub turn: u32,
    pub clans: usize,
    pub people: f64,
    pub mean_fed: f64,
    pub starving: usize,
    /// Tiles in some clan's territory.
    pub held: usize,
    /// Mean tiles in a clan's territory.
    pub territory: f64,
    /// Mean labour a clan's walks inside its territory cost this turn.
    pub walk_cost: f64,
    /// People moved between clans at festivals last turn, and over the whole run.
    pub moved: f64,
    pub moved_total: f64,
    pub trails: usize,
    /// Tiles depleted: some food's wear is at least `DEPLETION.depleted_at`.
    pub depleted: usize,
    pub hearths: usize,
    pub markets: usize,
    /// Clans with a market.
    pub trading: usize,
    /// The centre clans: so far, before the end; the end's, after it.
    pub centres: Vec<u32>,
    pub cover: f64,
    pub end: Option<End>,
    /// Each civ's Standing: see `standing`. Empty before founding.
    pub standing: BTreeMap<u32, f64>,
}

pub fn summary(w: &World) -> Summary {
    let n = w.clans.len().max(1) as f64;
    let (centres, cover) = match &w.end {
        Some(e) => (e.clans.iter().flatten().copied().collect(), land_cover(w, &sites(w, &e.centres))),
        None => {
            let ids = centres(w);
            let cover = land_cover(w, &homes(w, &ids));
            (ids, cover)
        }
    };
    Summary {
        turn: w.turn,
        clans: w.clans.len(),
        people: w.clans.values().map(|c| c.people).sum(),
        mean_fed: w.clans.values().map(|c| c.fed_ratio).sum::<f64>() / n,
        starving: w.clans.values().filter(|c| c.starving).count(),
        held: w.tiles.iter().filter(|t| t.claim.is_some()).count(),
        territory: w.clans.values().map(|c| c.territory.len() as f64).sum::<f64>() / n,
        walk_cost: w.clans.values().map(|c| c.walk).sum::<f64>() / n,
        moved: w.markets.values().map(|m| m.moved).sum(),
        moved_total: w.moved,
        trails: w.tiles.iter().filter(|t| t.trail).count(),
        depleted: w.tiles.iter().filter(|t| is_depleted(t)).count(),
        hearths: w.tiles.iter().filter(|t| is_hearth(t)).count(),
        markets: w.markets.len(),
        trading: w.clans.values().filter(|c| c.market.is_some()).count(),
        centres,
        cover,
        end: w.end.clone(),
        standing: standing(w),
    }
}

/// Each civ's Standing: the people of its clans and of its units over the people of every clan and
/// every unit. Warriors and scouts are people the civ took from its clans, so raising a camp moves
/// nobody between civs. Every civ is there, at 0 when the world has nobody.
pub fn standing(w: &World) -> BTreeMap<u32, f64> {
    let mut mine: BTreeMap<u32, f64> = w.civs.keys().map(|&c| (c, 0.0)).collect();
    let clans = w.clans.values().filter_map(|k| k.civ.map(|c| (c, k.people)));
    let units = w.units.values().map(|u| (u.civ, u.people));
    for (civ, people) in clans.chain(units) {
        *mine.entry(civ).or_insert(0.0) += people;
    }
    let all: f64 = w.clans.values().map(|k| k.people).sum::<f64>() + w.units.values().map(|u| u.people).sum::<f64>();
    if all > 0.0 {
        for share in mine.values_mut() {
            *share /= all;
        }
    }
    mine
}

/// Hex steps from a centre's market within which fertility is compared.
pub const NEAR_STEPS: i32 = 2;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CentreReport {
    pub market: u32,
    pub q: i32,
    pub r: i32,
    pub traders: usize,
    pub age: u32,
    pub hearths_in_reach: usize,
    /// Mean fertility of land within `NEAR_STEPS` of the market.
    pub fertility_near: f64,
    /// Mean fertility over all land.
    pub fertility_land: f64,
}

/// Mean fertility over land tiles passing `keep`; 0 with none.
fn mean_fertility(w: &World, keep: impl Fn(usize) -> bool) -> f64 {
    let land: Vec<f64> = (0..w.tiles.len())
        .filter(|&i| TERRAIN[w.tiles[i].terrain].land && keep(i))
        .map(|i| w.tiles[i].fertility)
        .collect();
    if land.is_empty() {
        0.0
    } else {
        land.iter().sum::<f64>() / land.len() as f64
    }
}

/// One report per market id in `ids`, skipping markets that are gone.
pub fn centre_report(w: &World, ids: &[u32]) -> Vec<CentreReport> {
    let all = mean_fertility(w, |_| true);
    ids.iter()
        .filter_map(|id| w.markets.get(id))
        .map(|m| {
            let at = w.tiles[m.tile].hex();
            CentreReport {
                market: m.id,
                q: at.q,
                r: at.r,
                traders: m.traders,
                age: w.turn.saturating_sub(m.founded),
                hearths_in_reach: reach(w, m.tile).keys().filter(|&&t| is_hearth(&w.tiles[t])).count(),
                fertility_near: mean_fertility(w, |i| w.tiles[i].hex().distance(at) <= NEAR_STEPS),
                fertility_land: all,
            }
        })
        .collect()
}

/// Hex steps between each pair of the markets in `ids`, in order `(0,1), (0,2), (1,2), …`.
pub fn distances(w: &World, ids: &[u32]) -> Vec<i32> {
    let tiles: Vec<_> = ids.iter().filter_map(|id| w.markets.get(id)).map(|m| w.tiles[m.tile].hex()).collect();
    let mut out = Vec::new();
    for (i, a) in tiles.iter().enumerate() {
        for b in &tiles[i + 1..] {
            out.push(a.distance(*b));
        }
    }
    out
}

/// The mean of `fertility_near / fertility_land` over the reports; `None` with none.
pub fn mean_ratio(reports: &[CentreReport]) -> Option<f64> {
    if reports.is_empty() {
        return None;
    }
    Some(reports.iter().map(|r| r.fertility_near / r.fertility_land).sum::<f64>() / reports.len() as f64)
}

/// One value that changes if any bit of the world does, so two builds (native and wasm) can be
/// compared turn by turn: FNV-1a 64 over the world's JSON, as 16 lowercase hex digits.
pub fn digest(w: &World) -> String {
    fnv(&serde_json::to_string(w).unwrap())
}

/// The digest of the world without its `log`: the same FNV-1a 64 over the world's JSON as a
/// `serde_json::Value` with the `log` key removed. A scripted game's orders are in its log, so
/// this is the digest that stays put when the log's shape changes.
pub fn state_digest(w: &World) -> String {
    let mut v = serde_json::to_value(w).unwrap();
    v.as_object_mut().unwrap().remove("log");
    fnv(&serde_json::to_string(&v).unwrap())
}

fn fnv(text: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}
