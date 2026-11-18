//! Civs: founding, their claim, leanings, signposts and settlements.

use super::*;

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

pub const CIV: CivRules = CivRules {
    endowment: [0.0, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    bridge_span: 1,
    levy: 0.2,
    store_cap: 60.0,
    pull: 3.0 * DAY,
};

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
    // Leaving after 3 turns drawn at 0.05: 5 turns at 0.1 left 2 to 4 clans over the seven retro maps with the
    // planner playing, 3 at 0.05 leaves 8 (relay/specs/157/canvas.md); 10/0.3 left none.
    leave_run: 3,
    leave_at: 0.05,
};

/// The land a civ claims (`claim.rs`): a civ's first claim is every land tile within `ring` steps of
/// the capital or of a clan's home within `reach` steps of it.
pub struct ClaimRules {
    /// Hex steps.
    pub ring: i32,
    /// Hex steps.
    pub reach: i32,
}

// Ring 1 and reach 4 keep the first claim near the capital and its clans (canvas.md).
pub const CLAIM: ClaimRules = ClaimRules { ring: 1, reach: 4 };

/// Signposts and settlements (`settle.rs`).
pub struct SettleRules {
    /// Signposts a civ keeps at a time.
    pub signposts: usize,
    /// Hex steps: a signpost stands at least this far from every capital and settlement.
    pub spacing: i32,
    /// Days: the longest walk from a clan's home to a signpost it would go to.
    pub reach: f64,
    /// The chance a daughter, or a clan moving on, goes to a signpost at its door that would feed it in
    /// full; it falls with the walk and the tile's worth.
    pub draw: f64,
    /// Days: the walk over which the chance falls to 1/e.
    pub walk: f64,
    /// A signpost's tile must feed the clan this well for it to go.
    pub fed: f64,
    /// The most goods, in all, a settlement's store holds; the excess is lost.
    pub store_cap: f64,
    /// Hex steps from its capital or a settlement within which the planner posts.
    pub plan_range: i32,
    /// Turns between the planner's looks for a site.
    pub plan_every: u32,
    /// Turns a signpost nobody has settled stands before the planner removes it.
    pub plan_stale: u32,
    /// Why these numbers: where they came from.
    pub note: &'static str,
}

pub const SETTLE: SettleRules = SettleRules {
    signposts: 3,
    spacing: 4,
    reach: 14.0 * DAY,
    draw: 0.8,
    walk: 7.0 * DAY,
    fed: 0.9,
    store_cap: 30.0,
    plan_range: 8,
    plan_every: 5,
    plan_stale: 40,
    note: "Three signposts are a few. A clan walks as far to a signpost as to a festival (`MARKET.reach_walk`), \
           and the site must feed it as a daughter's own site must (`CLAN.settle_fed`). A settlement's store \
           holds half the capital's. Spacing 4 keeps a settlement's land clear of the capital's first claim. \
           The draw and the walk are first numbers, not yet tuned on the retro maps (relay/specs/353/scores.md).",
};

/// The fewest people a civ's clans must send to clans not of it in a turn for `PeopleLeft` to report it.
pub const PEOPLE_LEFT_MIN: f64 = 0.1;
