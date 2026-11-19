//! Clans: the market they bring to a festival, the pre-game and a clan's rules, and the field and town readings.

use super::*;

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

/// Settling (`settling.rs`): a way of life a clan takes up and leaves, read off its field share. It is a
/// reading: no rule reads it.
pub struct SettlingRules {
    /// The field share from which a turn counts toward settling.
    pub farms: f64,
    /// Turns in a row at `farms` or more after which a clan settles.
    pub settle_after: u32,
    /// The field share under which a turn counts toward going back to gathering.
    pub gathers: f64,
    /// Turns in a row under `gathers` after which a settled clan goes back to gathering.
    pub leave_after: u32,
    /// Why these numbers.
    pub note: &'static str,
}

pub const SETTLING: SettlingRules = SettlingRules {
    farms: 0.2,
    settle_after: 10,
    gathers: 0.1,
    leave_after: 10,
    note: "A clan settles once it has been a farmer (`FIELD.farmer`, a fifth of its days) for some turns in a row, \
           and goes back once under a tenth for some turns; between the two it stays as it is. The turns are \
           tuned on the retro maps so that a clan's state changes rarely (relay/specs/427/sweep.md).",
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
