use serde::{Deserialize, Serialize};

/// The foods. Only food exists for now; the market library trades these.
pub const GOODS: [&str; 5] = ["game", "grain", "fruit", "fish", "shellfish"];
pub const FOODS: [&str; 5] = GOODS;
pub const N: usize = GOODS.len();

pub const GAME: usize = 0;
pub const GRAIN: usize = 1;
pub const FRUIT: usize = 2;
pub const FISH: usize = 3;
pub const SHELLFISH: usize = 4;

pub struct Terrain {
    pub name: &'static str,
    pub color: &'static str,
    pub land: bool,
    /// Labour to enter the tile; `None` is impassable.
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
/// not land: clans don't stand on it and nobody walks it. Marsh is wet and rich, and slow.
pub const TERRAIN: [Terrain; 8] = [
    Terrain { name: "sea", color: "#2b5d8a", land: false, move_cost: None, food: [0.0; N] },
    Terrain { name: "coast", color: "#c9b77a", land: true, move_cost: Some(1.0), food: [1.0, 0.0, 0.0, 4.0, 3.0] },
    Terrain { name: "plains", color: "#9cbf5a", land: true, move_cost: Some(1.0), food: [3.0, 4.0, 0.0, 0.0, 0.0] },
    Terrain { name: "forest", color: "#3f7a3a", land: true, move_cost: Some(2.0), food: [3.0, 0.0, 4.0, 0.0, 0.0] },
    Terrain { name: "hills", color: "#8a7a5c", land: true, move_cost: Some(2.0), food: [2.0, 0.0, 1.0, 0.0, 0.0] },
    Terrain { name: "mountain", color: "#6e6a73", land: false, move_cost: None, food: [0.0; N] },
    Terrain { name: "desert", color: "#e0cf94", land: true, move_cost: Some(1.0), food: [0.0; N] },
    Terrain { name: "marsh", color: "#5b8c78", land: true, move_cost: Some(3.0), food: [2.0, 2.0, 0.0, 4.0, 3.0] },
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
    /// Hidden trail costs, as `(value, weight)`: forest, hills, the mountain's cost, and its bridge span.
    pub forest_cost: [(u8, u32); 2],
    pub hills_cost: [(u8, u32); 2],
    pub mountain_cost: u8,
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
    forest_cost: [(1, 1), (2, 1)],
    hills_cost: [(1, 1), (2, 2)],
    mountain_cost: 3,
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

pub const FEATURES: Features = Features { river: [0.0, 1.0, 0.0, 5.0, 0.0] };

pub struct RiverRules {
    /// Extra labour to enter a river tile off its course, unless a trail bridges it.
    pub cross: f64,
}

pub const RIVER: RiverRules = RiverRules { cross: 2.0 };

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
];

/// The range a tile's fertility scales its caps by.
pub const FERTILITY: (f64, f64) = (0.4, 2.0);

pub struct TrailRules {
    /// Turns in a row held before a tile gets a trail.
    pub after: u32,
    /// Turns held by nobody before it wears away.
    pub fade: u32,
    /// Move cost on a trail.
    pub cost: f64,
}

pub const TRAIL: TrailRules = TrailRules { after: 10, fade: 30, cost: 1.0 };

pub struct TendingRules {
    /// Turns in a row held before its grain starts to tame.
    pub after: u32,
    /// Steps of `tame` to a hearth: one a turn held, one back a turn unheld.
    pub tame_turns: u32,
    /// A hearth's grain cap as a multiple of its wild grain.
    pub domestic: f64,
}

pub const TENDING: TendingRules = TendingRules { after: 15, tame_turns: 120, domestic: 2.5 };

pub struct MarketRules {
    /// Turns a home must have been held for a market to form there.
    pub form_after: u32,
    /// Hex steps from every other market.
    pub spacing: i32,
    /// Path cost from a market's tile that counts as its land: for a centre's hearths and the
    /// clans a civ founded there takes in.
    pub reach: f64,
    /// The most a festival route may cost a turn (`walk_cost`, there and back) for a clan to
    /// weigh going, and for a clan to count as in reach of a home where a market might form.
    pub reach_walk: f64,
    /// Clans in reach for a market to form.
    pub min_clans: usize,
    /// How much of last turn's activity is kept.
    pub fade: f64,
    /// A market whose activity falls under this goes.
    pub min_volume: f64,
    /// Labour a clan with a surplus is willing to walk to gather, on top of `desire`.
    pub gather_bonus: f64,
    /// Labour any clan is willing to walk to gather.
    pub desire: f64,
    /// More labour for each clan that came to the festival last turn ("to see others")…
    pub company: f64,
    /// …counting at most this many.
    pub company_cap: usize,
    /// How much more another festival must be worth, in labour, for a clan to change to it.
    pub switch_margin: f64,
    /// A store of at least this × people is a surplus.
    pub surplus_at: f64,
}

pub const MARKET: MarketRules = MarketRules {
    form_after: 20,
    spacing: 8,
    reach: 3.0,
    reach_walk: 12.0,
    min_clans: 4,
    fade: 0.9,
    min_volume: 1.0,
    gather_bonus: 6.0,
    desire: 1.0,
    company: 0.5,
    company_cap: 6,
    switch_margin: 1.0,
    surplus_at: 0.1,
};

pub struct PregameRules {
    /// Centres the pre-game waits for.
    pub players: usize,
    /// Turns a centre's market has lasted.
    pub centre_age: u32,
    /// Clans that traded at a centre last turn.
    pub centre_clans: usize,
    /// Hex steps between centres.
    pub centre_spacing: i32,
    /// Share of the centres' land near some clan's territory…
    pub cover: f64,
    /// …within this many hex steps.
    pub cover_steps: i32,
    /// The backstop.
    pub max_turns: u32,
}

pub const PREGAME: PregameRules = PregameRules {
    players: 3,
    centre_age: 30,
    centre_clans: 4,
    centre_spacing: 10,
    cover: 0.8,
    cover_steps: 3,
    max_turns: 500,
};

pub struct ClanRules {
    /// Seed clans.
    pub seeds: usize,
    /// People in a seed clan.
    pub start_pop: f64,
    /// Land tiles for a landmass to get its own seed.
    pub min_landmass: usize,
    /// Labour a person gives a turn.
    pub work: f64,
    /// Labour to harvest one unit of food.
    pub harvest_labour: f64,
    /// Most of a tile's stock a clan takes in a turn.
    pub take_rate: f64,
    /// A clan harvests up to this × need, less its store.
    pub surplus_target: f64,
    /// A food counts in full up to this share of need…
    pub diet_share: f64,
    /// …and at this beyond it.
    pub excess_value: f64,
    /// The most a fed clan grows a turn: just moved → settled.
    pub growth: (f64, f64),
    /// It grows that much when this turn's harvest beyond need is this × need, less in proportion.
    pub grow_full: f64,
    /// Turns to reach the settled growth rate.
    pub settle_turns: u32,
    /// Fed below this share, a clan starves.
    pub stable_at: f64,
    /// How fast a starving clan shrinks.
    pub starve: f64,
    /// Share of the store lost each turn.
    pub spoilage: f64,
    /// A clan this big divides.
    pub split_at: f64,
    /// A starving clan this big sends a daughter.
    pub hungry_split: f64,
    /// A daughter's site must feed her this well.
    pub settle_fed: f64,
    /// Starving turns before a clan moves on.
    pub patience: u32,
    /// Steps a clan looks when moving on.
    pub move_range: i32,
    /// How much better a site must be to move.
    pub move_margin: f64,
    /// Steps a daughter looks.
    pub scout_range: i32,
    /// Softness of site choice.
    pub choice_spread: f64,
    /// Gain in value (food per person) a hungry clan's change of territory must make.
    pub land_margin: f64,
    /// No territory tile is further than this from home, in hex steps.
    pub territory_radius: i32,
    /// The weight of a clan's walks to its own tiles (`actors::walk_cost`): little, but not nothing.
    pub inside: f64,
}

pub const CLAN: ClanRules = ClanRules {
    seeds: 3,
    start_pop: 6.0,
    min_landmass: 30,
    work: 3.5,
    harvest_labour: 1.0,
    take_rate: 0.5,
    surplus_target: 1.5,
    diet_share: 0.4,
    excess_value: 0.3,
    growth: (0.03, 0.07),
    grow_full: 0.3,
    settle_turns: 30,
    stable_at: 0.8,
    starve: 0.5,
    spoilage: 0.1,
    split_at: 12.0,
    hungry_split: 8.0,
    settle_fed: 0.9,
    patience: 3,
    move_range: 4,
    move_margin: 0.1,
    scout_range: 6,
    choice_spread: 0.05,
    land_margin: 0.02,
    territory_radius: 2,
    inside: 0.25,
};

/// The civs founding makes, in the order of the centres: name and colour.
pub const CIVS: [(&str, &str); 3] = [("Red", "#d64545"), ("Blue", "#3b6fd6"), ("Gold", "#d6a63b")];

pub struct CivRules {
    /// Hex steps from the capital that count as the capital's land.
    pub territory: i32,
    /// What a civ's store starts with, standing in for the boon year.
    pub endowment: [f64; N],
    /// The share of its store a fed civ clan hands the civ each turn.
    pub levy: f64,
    /// The most goods, in all, a civ's store holds; the excess is lost.
    pub store_cap: f64,
}

pub const CIV: CivRules = CivRules { territory: 2, endowment: [0.0, 30.0, 0.0, 0.0, 0.0], levy: 0.5, store_cap: 50.0 };

pub struct UnitRules {
    /// One person's work a turn: what a unit spends on its steps (`CLAN.work`).
    pub labour: f64,
    /// What a unit eats a turn from its civ's store: one person's need.
    pub food: f64,
    /// Hungry turns in a row before a unit disbands.
    pub hungry_turns: u32,
    /// The fewest people a clan must have to give one to a unit.
    pub min_clan: f64,
}

pub const UNIT: UnitRules = UnitRules { labour: 3.5, food: 1.0, hungry_turns: 3, min_clan: 2.0 };

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub seed: u32,
    pub radius: i32,
}

impl Default for Config {
    /// Radius 16: three centres 10 steps apart need the room.
    fn default() -> Self {
        Config { seed: 1, radius: 16 }
    }
}
