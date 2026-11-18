//! The map's terrain, sizes, features, rivers and the foods a tile yields.

use super::*;

pub struct Terrain {
    pub name: &'static str,
    pub color: &'static str,
    pub land: bool,
    /// Days a step to enter the tile; `None` is impassable.
    pub move_cost: Option<f64>,
    /// Each food's cap before fertility.
    pub food: [f64; N],
}

pub const SEA: usize = 0;
pub const COAST: usize = 1;
pub const PLAINS: usize = 2;
pub const FOREST: usize = 3;
pub const HILLS: usize = 4;
pub const MOUNTAIN: usize = 5;
pub const DESERT: usize = 6;
pub const MARSH: usize = 7;

/// Coast tiles stand for the shore and the shallows, so they carry the sea foods. A mountain is
/// not land: clans don't stand on it and nobody walks it. Marsh is wet and rich, and slow. Desert
/// is barren and costs 2 to enter: clans route round it, not through it (owner note 1).
pub const TERRAIN: [Terrain; 8] = [
    Terrain { name: "sea", color: "#2b5d8a", land: false, move_cost: None, food: [0.0; N] },
    Terrain {
        name: "coast",
        color: "#c9b77a",
        land: true,
        move_cost: Some(1.0 * DAY),
        food: [1.0, 0.0, 0.0, 4.0, 3.0, 0.0, 0.0],
    },
    Terrain {
        name: "plains",
        color: "#9cbf5a",
        land: true,
        move_cost: Some(1.0 * DAY),
        food: [3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    },
    Terrain {
        name: "forest",
        color: "#3f7a3a",
        land: true,
        move_cost: Some(2.0 * DAY),
        food: [3.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0],
    },
    Terrain {
        name: "hills",
        color: "#8a7a5c",
        land: true,
        move_cost: Some(2.0 * DAY),
        food: [2.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    },
    Terrain { name: "mountain", color: "#6e6a73", land: false, move_cost: None, food: [0.0; N] },
    Terrain { name: "desert", color: "#e0cf94", land: true, move_cost: Some(2.0 * DAY), food: [0.0; N] },
    Terrain {
        name: "marsh",
        color: "#5b8c78",
        land: true,
        move_cost: Some(3.0 * DAY),
        food: [2.0, 2.0, 0.0, 4.0, 3.0, 0.0, 0.0],
    },
];

/// The map sizes, by name: the radius of the hex.
pub const SIZES: [(&str, i32); 3] = [("small", 16), ("medium", 23), ("large", 33)];

/// The tiles of the small map, which the map rules' counts are per.
pub const SMALL_TILES: usize = 817;

/// The radius of the size called `name`.
pub fn size_radius(name: &str) -> Option<i32> {
    SIZES.iter().find(|(n, _)| *n == name).map(|&(_, r)| r)
}

pub struct MapRules {
    /// Per `SMALL_TILES` tiles; scaled to the map's area, never below `ridges_floor`.
    pub ridges: usize,
    pub ridges_floor: usize,
    /// Tiles in a ridge, `lo..hi`.
    pub ridge_length: (usize, usize),
    /// Chance a ridge tile is a pass (hills) rather than mountain.
    pub pass_chance: f64,
    /// Chance a plains, forest or desert tile beside a ridge tile turns to hills.
    pub flank_hills: f64,
    /// Water cut in from the coast, and blobs of inland sea, and lakes (per small map).
    pub inlets: usize,
    pub inland_seas: usize,
    pub lakes: usize,
    /// The share of the land (not sea) that is desert.
    pub desert_share: f64,
    /// Tiles upwind a ridge shades a plains tile, and how much a ridge tile there takes off its score.
    pub shadow_reach: i32,
    pub shadow_weight: f64,
    /// Hidden trail costs, as `(value, weight)`: forest, hills, the mountain's cost (days a step), and its bridge span (hex steps).
    pub forest_cost: [(f64, u32); 2],
    pub hills_cost: [(f64, u32); 2],
    pub mountain_cost: f64,
    pub mountain_span: [(Option<u8>, u32); 3],
    /// River sources tried (per small map, never below `rivers_floor`), the steps of land a
    /// source must be from water (fewer where the map's land isn't that deep: one less than its
    /// deepest), and the chance a river sidesteps when it could go downhill.
    pub rivers: usize,
    pub rivers_floor: usize,
    pub river_min: u32,
    /// Noisy height from which land counts as high ground a river may start on, as hills and
    /// tiles beside a mountain do (sea is under 0.35, hills over 0.72).
    pub upland: f64,
    pub river_wiggle: f64,
    /// A river this big (tiles upstream) floods its last `flood_reach` steps to its mouth.
    pub big_river: u32,
    pub flood_reach: u32,
    /// A river this big ends in a delta where it meets the ocean.
    pub delta_river: u32,
    /// Floodplain and delta fertility: above `FERTILITY.1`, the richest land.
    pub flood_fertility: f64,
    /// The radius from which a map has closed country and its open land is held to three sites.
    pub closed_from: i32,
    /// Sealed valleys, valleys behind a pass and peninsulas on a neck (per small map, never below
    /// their floors), each skipped after `closed_tries_each` random tries that don't fit.
    pub sealed: usize,
    pub sealed_floor: usize,
    pub valleys: usize,
    pub valleys_floor: usize,
    pub necks: usize,
    pub necks_floor: usize,
    pub closed_tries_each: usize,
    /// Hex steps between closed features (and from a feature to the rim).
    pub closed_clearance: i32,
    /// Land tiles a valley's inside must have before it is ringed, and a peninsula's range.
    pub valley_land: usize,
    pub neck_land: (usize, usize),
    /// Closed-country layouts tried before they are dropped, and whole maps re-rolled after that,
    /// until the open land holds `PREGAME.players` sites.
    pub closed_tries: usize,
    pub map_tries: usize,
    /// A site for a centre: `site_tiles` open tiles with grain or fruit within `site_steps`
    /// steps, of mean fertility `site_fertility` or more.
    pub site_steps: i32,
    pub site_tiles: usize,
    pub site_fertility: f64,
}

pub const MAP: MapRules = MapRules {
    ridges: 3,
    ridges_floor: 1,
    ridge_length: (10, 24),
    pass_chance: 0.06,
    flank_hills: 0.4,
    inlets: 3,
    inland_seas: 1,
    lakes: 3,
    desert_share: 0.2,
    shadow_reach: 5,
    shadow_weight: 0.25,
    forest_cost: [(1.0 * DAY, 1), (2.0 * DAY, 1)],
    hills_cost: [(1.0 * DAY, 1), (2.0 * DAY, 2)],
    mountain_cost: 3.0 * DAY,
    mountain_span: [(Some(1), 1), (Some(2), 1), (None, 8)],
    rivers: 16,
    rivers_floor: 3,
    river_min: 6,
    upland: 0.6,
    river_wiggle: 0.25,
    big_river: 8,
    flood_reach: 6,
    delta_river: 10,
    flood_fertility: 2.2,
    closed_from: 16,
    sealed: 1,
    sealed_floor: 1,
    valleys: 2,
    valleys_floor: 1,
    necks: 2,
    necks_floor: 1,
    closed_tries_each: 50,
    closed_clearance: 3,
    valley_land: 7,
    neck_land: (8, 20),
    closed_tries: 8,
    map_tries: 8,
    site_steps: 3,
    site_tiles: 12,
    site_fertility: 0.8,
};

pub struct Features {
    /// Food a river adds to its tile's caps, before fertility.
    pub river: [f64; N],
}

pub const FEATURES: Features = Features { river: [0.0, 1.0, 0.0, 5.0, 0.0, 0.0, 0.0] };

pub struct RiverRules {
    /// Extra days a step to enter a river tile off its course, unless a trail bridges it.
    pub cross: f64,
}

pub const RIVER: RiverRules = RiverRules { cross: 2.0 * DAY };

/// `per_small` features per `SMALL_TILES` tiles, for a map of `tiles`, rounded.
pub fn scaled(per_small: usize, tiles: usize) -> usize {
    (per_small * tiles + SMALL_TILES / 2) / SMALL_TILES
}

pub struct Food {
    /// Share of the cap that grows back a turn (for game, the breeding rate).
    pub regen: f64,
    /// Breeds: regrows by `regen × stock × (1 − stock / cap)` plus `restock × cap`.
    pub breeds: bool,
    pub restock: f64,
}

pub const FOOD: [Food; N] = [
    Food { regen: 0.35, breeds: true, restock: 0.02 },
    Food { regen: 0.25, breeds: false, restock: 0.0 },
    Food { regen: 0.2, breeds: false, restock: 0.0 },
    Food { regen: 0.3, breeds: false, restock: 0.0 },
    Food { regen: 0.2, breeds: false, restock: 0.0 },
    Food { regen: 0.0, breeds: false, restock: 0.0 },
    Food { regen: 0.0, breeds: false, restock: 0.0 },
];

/// The range a tile's fertility scales its caps by.
pub const FERTILITY: (f64, f64) = (0.4, 2.0);
