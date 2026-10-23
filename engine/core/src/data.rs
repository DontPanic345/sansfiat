use serde::{Deserialize, Serialize};

pub const GOODS: [&str; 4] = ["grain", "fish", "game", "fruit"];
pub const N: usize = GOODS.len();

pub struct Terrain {
    pub name: &'static str,
    pub color: &'static str,
    pub land: bool,
    pub yields: [f64; N],
}

pub const SEA: usize = 0;
pub const COAST: usize = 1;
pub const PLAINS: usize = 2;
pub const FOREST: usize = 3;
pub const HILLS: usize = 4;

pub const TERRAIN: [Terrain; 5] = [
    Terrain { name: "sea", color: "#2b5d8a", land: false, yields: [0.0, 0.0, 0.0, 0.0] },
    Terrain { name: "coast", color: "#c9b77a", land: true, yields: [0.5, 3.0, 0.0, 0.5] },
    Terrain { name: "plains", color: "#9cbf5a", land: true, yields: [3.0, 0.0, 0.5, 0.5] },
    Terrain { name: "forest", color: "#3f7a3a", land: true, yields: [0.0, 0.0, 2.0, 1.0] },
    Terrain { name: "hills", color: "#8a7a5c", land: true, yields: [0.5, 0.0, 1.0, 2.0] },
];

/// Terrain a Voronoi region is drawn from, uniformly. Coast is assigned afterwards.
pub const REGION_TERRAIN: [usize; 9] = [SEA, SEA, PLAINS, PLAINS, PLAINS, FOREST, FOREST, HILLS, HILLS];
pub const REGIONS: usize = 10;

/// Each turn every terrain's yield is scaled by a factor drawn uniformly from this range.
pub const SEASON: (f64, f64) = (0.5, 1.5);
/// A clan harvests every land tile within this distance of home.
pub const FORAGE: i32 = 1;
pub const PEOPLE: (f64, f64) = (4.0, 12.0);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Config {
    pub seed: u32,
    pub radius: i32,
    pub clans: usize,
    pub markets: usize,
    /// A clan trades at the nearest market within this hex distance, or not at all.
    pub reach: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config { seed: 1, radius: 8, clans: 40, markets: 3, reach: 4 }
    }
}
