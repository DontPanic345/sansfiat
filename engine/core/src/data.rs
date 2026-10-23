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

/// Coast tiles stand for the shore and the shallows, so they carry the sea foods.
pub const TERRAIN: [Terrain; 5] = [
    Terrain { name: "sea", color: "#2b5d8a", land: false, move_cost: None, food: [0.0; N] },
    Terrain { name: "coast", color: "#c9b77a", land: true, move_cost: Some(1.0), food: [1.0, 0.0, 0.0, 4.0, 3.0] },
    Terrain { name: "plains", color: "#9cbf5a", land: true, move_cost: Some(1.0), food: [3.0, 4.0, 0.0, 0.0, 0.0] },
    Terrain { name: "forest", color: "#3f7a3a", land: true, move_cost: Some(2.0), food: [3.0, 0.0, 4.0, 0.0, 0.0] },
    Terrain { name: "hills", color: "#8a7a5c", land: true, move_cost: Some(2.0), food: [2.0, 0.0, 1.0, 0.0, 0.0] },
];

/// Terrain a Voronoi region is drawn from, uniformly. Coast is assigned afterwards.
pub const REGION_TERRAIN: [usize; 9] = [SEA, SEA, PLAINS, PLAINS, PLAINS, FOREST, FOREST, HILLS, HILLS];
pub const REGIONS: usize = 10;

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
    /// Turns in a row on a round before a tile gets a trail.
    pub after: u32,
    /// Turns on nobody's round before it wears away.
    pub fade: u32,
    /// Move cost on a trail.
    pub cost: f64,
}

pub const TRAIL: TrailRules = TrailRules { after: 10, fade: 30, cost: 1.0 };

pub struct TendingRules {
    /// Turns in a row on a round before its grain starts to tame.
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
    /// Path cost from the market a clan's round must come within.
    pub reach: f64,
    /// Clans in reach for a market to form.
    pub min_clans: usize,
    /// How much of last turn's activity is kept.
    pub fade: f64,
    /// A market whose activity falls under this goes.
    pub min_volume: f64,
    /// Labour a clan with a surplus is willing to walk to gather.
    pub gather_bonus: f64,
    /// A store of at least this × people is a surplus.
    pub surplus_at: f64,
}

pub const MARKET: MarketRules = MarketRules {
    form_after: 20,
    spacing: 5,
    reach: 3.0,
    min_clans: 4,
    fade: 0.8,
    min_volume: 2.0,
    gather_bonus: 2.0,
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
    /// A clan plans to harvest up to this × need, less its store.
    pub surplus_target: f64,
    /// A food counts in full up to this share of need…
    pub diet_share: f64,
    /// …and at this beyond it.
    pub excess_value: f64,
    /// Growth when fed: just moved → settled.
    pub growth: (f64, f64),
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
    /// Gain a round change must make.
    pub land_margin: f64,
    /// No round tile is further than this from home.
    pub territory_radius: i32,
    /// Path cost from the round a clan will fetch food.
    pub fetch_range: f64,
}

pub const CLAN: ClanRules = ClanRules {
    seeds: 3,
    start_pop: 6.0,
    min_landmass: 30,
    work: 2.5,
    harvest_labour: 1.0,
    take_rate: 0.5,
    surplus_target: 1.5,
    diet_share: 0.4,
    excess_value: 0.3,
    growth: (0.03, 0.07),
    settle_turns: 30,
    stable_at: 0.8,
    starve: 0.5,
    spoilage: 0.3,
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
    fetch_range: 3.0,
};

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
