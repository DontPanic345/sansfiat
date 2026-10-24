//! Placeholder map: Voronoi regions of terrain inside a hex, sea at the rim, coast beside sea,
//! and food on the land, scaled by fertility in patches. A map piece replaces it.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::hex::Hex;
use crate::market::Basket;
use crate::rng::{rand, rand_index};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: usize,
    /// Scales the terrain's food caps, `FERTILITY.0..=FERTILITY.1`.
    pub fertility: f64,
    pub cap: Basket,
    pub stock: Basket,
    /// The grain cap before taming.
    pub wild_grain: f64,
    pub tame: u32,
    /// The clan whose round the tile is on.
    pub claim: Option<u32>,
    /// Turns in a row on a round, and on none.
    pub held: u32,
    pub unheld: u32,
    pub trail: bool,
}

impl Tile {
    pub fn hex(&self) -> Hex {
        Hex { q: self.q, r: self.r }
    }
}

/// The position of `h` in `Hex::ORIGIN.within(radius)`, the order `generate` builds tiles in.
pub fn index(radius: i32, h: Hex) -> Option<usize> {
    if h.distance(Hex::ORIGIN) > radius {
        return None;
    }
    let (q, r, rad) = (h.q as i64, h.r as i64, radius as i64);
    // Columns -rad..q come first; column c holds 2·rad + 1 − |c| tiles.
    let cols = q + rad;
    let tri = |n: i64| n * (n + 1) / 2;
    let abs_sum = if q > 0 { tri(rad) + tri(q - 1) } else { tri(rad) - tri(-q) };
    let before = cols * (2 * rad + 1) - abs_sum;
    Some((before + r - (-rad).max(-q - rad)) as usize)
}

pub fn generate(radius: i32, rng: &mut u32) -> Vec<Tile> {
    let hexes = Hex::ORIGIN.within(radius);
    let regions: Vec<(Hex, usize)> = (0..REGIONS)
        .map(|_| {
            let h = hexes[rand_index(rng, hexes.len())];
            (h, REGION_TERRAIN[rand_index(rng, REGION_TERRAIN.len())])
        })
        .collect();
    let mut terrain: Vec<usize> = hexes
        .iter()
        .map(|&h| {
            if h.distance(Hex::ORIGIN) == radius {
                SEA
            } else {
                regions.iter().min_by_key(|(c, _)| c.distance(h)).unwrap().1
            }
        })
        .collect();
    let neighbours = |i: usize| hexes[i].neighbors().filter_map(|n| index(radius, n)).collect::<Vec<_>>();
    let shore: Vec<usize> = (0..hexes.len())
        .filter(|&i| TERRAIN[terrain[i]].land && neighbours(i).iter().any(|&j| terrain[j] == SEA))
        .collect();
    for i in shore {
        terrain[i] = COAST;
    }

    // Fertility in patches, drawn after the terrain so a seed's terrain is unchanged.
    let mut y: Vec<f64> = hexes.iter().map(|_| rand(rng)).collect();
    for _ in 0..3 {
        y = (0..hexes.len())
            .map(|i| {
                let ns = neighbours(i);
                (2.0 * y[i] + ns.iter().map(|&j| y[j]).sum::<f64>()) / (2.0 + ns.len() as f64)
            })
            .collect();
    }
    let lo = y.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = y.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = if hi > lo { hi - lo } else { 1.0 };
    let round1 = |v: f64| (v * 10.0).round() / 10.0;

    hexes
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let t = (y[i] - lo) / span;
            let fertility = round1(FERTILITY.0 + (FERTILITY.1 - FERTILITY.0) * libm::pow(t, 1.5));
            let cap: Basket = TERRAIN[terrain[i]].food.map(|f| round1(f * fertility));
            Tile {
                q: h.q,
                r: h.r,
                terrain: terrain[i],
                fertility,
                cap,
                stock: cap,
                wild_grain: cap[GRAIN],
                tame: 0,
                claim: None,
                held: 0,
                unheld: 0,
                trail: false,
            }
        })
        .collect()
}

/// The connected groups of land tiles, each sorted, the groups in order of their lowest tile.
pub fn landmasses(tiles: &[Tile], radius: i32) -> Vec<Vec<usize>> {
    let land = |i: usize| TERRAIN[tiles[i].terrain].land;
    let mut seen = vec![false; tiles.len()];
    let mut out = Vec::new();
    for start in 0..tiles.len() {
        if seen[start] || !land(start) {
            continue;
        }
        seen[start] = true;
        let (mut group, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for j in tiles[i].hex().neighbors().filter_map(|n| index(radius, n)) {
                if !seen[j] && land(j) {
                    seen[j] = true;
                    group.push(j);
                    todo.push(j);
                }
            }
        }
        group.sort_unstable();
        out.push(group);
    }
    out
}
