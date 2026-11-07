//! Actors: everything that acts in the world works the same way (`docs/design/actors.md`). An
//! actor is people with a home, a store, a territory it spreads its labour over and routes it
//! walks beyond it. Every walk is priced by `walk_cost`; labour is divided evenly over the
//! territory by `divide` (or `split`, over several jobs), each tile's walk paid from its share;
//! `change` is the one rule for adding and dropping a tile, and `meal` the one rule for eating from
//! kin's stores. Nothing here knows what an actor spends its work on.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{neighbours, passable, step_cost, steps};
use crate::market::{take, total, Basket};
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

    /// The store spoils a turn: each good keeps `1 - CLAN.spoilage` of itself.
    pub fn spoil(&mut self) {
        self.store = self.store.map(|v| v * (1.0 - CLAN.spoilage));
    }

    /// Eats `need` from the store in proportion to what it holds, and returns what was eaten.
    pub fn eat_by_share(&mut self, need: f64) -> Basket {
        take(&mut self.store, need)
    }

    /// Eats for a diet from the store, which holds what the people have to eat this turn: each good
    /// up to `CLAN.diet_share × need`, then the excess at `CLAN.excess_value` a unit until `need` is
    /// met. What is left stays in the store, each good kept in the share the excess took. Returns
    /// what the people got, in need units.
    pub fn eat_by_diet(&mut self, need: f64) -> f64 {
        let k = CLAN.diet_share * need;
        let mut got: f64 = self.store.iter().map(|&v| v.min(k)).sum();
        let left: Basket = self.store.map(|v| (v - k).max(0.0));
        let total: f64 = left.iter().sum();
        let eaten = total.min((need - got).max(0.0) / CLAN.excess_value);
        got += eaten * CLAN.excess_value;
        let keep = if total > 0.0 { 1.0 - eaten / total } else { 0.0 };
        self.store = left.map(|v| v * keep);
        got
    }
}

/// Walks `path` from `from` on `labour`: each step is taken while its cost is no more than what is
/// left plus `slack`, and its cost comes off what is left. Returns the steps taken and the labour
/// left. A step that cannot be paid, or is not a step, ends the walk.
pub fn stride(w: &World, from: usize, path: &[usize], labour: f64, slack: f64) -> (usize, f64) {
    let mut left = labour;
    let mut at = from;
    let mut taken = 0;
    for &next in path {
        let Some(cost) = step_cost(w, at, next) else { break };
        if cost > left + slack {
            break;
        }
        left -= cost;
        at = next;
        taken += 1;
    }
    (taken, left)
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
/// tile's work, 0 where the walk eats the share. `split` with one job of share 1.
pub fn divide(labour: f64, routes: &[Route], walks: &[f64]) -> Vec<f64> {
    split(labour, routes, &[1.0], &[walks.to_vec()]).swap_remove(0)
}

/// Splits `labour` over several jobs (`jobs`: each job's walks, yearly, one per tile). The routes
/// are paid once from `labour`; what is left goes to each job by its share (`shares`, summing to
/// one). Each job divides its part evenly over its tiles, each tile's share paying its walk before
/// the work, as `divide` does. Returns each job's work per tile, 0 where a walk eats the share.
/// A job with no tiles gets nothing, and its share is not handed on to the other jobs.
pub fn split(labour: f64, routes: &[Route], shares: &[f64], jobs: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let left = (labour - routes.iter().map(|r| r.cost).sum::<f64>()).max(0.0);
    jobs.iter()
        .zip(shares)
        .map(|(walks, &share)| {
            if walks.is_empty() {
                return Vec::new();
            }
            let each = left * share / walks.len() as f64;
            walks.iter().map(|&w| (each - w).max(0.0)).collect()
        })
        .collect()
}

/// Eats `need` for an actor: from its own `store` first, then from its kin's stores, biggest first
/// (ties to the lower id), until `need` is met or the kin are out. Returns what was eaten. The kin
/// are `(id, store)` pairs; each store gives what it holds in turn.
pub fn meal<'a>(need: f64, store: &mut Basket, kin: impl IntoIterator<Item = (u32, &'a mut Basket)>) -> f64 {
    const EPS: f64 = 1e-9;
    let mut got = total(&take(store, need));
    let mut kin: Vec<(f64, u32, &mut Basket)> = kin.into_iter().map(|(id, s)| (total(s), id, s)).collect();
    kin.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    for (_, _, s) in kin {
        if got + EPS >= need {
            break;
        }
        got += total(&take(s, need - got));
    }
    got
}

/// Whether `territory` (sorted) is connected over its own passable tiles and holds `home`.
pub fn connected(w: &World, home: usize, territory: &[usize]) -> bool {
    territory.binary_search(&home).is_ok() && Walks::new(w, home, territory).connected()
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

/// Adds `people` to the traffic of every tile on each of `walks`' paths and each route's path, home
/// included: every walk starts there, though `Walks::path` leaves it out.
pub fn add_traffic(w: &mut World, home: usize, people: f64, walks: &Walks, routes: &[Route]) {
    for k in 0..walks.tiles.len() {
        let path = walks.path(k);
        if path.is_empty() {
            continue;
        }
        w.tiles[home].traffic += people;
        for t in path {
            w.tiles[t].traffic += people;
        }
    }
    for r in routes {
        if r.path.is_empty() {
            continue;
        }
        w.tiles[home].traffic += people;
        for &t in &r.path {
            w.tiles[t].traffic += people;
        }
    }
}
