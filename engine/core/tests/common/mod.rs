//! Hand-built worlds for the clan tests.
#![allow(dead_code)]

use engine::clans::new_clan;
use engine::data::*;
use engine::hex::{Hex, DIRS};
use engine::map::{index, Zone};
use engine::{Config, Market, World};

pub const R: i32 = 6;

/// A world of bare plains with sea at the rim: no food, no clans, no markets. Its random state
/// and next id are what the placeholder generator left, so every test's draws and ids hold; no
/// river, lake, zone or hidden cost is left on a tile.
pub fn flat() -> World {
    let mut w = World::new(Config { seed: 1, radius: R });
    w.clans.clear();
    w.rng = 4151932303;
    w.next_id = 4;
    for t in &mut w.tiles {
        if t.hex().distance(Hex::ORIGIN) < R {
            t.terrain = PLAINS;
        }
        t.cap = [0.0; N];
        t.stock = [0.0; N];
        t.wild_grain = 0.0;
        t.tame = 0;
        t.claim = None;
        t.held = 0;
        t.unheld = 0;
        t.trail = false;
        t.river = 0;
        t.flow = None;
        t.lake = false;
        t.flood = false;
        t.zone = Zone::Open;
        t.pass = false;
        t.trail_cost = None;
        t.span = None;
    }
    w.markets.clear();
    w
}

/// As `flat`, but only `land` is land.
pub fn island(land: &[usize]) -> World {
    let mut w = flat();
    for i in 0..w.tiles.len() {
        if !land.contains(&i) {
            w.tiles[i].terrain = SEA;
        }
    }
    w
}

/// The tile at `(q, r)`.
pub fn at(q: i32, r: i32) -> usize {
    index(R, Hex { q, r }).unwrap()
}

/// The middle tile's neighbour `k` steps along `DIRS[d]`.
pub fn out(d: usize, k: i32) -> usize {
    at(DIRS[d].0 * k, DIRS[d].1 * k)
}

pub fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// Puts a clan of `pop` on `round` and claims it.
pub fn put(w: &mut World, round: &[usize], pop: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, round[0], pop);
    c.round = round.to_vec();
    for &t in round {
        w.tiles[t].claim = Some(id);
    }
    w.clans.insert(id, c);
    id
}

/// Checks every round against the tiles' claims and the limit.
pub fn check_rounds(w: &World) {
    let mut owner = vec![None; w.tiles.len()];
    for c in w.clans.values() {
        assert_eq!(c.round[0], c.home, "clan {}", c.id);
        for &t in &c.round {
            assert_eq!(owner[t], None, "tile {t} on two rounds, or twice on clan {}'s", c.id);
            owner[t] = Some(c.id);
            assert!(steps(w, c.home, t) <= CLAN.territory_radius, "clan {}: tile {t} too far from home", c.id);
        }
    }
    for (t, tile) in w.tiles.iter().enumerate() {
        assert_eq!(tile.claim, owner[t], "tile {t}'s claim");
    }
}

/// Opens a market on `tile` as `form` would, founded on turn 0.
pub fn open(w: &mut World, tile: usize) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let m = Market {
        id,
        tile,
        founded: 0,
        prices: [1.0; N],
        goods: Vec::new(),
        activity: 2.0 * MARKET.min_volume,
        volume: 0.0,
        traders: 0,
    };
    w.markets.insert(id, m);
    id
}

/// Makes `tile` a hearth with `wild` grain.
pub fn hearth(w: &mut World, tile: usize, wild: f64) {
    let t = &mut w.tiles[tile];
    t.wild_grain = wild;
    t.tame = TENDING.tame_turns;
    t.cap[GRAIN] = wild * TENDING.domestic;
}
