use serde::{Deserialize, Serialize};

// A day is one person's labour for one day; a turn is a year of `CLAN.work` days, the days a person
// gives (`labour.md`). Every number's doc says its unit.

/// The days a bare plains step takes, so a trail on flat land (`TRAIL.flat` of it) takes one day.
/// Every cost and labour is written as its old number times `DAY`, every food a day over `DAY`.
pub const DAY: f64 = 4.0 / 3.0;

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
        food: [1.0, 0.0, 0.0, 4.0, 3.0],
    },
    Terrain {
        name: "plains",
        color: "#9cbf5a",
        land: true,
        move_cost: Some(1.0 * DAY),
        food: [3.0, 4.0, 0.0, 0.0, 0.0],
    },
    Terrain {
        name: "forest",
        color: "#3f7a3a",
        land: true,
        move_cost: Some(2.0 * DAY),
        food: [3.0, 0.0, 4.0, 0.0, 0.0],
    },
    Terrain {
        name: "hills",
        color: "#8a7a5c",
        land: true,
        move_cost: Some(2.0 * DAY),
        food: [2.0, 0.0, 1.0, 0.0, 0.0],
    },
    Terrain { name: "mountain", color: "#6e6a73", land: false, move_cost: None, food: [0.0; N] },
    Terrain { name: "desert", color: "#e0cf94", land: true, move_cost: Some(2.0 * DAY), food: [0.0; N] },
    Terrain {
        name: "marsh",
        color: "#5b8c78",
        land: true,
        move_cost: Some(3.0 * DAY),
        food: [2.0, 2.0, 0.0, 4.0, 3.0],
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

pub const FEATURES: Features = Features { river: [0.0, 1.0, 0.0, 5.0, 0.0] };

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
];

/// The range a tile's fertility scales its caps by.
pub const FERTILITY: (f64, f64) = (0.4, 2.0);

pub struct TrailRules {
    /// How much of a tile's traffic is kept from one turn to the next.
    pub keep: f64,
    /// A tile whose traffic (people-walks, smoothed) reaches this gets a trail…
    pub on: f64,
    /// …and keeps it until its traffic falls under this (`off < on`).
    pub off: f64,
    /// Days to finish a built trail, by the terrain it goes on (`scout`'s work: forest, hills, a
    /// mountain pass). A trail brings a tile to its hidden `trail_cost`, not to a flat cost.
    pub work: [(usize, f64); 3],
    /// A share: of its bare cost (days a step) that a trail brings plains and coast to (M4 answer 4). It is the
    /// same on every flat tile, so nothing about it is hidden, and the scout builds none there.
    pub flat: f64,
}

/// Traffic is people-walks, so a tile's steady traffic is ten times a turn's. On the retro maps at
/// the pick a home carries a median of ~185 (a quarter under ~140) and a tile two clans' routes
/// share ~225, while a worked tile carries ~70 (a quarter over ~110): `on` 110 trails most homes
/// and shared routes and under a third of the worked tiles; `off` 90 lets a trail ride out a dip
/// without keeping the ones once worn.
pub const TRAIL: TrailRules = TrailRules {
    keep: 0.9,
    on: 110.0,
    off: 90.0,
    work: [(FOREST, 2.0 * DAY), (HILLS, 3.0 * DAY), (MOUNTAIN, 6.0 * DAY)],
    flat: 0.75,
};

pub struct DepletionRules {
    /// Wear a food gains a turn for each share of its cap taken past its sustainable yield.
    pub rate: f64,
    /// Wear a food loses a turn it is taken at or under its yield (or not at all).
    pub recover: f64,
    /// The most wear a food takes: its cap never falls below `1 - max` of its unworn cap.
    pub max: f64,
    /// A tile is depleted when some food's wear is at least this.
    pub depleted_at: f64,
}

pub const DEPLETION: DepletionRules = DepletionRules { rate: 0.2, recover: 0.05, max: 0.6, depleted_at: 0.1 };

pub struct TendingRules {
    /// Turns in a row held before its grain starts to tame.
    pub after: u32,
    /// Steps of `tame` to a hearth: one a turn held, one back a turn unheld.
    pub tame_turns: u32,
    /// A hearth's grain cap as a multiple of its wild grain.
    pub domestic: f64,
    /// Past a hearth, the grain cap rises by this multiple of its wild grain for each further
    /// `tame_turns` of taming: slower than the way to a hearth, and never capped.
    pub beyond: f64,
}

pub const TENDING: TendingRules = TendingRules { after: 15, tame_turns: 120, domestic: 2.5, beyond: 0.1 };

/// The field job (`fields.rs`): where a clan's fields lie, what a field day yields, and how its
/// split between gathering and fields moves. Build-4 tunes these against the retro maps.
pub struct FieldRules {
    /// Days of fields a floodplain takes a turn.
    pub flood: f64,
    /// Days of fields a river tile takes a turn, at `MAP.big_river` or more; in proportion below it.
    pub river: f64,
    /// Food a field day yields, per unit of the tile's fertility and of its tame past wild (`domestic − 1`).
    pub grain: f64,
    /// The share of its days a clan's split moves a turn.
    pub notch: f64,
    /// Food a person a turn by which the better split must beat the current one.
    pub margin: f64,
    /// The share from which a clan is a farmer, whose home follows its fields.
    pub farmer: f64,
    /// Steps of tame a turn a field adds to its tile, on top of the one a held tile gains.
    pub tame: u32,
    /// Why these numbers: where they came from, and what tuning them did (relay/specs/155/canvas.md).
    pub note: &'static str,
}

pub const FIELD: FieldRules = FieldRules {
    flood: 15.0 * DAY,
    river: 7.5 * DAY,
    grain: 0.8,
    notch: 0.1,
    margin: 0.01,
    farmer: 0.2,
    tame: 1,
    note: "Tuned on the retro maps (relay/specs/155/canvas.md), from the probe's y8 values: a floodplain feeds a \
           small clan's days, a field day yields 1.2 food at a hearth against a gathering day's 0.75, a split \
           moves a tenth at a time, and a clan is a farmer, its home following its fields, from a fifth of its \
           days: settled farmers' home moves are a quarter of gatherers', and the store holds a pack on 5 of 7 \
           maps.",
};

/// Towns: a tile with a festival and this many homes reads as a town (`towns.rs`). It is the reading, not
/// a rule: nothing acts on it. Build-4 adds its rule's numbers here.
pub struct TownRules {
    /// Homes on a festival's tile from which it reads as a town.
    pub homes: usize,
    /// Food a person a turn by which a farmer's home step (toward its fields or its festival) must beat
    /// staying (`homes::step`).
    pub margin: f64,
    /// Why this number: where it came from.
    pub note: &'static str,
}

pub const TOWN: TownRules = TownRules {
    homes: 3,
    margin: 0.01,
    note: "Three is the least that reads as many hearths on one tile, and two clans side by side are \
           neighbours (relay/specs/156/spec.md). The sketch's towns at three came out on 5 of 7 maps. \
           A home step must beat staying by the field split's margin, `FIELD.margin`. Tuned on the retro maps \
           (relay/specs/156/canvas.md): 0.005 and 0.02 form no more towns.",
};

/// What a person carries on a route: a walk beyond the territory is paid by every person carrying
/// what the actor brings (`actors::carriers`).
pub struct CarryRules {
    /// Food a person carries on a route. A route's walkers are what the actor brings over this, at
    /// least one: a clan's festival route, and a civ clan's route to the capital.
    pub load: f64,
}

/// A load of 2 food is about nine days' eating (a person eats a food a turn of `CLAN.work`, 4.67 days),
/// "some days' food" (`labour.md`). A clan brings at most `MARKET.carry` × people, 0.2 a person, so a load
/// of 3 or more sends every clan with one walker and changes nothing, and 1 collapses the festivals
/// (`walk.md`).
pub const CARRY: CarryRules = CarryRules { load: 2.0 };

pub struct MarketRules {
    /// Turns a home must have been held for a market to form there.
    pub form_after: u32,
    /// Hex steps from every other market.
    pub spacing: i32,
    /// Days of path cost from a market's tile that count as its land: for a centre's hearths and the
    /// clans a civ founded there takes in.
    pub reach: f64,
    /// The most days a festival route may cost a turn (`walk_cost`, there and back) for a clan to
    /// weigh going, and for a clan to count as in reach of a home where a market might form.
    pub reach_walk: f64,
    /// How much of last turn's activity is kept.
    pub fade: f64,
    /// A market whose activity (clans that came, smoothed) falls under this goes.
    pub min_attendance: f64,
    /// The most food a clan brings to a festival, food a person.
    pub carry: f64,
    /// A starving clan offers this × people × (1 − fed_ratio) people at a festival.
    pub shed: f64,
    /// Days a clan with a surplus is willing to walk to gather, on top of `desire`.
    pub gather_bonus: f64,
    /// Days any clan is willing to walk to gather.
    pub desire: f64,
    /// More days for each clan that came to the festival last turn ("to see others")…
    pub company: f64,
    /// …counting at most this many.
    pub company_cap: usize,
    /// How many more days another festival must be worth for a clan to change to it.
    pub switch_margin: f64,
    /// Turns a clan keeps a festival it has just started going to, whatever the worths.
    pub keep_turns: u32,
    /// After `keep_turns` a clan keeps its festival while its worth is above minus this.
    pub stay_margin: f64,
    /// A store of at least this × people is a surplus.
    pub surplus_at: f64,
}

pub const MARKET: MarketRules = MarketRules {
    form_after: 20,
    spacing: 6,
    reach: 3.0 * DAY,
    reach_walk: 14.0 * DAY,
    fade: 0.9,
    min_attendance: 0.5,
    carry: 0.2,
    shed: 1.0,
    gather_bonus: 8.0 * DAY,
    desire: 3.0 * DAY,
    company: 1.0 * DAY,
    company_cap: 6,
    switch_margin: 1.0 * DAY,
    keep_turns: 5,
    stay_margin: 3.0 * DAY,
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
    /// People must have changed by under this share…
    pub settled: f64,
    /// …over this many turns.
    pub settle_turns: usize,
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
    settled: 0.05,
    settle_turns: 100,
    max_turns: 500,
};

pub struct ClanRules {
    /// Seed clans (people: `start_pop`).
    pub seeds: usize,
    /// People in a seed clan.
    pub start_pop: f64,
    /// Land tiles for a landmass to get its own seed.
    pub min_landmass: usize,
    /// Days a person gives a turn.
    pub work: f64,
    /// Days a food to harvest: labour to harvest one unit of food.
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
    /// A clan under this many people at a festival merges into another there.
    pub min_pop: f64,
    /// A daughter's site must feed her this well.
    pub settle_fed: f64,
    /// Starving turns before a clan moves on.
    pub patience: u32,
    /// Hex steps a clan looks when moving on.
    pub move_range: i32,
    /// How much better a site must be to move.
    pub move_margin: f64,
    /// Hex steps a daughter looks.
    pub scout_range: i32,
    /// Softness of site choice.
    pub choice_spread: f64,
    /// Gain in value (food per person) a hungry clan's change of territory must make.
    pub land_margin: f64,
    /// No territory tile is further than this from home, in hex steps.
    pub territory_radius: i32,
    /// A share: the weight of a clan's walks to its own tiles (`actors::walk_cost`): little, but not nothing.
    /// Every person walking a tile pays it: the clan's people over its tiles (`actors::split`).
    /// 0.2 (the weight a camp's zone and a guard's claim walk at): the territory walk at 0.125 with a
    /// load of 2 held the pre-game and the festivals but left the AI's claims and the land that changes
    /// hands under three quarters of the controls on every pick tried; 0.25 and 0.2 held (`walk.md`).
    pub inside: f64,
}

pub const CLAN: ClanRules = ClanRules {
    seeds: 3,
    start_pop: 6.0,
    min_landmass: 30,
    work: 3.5 * DAY,
    harvest_labour: 1.0 * DAY,
    take_rate: 0.5,
    surplus_target: 2.0,
    diet_share: 0.4,
    excess_value: 0.3,
    growth: (0.05, 0.15),
    grow_full: 0.05,
    settle_turns: 30,
    stable_at: 0.8,
    starve: 0.5,
    spoilage: 0.1,
    split_at: 8.0,
    hungry_split: 5.0,
    min_pop: 2.0,
    settle_fed: 0.9,
    patience: 3,
    move_range: 4,
    move_margin: 0.1,
    scout_range: 14,
    choice_spread: 0.05,
    land_margin: 0.02,
    territory_radius: 2,
    inside: 0.2,
};

/// The civs founding makes, in the order of the centres: name and colour.
pub const CIVS: [(&str, &str); 3] = [("Red", "#d64545"), ("Blue", "#3b6fd6"), ("Gold", "#d6a63b")];

pub struct CivRules {
    /// What a civ's store starts with, standing in for the boon year.
    pub endowment: [f64; N],
    /// The widest a mountain's span may be for a trail to bridge it: every civ has the shortest span
    /// for now, standing in for the bridge-span technology.
    pub bridge_span: u8,
    /// The share of what it brings (its basket) a civ clan pays into the store when it walks to the
    /// capital's market, while the store has room; its harvest drops by the same share.
    pub levy: f64,
    /// The most goods, in all, a civ's store holds; the excess is lost.
    pub store_cap: f64,
    /// What a civ clan makes of going to its capital's market over another festival, in days.
    pub pull: f64,
}

pub const CIV: CivRules =
    CivRules { endowment: [0.0, 20.0, 0.0, 0.0, 0.0], bridge_span: 1, levy: 0.2, store_cap: 60.0, pull: 3.0 * DAY };

pub struct ScoutRules {
    /// People a scout is made with, from the civ's clans, one each.
    pub people: u32,
    /// Days to reveal a forest or hills tile (a mountain takes a whole turn's search labour).
    pub reveal: f64,
    /// Hex steps from its tile a scout forages: its own tile and those within this many steps, each walk
    /// there and back paid from its labour. A scout on a long errand has no line to live off, and
    /// the tile it stands on is soon bare; one step more finds a tile that isn't.
    pub reach: usize,
    /// Hex steps from its tile a scout looks for rich ground when it runs low: ground it can walk to where
    /// a turn's foraging brings the most, so a scout far from its line finds a meal, not a march.
    pub wander: usize,
    /// Turns of eating (food a person a turn) in the pack under which a routed scout leaves its errand for rich ground.
    pub low: f64,
    /// Turns of eating in the pack at which a wandering scout is fed and goes back to its errand: the
    /// gap between `low` and `fed` stops it leaving again the turn after it returns.
    pub fed: f64,
    /// Turns of eating a turn's foraging must bring for ground to count as rich: ground that only just
    /// feeds a scout is left again as soon as it takes from it.
    pub rich: f64,
}

// Tuned against the 28 errands on the seven retro maps and the AI's scouts (relay/specs/153/errand.md): of the
// settings that keep every scout alive, the most tiles and trails on the 14 routed errands. Wander 4 left the
// ground a scout needed out of range (6 of 28 lost); 8 finds it (10 and 12 find more work, but then one of the
// AI's idle scouts loses a person on medium 9). Rich 1.5 passed over ground that only just fed a scout, and it
// starved beside it; 1.2 takes it. Low 1.5 and fed 2.5 leave the errand later and come back sooner than 2 and 4:
// the same lives for a third fewer turns off the errand. Two people: one lost 2 to 8 of 28 at every setting.
// The scout walks at one person's pace (`walk.md`): its turn's steps halve. Rich 0.8 takes ground that only
// half feeds it and so loses one scout fewer than 1.2 (4 of 28, all on medium 9, against 5) and reveals and
// trails more (34 tiles and 6 trails against 28 and 4); wander, low and fed at 6 to 12, 1.5 to 3 and 2.5 to 5
// lost 4 or 5 (walk.md). Medium 9's four scouts starve with the walk off as well: its pick, not the walk.
pub const SCOUT: ScoutRules =
    ScoutRules { people: 2, reveal: 1.0 * DAY, reach: 1, wander: 8, low: 1.5, fed: 2.5, rich: 0.8 };

pub struct LeaningRules {
    /// Leaning a clan gains towards a civ per food a person it gained by trading at the civ's
    /// market on a turn the civ posted.
    pub trade: f64,
    /// Leaning a gift buys: `gift × v × ln(1 + g / (gift_scale × people))` for `g` food given.
    pub gift: f64,
    /// Food a person that a gift must reach to count much: the log's scale.
    pub gift_scale: f64,
    /// How much more a starving clan values a gift, and how much more of the gift it is given.
    pub starving: f64,
    /// The share of a leaning lost each turn.
    pub fade: f64,
    /// The leaning at which a clan joins the civ.
    pub join: f64,
    /// The food a turn every civ gives at most when founded.
    pub gift_default: f64,
    /// A leaning under this is dropped.
    pub floor: f64,
    /// Leaning a clan gains each turn towards the civ whose held land its home is on.
    pub land: f64,
    /// Further leaning a turn per share of guard its home tile has (`Civ.guard`).
    pub guarded: f64,
    /// Turns in a row a civ clan must be drawn to a rival before it leaves for it.
    pub leave_run: u32,
    /// The leaning to a rival a civ clan must reach to be drawn to it at all; below it, it never leaves.
    pub leave_at: f64,
}

pub const LEANING: LeaningRules = LeaningRules {
    trade: 1.0,
    gift: 0.3,
    gift_scale: 0.5,
    starving: 5.0,
    fade: 0.03,
    join: 1.0,
    gift_default: 2.0,
    floor: 0.001,
    // A clan on a held home gains land + guarded × guard a turn, against a fade of 0.03: land alone
    // settles at 3.3, past the join line, so a held home joins in about 11 turns (canvas.md).
    land: 0.1,
    guarded: 0.5,
    // Leaving after 5 turns drawn at 0.1: a held rival home wins a clan; 10/0.3 left none (canvas.md).
    leave_run: 5,
    leave_at: 0.1,
};

/// The land a civ claims and holds (`claim.rs`). The floor is the guard share a claimed tile must keep
/// to be held; a civ's first claim is every land tile within `ring` steps of the capital or of
/// a clan's home within `reach` steps of it.
pub struct ClaimRules {
    /// A share (of a person's days a turn).
    pub floor: f64,
    /// Hex steps.
    pub ring: i32,
    /// Hex steps.
    pub reach: i32,
}

// The floor 0.05 lets three guards hold the ring around the clans' homes near the capital; 0.1 held 13 tiles a
// civ, 0.05 holds 16 and wins 14 of 21 solo gains (canvas.md). Ring 1 and reach 4 keep the
// stand-in near; ring 2 or reach 6 claimed more than the guard holds and the solo gain fell.
pub const CLAIM: ClaimRules = ClaimRules { floor: 0.05 * DAY, ring: 1, reach: 4 };

/// The planner (`units/ai.rs`): what the AI's civs, and the automatic player, weigh each turn.
pub struct PlanRules {
    /// A share: the levy a turn must cover the civ's mouths and one more this many times over before
    /// it raises that one (a warrior, or the guard a wider claim needs).
    pub feed: f64,
    /// A share of the store's cap: a store this full has its levy cut to the room left, so the levy
    /// counts as feeding one more whenever it covers the mouths at all.
    pub full: f64,
    /// Turns of its mouths' food the store keeps beyond a new warrior's pack.
    pub reserve: f64,
    /// The most of a civ's people its army may be, a share.
    pub army: f64,
    /// Food a turn it gives while a clan at its festival is a rival's or leans to one.
    pub gift_contested: f64,
    /// Tiles a turn its claim widens by, while all of it is held and it can feed one more guard.
    pub widen: usize,
    /// How many floors of guard every held tile must have before an unfed guard sends a person home.
    pub thick: f64,
    /// Hex steps: the claim widens only toward a clan's home this near its edge.
    pub widen_reach: i32,
    /// How many times a defender's strength an army's must be before it marches onto it.
    pub edge: f64,
    /// Turns: a capital the army outweighs falls to it within this many, by the fight's own rule.
    pub siege: u32,
    /// The share of its people an army must have left when the capital falls.
    pub left: f64,
    /// Turns' walk: the farthest capital an army larger than a party marches on, on its pack.
    pub march: u32,
    /// People: the smallest army that leaves its post to pillage or to cut a line.
    pub raid: f64,
    /// A share of an army's days: the most its line may take where it pillages (`units::ring`).
    pub reach: f64,
    /// The share of its pack an army must hold to leave its own land.
    pub packed: f64,
    /// Turns of food in its pack under which an army abroad halts to forage.
    pub low: f64,
    /// People: the largest army that goes past its line's reach, living off the land.
    pub party: f64,
    /// How many of the nearest rival tiles an army weighs when it picks one to pillage.
    pub looks: usize,
    pub note: &'static str,
}

pub const PLAN: PlanRules = PlanRules {
    feed: 1.5,
    full: 0.75,
    reserve: 2.0,
    army: 0.25,
    gift_contested: 4.0,
    widen: 1,
    thick: 2.0,
    widen_reach: 3,
    edge: 1.5,
    siege: 8,
    left: 0.3,
    march: 3,
    raid: 3.0,
    reach: 0.5,
    packed: 0.8,
    low: 2.0,
    party: 4.0,
    looks: 12,
    note: "relay/specs/157/canvas.md",
};

/// The guard: the people a civ keeps on its claim at its capital (`units/guard.rs`, `claim.rs`).
pub struct GuardRules {
    /// Guards raised at founding, from the civ's clans, one person each. 0: a civ starts with its
    /// scout and nothing else, and raises its guard on its claim itself (`founding.md`, The start).
    pub start: u32,
    /// The guards a staged civ has (`StageSpec`): the guard founding gave a civ before the start moved.
    pub first: u32,
    /// Days a person a turn a guard puts into holding the claim: its whole labour (`UNIT.labour`), since a
    /// guard does nothing else. Divided over the claim with `inside`.
    pub guarding: f64,
    /// The weight of a guard's walk from the capital over the claim (`walk_cost`), paid by every guard
    /// walking a tile: the guard's people over the claim. The camp's `UNIT.inside`: the walk is a
    /// share of a person's days on a tile whatever the guard's size, so a weight a tenth of that left
    /// the walk free and one guard held the claim (the AI's raises and claims fell to a seventh; `walk.md`).
    pub inside: f64,
    /// Turns of a guard's food the civ keeps in its store out of posting and gifts (`civs::keep`).
    pub keep_turns: f64,
}

// Inside 0.2 (the camp's; 0.02, 0.1 and 0.15 left the AI's orders at 1.2 to 3.4 a hundred turns against the base's 8.3: walk.md)
// and keep_turns 1, so the store still gives (canvas.md).
pub const GUARD: GuardRules = GuardRules { start: 0, first: 3, guarding: 3.5 * DAY, inside: 0.2, keep_turns: 1.0 };

pub struct UnitRules {
    /// Days a person a turn: one person's work: what a camp spends on its steps, its foraging and its line (`CLAN.work`).
    pub labour: f64,
    /// Food a person a turn a warrior eats from the pack: one person's need.
    pub food: f64,
    /// Hungry turns in a row before the camp loses a warrior (and one more each turn after).
    pub hungry_turns: u32,
    /// The fewest people a clan must have to give one to the camp.
    pub min_clan: f64,
    /// Warriors raised at founding. 0: a civ starts with its scout and nothing else, and raises its
    /// army itself (`founding.md`, The start).
    pub start: u32,
    /// The warriors of a first army, where a script or the judge needs a number (`Lead`, the stories,
    /// the camp sweep's lever, the economy group's pack).
    pub first: u32,
    /// Turns of food (food a person a turn) a warrior's share of the pack holds.
    pub pack: f64,
    /// Food a day of line per day it costs (a line of cost `c` days carries
    /// `labour × carry / c` food).
    pub carry: f64,
    /// Food a day a unit takes from a tile it forages (own or unclaimed land).
    pub forage: f64,
    /// Food a day a unit takes from a rival's tile it pillages.
    pub pillage: f64,
    /// Turns a stripped tile yields nothing to anyone and regrows nothing.
    pub fallow: u32,
    /// Hex steps from the capital the AI posts its camp.
    pub post: usize,
    /// Days a person a turn a warrior sets aside to guard a deployed camp's zone, divided over its held tiles.
    pub guarding: f64,
    /// A share: the guard share a tile must keep after its walk to stay held (`divide`).
    pub guard: f64,
    /// The weight of a guard's walk inside the zone (`walk_cost`), as a clan's inside its territory.
    pub inside: f64,
    /// Tiles (hex steps) of its line the AI's zone covers past the camp's own, at most `people − 1`.
    pub zone: usize,
}

pub const UNIT: UnitRules = UnitRules {
    labour: 3.5 * DAY,
    food: 1.0,
    hungry_turns: 3,
    min_clan: 2.0,
    start: 0,
    first: 3,
    pack: 5.0,
    carry: 3.0,
    forage: 0.5 / DAY,
    pillage: 1.0 / DAY,
    fallow: 5,
    post: 3,
    guarding: 1.0 * DAY,
    guard: 0.1 * DAY,
    inside: 0.2,
    zone: 2,
};

/// The kinds of fighter an army's people are (`war.md`): its stack counts its people by kind, and
/// each kind indexes `FIGHTERS`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Fighter {
    #[default]
    Warriors,
    Archers,
}

impl Fighter {
    pub const ALL: [Fighter; 2] = [Fighter::Warriors, Fighter::Archers];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn rules(self) -> &'static FighterRules {
        &FIGHTERS[self.index()]
    }
}

/// One kind of fighter's strength per person.
pub struct FighterRules {
    /// Its name, plural, for notes and the view.
    pub name: &'static str,
    /// Strength per person on a tile its side didn't hold at the turn's start.
    pub attack: f64,
    /// Strength per person on a tile its side held at the turn's start.
    pub defence: f64,
    pub note: &'static str,
}

/// The kinds, indexed by `Fighter`.
pub const FIGHTERS: [FighterRules; 2] = [
    FighterRules {
        name: "warriors",
        attack: 1.0,
        defence: 1.0,
        note: "The measure of the others: a person with a club or spear, as good taking as holding.",
    },
    FighterRules {
        name: "archers",
        attack: 0.8,
        defence: 1.4,
        note: "Bows hold ground from behind cover better than they take it; the fight is over one tile, so no range.",
    },
];

/// The fight's rule (`units/fight.rs`): Lanchester-linear losses on each contested tile.
pub struct FightRules {
    /// People a side loses a turn per unit of the others' strength there.
    pub rate: f64,
    /// A defending army's strength gained per turn it has stood still.
    pub entrench: f64,
    /// The most entrenchment adds; a guard always defends at it.
    pub entrench_cap: f64,
    /// A side whose strength after the losses is under this times the others' retreats (armies only).
    pub retreat: f64,
    /// An army that went hungry last turn fights at this times its strength.
    pub hungry: f64,
    /// The fewest people a civ must have on a tile to be a side there.
    pub floor: f64,
    /// A militia person's strength, attacking or defending, times a warrior's defence.
    pub militia: f64,
    /// Hungry turns in a row, with its line cut by a rival, before an army surrenders (before hunger's take).
    pub surrender: u32,
    pub note: &'static str,
}

pub const FIGHT: FightRules = FightRules {
    rate: 0.2,
    entrench: 0.1,
    entrench_cap: 0.5,
    retreat: 0.5,
    hungry: 0.5,
    floor: 0.01,
    militia: 0.3,
    surrender: 3,
    note: "Tuned on small seed 2's stories (relay/specs/104/glaze.md): rate 0.2 makes the guards' skirmish \
           move Standing (Red +2.9, Blue -1.0 against the control) while a guard of 2-3 still lasts its \
           19 contested turns; the rest held, since a beaten stack still retreats a step and starves or \
           surrenders at surrender 3 (the stack's and surrender story's armies both do).",
};

/// Which civs may claim land (`claim.rs`): `All` (the default), `None` (the control: no civ claims
/// past its capital and raises no guard), or `Only(k)` (the k-th civ in id order, 0-based, claims as
/// under `All`; the others as under `None`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Claims {
    #[default]
    All,
    None,
    Only(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub seed: u32,
    pub radius: i32,
    /// Whether civs have armies: off is the control (no camp at founding, the AI raises none, no
    /// levy). Old saves load with it on.
    #[serde(default = "yes")]
    pub army: bool,
    /// Whether civs give gifts: off is the control (no civ gives, whatever its `gift`). Old saves
    /// load with it on.
    #[serde(default = "yes")]
    pub gifts: bool,
    /// Whether AI camps paint a zone over their line: off is the control (the sweep's `--no-zone`).
    /// Old saves load with it on.
    #[serde(default = "yes")]
    pub zone: bool,
    /// Which civs may claim land: see `Claims`. Old saves load with `All`.
    #[serde(default)]
    pub claims: Claims,
    /// Whether people of two civs on a tile fight (`units/fight.rs`): off is the control (no losses,
    /// no retreats, no militia, no surrender, no fall, no pressed capital and no siege; holding is
    /// the strength comparison alone). Old saves load with it on.
    #[serde(default = "yes")]
    pub fight: bool,
    /// The turn limit in turns after the pick, which replaces the size's (`END`); `None` is the size's.
    /// A stage, a test or a script sets a short one. Old saves load with `None`.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Whether clans farm, once civs are founded (`fields.rs`): off is the control, in which no clan
    /// farms. Old saves load with it on.
    #[serde(default = "yes")]
    pub fields: bool,
    /// Whether a farmer's home may step onto a tile another clan holds, where living feeds it better.
    /// Off is the control: a farmer's home never steps onto a tile another clan holds, so no two clans
    /// share a home. Old saves load with it on.
    #[serde(default = "yes")]
    pub towns: bool,
}

fn yes() -> bool {
    true
}

impl Default for Config {
    /// Radius 16: three centres 10 steps apart need the room.
    fn default() -> Self {
        Config {
            seed: 1,
            radius: 16,
            army: true,
            gifts: true,
            zone: true,
            claims: Claims::All,
            fight: true,
            limit: None,
            fields: true,
            towns: true,
        }
    }
}

/// How a game ends (`over.rs`): its turn limit, in turns after the pick, by map radius as `SIZES`. A
/// radius between sizes takes the next size up's; one past the largest takes the largest's.
pub struct EndRules {
    pub limit: [(i32, u32); 3],
    pub note: &'static str,
}

pub const END: EndRules = EndRules {
    limit: [(16, 200), (23, 250), (33, 300)],
    note: "Measured on the seven retro maps (relay/specs/152/limit.md). In time to matter: all on the AI to \
           +600, the judge's leader at the limit is the leader at +600 on six of seven maps at these \
           limits (the AI's lead settles by +73 on six maps; small 9's top two swap to +348, so only a \
           limit of 150 or 300 and up matches there, and it is a coin flip). Room: the conquest story's \
           best armies reach a rival capital by +21 on small and a capital falls by +76; on medium and \
           large no army reaches one, and the farthest is out by +87 and +37; each limit is 2.6 times \
           the latest of these or more.",
};

/// How many reasons (`World::note`) a subject keeps, and how many the selection panel shows.
pub struct NotesRules {
    /// The most reasons a subject keeps; the oldest go.
    pub kept: usize,
    /// The most reasons the panel shows, newest first.
    pub shown: usize,
}

pub const NOTES: NotesRules = NotesRules { kept: 8, shown: 5 };

/// The fewest people a civ's clans must send to clans not of it in a turn for `PeopleLeft` to report it.
pub const PEOPLE_LEFT_MIN: f64 = 0.1;

/// The judge's numbers (`judge.rs`): how far a civ must lead to take the lead, and the turns between
/// the samples the swings are counted on.
pub struct JudgeRules {
    /// Standing a civ must be ahead by to take the lead from the civ that has it, and a difference
    /// between samples of the spread must exceed to count as up or down. Two civs within half a point
    /// of each other swap places on noise: the medium maps count 7 and 8 such changes at 0 (probe.md).
    pub lead_margin: f64,
    /// Turns between the samples the swings are counted on.
    pub sample: u32,
    /// A clan's festival change back to one it had within this many turns is a flip. A fixed window,
    /// so a build with no `MARKET.keep_turns` counts as one with it.
    pub flip_window: u32,
    /// A civ's Standing gain from claiming alone (`sweep --judge --solo`) counts as a gain above this.
    /// It is the lead margin's size: a gain of less is noise in the Standing the judge reads.
    pub solo_above: f64,
}

pub const JUDGE: JudgeRules = JudgeRules { lead_margin: 0.005, sample: 10, flip_window: 10, solo_above: 0.005 };
