//! Actors: everything that acts in the world works the same way (`docs/design/actors.md`). An
//! actor is people with a home, a store, a territory it spreads its labour over and routes it
//! walks beyond it. Every walk is priced by `walk_cost`; labour is divided evenly over the
//! territory by `divide`, each tile's walk paid from its share; `change` is the one rule for
//! adding and dropping a tile. Nothing here knows what an actor spends its work on.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{neighbours, passable, step_cost};
use crate::market::Basket;
use crate::world::World;

/// A walk beyond the territory, made there and back once a turn.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route {
    /// The tile it goes to.
    pub to: usize,
    /// The tiles after home, ending at `to`.
    pub path: Vec<usize>,
    /// Its yearly cost: `walk_cost` of the path.
    pub cost: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub people: f64,
    pub home: usize,
    pub store: Basket,
    /// Tile indices, sorted, home among them, connected over passable land.
    pub territory: Vec<usize>,
    pub routes: Vec<Route>,
    pub market: Option<u32>,
}

impl Actor {
    /// An actor of `people` at `home`, holding only home.
    pub fn new(home: usize, people: f64) -> Actor {
        Actor { people, home, store: [0.0; N], territory: vec![home], routes: Vec::new(), market: None }
    }

    /// What its routes cost a turn.
    pub fn trip(&self) -> f64 {
        self.routes.iter().map(|r| r.cost).sum()
    }
}

/// The labour of a walk of `path_cost` made there and back once a turn, at `weight`: 1 for a
/// route, less for walks inside a territory that cost little.
pub fn walk_cost(path_cost: f64, weight: f64) -> f64 {
    2.0 * path_cost * weight
}

/// The walks from home to each tile of a territory over the territory's own tiles.
#[derive(Clone, Debug, PartialEq)]
pub struct Walks {
    /// The territory, sorted, as given.
    pub tiles: Vec<usize>,
    /// Each tile's path cost from home (`step_cost`), `INFINITY` if cut off; home is 0.
    pub cost: Vec<f64>,
    /// Each tile's predecessor on its cheapest path, as an index into `tiles`; `None` at home and
    /// for tiles cut off.
    pub before: Vec<Option<usize>>,
}

impl Walks {
    /// The cheapest paths from `home` over `territory` (sorted) by `step_cost`: Dijkstra over the
    /// territory's tiles, the cheapest then the lowest tile first, a tile keeping the first
    /// predecessor that reached it at its cost.
    pub fn new(w: &World, home: usize, territory: &[usize]) -> Walks {
        let n = territory.len();
        let mut cost = vec![f64::INFINITY; n];
        let mut before = vec![None; n];
        let mut done = vec![false; n];
        if let Ok(h) = territory.binary_search(&home) {
            cost[h] = 0.0;
        }
        loop {
            // Territories are small: a scan beats a heap.
            let mut next: Option<usize> = None;
            for k in 0..n {
                if !done[k] && cost[k].is_finite() && next.is_none_or(|b| cost[k] < cost[b]) {
                    next = Some(k);
                }
            }
            let Some(k) = next else { break };
            done[k] = true;
            for j in neighbours(w, territory[k]) {
                let Ok(m) = territory.binary_search(&j) else { continue };
                if done[m] {
                    continue;
                }
                let Some(step) = step_cost(w, territory[k], j) else { continue };
                if cost[k] + step < cost[m] {
                    cost[m] = cost[k] + step;
                    before[m] = Some(k);
                }
            }
        }
        Walks { tiles: territory.to_vec(), cost, before }
    }

    /// Each tile's yearly walk at `weight`, in `tiles` order.
    pub fn yearly(&self, weight: f64) -> Vec<f64> {
        self.cost.iter().map(|&c| walk_cost(c, weight)).collect()
    }

    /// Whether every tile can be reached from home.
    pub fn connected(&self) -> bool {
        self.cost.iter().all(|c| c.is_finite())
    }

    /// The path to the `k`th tile: the tiles after home, ending at it; empty for home or a tile cut
    /// off.
    pub fn path(&self, k: usize) -> Vec<usize> {
        let mut out = Vec::new();
        if self.cost[k].is_infinite() {
            return out;
        }
        let mut at = k;
        while let Some(b) = self.before[at] {
            out.push(self.tiles[at]);
            at = b;
        }
        out.reverse();
        out
    }
}

/// Divides `labour` over a territory: the routes are paid first, what's left is split evenly over
/// the tiles, and each tile's share pays its walk (`walks`, yearly) before the work. Returns each
/// tile's work, 0 where the walk eats the share.
pub fn divide(labour: f64, routes: &[Route], walks: &[f64]) -> Vec<f64> {
    if walks.is_empty() {
        return Vec::new();
    }
    let left = (labour - routes.iter().map(|r| r.cost).sum::<f64>()).max(0.0);
    let share = left / walks.len() as f64;
    walks.iter().map(|&w| (share - w).max(0.0)).collect()
}

/// Whether `territory` (sorted) is connected over its own passable tiles and holds `home`.
pub fn connected(w: &World, home: usize, territory: &[usize]) -> bool {
    territory.binary_search(&home).is_ok() && Walks::new(w, home, territory).connected()
}

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// Tiles a territory could add, sorted: passable land next to it, held by nobody, within `radius`
/// hex steps of home.
pub fn edge(w: &World, home: usize, territory: &[usize], radius: i32) -> Vec<usize> {
    let mut out: Vec<usize> = territory
        .iter()
        .flat_map(|&i| neighbours(w, i))
        .filter(|&j| {
            let t = &w.tiles[j];
            TERRAIN[t.terrain].land
                && passable(w, j)
                && t.claim.is_none()
                && steps(w, home, j) <= radius
                && territory.binary_search(&j).is_err()
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `territory` (sorted) with `t` added, sorted.
fn with(territory: &[usize], t: usize) -> Vec<usize> {
    let mut out = territory.to_vec();
    if let Err(k) = out.binary_search(&t) {
        out.insert(k, t);
    }
    out
}

/// The edge tile whose adding raises `value` most, ties to the lower tile: the territory with it,
/// the tile and the value.
pub fn best_add(
    w: &World,
    home: usize,
    territory: &[usize],
    radius: i32,
    value: &impl Fn(&[usize]) -> f64,
) -> Option<(Vec<usize>, usize, f64)> {
    let mut best: Option<(Vec<usize>, usize, f64)> = None;
    for t in edge(w, home, territory, radius) {
        let r = with(territory, t);
        let v = value(&r);
        if best.as_ref().is_none_or(|&(_, _, bv)| v > bv) {
            best = Some((r, t, v));
        }
    }
    best
}

/// The tile (not home, not `keep`) whose loss leaves the most `value`, among those whose loss
/// leaves the territory connected; ties to the lower tile: the territory without it, and the value.
pub fn best_drop(
    w: &World,
    home: usize,
    territory: &[usize],
    keep: Option<usize>,
    value: &impl Fn(&[usize]) -> f64,
) -> Option<(Vec<usize>, f64)> {
    let mut best: Option<(Vec<usize>, f64)> = None;
    for (k, &t) in territory.iter().enumerate() {
        if t == home || Some(t) == keep {
            continue;
        }
        let mut r = territory.to_vec();
        r.remove(k);
        if !connected(w, home, &r) {
            continue;
        }
        let v = value(&r);
        if best.as_ref().is_none_or(|&(_, bv)| v > bv) {
            best = Some((r, v));
        }
    }
    best
}

/// The one rule for holding land, at most one change: an actor that `wants` more adds the edge
/// tile that raises `value` most if it rises by `margin`, else swaps that tile for the one it
/// misses least if that rises by `margin`; one that doesn't drops the tile it misses least if the
/// value doesn't fall. Never drops home or disconnects. Returns the new territory, sorted.
pub fn change(
    w: &World,
    home: usize,
    territory: &[usize],
    wants: bool,
    radius: i32,
    margin: f64,
    value: impl Fn(&[usize]) -> f64,
) -> Vec<usize> {
    let now = value(territory);
    if !wants {
        return match best_drop(w, home, territory, None, &value) {
            Some((dropped, v)) if v >= now => dropped,
            _ => territory.to_vec(),
        };
    }
    let Some((added, t, v)) = best_add(w, home, territory, radius, &value) else { return territory.to_vec() };
    if v > now + margin {
        return added;
    }
    match best_drop(w, home, &added, Some(t), &value) {
        Some((swapped, sv)) if sv > now + margin => swapped,
        _ => territory.to_vec(),
    }
}

/// The tile among `tiles` with the least total hex distance to the others; ties to `prefer`,
/// then the lowest index.
pub fn middle(w: &World, tiles: &[usize], prefer: usize) -> usize {
    let total = |i: usize| tiles.iter().map(|&j| steps(w, i, j)).sum::<i32>();
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

/// Home steps to a neighbouring territory tile nearer the territory's middle (the lowest such
/// tile), then the territory keeps only the tiles within `radius` of home and connected to it.
/// Returns home and the territory.
pub fn recentre(w: &World, home: usize, territory: &[usize], radius: i32) -> (usize, Vec<usize>) {
    let mut home = home;
    let m = middle(w, territory, home);
    if m != home {
        let d = steps(w, home, m);
        if let Some(n) =
            neighbours(w, home).filter(|&n| territory.binary_search(&n).is_ok() && steps(w, n, m) < d).min()
        {
            home = n;
        }
    }
    (home, trim(w, home, territory, radius))
}

/// The tiles of `territory` (sorted) within `radius` of home and connected to it over them.
pub fn trim(w: &World, home: usize, territory: &[usize], radius: i32) -> Vec<usize> {
    let near: Vec<usize> = territory.iter().copied().filter(|&t| steps(w, home, t) <= radius).collect();
    let walks = Walks::new(w, home, &near);
    near.iter().zip(&walks.cost).filter(|(_, c)| c.is_finite()).map(|(&t, _)| t).collect()
}

/// Marks every tile on `walks`' paths (not home) and on every route's path as walked this turn.
pub fn mark_walked(w: &mut World, walks: &Walks, routes: &[Route]) {
    for (k, &t) in walks.tiles.iter().enumerate() {
        if walks.cost[k].is_finite() && walks.before[k].is_some() {
            w.tiles[t].walked = true;
        }
    }
    for r in routes {
        for &t in &r.path {
            w.tiles[t].walked = true;
        }
    }
}
