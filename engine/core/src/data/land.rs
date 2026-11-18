//! Land under use: the trail's wear, the land's depletion and the tending that answers it.

use super::*;

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
