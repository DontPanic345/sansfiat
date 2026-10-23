//! Placeholder map: Voronoi regions of terrain inside a hex, sea at the rim, coast beside sea.
//! A map piece replaces it.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::hex::Hex;
use crate::rng::rand_index;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: usize,
}

impl Tile {
    pub fn hex(&self) -> Hex {
        Hex { q: self.q, r: self.r }
    }
}

pub fn generate(radius: i32, rng: &mut u32) -> Vec<Tile> {
    let hexes = Hex::ORIGIN.within(radius);
    let regions: Vec<(Hex, usize)> = (0..REGIONS)
        .map(|_| {
            let h = hexes[rand_index(rng, hexes.len())];
            (h, REGION_TERRAIN[rand_index(rng, REGION_TERRAIN.len())])
        })
        .collect();
    let mut tiles: Vec<Tile> = hexes
        .iter()
        .map(|&h| {
            let terrain = if h.distance(Hex::ORIGIN) == radius {
                SEA
            } else {
                regions.iter().min_by_key(|(c, _)| c.distance(h)).unwrap().1
            };
            Tile { q: h.q, r: h.r, terrain }
        })
        .collect();
    let index: HashMap<Hex, usize> = tiles.iter().enumerate().map(|(i, t)| (t.hex(), i)).collect();
    let shore: Vec<usize> = (0..tiles.len())
        .filter(|&i| {
            TERRAIN[tiles[i].terrain].land
                && tiles[i].hex().neighbors().any(|n| index.get(&n).is_some_and(|&j| tiles[j].terrain == SEA))
        })
        .collect();
    for i in shore {
        tiles[i].terrain = COAST;
    }
    tiles
}
