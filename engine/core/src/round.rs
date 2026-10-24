//! The round: a clan's yearly walk, a loop of distinct tiles from home (`round[0]`) and back.
//! Each leg between consecutive tiles, and from the last back home, is the cheapest path over
//! passable tiles within `territory_radius` of home.

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::clans::Clan;
use crate::data::*;
use crate::land::{cost_map, neighbours, passable};
use crate::world::World;

/// Legs within `territory_radius` of one home, each source's paths found once.
pub struct Legs<'a> {
    w: &'a World,
    home: usize,
    from: RefCell<BTreeMap<usize, BTreeMap<usize, f64>>>,
}

impl<'a> Legs<'a> {
    pub fn new(w: &'a World, home: usize) -> Self {
        Legs { w, home, from: RefCell::new(BTreeMap::new()) }
    }

    /// The cheapest path from `a` to `b` over passable tiles within `territory_radius` of home.
    pub fn get(&self, a: usize, b: usize) -> Option<f64> {
        if a == b {
            return Some(0.0);
        }
        let mut from = self.from.borrow_mut();
        let paths = from.entry(a).or_insert_with(|| {
            let h = self.w.tiles[self.home].hex();
            let near = |i: usize| self.w.tiles[i].hex().distance(h) <= CLAN.territory_radius;
            cost_map(self.w, &[a], f64::INFINITY, near)
        });
        paths.get(&b).copied()
    }

    /// The sum of the legs around a round from this home; a leg with no path is infinite.
    pub fn cost(&self, round: &[usize]) -> f64 {
        debug_assert!(round.first().is_none_or(|&h| h == self.home));
        if round.len() < 2 {
            return 0.0;
        }
        (0..round.len()).map(|k| self.get(round[k], round[(k + 1) % round.len()]).unwrap_or(f64::INFINITY)).sum()
    }

    /// Where adding `t` to a round from this home lengthens the loop least: see `insertion`.
    pub fn insertion(&self, round: &[usize], t: usize) -> Option<(usize, f64)> {
        let mut best: Option<(usize, f64)> = None;
        for k in 0..round.len() {
            let (a, b) = (round[k], round[(k + 1) % round.len()]);
            let (Some(at), Some(tb), Some(ab)) = (self.get(a, t), self.get(t, b), self.get(a, b)) else {
                continue;
            };
            let extra = at + tb - ab;
            if best.is_none_or(|(_, e)| extra < e) {
                best = Some((k + 1, extra));
            }
        }
        best
    }
}

/// The cheapest path from `a` to `b` over passable tiles within `territory_radius` of `home`.
pub fn leg(w: &World, home: usize, a: usize, b: usize) -> Option<f64> {
    Legs::new(w, home).get(a, b)
}

/// The sum of the legs around the loop; a leg with no path is infinite.
pub fn cost(w: &World, round: &[usize]) -> f64 {
    match round.first() {
        Some(&home) => Legs::new(w, home).cost(round),
        None => 0.0,
    }
}

/// The round's tiles and their land neighbours, sorted.
pub fn territory(w: &World, round: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = round
        .iter()
        .flat_map(|&i| std::iter::once(i).chain(neighbours(w, i).filter(|&j| TERRAIN[w.tiles[j].terrain].land)))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// What clan `clan_id` can harvest and the setup labour for each, sorted by tile: its round
/// tiles at 0, and every tile within `fetch_range` of the round with food in stock and on no
/// other clan's round at 2 × path cost (there and back).
pub fn options(w: &World, clan_id: u32, round: &[usize]) -> Vec<(usize, f64)> {
    cost_map(w, round, CLAN.fetch_range, |i| passable(w, i))
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

/// Where adding `t` lengthens the loop least: the position to insert it at (after `a`) and the
/// extra cost `leg(a, t) + leg(t, b) − leg(a, b)`, the first position on ties; `None` if no leg
/// reaches `t`. A one-tile round has the single pair home → home.
pub fn insertion(w: &World, round: &[usize], t: usize) -> Option<(usize, f64)> {
    Legs::new(w, round[0]).insertion(round, t)
}

/// Tiles a clan could add: land, passable, next to a round tile, on no round, within
/// `territory_radius` of home. Sorted.
pub fn free_edge(w: &World, clan: &Clan) -> Vec<usize> {
    let h = w.tiles[clan.home].hex();
    let mut out: Vec<usize> = clan
        .round
        .iter()
        .flat_map(|&i| neighbours(w, i))
        .filter(|&j| {
            let t = &w.tiles[j];
            TERRAIN[t.terrain].land
                && passable(w, j)
                && t.claim.is_none()
                && t.hex().distance(h) <= CLAN.territory_radius
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// The tile among `tiles` with the least total hex distance to the others; ties to `prefer`,
/// then the lowest index.
pub fn middle(w: &World, tiles: &[usize], prefer: usize) -> usize {
    let total = |i: usize| tiles.iter().map(|&j| w.tiles[i].hex().distance(w.tiles[j].hex())).sum::<i32>();
    let mut best: Option<(i32, bool, usize)> = None;
    for &i in tiles {
        // Least total, then `prefer`, then the lowest index.
        let key = (total(i), i != prefer, i);
        if best.is_none_or(|b| key < b) {
            best = Some(key);
        }
    }
    best.expect("middle of no tiles").2
}
