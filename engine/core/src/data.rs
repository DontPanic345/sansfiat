use serde::{Deserialize, Serialize};

/// Goods the market library trades. The clans piece decides the real list.
pub const GOODS: [&str; 4] = ["grain", "fish", "game", "fruit"];
pub const N: usize = GOODS.len();

pub struct Terrain {
    pub name: &'static str,
    pub color: &'static str,
    pub land: bool,
}

pub const SEA: usize = 0;
pub const COAST: usize = 1;
pub const PLAINS: usize = 2;
pub const FOREST: usize = 3;
pub const HILLS: usize = 4;

pub const TERRAIN: [Terrain; 5] = [
    Terrain { name: "sea", color: "#2b5d8a", land: false },
    Terrain { name: "coast", color: "#c9b77a", land: true },
    Terrain { name: "plains", color: "#9cbf5a", land: true },
    Terrain { name: "forest", color: "#3f7a3a", land: true },
    Terrain { name: "hills", color: "#8a7a5c", land: true },
];

/// Terrain a Voronoi region is drawn from, uniformly. Coast is assigned afterwards.
pub const REGION_TERRAIN: [usize; 9] = [SEA, SEA, PLAINS, PLAINS, PLAINS, FOREST, FOREST, HILLS, HILLS];
pub const REGIONS: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub seed: u32,
    pub radius: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config { seed: 1, radius: 8 }
    }
}
