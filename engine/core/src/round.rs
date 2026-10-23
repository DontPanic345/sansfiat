//! The round: a clan's yearly walk, a loop of distinct tiles from home (`round[0]`) and back.
//! Each leg between consecutive tiles, and from the last back home, is the cheapest path over
//! passable tiles within `territory_radius` of home.

use crate::data::*;
use crate::land::{cost_map, step_cost};
use crate::world::World;

/// The cheapest path from `a` to `b` over passable tiles within `territory_radius` of `home`.
pub fn leg(w: &World, home: usize, a: usize, b: usize) -> Option<f64> {
    if a == b {
        return Some(0.0);
    }
    let h = w.tiles[home].hex();
    let near = |i: usize| w.tiles[i].hex().distance(h) <= CLAN.territory_radius;
    cost_map(w, &[a], f64::INFINITY, near).get(&b).copied()
}

/// The sum of the legs around the loop; a leg with no path is infinite.
pub fn cost(w: &World, round: &[usize]) -> f64 {
    if round.len() < 2 {
        return 0.0;
    }
    let home = round[0];
    (0..round.len()).map(|k| leg(w, home, round[k], round[(k + 1) % round.len()]).unwrap_or(f64::INFINITY)).sum()
}

/// The round's tiles and their land neighbours, sorted.
pub fn territory(w: &World, round: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = round
        .iter()
        .flat_map(|&i| {
            std::iter::once(i).chain(crate::land::neighbours(w, i).filter(|&j| TERRAIN[w.tiles[j].terrain].land))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// What clan `clan_id` can harvest and the setup labour for each, sorted by tile: its round
/// tiles at 0, and every tile within `fetch_range` of the round with food in stock and on no
/// other clan's round at 2 × path cost (there and back).
pub fn options(w: &World, clan_id: u32, round: &[usize]) -> Vec<(usize, f64)> {
    cost_map(w, round, CLAN.fetch_range, |i| step_cost(w, i).is_some())
        .into_iter()
        .filter_map(|(i, c)| {
            if round.contains(&i) {
                return Some((i, 0.0));
            }
            let t = &w.tiles[i];
            let free = t.claim.is_none_or(|id| id == clan_id);
            (free && t.stock.iter().any(|&s| s > 0.0)).then_some((i, 2.0 * c))
        })
        .collect()
}
