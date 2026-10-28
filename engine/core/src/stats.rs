//! Numbers read off a world: a summary line, and the evidence that centres form around fertile
//! land. Nothing here changes the world.

use serde::Serialize;

use crate::data::*;
use crate::land::is_hearth;
use crate::markets::reach;
use crate::pregame::{centres, land_cover, End};
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
    pub trails: usize,
    pub hearths: usize,
    pub markets: usize,
    /// Clans with a market.
    pub trading: usize,
    pub centres: Vec<u32>,
    pub cover: f64,
    pub end: Option<End>,
}

pub fn summary(w: &World) -> Summary {
    let n = w.clans.len().max(1) as f64;
    let centres = w.end.as_ref().map_or_else(|| centres(w), |e| e.centres.clone());
    let cover = land_cover(w, &centres);
    Summary {
        turn: w.turn,
        clans: w.clans.len(),
        people: w.clans.values().map(|c| c.people).sum(),
        mean_fed: w.clans.values().map(|c| c.fed_ratio).sum::<f64>() / n,
        starving: w.clans.values().filter(|c| c.starving).count(),
        held: w.tiles.iter().filter(|t| t.claim.is_some()).count(),
        territory: w.clans.values().map(|c| c.territory.len() as f64).sum::<f64>() / n,
        walk_cost: w.clans.values().map(|c| c.walk).sum::<f64>() / n,
        trails: w.tiles.iter().filter(|t| t.trail).count(),
        hearths: w.tiles.iter().filter(|t| is_hearth(t)).count(),
        markets: w.markets.len(),
        trading: w.clans.values().filter(|c| c.market.is_some()).count(),
        centres,
        cover,
        end: w.end.clone(),
    }
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
    let mut h: u64 = 0xcbf29ce484222325;
    for b in serde_json::to_string(w).unwrap().bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}
